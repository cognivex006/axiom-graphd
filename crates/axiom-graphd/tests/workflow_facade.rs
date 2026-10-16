//! Real native facade boundaries; no graph or publication is mocked.
use std::path::Path;
use std::process::Command;

fn call(home: &Path, words: &[&str]) -> serde_json::Value {
    let result = Command::new(env!("CARGO_BIN_EXE_axiom-graphd"))
        .env("AXIOM_HOME", home)
        .args(words)
        .arg("--json")
        .output()
        .expect("engine launch");
    assert!(
        result.status.success(),
        "{} {}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    serde_json::from_slice(&result.stdout).expect("exact JSON result")
}

#[test]
fn strict_facade_arguments_and_no_natural_language_shell() {
    use axiom_graphd::cli::{parse, Command as Parsed};
    for words in [
        vec!["build", "--solution", "demo"],
        vec!["watch"],
        vec!["export", "--solution", "demo", "--out", "new"],
    ] {
        let argv: Vec<String> = words.into_iter().map(str::to_owned).collect();
        assert!(!matches!(parse(&argv).command(), Parsed::Rejected { .. }));
    }
    for words in [
        vec!["ask", "who calls PingPong"],
        vec!["watch", "--project", "app"],
        vec!["build"],
        vec![
            "export",
            "--solution",
            "demo",
            "--out",
            "new",
            "--format",
            "shell",
        ],
    ] {
        let argv: Vec<String> = words.into_iter().map(str::to_owned).collect();
        assert!(matches!(parse(&argv).command(), Parsed::Rejected { .. }));
    }
}

#[test]
fn scoped_build_advances_complete_catalog_and_preserves_unselected_generations() {
    let dir = tempfile::tempdir().expect("fixture");
    #[cfg(not(windows))]
    let root = std::fs::canonicalize(dir.path()).expect("canonical fixture");
    #[cfg(windows)]
    let root = dir.path().to_path_buf();
    let repo = root.join("repo");
    for project in ["a", "b", "c"] {
        let folder = repo.join("src").join(project);
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(folder.join("Example.cs"), format!("namespace Multi;\npublic class Project{project}Needle {{\n public void Run() {{}}\n}}\n")).unwrap();
    }
    let config = root.join("solution.json");
    std::fs::write(&config, serde_json::to_vec(&serde_json::json!({"id":"multi", "profile":"default", "catalog_host_repo":"primary", "projects":[
        {"id":"a","repo_id":"primary","path":"src/a"},
        {"id":"b","repo_id":"primary","path":"src/b"},
        {"id":"c","repo_id":"primary","path":"src/c"}]})).unwrap()).unwrap();
    let bindings = root.join("bindings.json");
    std::fs::write(
        &bindings,
        serde_json::to_vec(&serde_json::json!({"bindings":{"primary":repo}})).unwrap(),
    )
    .unwrap();
    let home = root.join("home");
    call(
        &home,
        &[
            "solution",
            "register",
            "--config",
            config.to_str().unwrap(),
            "--bindings",
            bindings.to_str().unwrap(),
            "--apply",
        ],
    );
    call(&home, &["build", "--solution", "multi"]);
    let catalog = repo.join(".axiom/graph/multi/_catalog/live");
    let vector = || {
        let pointer: serde_json::Value =
            serde_json::from_slice(&std::fs::read(catalog.join("current.json")).unwrap()).unwrap();
        let manifest: serde_json::Value = serde_json::from_slice(
            &std::fs::read(
                catalog
                    .join("generations")
                    .join(pointer["generation_id"].as_str().unwrap())
                    .join("manifest.json"),
            )
            .unwrap(),
        )
        .unwrap();
        manifest["projects"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| {
                (
                    p["project_id"].as_str().unwrap().to_string(),
                    p["generation_id"].as_str().unwrap().to_string(),
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>()
    };
    let before = vector();
    assert_eq!(before.len(), 3);
    std::fs::write(
        repo.join("src/c/Example.cs"),
        "namespace Multi;\npublic class ChangedNeedle {\n public void Run() {}\n}\n",
    )
    .unwrap();
    let result = call(&home, &["build", "--solution", "multi", "--project", "c"]);
    assert_eq!(result["projects"], 1);
    let after = vector();
    assert_eq!(after.len(), 3);
    assert_eq!(before["a"], after["a"]);
    assert_eq!(before["b"], after["b"]);
    assert_ne!(before["c"], after["c"]);
    let pointer_before = std::fs::read(catalog.join("current.json")).unwrap();
    let unselected = repo
        .join(".axiom/graph/multi/b/live/generations")
        .join(&after["b"])
        .join("manifest.json");
    let mut bytes = std::fs::read(&unselected).unwrap();
    bytes.push(b' ');
    std::fs::write(unselected, bytes).unwrap();
    let failed = Command::new(env!("CARGO_BIN_EXE_axiom-graphd"))
        .env("AXIOM_HOME", &home)
        .args(["build", "--solution", "multi", "--project", "c", "--json"])
        .output()
        .unwrap();
    assert!(!failed.status.success());
    assert_eq!(
        std::fs::read(catalog.join("current.json")).unwrap(),
        pointer_before
    );
}

#[test]
fn build_noop_full_export_and_refuse_overwrite() {
    let dir = tempfile::tempdir().expect("temporary fixture");
    // macOS temporary roots can start at /var, a symlink to /private/var.
    // Resolve the owned fixture root without weakening runtime link refusal.
    #[cfg(not(windows))]
    let root = std::fs::canonicalize(dir.path()).expect("canonical fixture root");
    #[cfg(windows)]
    let root = dir.path().to_path_buf();
    let repo = root.join("repo");
    std::fs::create_dir_all(repo.join("src")).expect("source root");
    std::fs::write(
        repo.join("src/Example.cs"),
        "namespace Example;\npublic class PingPong {\n public void RemoveFilesAsync() {}\n}\n",
    )
    .expect("source");
    let config = root.join("solution.json");
    std::fs::write(&config, r#"{"id":"demo","profile":"default","catalog_host_repo":"primary","projects":[{"id":"app","repo_id":"primary","path":"src"}]}"#).expect("config");
    let bindings = root.join("bindings.json");
    std::fs::write(
        &bindings,
        serde_json::to_vec(&serde_json::json!({"bindings":{"primary":repo}})).expect("JSON"),
    )
    .expect("bindings");
    let home = root.join("home");
    call(
        &home,
        &[
            "solution",
            "register",
            "--config",
            config.to_str().expect("path"),
            "--bindings",
            bindings.to_str().expect("path"),
            "--apply",
        ],
    );
    let first = call(&home, &["build", "--solution", "demo"]);
    assert!(first["published_generations"].as_u64().expect("count") > 0);
    assert!(first["cycle"]["processing_ms"].is_u64());
    let second = call(&home, &["build", "--solution", "demo"]);
    assert_eq!(second["published_generations"], 0);
    assert_eq!(
        first["cycle"]["generation_after"],
        second["cycle"]["generation_after"]
    );
    let full = call(&home, &["build", "--solution", "demo", "--full"]);
    assert!(full["cycle"]["files_analyzed"].as_u64().expect("count") > 0);
    let out = root.join("export");
    let exported = call(
        &home,
        &[
            "export",
            "--solution",
            "demo",
            "--project",
            "app",
            "--out",
            out.to_str().expect("path"),
        ],
    );
    assert_eq!(exported["format"], "json");
    let bytes = std::fs::read(out.join("graph.json")).expect("export artifact");
    let artifact: serde_json::Value = serde_json::from_slice(&bytes).expect("artifact JSON");
    assert_eq!(
        artifact["selection"]["projects"],
        serde_json::json!(["app"])
    );
    assert!(
        artifact["projects"][0]["nodes"]
            .as_array()
            .expect("nodes")
            .len()
            > 1
    );
    let result = Command::new(env!("CARGO_BIN_EXE_axiom-graphd"))
        .env("AXIOM_HOME", &home)
        .args([
            "export",
            "--solution",
            "demo",
            "--out",
            out.to_str().expect("path"),
            "--json",
        ])
        .output()
        .expect("launch");
    assert!(!result.status.success());
    assert_eq!(
        std::fs::read(out.join("graph.json")).expect("preserved"),
        bytes
    );
    let node = artifact["projects"][0]["nodes"][0]["id"]
        .as_str()
        .expect("observed node");
    let selected = root.join("selected");
    call(
        &home,
        &[
            "export",
            "--solution",
            "demo",
            "--project",
            "app",
            "--node-id",
            node,
            "--format",
            "html",
            "--out",
            selected.to_str().expect("path"),
        ],
    );
    let filtered: serde_json::Value =
        serde_json::from_slice(&std::fs::read(selected.join("graph.json")).expect("JSON artifact"))
            .expect("JSON");
    assert_eq!(
        filtered["projects"][0]["nodes"]
            .as_array()
            .expect("nodes")
            .len(),
        1
    );
    assert_eq!(filtered["completeness"], "filtered_projection");
    let summary: serde_json::Value = serde_json::from_slice(
        &std::fs::read(selected.join("summary.json")).expect("HTML summary"),
    )
    .expect("JSON");
    assert_eq!(summary["nodes"].as_array().expect("drawn nodes").len(), 1);
    let old_catalog = exported["catalog_generation_id"]
        .as_str()
        .expect("catalog identity");
    std::fs::write(
        repo.join("src/Example.cs"),
        "namespace Example;\npublic class AfterPin {}\n",
    )
    .expect("new source");
    call(&home, &["build", "--solution", "demo"]);
    let pinned = root.join("pinned-selection");
    let old = call(
        &home,
        &[
            "export",
            "--solution",
            "demo",
            "--catalog-generation",
            old_catalog,
            "--node-id",
            node,
            "--exact-projection",
            "--out",
            pinned.to_str().expect("path"),
        ],
    );
    assert_eq!(old["catalog_generation_id"], old_catalog);
    let old_artifact: serde_json::Value =
        serde_json::from_slice(&std::fs::read(pinned.join("graph.json")).expect("pinned bytes"))
            .expect("JSON");
    assert!(old_artifact["projects"][0]["edges"]
        .as_array()
        .expect("exact edges")
        .is_empty());
    assert_eq!(
        old_artifact["projects"][0]["generation_id"],
        artifact["projects"][0]["generation_id"]
    );
    let selected_vector = format!(
        "app={}",
        artifact["projects"][0]["generation_id"]
            .as_str()
            .expect("observed member")
    );
    let vector_out = root.join("vector-selected");
    let vector_receipt = call(
        &home,
        &[
            "export",
            "--solution",
            "demo",
            "--project-generation",
            &selected_vector,
            "--out",
            vector_out.to_str().expect("path"),
        ],
    );
    assert_eq!(
        vector_receipt["generation_vector"]["app"],
        artifact["projects"][0]["generation_id"]
    );
    let generation = full["outcomes"][0]["generation_id"]
        .as_str()
        .expect("generation");
    let shard = repo
        .join(".axiom/graph/demo/app/live/generations")
        .join(generation)
        .join("nodes/000000.json");
    std::fs::write(shard, b"[]\n").expect("corrupt member fixture");
    let rejected = root.join("corrupt-export");
    let result = Command::new(env!("CARGO_BIN_EXE_axiom-graphd"))
        .env("AXIOM_HOME", &home)
        .args([
            "export",
            "--solution",
            "demo",
            "--catalog-generation",
            old_catalog,
            "--out",
            rejected.to_str().expect("path"),
            "--json",
        ])
        .output()
        .expect("launch");
    assert!(!result.status.success());
    assert!(
        !rejected.exists(),
        "corrupt members must be refused before artifact creation"
    );
}

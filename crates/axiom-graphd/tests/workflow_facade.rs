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
fn build_noop_full_export_and_refuse_overwrite() {
    let dir = tempfile::tempdir().expect("temporary fixture");
    let repo = dir.path().join("repo");
    std::fs::create_dir_all(repo.join("src")).expect("source root");
    std::fs::write(
        repo.join("src/Example.cs"),
        "namespace Example;\npublic class PingPong {\n public void RemoveFilesAsync() {}\n}\n",
    )
    .expect("source");
    let config = dir.path().join("solution.json");
    std::fs::write(&config, r#"{"id":"demo","profile":"default","catalog_host_repo":"primary","projects":[{"id":"app","repo_id":"primary","path":"src"}]}"#).expect("config");
    let bindings = dir.path().join("bindings.json");
    std::fs::write(
        &bindings,
        serde_json::to_vec(&serde_json::json!({"bindings":{"primary":repo}})).expect("JSON"),
    )
    .expect("bindings");
    let home = dir.path().join("home");
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
    let out = dir.path().join("export");
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
    let selected = dir.path().join("selected");
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
    let pinned = dir.path().join("pinned-selection");
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
    let vector_out = dir.path().join("vector-selected");
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
    let rejected = dir.path().join("corrupt-export");
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

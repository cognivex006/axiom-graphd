//! ADR-0047: `config/registry.json` is the solution registry MCP reads, written
//! by registration; daemon settings live in `config/daemon.json`.
use std::path::{Path, PathBuf};
use std::process::Command;

fn run(home: &Path, words: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_axiom-graphd"))
        .env("AXIOM_HOME", home)
        .args(words)
        .arg("--json")
        .output()
        .expect("engine launch")
}

struct Fixture {
    _dir: tempfile::TempDir,
    home: PathBuf,
    repo: PathBuf,
    config: PathBuf,
    bindings: PathBuf,
}

fn fixture() -> Fixture {
    let dir = tempfile::tempdir().unwrap();
    #[cfg(not(windows))]
    let root = std::fs::canonicalize(dir.path()).unwrap();
    #[cfg(windows)]
    let root = dir.path().to_path_buf();
    let repo = root.join("repo");
    std::fs::create_dir_all(&repo).unwrap();
    std::fs::write(repo.join("A.cs"), "namespace N;\npublic class A {}\n").unwrap();
    let config = root.join("solution.json");
    std::fs::write(
        &config,
        serde_json::to_vec(&serde_json::json!({
            "id": "my-app",
            "profile": "default",
            "catalog_host_repo": "my-app-repo",
            "projects": [{"id": "app", "repo_id": "my-app-repo", "path": "."}]
        }))
        .unwrap(),
    )
    .unwrap();
    let bindings = root.join("bindings.json");
    std::fs::write(
        &bindings,
        serde_json::to_vec(&serde_json::json!({"bindings": {"my-app-repo": repo}})).unwrap(),
    )
    .unwrap();
    Fixture {
        home: root.join("home"),
        _dir: dir,
        repo,
        config,
        bindings,
    }
}

fn register(f: &Fixture) -> std::process::Output {
    run(
        &f.home,
        &[
            "solution",
            "register",
            "--config",
            f.config.to_str().unwrap(),
            "--bindings",
            f.bindings.to_str().unwrap(),
            "--apply",
        ],
    )
}

fn ok(output: &std::process::Output) {
    assert!(
        output.status.success(),
        "{} {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn register_writes_the_solution_registry_and_remove_rewrites_it() {
    let f = fixture();
    let dry = run(
        &f.home,
        &[
            "solution",
            "register",
            "--config",
            f.config.to_str().unwrap(),
            "--bindings",
            f.bindings.to_str().unwrap(),
            "--dry-run",
        ],
    );
    ok(&dry);
    assert!(!f.home.join("config/registry.json").exists());
    ok(&register(&f));
    let registry: serde_json::Value =
        serde_json::from_slice(&std::fs::read(f.home.join("config/registry.json")).unwrap())
            .unwrap();
    assert_eq!(registry["schema_version"], 1);
    let solution = &registry["solutions"][0];
    assert_eq!(solution["solution_id"], "my-app");
    assert_eq!(solution["catalog_host_repo"], "my-app-repo");
    assert_eq!(
        registry["instances"][0]["instance_id"],
        solution["instance_id"]
    );
    let repo = &solution["repositories"][0];
    assert_eq!(repo["repo_id"], "my-app-repo");
    assert_eq!(repo["repo_root"], f.repo.to_str().unwrap());
    assert_eq!(repo["projects"][0]["project_id"], "app");
    // The daemon still starts its operator verbs with a solution registry present.
    ok(&run(&f.home, &["solution", "list"]));
    ok(&run(
        &f.home,
        &["solution", "remove", "--solution", "my-app", "--apply"],
    ));
    let after: serde_json::Value =
        serde_json::from_slice(&std::fs::read(f.home.join("config/registry.json")).unwrap())
            .unwrap();
    assert_eq!(after["solutions"], serde_json::json!([]));
}

#[test]
fn legacy_daemon_settings_move_to_daemon_json_and_are_kept() {
    let f = fixture();
    let settings = r#"{"storage":{"journalMode":"DELETE"}}"#;
    std::fs::create_dir_all(f.home.join("config")).unwrap();
    std::fs::write(f.home.join("config/registry.json"), settings).unwrap();
    ok(&register(&f));
    assert_eq!(
        std::fs::read_to_string(f.home.join("config/daemon.json")).unwrap(),
        settings
    );
    let registry = std::fs::read_to_string(f.home.join("config/registry.json")).unwrap();
    assert!(registry.contains("\"schema_version\": 1"));
}

#[test]
fn conflicting_daemon_settings_are_refused_not_overwritten() {
    let f = fixture();
    std::fs::create_dir_all(f.home.join("config")).unwrap();
    let legacy = r#"{"storage":{"journalMode":"DELETE"}}"#;
    let current = r#"{"storage":{"journalMode":"WAL"}}"#;
    std::fs::write(f.home.join("config/registry.json"), legacy).unwrap();
    std::fs::write(f.home.join("config/daemon.json"), current).unwrap();
    let refused = register(&f);
    assert!(!refused.status.success());
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&refused.stdout),
        String::from_utf8_lossy(&refused.stderr)
    );
    assert!(text.contains("legacy_daemon_config_location"), "{text}");
    assert_eq!(
        std::fs::read_to_string(f.home.join("config/registry.json")).unwrap(),
        legacy
    );
    assert_eq!(
        std::fs::read_to_string(f.home.join("config/daemon.json")).unwrap(),
        current
    );
}

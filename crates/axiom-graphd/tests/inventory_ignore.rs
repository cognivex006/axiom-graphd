//! ADR-0048: Git ignore rules exclude untracked files, solution `ignore` globs
//! exclude matching files even when tracked, and the built-in list stays.
use std::path::Path;
use std::process::Command;

fn run(home: &Path, words: &[&str]) -> serde_json::Value {
    let output = Command::new(env!("CARGO_BIN_EXE_axiom-graphd"))
        .env("AXIOM_HOME", home)
        .args(words)
        .arg("--json")
        .output()
        .expect("engine launch");
    assert!(
        output.status.success(),
        "{} {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("one JSON object")
}

fn git(repo: &Path, args: &[&str]) {
    let status = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .status()
        .expect("git is required for this test");
    assert!(status.success(), "git {args:?}");
}

fn write(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

#[test]
fn git_ignored_untracked_and_solution_globs_are_excluded_and_counted() {
    let dir = tempfile::tempdir().unwrap();
    #[cfg(not(windows))]
    let root = std::fs::canonicalize(dir.path()).unwrap();
    #[cfg(windows)]
    let root = dir.path().to_path_buf();
    let repo = root.join("repo");
    write(
        &repo.join("src/App.cs"),
        "namespace N;\npublic class App {}\n",
    );
    write(&repo.join(".gitignore"), "cache/\n*.secret\n");
    write(&repo.join("agent/notes.md"), "tracked but not code\n");
    write(&repo.join("cache/blob.bin"), "untracked and ignored\n");
    write(&repo.join("token.secret"), "untracked and ignored\n");
    git(&repo, &["init", "-q"]);
    git(&repo, &["add", "src", ".gitignore", "agent"]);
    let config = root.join("solution.json");
    std::fs::write(
        &config,
        serde_json::to_vec(&serde_json::json!({
            "id": "app",
            "profile": "default",
            "catalog_host_repo": "app-repo",
            "ignore": ["agent/**"],
            "projects": [{"id": "app", "repo_id": "app-repo", "path": "."}]
        }))
        .unwrap(),
    )
    .unwrap();
    let bindings = root.join("bindings.json");
    std::fs::write(
        &bindings,
        serde_json::to_vec(&serde_json::json!({"bindings": {"app-repo": repo}})).unwrap(),
    )
    .unwrap();
    let home = root.join("home");
    run(
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
    let built = run(&home, &["build", "--solution", "app"]);
    let outcome = &built["outcomes"][0];
    let excluded = &outcome["inventory_excluded"];
    assert!(
        excluded["solution_ignore"].as_u64().unwrap() >= 1,
        "{excluded}"
    );
    assert!(excluded["git_ignored"].as_u64().unwrap() >= 2, "{excluded}");
    assert!(excluded["builtin"].as_u64().unwrap() >= 1, "{excluded}");
    // Only src/App.cs and .gitignore remain as inputs.
    assert_eq!(outcome["inventory_added"], 2, "{outcome}");
}

#[test]
fn ignore_globs_must_be_repository_relative() {
    let dir = tempfile::tempdir().unwrap();
    let config = dir.path().join("solution.json");
    for glob in ["../outside/**", "C:/abs/**", "a\\b"] {
        std::fs::write(
            &config,
            serde_json::to_vec(&serde_json::json!({
                "id": "app",
                "ignore": [glob],
                "projects": [{"id": "app", "repo_id": "app-repo", "path": "."}]
            }))
            .unwrap(),
        )
        .unwrap();
        let document: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&config).unwrap()).unwrap();
        assert!(
            axiom_graphd::commands::solution::parse_config(&document).is_err(),
            "{glob} must be refused"
        );
    }
}

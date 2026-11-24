//! Field report #14: C# call sites become published `CALLS` edges. A unique
//! cross-file match is `inferred_static`; anything else stays `unresolved`.
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

fn published_edges(
    lane: &Path,
) -> (
    Vec<serde_json::Value>,
    std::collections::BTreeMap<String, String>,
) {
    let mut edges = Vec::new();
    let mut names = std::collections::BTreeMap::new();
    let mut stack = vec![lane.join("generations")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let Ok(value) =
                serde_json::from_slice::<serde_json::Value>(&std::fs::read(&path).unwrap())
            else {
                continue;
            };
            let records = value
                .get("records")
                .or_else(|| value.get("edges"))
                .or_else(|| value.get("nodes"))
                .cloned()
                .unwrap_or(value);
            for record in records.as_array().cloned().unwrap_or_default() {
                if record.get("source_id").is_some() {
                    edges.push(record);
                } else if let Some(q) = record.get("qualified_name").and_then(|q| q.as_str()) {
                    names.insert(record["id"].as_str().unwrap().to_owned(), q.to_owned());
                }
            }
        }
    }
    (edges, names)
}

#[test]
fn typed_and_same_type_calls_resolve_and_others_stay_unresolved() {
    let dir = tempfile::tempdir().unwrap();
    #[cfg(not(windows))]
    let root = std::fs::canonicalize(dir.path()).unwrap();
    #[cfg(windows)]
    let root = dir.path().to_path_buf();
    let repo = root.join("repo");
    std::fs::create_dir_all(&repo).unwrap();
    std::fs::write(
        repo.join("Repo.cs"),
        "namespace App;\npublic interface IRepo\n{\n    int Load(int id);\n}\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("UseCase.cs"),
        "namespace App;\npublic class UseCase\n{\n    private readonly IRepo _repo;\n    public UseCase(IRepo repo) { _repo = repo; }\n    public int Run(int id)\n    {\n        var value = _repo.Load(id);\n        Audit(value);\n        return System.Math.Max(value, 0);\n    }\n    private void Audit(int value)\n    {\n    }\n}\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("SqlRepo.cs"),
        "namespace App;
public class SqlRepo : RepoBase, IRepo
{
    public int Load(int id) => id;
}
public class RepoBase : ControllerBase
{
}
",
    )
    .unwrap();
    let config = root.join("solution.json");
    std::fs::write(
        &config,
        serde_json::to_vec(&serde_json::json!({
            "id": "app", "catalog_host_repo": "app-repo",
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
    run(&home, &["build", "--solution", "app"]);
    let (edges, names) = published_edges(&repo.join(".axiom/graph/app/app/live"));
    let calls: Vec<(String, String, String)> = edges
        .iter()
        .filter(|e| e["kind"] == "CALLS")
        .map(|e| {
            let target = e
                .get("target_id")
                .and_then(|t| t.as_str())
                .map(|t| names[t].clone())
                .unwrap_or_else(|| e["unresolved_target"].as_str().unwrap().to_owned());
            (
                names[e["source_id"].as_str().unwrap()].clone(),
                target,
                e["resolution"].as_str().unwrap().to_owned(),
            )
        })
        .collect();
    assert!(
        calls.contains(&(
            "App.UseCase.Run".into(),
            "App.IRepo.Load".into(),
            "inferred_static".into()
        )),
        "{calls:?}"
    );
    // The call through IRepo also reaches the one implementation, as a
    // separate inferred edge; the interface edge stays.
    assert!(
        calls.contains(&(
            "App.UseCase.Run".into(),
            "App.SqlRepo.Load".into(),
            "inferred_static".into()
        )),
        "{calls:?}"
    );
    assert!(
        calls.contains(&(
            "App.UseCase.Run".into(),
            "App.UseCase.Audit".into(),
            "inferred_static".into()
        )),
        "{calls:?}"
    );
    assert!(
        calls
            .iter()
            .any(|(s, t, r)| s == "App.UseCase.Run" && t.ends_with("Max") && r == "unresolved"),
        "{calls:?}"
    );
    assert!(
        !calls.iter().any(|(_, _, r)| r == "exact_static"),
        "a lexical scan never claims exact_static"
    );
    let types: Vec<(String, String, String, String)> = edges
        .iter()
        .filter(|e| e["kind"] == "IMPLEMENTS" || e["kind"] == "INHERITS")
        .map(|e| {
            let target = e
                .get("target_id")
                .and_then(|t| t.as_str())
                .map(|t| names[t].clone())
                .unwrap_or_else(|| e["unresolved_target"].as_str().unwrap().to_owned());
            (
                names[e["source_id"].as_str().unwrap()].clone(),
                e["kind"].as_str().unwrap().to_owned(),
                target,
                e["resolution"].as_str().unwrap().to_owned(),
            )
        })
        .collect();
    for expected in [
        ("App.SqlRepo", "IMPLEMENTS", "App.IRepo", "inferred_static"),
        ("App.SqlRepo", "INHERITS", "App.RepoBase", "inferred_static"),
        ("App.RepoBase", "INHERITS", "ControllerBase", "unresolved"),
    ] {
        let wanted = (
            expected.0.to_owned(),
            expected.1.to_owned(),
            expected.2.to_owned(),
            expected.3.to_owned(),
        );
        assert!(types.contains(&wanted), "{wanted:?} missing from {types:?}");
    }
}

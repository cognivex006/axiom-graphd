# Solutions test scenario

Internal behavioral fixture, replayed by the Rust integration suite. This is synthetic test input, not captured command output or an installation guide.

<!-- BEGIN SOLUTIONS EXAMPLE -->
```json
{
  "schema_version": 2,
  "solution_id": "demo-solution",
  "profile": "default",
  "catalog_host_repo": "repo-alpha",
  "repositories": [
    { "repo_id": "repo-alpha", "binding_key": "alpha" },
    { "repo_id": "repo-beta", "binding_key": "beta" }
  ],
  "projects": [
    { "project_id": "proj-app", "repo_id": "repo-alpha", "path": "src/App", "language_profile": "csharp-dotnet" },
    { "project_id": "proj-portal", "repo_id": "repo-alpha", "path": "src/Portal", "language_profile": "typescript-angular" },
    { "project_id": "proj-jobs", "repo_id": "repo-beta", "path": "services/Jobs", "language_profile": "csharp-dotnet" },
    { "project_id": "proj-tools", "repo_id": "repo-beta", "path": "tools/GraphTools", "language_profile": "mixed", "allow_untracked": true }
  ],
  "graph_output_root": ".axiom/graph",
  "ignore": [],
  "watch": { "debounce_ms": 250, "max_wait_ms": 2000, "inventory_interval_seconds": 300, "backend": "auto" },
  "queue": { "worker_limit": 2, "max_batch_files": 256, "max_batch_bytes": 33554432, "retry_limit": 3 },
  "semantic_links": [],
  "workspace_layout": 2
}
```
<!-- END SOLUTIONS EXAMPLE -->

The portable model uses a `binding_key`; the internal AXIOM_HOME/config/registry.json location is distinct from the native registration input and persisted config/bindings.json.

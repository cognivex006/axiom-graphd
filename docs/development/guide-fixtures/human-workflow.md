# Human Workflow test scenario

Internal behavioral fixture, replayed by the Rust integration suite. This is synthetic test input, not captured command output or an installation guide.

<!-- BEGIN HUMAN WORKFLOW EXAMPLE -->
```json
{
  "schema_version": 2,
  "session_id": "demo-session",
  "debounce_ms": 250,
  "max_wait_ms": 2000,
  "save_burst": ["src/App.cs", "src/Widget.cs", "src/App.cs", "src/App.cs", "src/Widget.cs"],
  "rename": { "from": "src/Old.cs", "to": "src/New.cs" },
  "case_only_rename": { "from": "src/App.cs", "to": "src/app.cs" },
  "generated_output_root": "live",
  "formatter_writes": ["live/generations/shard-0000.json", "src/App.cs"],
  "offline_edits": {
    "modified": ["src/App.cs"],
    "created": ["src/Added.cs"],
    "removed": ["src/Removed.cs"]
  },
  "backend_preference": "prefer_native",
  "manual_file_submissions_required": 0
}
```
<!-- END HUMAN WORKFLOW EXAMPLE -->

The portable model uses a `binding_key`; the internal AXIOM_HOME/config/registry.json location is distinct from the native registration input and persisted config/bindings.json.

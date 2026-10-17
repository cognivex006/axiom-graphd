# Engine command reference

Use `axge` from an installed Axiom runtime. The underlying executable is named
`axiom-graphd`. Examples below use the public launcher. Start with the
[quickstart](../21-INSTALLATION-QUICKSTART.md) for working configuration files.

## Output and scope

`--json` writes one result object to stdout. Diagnostics and work-cycle records
go to stderr. Commands reject unknown flags rather than silently ignoring them.
Pass solution/project IDs from your registration; a source filename is not a solution ID.

<!-- BEGIN COMMAND STATUS -->
| Command | State | Purpose |
|---|---|---|
| build | available | Refresh source and publish a graph |
| watch | available | Refresh registered source continuously |
| export | available | Write a published snapshot to a new directory |
| help | available | Show the installed command surface |
| version | available | Identify the engine and runtime |
| doctor | available | Inspect health without automatic repair |
| serve | available | Run the lower-level foreground daemon |
| status | available | Inspect graph and queued-work state |
| solution | available | Preview and manage registered scope |
| changed | available | Submit an explicit changed-file hint |
| queue | available | Inspect and steer queued work |
| reconcile | available | Run a bounded source pass |
| query | available | Read context or impact from a published graph |
| render | available | Render a published relationship diagram |
| update | available | Delegate to the trusted runtime manager |
<!-- END COMMAND STATUS -->

## 1. `build`

Refresh source and publish a graph.

```text
axge build --solution my-app [--project app] [--full] [--json]
```

Requires a registered solution. The default is incremental; unchanged source is a no-op. `--full` deliberately invalidates selected source facts. Inspect publication and status before treating work as current.

## 2. `watch`

Refresh registered source continuously.

```text
axge watch [--registry PATH] [--json]
```

Foreground polling for every registration in the selected home. There is no project-only watch flag. Ctrl+C requests shutdown. Reuse an existing writer rather than starting a second one.

## 3. `export`

Write a published snapshot to a new directory.

```text
axge export --solution my-app [--project app]... --out NEW-DIRECTORY [--format json|html] [--json]
```

JSON is the default. HTML also writes `result.html` and `summary.json`. Export validates a pinned catalog and does not refresh source. An existing destination is refused. See the [export guide](../guides/export.md) for snapshot selectors.

## 4. `help`

Show the installed command surface.

```text
axge --help
```

No arguments, `help`, `--help` and `-h` show help. Use the installed help when checking an older release.

## 5. `version`

Identify the engine and runtime.

```text
axge version [--json]
```

`--version` and `-V` also identify the engine. This is installed-version information, not a remote update check.

## 6. `doctor`

Inspect health without automatic repair.

```text
axge doctor [--solution my-app] [--json]
```

Reports observed runtime and optional solution health. Diagnosis is separate from source freshness, language coverage and agent connectivity.

## 7. `serve`

Run the lower-level foreground daemon.

```text
axge serve [--registry PATH] [--json]
```

Requires registered source and local bindings. Takes the instance writer claim and processes source until cooperative shutdown. Prefer `watch` for the ordinary editing workflow.

## 8. `status`

Inspect graph and queued-work state.

```text
axge status --solution my-app [--json]
```

Requires a solution ID. Reports queue, dirty-source and generation state. Pending work and a readable older snapshot can coexist.

## 9. `solution`

Preview and manage registered scope.

```text
axge solution register --config solution.json --bindings bindings.json (--dry-run|--apply) [--json]
axge solution list [--solution my-app] [--json]
axge solution remove --solution my-app (--dry-run|--apply) [--json]
```

Register and remove require exactly one of `--dry-run` or `--apply`. Preview first. Local bindings map repository IDs to absolute local folders. See [solutions](../guides/solutions.md).

## 10. `changed`

Submit an explicit changed-file hint.

```text
axge changed --solution my-app --project app --path src/Example.cs --reason manual [--json]
axge changed --solution my-app --from-json changes.json [--json]
```

For integrations that already know changed paths. Reasons are `manual`, `git`, `tool` or `reconcile`. Acceptance records pending work; the writer still needs to process it. Normal users can save source with watch running.

## 11. `queue`

Inspect and steer queued work.

```text
axge queue list --solution my-app [--limit N] [--json]
axge queue retry --job ID [--json]
axge queue cancel --job ID [--json]
```

Retry and cancel change job state. Inspect the job and correct its cause before retrying. An invalid path or malformed source is not repaired by repeated retries.

## 12. `reconcile`

Run a bounded source pass.

```text
axge reconcile --solution my-app --scope dirty [--wait] [--timeout 10s] [--json]
axge reconcile --solution my-app --scope project --project app [--json]
axge reconcile --solution my-app --scope full [--json]
```

The lower-level interface to bounded source work. Use `build` for the normal one-shot workflow. Full scope is deliberate. Waits are bounded; a timeout does not prove the server completed work.

## 13. `query`

Read context or impact from a published graph.

```text
axge query context --solution my-app (--symbol NAME|--node-id ID) [--max-bytes N] [--max-nodes N] [--depth N] [--json]
axge query impact --solution my-app (--symbol NAME|--node-id ID) [--max-bytes N] [--max-nodes N] [--depth N] [--json]
```

Select a symbol or observed node ID and bounded output. Check records and truncation. The current build profile is declarations and containment; missing edges do not prove an absence of impact. See [coverage](../15-STATIC-ANALYSIS-COVERAGE.md).

## 14. `render`

Render a published relationship diagram.

```text
axge render --solution my-app [--project app]... --out DIRECTORY [--json]
```

Writes `result.html` and `summary.json` using published graph data. Prefer `export --format html` when you want the structured export and artifact receipt together.

## 15. `update`

Delegate to the trusted runtime manager.

```text
axge update check [--json]
axge update apply --plan plan.json [--approve-digest SHA256] [--json]
```

An advanced delegation surface requiring the matching trusted orchestrator. Routine installed-user updates use `axm update`. Missing peers or invalid approvals are refused.

## Export snapshot selectors

The export command also accepts repeated `--node-id ID`, `--edge-id ID` and
`--project-generation ID=SHA256`, plus `--exact-projection` and
`--catalog-generation SHA256`. Use observed IDs. Ordinary node selection creates
an induced projection; exact projection preserves the observed edge set,
including an empty one. A missing retained generation is an error.

## Errors

Exit `0` means success. A JSON failure includes `code`, `message`, `retryable`,
`details` and `request_id`. Use stable codes in scripts; wording can change.
See [exit codes](../CLI-EXIT-CODES.md) and [troubleshooting](../guides/troubleshooting.md).
Installation, managed-host setup and uninstall belong to `axm`; they are not
additional `axge` commands.

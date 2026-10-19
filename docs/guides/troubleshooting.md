# Troubleshooting

Start with the installed version, registered scope and current state:

```text
axm version
axge solution list
axge doctor --solution my-app --json
axge status --solution my-app --json
```

Replace `my-app` with your solution ID. `doctor` diagnoses; it does not repair
automatically. Keep the error code and message when reporting a problem.

## Setup and command errors

| Symptom | What to do |
|---|---|
| `axge` is not found | Open a new terminal after installation; follow the [installer's PATH guidance](https://github.com/orchex006/axiom-cli/blob/main/docs/INSTALL.md) |
| `build requires --solution` | Run `solution list`; pass an existing ID or complete the [quickstart](../21-INSTALLATION-QUICKSTART.md) |
| `NOT_FOUND: this solution id is not registered` | `--solution` needs a listed ID, not a directory or `.sln` path. Register first; `my-app` is an example, not a built-in solution. Check that registration and build use the same `AXIOM_HOME`. |
| Local bindings are required | Register with `--bindings bindings.json --dry-run`, then `--apply`, using the same Axiom home |
| Repository root does not exist | Correct the absolute local path in the private bindings file |
| Whole-repository `.` is rejected | On an older installed build, select an existing source subfolder; whole-repository roots are supported from 0.1.12 |
| Query returns no records | Check the fully qualified symbol, registered source folder, successful build and supported language |
| Export destination already exists | Choose a new output directory; export does not overwrite an old snapshot |

A failed build may also print a `work_cycle` diagnostic with empty generation
maps and null counts/timings. This reports the failed attempt; it is not evidence
that a graph was built, and the uninstrumented timing message is not its cause.
Use the accompanying error, such as `NOT_FOUND`, to choose the recovery step.

## Another writer owns the home

`WRITER_ALREADY_RUNNING` or exit `10` means another daemon or watcher owns the
instance. Reuse that process or stop it through its normal owner before a
separate build. Do not delete lock files to bypass ownership.

## The graph is behind the source

Check pending/failed work in status. If watch is stopped, start it or run one
build. If a cycle failed after a malformed save, fix the source; the previous
valid graph is retained until a later successful cycle. `NOT_READY` describes
the requested state, not successful completion.

## A shard is corrupt or missing

An integrity error means published files did not pass validation. Preserve
the error details, check disk access and rebuild from source when possible.
Do not hand-edit JSON shards or the database to make checksums pass.

## Work is retrying

Inspect `axge queue list --solution my-app --json`. Temporary failures can
enter retry backoff. Correct a permanent cause before retrying a job; repeated
retries do not repair invalid paths, unsupported storage or malformed source.

## State is on unsupported storage

Mutable SQLite state must be on local disk. A network share or cloud-synced
home can produce `SQLITE_NETWORK_STORAGE_UNSUPPORTED` (exit `9`). Select a
private local `AXIOM_HOME` and complete setup there; do not disable the storage check.

## An agent cannot query a working graph

Installation, host registration, new-session skill discovery and authenticated
graph access are separate checks. See [connect your agent](host-adapters.md)
and [verify agent access](host-verification.md).

## Share a useful bug report

Include the Axiom version, operating system, exact command, exit status and
error code, plus whether the same issue occurs with the [included demo](../../examples/first-graph/README.md).
Review diagnostics for private paths and source names before sharing.
[Exit-code reference](../CLI-EXIT-CODES.md).

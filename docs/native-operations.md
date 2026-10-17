# Native operations

Use these steps to run an installed engine and inspect its health. Installation
and updates are managed by `axm`; graph operations use `axge`.

## Before starting a writer

```text
axge solution list
axge doctor --json
```

Confirm the selected Axiom home and registered source. Watch covers every
registration in that instance. If an owned service already runs there,
use that writer rather than starting another one.

## Run in the foreground

```text
axge watch
```

`axge serve` is the lower-level foreground daemon entry. Both require
solution registration and local bindings. Watch remains active until stopped;
Ctrl+C requests a cooperative shutdown. Wait for the shell prompt before
starting a replacement writer.

## Inspect work

```text
axge status --solution my-app --json
axge queue list --solution my-app --json
```

Status reports pending work and publication state. Accepted changed-file hints
can still be pending; they are not a completed graph build. For one-shot work,
stop a foreground watcher before `axge build --solution my-app`.

## Recover after interruption

Restart in the same home and let the inventory scan discover offline edits.
Keep SQLite state on local disk. Preserve journals, existing snapshots and
source when diagnosing a problem. Do not clear database or lock files as a
first recovery step.

[Troubleshooting](guides/troubleshooting.md) covers writer conflicts and stale
results. [Updates](guides/updates.md) covers package rollback;
[migration](migration-runtime.md) covers legacy graph import.

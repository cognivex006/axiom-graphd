# Edit source with watch running

After the first graph is built, start the watcher in a terminal:

```text
axge watch
```

Keep editing and saving in your usual editor. You do not submit individual
files to Axiom. Watch scans all registered projects in the selected home and
publishes changed source through the same graph pipeline as build.

## What happens when you save

| Change | What to expect |
|---|---|
| Save a supported source file | Its updated declarations appear after a completed cycle |
| Save several files | Work is processed in bounded batches |
| Rename or delete a source file | The next completed cycle updates the published graph |
| Delete the last declaration | A new snapshot removes the old declaration |
| Save malformed or partially written source | The previous valid snapshot remains; the cycle reports a failure |
| Create generated graph output | Generated data is excluded from source analysis |

The current foreground watcher uses polling. A filesystem hint is not proof
that analysis has finished. Check status when you need to know whether the
graph has caught up:

```text
axge status --solution my-app --json
```

## Recover after a restart

Start watch again in the same Axiom home. Inventory scans find source changes
made while it was stopped; you do not need to re-save every file. A one-shot
`axge build --solution my-app` is also useful when you only need a fresh snapshot.

## Stop cleanly

Press Ctrl+C and wait for the shell prompt to return. Run only one writer per
home. If a managed service already owns it, use that service or stop it through
its owner before starting another foreground writer.

## When a change does not appear

Confirm the source folder is registered, the file uses a supported language,
and the latest cycle succeeded. Fix parse errors before expecting a new graph.
A query can continue to read an older valid publication while source work is
pending; inspect [freshness and coverage](../15-STATIC-ANALYSIS-COVERAGE.md).
[Troubleshooting](troubleshooting.md) covers setup, writer conflicts and corrupt data.

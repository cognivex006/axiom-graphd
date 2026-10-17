# Build, query, watch and export

These commands use a registered solution. If you have not registered one,
complete the [quickstart](21-INSTALLATION-QUICKSTART.md) first.

## Build a snapshot

```text
axge build --solution my-app --json
axge status --solution my-app --json
```

Build scans the selected source and publishes a new graph when its facts change.
Unchanged source is a no-op. For a single project, add `--project app`.
Use `--full` only when you intend to invalidate and rebuild the selected source facts.
Only one writer can own an Axiom home; stop a foreground watcher before a separate build.

The result includes publication information and measured processing time.
Review status for pending work and coverage before treating a graph as current.
[Captured demo results](../examples/first-graph/output/README.md) show actual output.

## Find a declaration

```text
axge query context --solution my-app --symbol MyNamespace.MyClass --max-nodes 20 --max-bytes 8192 --json
```

Replace the symbol with a fully qualified name from your source. A bounded
result may be truncated; increase the bounds deliberately or narrow the subject.
`query impact` explores recorded relationships with the same subject and bounds.
Missing edges do not prove there is no impact: the current build profile is
declarations and containment, with [analysis limits](15-STATIC-ANALYSIS-COVERAGE.md).

## Refresh while editing

```text
axge watch
```

Watch stays in the foreground and covers every registered solution in the
selected Axiom home. It uses polling. It has no per-project watch flag.
Save source normally; use Ctrl+C to request shutdown. Do not start a second
writer when a service or watcher already owns this home.

Saves, renames and deletes update the graph. A malformed or interrupted save
preserves the previous valid snapshot; fix the source and let the next cycle
run. [Editing with watch](guides/human-workflow.md) explains this workflow.

## Open a diagram

```text
axge export --solution my-app --out graph-export --format html --json
```

Open `graph-export/result.html`. Export reads the published snapshot without
rebuilding it. The destination must be new, and `--project` can be repeated to
select projects. [Export guide](guides/export.md).

## Share structured data

```text
axge export --solution my-app --out graph-data --json
```

JSON is the default export format. The command reports the files it wrote,
their SHA-256 values and their generation metadata. It does not include the
runtime database, machine bindings, credentials or logs. Review source labels
and graph content before sharing them outside your team.

## Read machine output

`--json` keeps one result object on stdout. Operational cycle diagnostics go
to stderr; redirect those streams separately in scripts. `processing_ms` is
measured within the engine. Unmeasured phases and delays are `null` with a
reason, so they should not be interpreted as zero.

See the [command reference](reference/axiom-graphd.md) for less common options.

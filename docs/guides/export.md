# Export and share a graph

Export turns a published graph into files you can inspect, attach to a review
or share with a teammate. It does not rebuild source or start a watcher.

## Export HTML

```text
axge export --solution my-app --out graph-html --format html --json
```

Open `graph-html/result.html` in a browser. It is a standalone diagram that
works offline. Its companion `summary.json` describes the diagram's content.
Keep the directory together when sharing the export.

**Try it now:** [actual sample HTML](../../examples/first-graph/output/html/result.html)
and [sample source](../../examples/first-graph/README.md). GitHub shows HTML as
source; download the files and open `result.html` locally to see the diagram.

## Export JSON

```text
axge export --solution my-app --out graph-json --json
```

JSON is the default. Use it when another tool needs the graph records and
snapshot metadata. The command result lists each artifact's relative path,
byte size and SHA-256. [Sample exported data](../../examples/first-graph/output/README.md).

## Choose the scope

Repeat `--project ID` to select projects. To export specific observed nodes,
repeat `--node-id ID`; ordinary node selections include the edges whose two
endpoints are selected. Use IDs returned by a query, rather than a guessed label.

Advanced readers can use `--exact-projection`, `--edge-id`,
`--catalog-generation` and `--project-generation` to retain an observed selection.
An MCP catalog ID identifies its member vector; it is not the engine catalog
manifest ID. Pass observed project generations as `--project-generation ID=SHA256`
when retaining an MCP snapshot. An unavailable retained snapshot is refused.

## Understand the snapshot

Export pins one catalog and checks its project manifests and shards. Corrupt
or missing required data produces an error instead of a mixed snapshot.
Coverage and completeness describe the selected data, not complete knowledge
of the program. A live-source export is also not proof of Git staged-source fidelity.

The output directory must not already exist. Choose a new name for another
export. A write error can leave a partial new directory; use a new destination
after correcting the error and confirm the final command succeeds.

Exports omit the operational database, credentials and machine bindings.
Source names and relationships can still be private; review the content before sharing.

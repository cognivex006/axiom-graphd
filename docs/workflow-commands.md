# Graph workflow commands

Build, watch and export are available in the 0.1.11 installed runtime. Start with
the [native quick start](21-INSTALLATION-QUICKSTART.md) to register a solution and
its local folders. Inspect `axge --help` when using an older installed release.

```text
axge build --solution ID [--project ID] [--full] --json
axge watch [--registry PATH] --json
axge export --solution ID [--project ID]... [--node-id ID]... --out NEW-DIRECTORY [--format json|html] --json
```

Build uses existing inventory/analysis/publication. Unchanged source is a no-op;
full invalidation requires --full. Registration remains explicit. An existing
instance writer is refused, never bypassed. Malformed declarations fail before
publication; unknown input languages are reported as partial coverage. The
current profile has declarations and containment, not complete method call edges.

Watch reuses serve and covers every registration in the instance. It uses polling,
not per-project isolation or native event guarantees. Reuse an existing authorized
service; do not create a second writer. Cooperative shutdown drains owned work.
Save/rename/delete, including deletion of the last source, update the published
graph without an additional manual reconcile. Idle fallback scans are not work events.
Transient source writes and malformed saves preserve the previous valid publication
and the watcher. Up to three immediate source retries occur before waiting for a
new hint or inventory fallback. Failed cycles retain measured timing; no stale
publication is promoted to proof of current-source freshness.

Build results and each non-idle watch cycle carry a monotonic processing_ms,
outcome, changed/analyzed files and before/after generation vector. Cycle records
appear on stderr as `{"target":"work_cycle","cycle":{...}}`; --json stdout keeps
one result object. Failed work also records elapsed time with unknown counts as
null. Uninstrumented phases/queue/observed-source delay are null with a reason.
These observations never enter graph content/hashes or MCP stdio output.

Export defaults to JSON, pins one catalog vector, verifies exact project manifests
and shards under native reader guards and rejects corrupt/missing members. Project
and node selectors preserve discussed scope; node selections are induced projections
and are labeled filtered. HTML comes from the same snapshots and renderer. The new
destination is claimed without overwriting an existing directory. A write failure
may leave a partial new directory and returns an error; it never reports success.

The result lists relative artifact names, sizes, SHA256 and generation metadata.
Export has no implicit refresh, installation, source/Git-index edit or external
publication. No database, bindings, credential or log is exported. Published-source
identity is not staged-source fidelity; staged checkpoints still need explicit
source-selector verification. Schema validity is not semantic accuracy.

Use --edge-id <id>, --exact-projection and --catalog-generation <sha256> to retain an observed edge set and conversation snapshot. An empty exact edge set stays empty; unavailable retained generations are refused. Default node selections remain induced projections.

MCP query catalog_generation_id is its member-vector digest, not the engine catalog manifest identity. Pass the actual project_generations through repeated --project-generation ID=SHA256; a retained matching catalog is selected within a bounded search. Use --catalog-generation only for an actual engine catalog ID.

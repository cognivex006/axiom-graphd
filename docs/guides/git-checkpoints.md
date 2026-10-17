# Use Git with graph output

Keep source changes easy to review. Live graph generations, databases and
machine bindings are generated or private state, rather than application source.

## Keep live output out of ordinary commits

Review ignore rules for `.axiom/graph/**/live/` and private `.axiom/local/`
content. Keep databases, WAL files, locks, credentials and local bindings out
of Git. Do not ignore the entire `.axiom/` directory: it can also hold portable
configuration, annotations and managed policy your team wants to track.

## Share a snapshot deliberately

Use [export](export.md) to produce a reviewed HTML or JSON artifact. A normal
live export represents a published source snapshot; it is not proof that its
contents exactly match the Git index or a particular commit.

After merging source, rebuild the graph from the resulting source. Do not
merge generated graph shards by hand or use a union merge to resolve them.

## Review before a checkpoint claim

A commit-bound checkpoint needs verified source selection and freshness for
the same staged bytes. Normal `build` and `export` alone do not establish that
guarantee. Use only a supported explicit checkpoint workflow for such a claim;
do not substitute a copied live directory.

# Watch, analyze and publish

`axge watch` keeps published graphs current as registered source changes.
The current foreground workflow uses polling and covers every registration
in the selected Axiom home.

## From save to snapshot

1. Inventory detects source changes.
2. Durable work records the files that need processing.
3. The analyzer extracts supported declarations and containment.
4. Validation checks the result before publication.
5. Queries and exports read a pinned set of published project snapshots.

A cycle can fail while an earlier valid graph remains readable. Check status
for pending work and inspect coverage before making a freshness claim.

[Editing with watch](guides/human-workflow.md) ·
[Coverage](15-STATIC-ANALYSIS-COVERAGE.md) ·
[Developer architecture](development/README.md).

# Migrate a legacy graph layout

Use migration when an existing project stores graph data under the legacy
`graph` or `grahp` layout. New projects can follow the normal quickstart.
Migration imports compatible immutable graph data into `.axiom/graph`.

## Preview from the project repository

```text
axm engine migrate plan --solution my-app --from auto --to-layout 2 --out migration-plan.json --json
```

Read the plan before proceeding. It identifies the repository, source and
destination, required space, expiry, transaction and approval digest.
A fresh repository reports that migration is not needed.

## Apply the reviewed plan

```text
axm engine migrate apply --plan migration-plan.json --approve-digest <plan_digest> --json
axm engine migrate status --transaction <transaction> --json
```

Replace the placeholders with the values returned by your reviewed plan.
A changed source, destination or approval makes the plan stale; generate and
review a new plan instead of editing its digest.

## Roll back

```text
axm engine migrate rollback --transaction <transaction> --approve-digest <plan_digest> --json
```

Rollback verifies ownership and refuses to overwrite later human changes.
Keep the status report and transaction journals when recovery is partial.

## What is preserved or refused

Legacy source files and unrelated human content remain intact. Import does
not copy a runtime database, rewrite host settings or convert an unsupported
graph format. Conflicting legacy folders, corrupt generations, unsafe paths
and stale plans are refused. Legacy cleanup is a separate explicit operation.

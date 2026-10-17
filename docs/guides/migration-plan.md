# Review a migration plan

The [migration guide](../migration-runtime.md) contains the complete command
sequence. Before applying, check the following against your intended project:

- The solution ID and repository are correct.
- The legacy source and `.axiom/graph` destination are the expected folders.
- Source hashes and required disk space are recorded.
- There are no unresolved conflicts or human-owned replacement targets.
- The plan is unexpired, and the approval digest is the one you reviewed.

If any source or target changes, create a new plan. A previous approval does
not approve different bytes. Keep the transaction ID for status and rollback.

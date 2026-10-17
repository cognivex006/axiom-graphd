# Update Axiom

Use the distribution manager to update the installed runtime:

```text
axm version
axm update
axm doctor
```

Review the plan before confirming it. The updater verifies the selected
release inputs and retains the previous version for rollback. It reports when
no newer release is available. A source-branch fix becomes available to an
installed user only after a corresponding delivery is published.

## If an update fails

Keep the original error and transaction information. Fix network, checksum,
permission or disk problems before retrying. Changed or incomplete retained
runtime files are preserved and refused rather than silently overwritten.

Follow the [distribution guide](https://github.com/orchex006/axiom-cli/blob/main/docs/INSTALL.md)
for rollback, recovery and upgrading older installations. Engine-level
operations are an advanced interface; use the public `axm` workflow for routine
updates. Do not mix binaries from different core releases.

## Update a host skill

For a managed host, use `axm skills update codex` or `axm skills update claude`.
Open a new agent session after a successful change. See [skills updates](skills-update.md).

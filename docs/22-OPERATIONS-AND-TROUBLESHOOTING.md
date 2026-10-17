# Operations and troubleshooting

Run `axge doctor --json` for diagnostics and
`axge status --solution my-app --json` for a registered solution's work state.

- [Troubleshooting](guides/troubleshooting.md): setup, stale data, writer ownership and integrity failures.
- [Native operations](native-operations.md): foreground watch, shutdown and restart.
- [Files and state](guides/files-and-state.md): private local state and generated snapshots.
- [Updates](guides/updates.md): runtime management and rollback guidance.
- [Exit codes](CLI-EXIT-CODES.md): stable values for scripts and bug reports.

Diagnose the named cause before retrying. Preserve source, journals and the
last valid snapshot; deleting state or bypassing a lock can hide the original problem.

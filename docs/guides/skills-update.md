# Update or recover a host skill

```text
axm skills update codex
axm skills doctor codex
```

Replace `codex` with `claude` for the managed Claude Code adapter. Review the
plan, then open a new session to load the updated skill. The `axsk` skill is
an instructional bundle; terminal lifecycle operations are managed by `axm`.

For a failed managed-host operation, inspect its report and use
`axm skills recover codex` for that same host. Retained snapshots support
recovery. Disabled managed plugins remain disabled during update.

`axm skills uninstall codex` removes a managed registration after review.
Runtime updates use [the normal update workflow](updates.md); a host failure
can be reported separately from a successful runtime installation.

# Install Axiom skills

A skill teaches your coding agent how to use the graph tools. It does not
replace the native engine or create graph data on its own.

```text
axm skills install codex
axm skills doctor codex
```

Use `claude` instead of `codex` for the managed Claude Code adapter. Review
the plan and open a new session after installation. [Connect your agent](host-adapters.md)
covers host namespaces and query verification.

`axm skills list` inspects managed registrations. The manager preserves foreign
and modified host settings; review a reported conflict before retrying.
Other hosts use their documented manual setup from the
[skills project](https://github.com/orchex006/axiom-skills/blob/main/docs/GETTING-STARTED.md).

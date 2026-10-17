# Runtime lifecycle

For routine installation and updates, use the public distribution manager:

```text
axm version
axm doctor
axm update
```

The [distribution guide](https://github.com/orchex006/axiom-cli/blob/main/docs/INSTALL.md)
documents options, uninstall, recovery and retained-version rollback. Review
its plan before applying a change. Project source, local graph registration
and host plugin state are distinct from the installed runtime payload.

Engine-level lifecycle commands are an advanced interface. Use only forms
reported by the installed `axm engine --help`, with the exact plan and approval
it requests. Preserve ownership records and changed human files during recovery.

[Updates](../guides/updates.md) · [Migration](../migration-runtime.md).

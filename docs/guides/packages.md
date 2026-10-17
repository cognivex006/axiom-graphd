# Components and versions

Axiom installs several tools with distinct jobs:

| Name | What you use it for |
|---|---|
| `axm` | Install, update and diagnose the runtime; manage supported host skills |
| `axge` | Register source, build/watch graphs, query declarations and export snapshots |
| `axmp` | Serve published graphs through the local API/MCP gateway |
| `axsk` | Agent workflow skill discovered inside a supported coding host |

`axsk` is a skill intent, not a terminal command for graph building. Host
namespaces can qualify it, such as `$axiom:axsk` or `/axiom:axsk`.

## Check versions

```text
axm version
axge version --json
```

The core engine and its operations executable share a core release. The
distribution CLI, MCP gateway and skills have their own package identities.
Version output describes observed installed state; an unchecked remote version
is not proof that an installation is up to date.

The public names are launcher names. Source folders in this repository are
implementation modules, explained in the [developer guide](../development/README.md).

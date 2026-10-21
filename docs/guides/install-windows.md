# Install on Windows

## Prerequisites

Windows 10/11 x64 and PowerShell 5.1 or later. Run as your normal user, not Administrator. A Rust compiler and an Axiom source checkout are not required.

## Install and verify

Use the [official installer](https://github.com/cognivex006/axiom-cli#install-and-update)
for this platform. It verifies downloaded checksums and asks before applying
the displayed plan. Keep this trust verification enabled.

Open a new terminal after installation:

```text
axm version
axm doctor
axge --help
```

Version output identifies the runtime. Doctor checks installation health;
your project still needs a solution registration and local folder bindings.
Follow the [quickstart](../21-INSTALLATION-QUICKSTART.md) or [try the demo](../../examples/first-graph/README.md).

## Choose where state lives

Keep `AXIOM_HOME` on a private local disk. [Files and state](files-and-state.md)
lists default folders. Use the same home for registration and graph commands.

## Keep the graph current

Start `axge watch` for foreground refresh and stop it with Ctrl+C.
A per-user service is optional; inspect installed service capabilities in
`axm --help` before configuring it. Run only one writer for a home.

## Connect a host or remove Axiom

[Agent setup](host-adapters.md) covers managed skills and MCP access.
Installation success alone does not verify an agent handshake.
For uninstall and PATH verification, follow the
[distribution guide](https://github.com/cognivex006/axiom-cli/blob/main/docs/INSTALL.md).
Preserve project source, private bindings and any reviewed snapshots you need.

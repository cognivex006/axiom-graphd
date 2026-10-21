# Install Axiom

Install the prebuilt Axiom runtime, then use `axge` to work with graphs.
The installer also provides `axm` for runtime management and `axmp` for agent access.

## Before you start

| Platform | Requirements |
|---|---|
| Windows x64 | Windows 10/11, PowerShell 5.1 or later, normal user account |
| Linux x64 / WSL2 | Normal user account, download and archive utilities; see the installer guide |
| macOS Intel | Normal user account; use the x64 package |

Apple silicon and Linux ARM packages are not currently available. Keep mutable
Axiom state on a local disk, outside a network share or cloud-synchronized folder.
Rust, Cargo and a source checkout of Axiom are not required.

## Install the runtime

Follow the [official installation instructions](https://github.com/cognivex006/axiom-cli#install-and-update)
for your platform. The installer verifies downloaded checksums, shows its plan
and asks before applying. Run it as your normal user, without Administrator or sudo.

After installation, open a new terminal:

```text
axm version
axm doctor
axge version --json
```

`version` identifies the installed components. `doctor` diagnoses installation
health; a healthy runtime still needs a registered solution before it can build
your source. Trust downloaded packages only from the configured official source;
checksum verification checks their bytes and should stay enabled.

## Build something

- [Run the included demo](../examples/first-graph/README.md) for known source and browser output.
- [Follow the quickstart](21-INSTALLATION-QUICKSTART.md) to register your repository and build its first graph.
- [Connect an agent](guides/host-adapters.md) when you want graph access from your coding assistant.

## Update or remove

`axm update` reviews and applies a newer runtime when one is available.
Use the [distribution guide](https://github.com/cognivex006/axiom-cli/blob/main/docs/INSTALL.md)
for install options, PATH setup, uninstall and rollback. Project source and local
graph registration are separate from package installation.

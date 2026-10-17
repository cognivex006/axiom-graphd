# Axiom Graph Engine documentation

Build a local code graph, find source declarations, and share a snapshot your
teammates can open in a browser. Start with an installed `axge` command.

## Get started

1. [Install Axiom](21-INSTALLATION.md) — prebuilt packages and a first health check.
2. [Try the demo](../examples/first-graph/README.md) — sample source, commands and actual output.
3. [Build your first graph](21-INSTALLATION-QUICKSTART.md) — register your own repository.

## Work with graphs

| Guide | What you will learn |
|---|---|
| [Build, query, watch and export](workflow-commands.md) | The everyday command sequence and what each result means |
| [Solutions and projects](guides/solutions.md) | Register one or several source roots without embedding local paths |
| [Editing with watch](guides/human-workflow.md) | Saves, renames, deletes and restarting after offline edits |
| [Export a diagram or JSON](guides/export.md) | Open HTML locally, share a snapshot and interpret its metadata |
| [Analysis coverage](15-STATIC-ANALYSIS-COVERAGE.md) | Supported source and the limits of relationship analysis |
| [Use Git with graph output](guides/git-checkpoints.md) | Keep generated data separate from source changes |
| [Connect an AI agent](guides/host-adapters.md) | Install a host skill and verify access to a graph |

## Operate and troubleshoot

- [Troubleshooting](guides/troubleshooting.md)
- [Native operations](native-operations.md)
- [Updates and rollback](guides/updates.md)
- [Migration](migration-runtime.md)
- Installation: [Windows](guides/install-windows.md), [Linux](guides/install-linux.md), [macOS Intel](guides/install-macos.md)

## Reference

- [Engine commands](reference/axiom-graphd.md)
- [Exit codes](CLI-EXIT-CODES.md)
- [Files and local state](guides/files-and-state.md)
- [Components and versions](guides/packages.md)

## For contributors

[Development](development/README.md) covers the Rust workspace, build and test
commands, benchmark scope and internal verification. Product guides describe
the commands and outcomes a user needs; engineering records stay with the
development workflow.

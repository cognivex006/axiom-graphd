# Files and local state

Axiom separates portable project descriptions, private machine state and
generated graph output.

| Location | Purpose | Share or commit? |
|---|---|---|
| Your `solution.json` | Logical solution/project IDs and relative source folders | Suitable after review |
| Your `bindings.json` | Absolute checkout folders for this machine | Keep private and out of Git |
| `AXIOM_HOME/config/bindings.json` | Accepted local bindings persisted by registration | Keep private |
| `.axiom/graph/<solution>/` | Published project and catalog snapshots | Live output is generated data |
| Export directory you choose | HTML/JSON snapshot for inspection or sharing | Review its source content first |
| `.axiom/agent/` | Optional managed policy and ownership records | Follow your team's repository policy |

## Axiom home

The home stores registry, queue, runtime and operational state for an instance.

| Platform | Default |
|---|---|
| Windows | `%LOCALAPPDATA%\Axiom` |
| Linux | `${XDG_STATE_HOME:-~/.local/state}/axiom` |
| macOS | `~/Library/Application Support/Axiom` |

`AXIOM_HOME` can select a different absolute private local folder. Use the same
value for registration, graph commands and the gateway. A different home is
a separate instance state; it does not inherit the previous registrations.

## Generated graph data

A project snapshot is published under immutable generation names. A solution
catalog records which project generations belong together. Commands validate
those files before reading them; hand edits break integrity checks.

Do not ignore `.axiom/` wholesale: it can also contain portable configuration,
policy and annotations. Review [Git guidance](git-checkpoints.md) for generated paths.

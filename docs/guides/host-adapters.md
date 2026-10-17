# Connect your coding agent

Build a graph first using the [quickstart](../21-INSTALLATION-QUICKSTART.md).
Then connect a supported host so its agent can discover Axiom workflows and
read the published graph through the MCP gateway.

## Install the host skill

For Codex:

```text
axm skills install codex
axm skills doctor codex
```

For Claude Code with a native `claude` executable on PATH:

```text
axm skills install claude
axm skills doctor claude
```

Review the installation plan. After it succeeds, open a new agent session.
Codex can discover `$axsk` or `$axiom:axsk`; Claude Code uses `/axiom:axsk`.
Use the namespace your host actually exposes.

## Ask about your graph

Select the discovered skill and ask it to locate a declaration or inspect the
registered solution. Skill discovery, gateway availability and an authenticated
graph query are separate results. Confirm the answer refers to your selected
source and reports freshness and coverage.

## Other hosts

Use the [skills getting-started guide](https://github.com/orchex006/axiom-skills/blob/main/docs/GETTING-STARTED.md)
for current host compatibility and manual setup. Do not copy another host's
configuration keys into a different host. [Verify agent access](host-verification.md)
if a plugin appears installed but cannot query your graph.

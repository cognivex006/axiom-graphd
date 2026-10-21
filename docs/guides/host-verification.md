# Verify agent access

A working setup has several independently observable parts:

| Check | What success establishes |
|---|---|
| `axm doctor` | The installed runtime's reported health |
| `axm skills doctor codex` or `claude` | Managed host registration state |
| Skill visible in a new host session | The host discovered the plugin in that session |
| Authenticated bounded graph query | The agent can read the selected published graph |

A running gateway or successful protocol initialization alone does not prove
the host has a usable graph tool. Ask the agent for a known symbol in a
registered project, then verify its returned source location and graph scope.

If registration is healthy but the skill is absent, start a new session.
If a query fails, check gateway configuration, permissions, selected Axiom home,
solution registration and published graph state. Do not change credentials
or reinstall the runtime solely because a graph has not been built yet.

[Host setup](host-adapters.md) ·
[Gateway guide](https://github.com/cognivex006/axiom-mcp/blob/main/docs/local-gateway.md)

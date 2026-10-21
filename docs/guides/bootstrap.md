# Project instructions and managed setup

Agent setup can add a small Axiom-owned instruction block and a policy file
to a repository. The purpose is to help an agent select graph scope, verify
freshness and read source before making a claim.

| File | Purpose |
|---|---|
| `AGENTS.md` or the host's instruction file | A short pointer to the graph policy |
| `.axiom/agent/POLICY.md` | Managed graph workflow instructions |
| `.axiom/agent/bootstrap.lock.json` | Ownership record for managed content |

Begin with [managed host setup](host-adapters.md) and the
[skills guide](https://github.com/cognivex006/axiom-skills/blob/main/docs/GETTING-STARTED.md).
Review the exact project plan before applying repository changes.

Existing human instructions remain authoritative. Missing, duplicate or
edited ownership markers need review; they are not permission to replace
an entire instruction file. A graph policy does not grant filesystem,
network, commit or deployment permissions.

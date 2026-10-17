# How source changes become graph updates

Build and watch compare the registered source with the recorded inventory,
analyze changed files and publish a validated snapshot. The engine keeps
durable work state in local SQLite so pending work can be recovered after restart.

## What you need to know

- Saves are discovered through inventory and watcher observations; you do not submit files manually.
- A queued change is pending work until a cycle completes.
- One writer owns an instance, while readers use validated published snapshots.
- An invalid save preserves the previous valid publication.
- Mutable state belongs on local disk, outside shared or cloud-synchronized storage.

Use `axge status --solution my-app --json` to inspect progress and
`axge queue list --solution my-app --json` to inspect queued work.
[Workflow guide](workflow-commands.md) · [Troubleshooting](guides/troubleshooting.md).

Contributors can find the module map and source checks in
[development](development/README.md). Shared storage and queue contracts are
owned by the pinned specification, rather than duplicated in this guide.

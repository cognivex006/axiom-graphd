# Check which hosts Axiom can manage

```text
axm skills list
axm skills doctor codex
axm skills doctor claude
```

Inspect the report for the selected host. A host executable being present is
separate from a managed plugin registration and a new session loading that plugin.
If the host is missing, install or locate it using its own setup instructions,
then retry the [managed skill setup](host-adapters.md).

Unknown host versions or foreign configuration need review. Preserve existing
settings and use the [current compatibility guide](https://github.com/cognivex006/axiom-skills/blob/main/docs/host-compatibility.md)
rather than assuming a configuration shape from another host.

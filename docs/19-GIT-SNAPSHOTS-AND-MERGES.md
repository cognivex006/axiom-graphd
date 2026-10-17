# Graph snapshots and Git

Live graph output is derived from source. Keep it separate from ordinary
application changes, and rebuild from merged source instead of merging
generated shards.

[Git guidance](guides/git-checkpoints.md) explains which state belongs outside
Git and why live exports do not establish staged-source fidelity.
Use [export](guides/export.md) for a shareable HTML or JSON snapshot.

A verified commit-bound checkpoint requires explicit source selection;
copying the live directory cannot provide that guarantee.

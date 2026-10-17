# Analysis coverage and limitations

A graph describes what the configured analyzer extracted from selected source.
Use its coverage and freshness information alongside the source itself.

## Current build and watch profile

The current workflow extracts supported C# and TypeScript declarations and
containment relationships. It helps locate a type or method and understand
which declaration contains it. The [store demo](../examples/first-graph/README.md)
shows this profile's actual output.

| Question | What the current graph can establish |
|---|---|
| Where is this class declared? | A supported declaration and its source location |
| Which declarations belong to this type? | Recorded containment relationships |
| Is the graph current? | Publication and pending-work observations for the registered scope |
| Does this method call another method? | Complete method-call analysis is outside this profile |
| Which implementation runs at runtime? | Runtime dependency injection, reflection and dynamic dispatch are outside this profile |

Broader parser libraries and engineering fixtures in this repository do not
expand the supported build/watch profile by themselves. Unsupported input
languages are reported as partial coverage. A schema-valid export does not
establish that every program relationship was discovered.

## Freshness is separate from coverage

A fresh graph can still have limited language coverage. A valid older graph
can also remain available while a new source cycle is pending or has failed.
Inspect status and the command result before presenting either as complete.

Malformed source does not advance a successful publication. Correct the source
and let build or watch finish again. Missing relationships must not be treated
as proof that a change has no effect.

## Source is read, not executed

Normal indexing does not need to run project build scripts, install application
dependencies or query your production database. Generated/build output and
excluded inputs do not become source facts automatically. Explicitly review
your registered folders before indexing a repository.

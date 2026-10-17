# Try a code graph in a few minutes

This example indexes three small C# files for an order service. You will query
a known class and open a diagram produced by the native graph engine. You do
not need to compile the application, install .NET or build Axiom from source.

## View the result first

Open [`index.html`](index.html) for the browser walkthrough with the actual diagram,
commands and linked results. This page works offline from a downloaded folder.

![Actual exported store graph](../../docs/images/first-graph.png)

Download this directory with the repository and open
[`output/html/result.html`](output/html/result.html) in a browser. The diagram
works offline. GitHub's file viewer shows HTML source rather than executing it.
[Captured results](output/README.md) include the structured graph and command outputs.

## Run the example

You need an [installed Axiom runtime](../../docs/21-INSTALLATION.md) with `axge`
on PATH. The optional convenience runner uses Python 3.10 or later.
From this example directory, choose a **new** output folder:

```text
python run_demo.py --out demo-run
```

On macOS/Linux, use `python3` if that is your Python command. The runner copies
the sample source, uses a private Axiom home under the new folder, previews and
applies only that isolated registration, then builds, queries and exports.
It does not change your installed runtime, real project registrations or original sample source.

Open `demo-run/html/result.html`. For a locally built engine, contributors can
pass `--engine /absolute/path/to/axiom-graphd`; installed users need no source build.
A run keeps its own workspace and private state for inspection. Reuse neither
the output directory nor its private `bindings.json` on another machine.

## Follow the commands yourself

Without the runner, use a separate terminal and select a new private local
Axiom home for the session. On PowerShell:

```powershell
$env:AXIOM_HOME = Join-Path (Get-Location) 'demo-home'
```

On macOS/Linux:

```sh
export AXIOM_HOME="$PWD/demo-home"
```

These examples assume the demo was saved on local disk. Create `bindings.json`
with `demo-shop-repo` mapped to the absolute path of the included `source` folder:

```json
{ "bindings": { "demo-shop-repo": "D:/examples/first-graph/source" } }
```

Replace that path with your existing folder. Then run:

```text
axge solution register --config solution.json --bindings bindings.json --dry-run
axge solution register --config solution.json --bindings bindings.json --apply
axge build --solution demo-shop --json
axge status --solution demo-shop --json
axge query context --solution demo-shop --symbol Demo.Shop.OrderService --json
axge export --solution demo-shop --out demo-html --format html --json
```

Open `demo-html/result.html`. Export requires a new destination. Close this
dedicated terminal when finished so the session's home override does not
affect your normal Axiom commands. The manual workflow writes generated
`.axiom/graph/` data under the sample source; the runner uses a copy instead.

## What to look for

- The query contains `OrderService` and its source location.
- The diagram shows declarations such as `Order`, `OrderRepository` and `OrderService`.
- Containment edges connect declarations to their members.
- Export reports the snapshot and checksums of the actual files it wrote.

The sample models order declarations rather than running a store. The current
build profile does not produce a complete call graph. The diagram demonstrates
declaration/containment extraction; it does not establish runtime behavior.

## Try a change

In a runner-created workspace, add a method to `workspace/src/OrderService.cs`.
Select that run's `state` folder as `AXIOM_HOME`, then build `demo-shop` again
or run `axge watch`. Query the new declaration and export to another new folder.
[Watch guide](../../docs/guides/human-workflow.md).

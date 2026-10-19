# Quickstart: register a repository and build your first graph

Install the [prebuilt Axiom runtime](https://github.com/orchex006/axiom-cli#install-and-update)
and open a new terminal. This guide uses the installed `axge` command. No Rust,
Cargo, Docker, .NET build or running MCP server is required for this workflow.

## Understand what you are selecting

`axge build` does not automatically index the current directory. Register the
source folder once, then select its **solution ID** on subsequent commands.
An Axiom solution is a named group of source projects; it is not a Visual Studio
`.sln` file.

| Value | Meaning in this guide |
|---|---|
| `my-app` | A solution ID that **you register below**, not a built-in example project |
| `my-app-repo` | A logical repository ID mapped to your checkout |
| `app` | An Axiom project ID selecting source within that repository |
| `.` in the configuration | All supported source beneath the **bound repository**, not the terminal's later working directory |
| `--solution my-app` | Look up the registered ID; this flag does not accept a folder path |

First inspect existing registrations:

```text
axge version
axge solution list
```

If your repository is already registered, use its listed ID with the
[everyday build and export commands](workflow-commands.md), rather than repeating
the first-run setup below. If the list is empty, continue below. Use the same
`AXIOM_HOME` for every command. A different home has a different registry.

## Windows PowerShell

These steps work with Windows PowerShell 5.1 and newer. Keep the same terminal
open so the variables remain available. Start **inside your source repository**,
for example the folder containing your `.csproj` or your source subfolders.

### 1. Create the two setup files

Choose unused solution/repository IDs. The following block captures your current
folder automatically and saves configuration outside your repository, under your
local Axiom setup folder. It stops if that setup folder already exists; it never
rewrites existing setup files. Change the IDs or inspect the existing setup rather
than deleting it to force another run.

<!-- BEGIN POWERSHELL FIRST RUN -->
```powershell
$solutionId = 'my-app'
$repoId = 'my-app-repo'
$repoRoot = (Get-Location).Path
$setupDir = Join-Path $env:LOCALAPPDATA ('Axiom\setup\' + $solutionId)
if (Test-Path -LiteralPath $setupDir) { throw 'Setup folder already exists; inspect it or choose a new solution ID.' }
New-Item -ItemType Directory -Path $setupDir -ErrorAction Stop | Out-Null
$configPath = Join-Path $setupDir 'solution.json'
$bindingsPath = Join-Path $setupDir 'bindings.json'
$utf8 = [System.Text.UTF8Encoding]::new($false)
$config = @{
    id = $solutionId
    profile = 'default'
    catalog_host_repo = $repoId
    projects = @(@{ id = 'app'; repo_id = $repoId; path = '.' })
}
$bindings = @{ bindings = @{ $repoId = $repoRoot } }
[System.IO.File]::WriteAllText($configPath, ($config | ConvertTo-Json -Depth 6), $utf8)
[System.IO.File]::WriteAllText($bindingsPath, ($bindings | ConvertTo-Json -Depth 6), $utf8)
```
<!-- END POWERSHELL FIRST RUN -->

`solution.json` names the source scope. `bindings.json` maps that name to the
actual checkout on this machine; keep bindings private and out of Git. The block
writes UTF-8 without a byte-order mark, including on PowerShell 5.1. It does not
register anything yet or change your source files.

### 2. Preview the registration

```powershell
axge solution register --config $configPath --bindings $bindingsPath --dry-run
```

Check the solution, repository and project scope. Continue only if the preview
succeeds and selects the source you intended. A duplicate ID or conflicting
binding is a reason to inspect the existing registration, not to overwrite it.

### 3. Apply once and confirm the ID

```powershell
axge solution register --config $configPath --bindings $bindingsPath --apply
axge solution list
```

The list must now contain `my-app` (or the ID you chose). Registration saves the
accepted model and local bindings in the selected Axiom home. Changing directory
later does not change which repository that ID selects.

### 4. Build and open HTML

<!-- BEGIN POWERSHELL BUILD EXPORT -->
```powershell
axge build --solution $solutionId
axge status --solution $solutionId --json
$exportDir = Join-Path $setupDir 'graph-export'
axge export --solution $solutionId --out $exportDir --format html --json
```
<!-- END POWERSHELL BUILD EXPORT -->

Continue to export only after build succeeds. The first build should publish a
generation; a later build with unchanged source may be a successful no-op.
Status should identify your solution and its graph state. Build writes generated
graph data under `.axiom/graph/my-app/` in the bound repository, using your actual
solution ID. It does not edit your source files.

Open the resulting diagram:

```powershell
Invoke-Item (Join-Path $exportDir 'result.html')
```

The HTML works offline. Its companion `summary.json` describes the selected
nodes and edges. Export requires a **new** destination: choose another name on a
later export rather than expecting the command to overwrite an existing diagram.

### 5. Optionally query a known class

```powershell
axge query context --solution $solutionId --symbol MyNamespace.MyClass --max-nodes 20 --max-bytes 8192 --json
```

`MyNamespace.MyClass` is a placeholder. Substitute a fully qualified class name
from your own source. An empty result does not establish complete indexing:
check the name, selected source folders and [analysis coverage](15-STATIC-ANALYSIS-COVERAGE.md).
The current profile extracts supported C# and TypeScript declarations and
containment; it does not claim complete method-call graphs or runtime behavior.

## Other shells: create the JSON files manually

Use the same sequence: create files, preview, apply, list, build, then export.
Save the following portable configuration as `solution.json`, using a new file
name if it already exists. This is the same single-project scope as above.

<!-- BEGIN NATIVE QUICKSTART CONFIG -->
```json
{
  "id": "my-app",
  "profile": "default",
  "catalog_host_repo": "my-app-repo",
  "projects": [
    { "id": "app", "repo_id": "my-app-repo", "path": "." }
  ]
}
```
<!-- END NATIVE QUICKSTART CONFIG -->

Save a private `bindings.json`. Substitute the existing absolute local checkout
path for the placeholder; do not paste the placeholder literally. JSON strings
on Windows can use forward slashes, or escaped backslashes.

```json
{
  "bindings": {
    "my-app-repo": "<absolute-repository-root>"
  }
}
```

Pass the actual paths to the files you saved:

```text
axge solution register --config solution.json --bindings bindings.json --dry-run
axge solution register --config solution.json --bindings bindings.json --apply
axge solution list
axge build --solution my-app
axge status --solution my-app --json
axge export --solution my-app --out graph-export --format html --json
```

Stop at a failed command and resolve its error before continuing. Open
`graph-export/result.html` after successful export. On older runtimes before
0.1.12, use an existing source subfolder instead of the whole-repository path `.`.
For multiple projects, select disjoint folders; see [solutions](guides/solutions.md).

## Keep the graph current

```text
axge watch
```

Watch stays in the foreground and covers **all registered solutions** in this
Axiom home. Inspect the list first. Run one writer per home; stop a foreground
watcher before a separate build. Ctrl+C requests shutdown.

Graph build, CLI queries and HTML export do not need MCP credentials. To let an
AI agent query the graph, follow [connect your agent](guides/host-adapters.md).
A listening `/mcp` or `/healthz` is not graph readiness; MCP still needs its own
registered access and credentials. A successful CLI build alone does not prove
that connection is ready.

## Resolve the errors in order

| Message or symptom | Meaning and next step |
|---|---|
| `build requires --solution` | No ID was selected. Run `axge solution list`; if empty, register first. |
| `NOT_FOUND: this solution id is not registered` | The supplied value is not an ID in this home. `my-app` only exists after registration. Do not pass a repository path to `--solution`. |
| `cycle.status: failed`, with empty generations and null counts | That attempt did not publish a graph. Read the accompanying error; null phase timings are not the cause of failure. |
| Machine-local bindings are required | Preview and apply both setup documents in the same Axiom home. |
| Setup folder already exists | Reuse the registered ID or inspect your existing files. The PowerShell block intentionally refuses to overwrite them. |
| Existing binding conflicts | Review the current IDs, folder and home. Do not remove another registration to bypass the conflict. |
| Query has no records | Use a real source symbol and check coverage; the example name is not a built-in symbol. |
| Export destination exists | Choose a new output folder. |

The [demo](../examples/first-graph/README.md) provides known source and an actual
HTML result if you want a first successful run before selecting your own project.
See the [command reference](reference/axiom-graphd.md) for additional flags.

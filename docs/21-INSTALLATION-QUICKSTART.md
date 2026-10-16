# First graph with the installed runtime

Install the [prebuilt Axiom runtime](https://github.com/orchex006/axiom-cli#install-and-update),
then run `axm version` and `axm doctor`. The following steps use `axge` directly;
you do not need Cargo, Rust, Docker or a source build of Axiom.

## 1. Name the solution and select source folders

Open a terminal in your code repository. Save this native registration document
as `solution.json` if that file does not already exist; otherwise choose a new
filename and use it in the commands below. Use your own solution, repository and
project IDs when a registration already exists.

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

`my-app` is the Axiom solution ID used by later commands. A `.sln` or `.csproj`
filename is not automatically an Axiom solution registration. Here `path: "."`
selects source beneath this repository; use a relative subfolder such as `src`
to narrow the project. For multiple projects, declare disjoint source folders.
Build outputs and generated graph files are excluded by the engine.

If an older installed binary rejects the whole-repository path `.`, select an
existing source subfolder before registering, or use a runtime containing the
whole-repository root correction described in the changelog.

## 2. Bind the repository ID to this machine's folder

Save `bindings.json` in a private local folder, using the absolute path to your
repository. This example path must be changed to your own existing folder:

```json
{
  "bindings": {
    "my-app-repo": "D:/projects/my-app"
  }
}
```

Use an absolute local path on Windows, Linux or macOS. Windows paths can use `/`
as shown. Keep this file out of Git: it contains local
machine paths. The key must match `repo_id` in `solution.json`.

## 3. Preview and register

Run these commands with the actual paths to the two files you created:

```text
axge solution register --config solution.json --bindings bindings.json --dry-run
axge solution register --config solution.json --bindings bindings.json --apply
axge solution list
```

Review the dry-run output before applying. Registration persists the folder
bindings in `config/bindings.json` under your Axiom home, so subsequent commands
use the same folders. Default Windows home is `%LOCALAPPDATA%\Axiom`;
`AXIOM_HOME` selects a different private home when explicitly set. Use the same
home for register, build and watch. Registration preserves other bindings and
refuses a conflicting rebind.

## 4. Build, inspect and query

```text
axge build --solution my-app
axge status --solution my-app --json
axge query context --solution my-app --symbol MyNamespace.MyClass --json
axge export --solution my-app --out graph-export --format html --json
```

Replace `MyNamespace.MyClass` with a symbol in your source. Build finishes after
publishing the graph. The generated data is under
`.axiom/graph/my-app/` in the bound repository. Export requires a new destination;
choose another name if `graph-export` already exists.

The current analysis profile reports declarations and containment. It does not
claim complete method-call relationships or runtime behavior.

## 5. Keep watching when wanted

```text
axge watch
```

Watch stays in the foreground and processes registered projects as source changes.
Ctrl+C requests shutdown. `axge serve` is the lower-level foreground daemon entry;
it also requires registration and folder bindings first. Run one writer for the
selected Axiom home; stop an existing watcher before running a separate build.

## If setup is missing

| Message | Next step |
|---|---|
| `build requires --solution` | Run `axge solution list`, then pass the ID with `--solution`; register first if the list is empty. |
| `machine-local bindings document is required` | Complete steps 2 and 3 in the same Axiom home before build/watch/serve. |
| Repository root does not exist | Correct the absolute folder in the local bindings file and preview registration again. |
| Existing binding conflicts | Check the existing registration and selected Axiom home; do not overwrite another project's binding. |

See the [command reference](reference/axiom-graphd.md) for additional flags and
the [workflow guide](workflow-commands.md) for graph publication details.

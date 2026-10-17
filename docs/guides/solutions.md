# Solutions and projects

A solution groups the source you want to query together. A project selects a
source folder in one repository. Repository IDs are logical names; a local
bindings document maps each name to this machine's checkout.

## Start with one project

The [quickstart](../21-INSTALLATION-QUICKSTART.md) provides the smallest working
`solution.json` and `bindings.json`. Register them with a preview, then apply:

```text
axge solution register --config solution.json --bindings bindings.json --dry-run
axge solution register --config solution.json --bindings bindings.json --apply
axge solution list
```

A `.sln` or `.csproj` filename does not register an Axiom solution automatically.
Use the solution's `id` in build, query and export commands.

## Add projects in separate folders

This native registration input declares two projects in one repository:

```json
{
  "id": "store",
  "profile": "default",
  "catalog_host_repo": "store-repo",
  "projects": [
    { "id": "api", "repo_id": "store-repo", "path": "src/api" },
    { "id": "web", "repo_id": "store-repo", "path": "src/web" }
  ]
}
```

Both folders must exist. Project paths are repository-relative and must not
overlap: selecting `src` and `src/api` as separate projects would be ambiguous.
`.` selects a whole repository on builds containing the whole-root correction.
On an older installed build, use an existing source subfolder instead.

For several repositories, give each its own `repo_id`, add its absolute checkout
path to the private bindings file, and choose a member as `catalog_host_repo`.
The catalog records the project snapshots read together by queries and exports.

## Keep bindings local

`solution.json` describes portable IDs and relative folders. `bindings.json`
contains absolute machine paths and should stay outside version control.
Registration persists accepted bindings under the selected Axiom home.
Reuse that home for build, watch, query and gateway access.

## Work on one project

```text
axge build --solution store --project api --json
axge export --solution store --project api --out api-graph --format html --json
```

A scoped build retains verified snapshots for unselected projects in the
complete solution catalog. Missing or corrupt members are reported explicitly.
Watch remains instance-wide; inspect `solution list` before starting it.

## Remove a registration

```text
axge solution remove --solution store --dry-run
axge solution remove --solution store --apply
```

Review the preview first. Removing registration is separate from editing your
source or uninstalling Axiom. See [files and state](files-and-state.md).

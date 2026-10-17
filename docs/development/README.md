# Develop the engine

This section is for contributors building or changing Axiom. For installed
commands, start with the [product documentation](../README.md).
Load the workspace/repository `AGENTS.md`, `Development.md`, selected task and
immutable specification pin before changing source.

## What is in crates?

A Rust crate is a compiled package or module. These directories contain the
production implementation; their tests live alongside it.

| Crate | Responsibility |
|---|---|
| `graph-core` | Domain types, errors, configuration, paths and membership |
| `graph-store` | SQLite storage, migrations, registry and graph records |
| `graph-watch` | Inventory, change hints, ignore rules and recovery |
| `graph-queue` | Durable work, leases, retry and scheduling |
| `graph-analyze` | Parser adapters, source facts and symbol identities |
| `graph-export` | Canonical publication, manifests, snapshots and reader validation |
| `axiom-graphd` | Native daemon, graph commands, workflow facade and HTML renderer |
| `axiom` / `axiom-cli` | Core installation/operations engine and its executable wrapper |
| `platform` / `config` / `migration` / `bootstrap` | Native lifecycle, configuration and managed setup |

The separate distribution repository delivers `axm` and wraps this engine.
Removing crates would remove product functionality. They do not need to be
visible in an installed-user tutorial.

## Build from source

Install the pinned Rust 1.85.0 toolchain and the native platform toolchain
appropriate for it. From this repository:

```text
cargo +1.85.0 build --locked --release -p axiom-cli -p axiom-graphd
```

This creates the core `axiom` and `axiom-graphd` executables in `target/release/`
(with `.exe` on Windows). A source build is not an installation and does not
add the public launcher names to PATH.

## Verify a change

```text
cargo +1.85.0 fmt --all -- --check
cargo +1.85.0 clippy --locked --all-targets -- -D warnings
cargo +1.85.0 test --locked --all-targets -- --test-threads=1
python tools/check-doc-status.py
python tools/check-doc-status.py --self-test
```

Dependency auditing uses cargo-deny 0.20.2, built separately with Rust 1.88.0.
The product compiler remains Rust 1.85.0. Run the pinned auditor's `check`
command; advisory parser failures are failures, not skipped advisories.
Keep `Cargo.lock` locked and do not regenerate it during editorial changes.

## Examples and regression fixtures

The [first-graph example](../../examples/first-graph/README.md) runs the native
engine in an isolated home and copies sample source before building it.
The [benchmark harness](../../benchmarks/README.md) has a separate, explicitly
modeled scope; its timings are not production engine benchmarks.

Synthetic [solution](guide-fixtures/solutions.md),
[editing](guide-fixtures/human-workflow.md) and
[diagnostic](guide-fixtures/troubleshooting.md) scenarios are replayed by the
Rust tests. These are test inputs, separate from captured public demo output.

## Why keep evidence?

`evidence/` and `.delivery/` retain task verification, provenance and delivery
records required by repository governance. They are engineering audit material,
rather than a user's setup procedure. Keep historical files intact. Read each
record's source revision and platform scope before reusing a claim; an old
`not_run` result is not a statement about the current release.

Shared contracts remain in the immutable `axiom-specs` pin. This guide does
not redefine schemas, native guard behavior, release policy or task acceptance.

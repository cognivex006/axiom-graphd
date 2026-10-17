# Index workload benchmark harness

This directory is for contributors studying workload behavior. To see the
actual graph engine in use, run the [first-graph demo](../examples/first-graph/README.md).

## Scope

`index_workload.py` creates a synthetic on-disk dataset and measures a
deterministic **Python model indexer**. It is not the Rust production pipeline.
Its wall time, CPU, resident memory, reads and artifact sizes describe this
harness on the machine running it. They must not be advertised as engine
throughput, query latency or an idle-daemon budget.

## Run

```text
python benchmarks/index_workload.py --tier small --json-out report.json
python benchmarks/index_workload.py --projects 1 --files-per-project 3
```

Tiers generate 2,001, 10,000 or 50,010 files. Overrides are useful for smoke
runs. The temporary dataset is removed on exit unless `--keep` is selected.
The report labels unmeasured production budgets `not_run` and `claimed: false`.

## Interpret a report

The harness compares incremental model results with its cold-rebuild oracle
and checks stale updates, same-size rewrites, unknown symbols and empty projects.
Passing these controls validates the model run; it does not certify production
analyzer accuracy. Record hardware, environment and dataset identity with any
numbers you share. Historical task results remain in engineering evidence.

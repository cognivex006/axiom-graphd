# Troubleshooting test scenario

Internal behavioral fixture, replayed by the Rust integration suite. This is synthetic test input, not captured command output or an installation guide.

<!-- BEGIN TROUBLESHOOTING EXAMPLE -->
```json
{
  "schema_version": 2,
  "solution_id": "demo-solution",
  "conditions": [
    {
      "condition": "missing-daemon",
      "first_signal": "watcher-disabled",
      "error_code": null,
      "exit_code": null,
      "retryable": false,
      "recovery": "start-the-daemon-or-read-the-last-published-generation"
    },
    {
      "condition": "stale-graph",
      "first_signal": "full-scan-required",
      "error_code": "NOT_READY",
      "exit_code": 4,
      "retryable": true,
      "recovery": "wait-for-reconcile-or-a-full-inventory-scan"
    },
    {
      "condition": "corrupt-shard",
      "first_signal": "export-integrity",
      "error_code": "export-integrity",
      "exit_code": null,
      "retryable": false,
      "recovery": "keep-the-last-valid-generation-and-rebuild-from-source"
    },
    {
      "condition": "queue-delay",
      "first_signal": "queue-retry-wait",
      "error_code": "RATE_LIMITED",
      "exit_code": 7,
      "retryable": true,
      "recovery": "wait-for-the-bounded-backoff"
    },
    {
      "condition": "unsupported-host",
      "first_signal": "sqlite-network-storage-unsupported",
      "error_code": "SQLITE_NETWORK_STORAGE_UNSUPPORTED",
      "exit_code": 9,
      "retryable": false,
      "recovery": "move-axiom-home-to-local-disk"
    }
  ]
}
```
<!-- END TROUBLESHOOTING EXAMPLE -->

The portable model uses a `binding_key`; the internal AXIOM_HOME/config/registry.json location is distinct from the native registration input and persisted config/bindings.json.

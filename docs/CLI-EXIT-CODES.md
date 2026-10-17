# CLI exit codes

Use the exit status to decide whether a command succeeded. With `--json`,
read the error `code`, `message`, `retryable` and `details` fields as well.
Exit code `1` is reserved. `axge --help` lists the engine exit codes.

<!-- BEGIN GENERATED EXIT CODES -->
| Code | Meaning | Raised when |
| ---- | ------- | ----------- |
| 0 | success | The command completed successfully. |
| 1 | (unassigned) | Reserved; axiom never returns it. |
| 2 | validation | Flags, arguments or configuration were rejected. |
| 3 | not found | A referenced entity does not exist. |
| 4 | not ready/stale | The service is not ready, or the requested state is stale. |
| 5 | authorization | The caller is not authenticated or not permitted for this scope. |
| 6 | conflict | The request conflicts with current state. |
| 7 | timeout/busy | A bounded wait expired or the resource is busy; retry may work. |
| 8 | I/O internal | An I/O failure occurred, or an internal Axiom defect was detected. |
| 9 | incompatible | Input, runtime or schema is incompatible with this binary. |
| 10 | lock unavailable | A required cross-process lock is held by another owner. |
| 20 | partial multi-repo operation | A multi-repository operation partially succeeded. |
<!-- END GENERATED EXIT CODES -->

Compact summary:

    0 success, 2 validation, 3 not found, 4 not ready/stale, 5 authorization, 6 conflict, 7 timeout/busy, 8 I/O internal, 9 incompatible, 10 lock unavailable, 20 partial multi-repo operation

## Error names

The following error names map to process exit statuses.

| Error code | Exit code |
| ---------- | --------- |
| VALIDATION_ERROR | 2 |
| UNAUTHENTICATED | 5 |
| FORBIDDEN | 5 |
| NOT_FOUND | 3 |
| CONFLICT | 6 |
| INCOMPATIBLE_INPUT | 9 |
| RATE_LIMITED | 7 |
| NOT_READY | 4 |
| WRITER_ALREADY_RUNNING | 10 |
| WORKTREE_BUSY | 6 |
| MIGRATION_CHECKSUM_MISMATCH | 9 |
| MIGRATION_ORDER_CONFLICT | 9 |
| SCHEMA_NEWER_THAN_BINARY | 9 |
| SQLITE_UNSUPPORTED_VERSION | 9 |
| SQLITE_NETWORK_STORAGE_UNSUPPORTED | 9 |
| UNSAFE_HOME_PATH | 2 |
| UNSAFE_PORTABLE_PATH | 2 |
| CONFIG_INVALID | 2 |
| SHUTTING_DOWN | 4 |
| INTERNAL | 8 |

`RATE_LIMITED`, `NOT_READY`, `CONFLICT`, `WORKTREE_BUSY` and `SHUTTING_DOWN`
are retryable by default; every other code is not.

The wire spelling of each `ErrorCode` must not change without a contract change
in `axiom-specs`; the numeric exit status is likewise frozen.

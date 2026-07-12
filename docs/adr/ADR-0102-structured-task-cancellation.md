# ADR-0102: Structured Task Cancellation

Status: Superseded by ADR-0104

## Decision

Neu originally added the compiler-recognized operation `cancel(task)`. Its
source spelling is superseded by ADR-0104, which defines `task.cancel()`.
The operation requests
cancellation of an owned child task and returns `Unit`; it never forcefully
terminates execution. The task handle remains owned by the caller and may be
awaited once after cancellation to observe the cancellation result.

Cancellation is idempotent. Requests for completed or already-cancelled tasks
have no additional effect. A task observes cancellation at spawn completion,
`await`, scope cleanup, and other accepted suspension points. A running task is
not interrupted between suspension points.

Cancellation flows from a scope to all unfinished descendants and from a
failed child to unfinished siblings through the owning scope. Cleanup runs
before a cancelled task completes. A cancellation request cannot detach a task
or bypass ownership destruction. Cancellation does not create a new source
reference, move, lifetime, or thread syntax.

`await` on a cancelled task reports a deterministic cancellation result through
the task failure boundary; no exception or standard-library error type is
introduced. Invalid task handles are diagnosed statically when known and trap
through the compiler-owned runtime otherwise. Cancellation provenance and the
requesting source span are preserved in HIR and MIR.

OS signals, forceful termination, cancellation tokens, cancellation callbacks,
timeouts, I/O interruption, and public runtime cancellation types remain
deferred.

## Dependencies

- ADR-0101
- ADR-0062
- ADR-0100

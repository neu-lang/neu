# ADR-0104: Member Task Cancellation

Status: Accepted

## Question

How is an owned task cancellation request expressed without adding a keyword
or exposing a runtime cancellation type?

## Decision

ADR-0104 supersedes the source-surface portion of ADR-0102. Neu uses the
compiler-recognized member operation `task.cancel()`. The receiver must be an
owned `Task<T>` value, the call takes no arguments, returns `Unit`, and retains
the task handle in the caller. Cancellation remains an idempotent request and
does not forcefully terminate execution.

The former free operation `cancel(task)` is not accepted. It is diagnosed as
an invalid task operation; no compatibility alias or new keyword is added.

All cancellation behavior from ADR-0102 remains unchanged: cancellation is
observed at accepted suspension and cleanup points, propagates through the
owning scope, preserves cleanup, and can be observed by one subsequent
`await(task)`. HIR and MIR retain the receiver task identity and cancellation
source span.

## Consequences

- Parser and type checking recognize member cancellation only for `Task<T>`.
- Missing arguments, extra arguments, and non-task receivers are rejected
  before HIR lowering.
- The private runtime and ABI continue to use the existing task cancellation
  boundary; no public layout, token, reference, or FFI surface is introduced.
- Existing examples and source-facing documentation use `task.cancel()`.

## Dependencies

- Supersedes the source spelling in ADR-0102.
- Preserves cancellation semantics from ADR-0102 and scope semantics from
  ADR-0101.
- Preserves ownership and capability rules from ADR-0062 and ADR-0037.

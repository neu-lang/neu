# ADR-0123: Precondition Failure

Status: Accepted; revises ADR-0111 for precondition failures. Its compile-time
follow-up is decided by ADR-0124.

## Question

How does library code report a violated documented precondition, such as the
`lower <= upper` requirement of `clamp` in ADR-0112, so that the failure is
diagnosed and the program stops?

## Decision

`precondition(condition, message)` is a built-in operation, callable from any
function. `condition` must be `Bool` and `message` must be `String`. It returns
no value and is used as a statement.

- Evaluation: `condition` and then `message` are evaluated left to right, as
  call arguments are. The check runs after both are evaluated.
- Holds: the call has no effect, and execution continues after it.
- Fails: the program writes `neu: precondition failed: `, then the message's
  bytes, then a newline, to stderr, and then traps with a user trap code. The
  process ends by signal, as for other runtime traps. Nothing after the call
  runs.
- Source span: the runtime message carries no source span. The compile-time
  diagnostics for this operation (argument count, condition type, and message
  type) are reported at the call expression.
- Optimization: a precondition is not removed, and it is not moved past the
  evaluation of its arguments.
- Scope: ADR-0111 still governs `panic`, `abort`, `assert`, `debug_assert`,
  `unreachable`, `todo`, and `unimplemented`. `assert` and `fail` remain
  test-only, as ADR-0107 requires for `stdlib/core`.

## Consequences

ADR-0112's invalid `clamp` range is reported through this operation. Known
constant arguments that violate a precondition are not diagnosed at compile
time by this ADR.

## Required follow-up

- A compile-time diagnostic for a precondition whose arguments are constants
  needs its own decision, because it requires evaluating a callee's
  precondition at the call site.
- Whether runtime messages should carry a source span needs its own decision.

# Diagnostic Contract

This document defines the compiler-wide diagnostic shape required by ADR-0015 before any compiler stage emits real errors. It is a contract for diagnostics infrastructure and tests, not a language semantic rule.

## Source Of Truth

Diagnostic behavior must cite:

- `docs/SPEC.md`
- the relevant file under `docs/adr/`
- the task or implementation phase that authorizes harness-only behavior

If the source of truth is ambiguous, main tasks must file an ambiguity report instead of creating a diagnostic expectation.

## Required Diagnostic Shape

Every user-facing diagnostic must provide:

- severity
- message
- primary span

Every diagnostic may provide:

- secondary span entries
- notes
- safe suggestion entries

The primary span identifies the user's most actionable source location. A secondary span explains related source locations only when it improves the diagnostic. Notes provide short supporting explanations. A safe suggestion must preserve the accepted semantics from `docs/SPEC.md` and `docs/adr/`.

## Severity

The initial severity model is:

- error
- warning
- note

Compiler stages may use only severities documented here unless a later implementation phase extends the contract.

## Root Cause And Follow-On Diagnostics

This section is a proposed rule for the parser. It is not yet confirmed by the project owner.

- A failure is reported once, by the innermost construct that detects it.
- An enclosing construct must not add a diagnostic for a sub-construct failure that has already emitted one.
- Recovery skips only to the boundary of the failed construct, such as the closing brace of a lambda body, so enclosing constructs do not re-read its tokens.
- Independent failures in separate statements each keep their own diagnostic.

## User-Facing Wording

Diagnostics must describe source-level concepts. They must not expose internal compiler jargon such as pass names, internal representation names, implementation IDs, or backend internals unless the user explicitly requested an internal diagnostic mode in a future implementation phase.

## Snapshot Expectations

Diagnostic snapshots must be stable enough for review. A snapshot should record:

- severity
- message
- primary span
- secondary span entries, when present
- notes, when present
- safe suggestion entries, when present
- source-of-truth citation

Snapshots must not be used to encode guessed language behavior. If a compiler rule is not specified, the expected diagnostic is blocked until the specification or ADRs resolve it.

## Concurrency Diagnostics

- `AwaitVoidResult` (ADR-0101): an `await` of a task with no result, bound to a
  local. Such an `await` may be used as a statement. Reported at the `await` call.
- `channel_op_outside_suspend` (ADR-0122): a channel `send` or `receive` outside
  a `suspend` function. Reported at the call expression.
- Runtime deadlock (ADR-0101, ADR-0122): when no task can run, the program writes
  `neu: deadlock: no task can run` to stderr and traps. This is a runtime message,
  not a compile-time diagnostic.
- Runtime precondition failure (ADR-0123): when a `precondition` condition is
  false at run time, the program writes `neu: precondition failed: ` with the
  message and a newline to stderr, and traps. Its compile-time argument checks
  are reported at the call.
- `precondition_violation` (ADR-0124): a call whose leading `precondition` fails
  for constant arguments. Reported at the call expression. Other preconditions
  are checked at run time only.
- Runtime division by zero (SPEC, ADR-0043): when a divisor is zero at run time,
  the program writes `neu: division by zero` to stderr and traps. Statically known
  zero divisors are the compile-time `division_by_zero` diagnostic instead.

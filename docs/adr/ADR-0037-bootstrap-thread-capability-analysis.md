# ADR-0037: Bootstrap Thread Capability Analysis

Status: Accepted

## Question

What bootstrap thread-safety capability semantics are sufficient for M0024
without defining coroutine syntax, task-spawn syntax, synchronization APIs, or
generic capability enforcement?

## Competing Designs

1. Full source-level concurrency model now.
2. Enforce parsed generic bounds such as `Send` and `Share` immediately.
3. Metadata-only boundary and capture records with a small capability catalog.
4. Defer all thread-safety analysis until coroutine syntax exists.

## Trade-offs

Full source-level concurrency would exercise the user-facing model, but depends
on syntax and runtime decisions assigned to later milestones.

Immediate generic-bound enforcement would reuse existing parsed capability-bound
syntax, but ADR-0032 explicitly defers generic constraint enforcement until its
ownership and thread-capability inputs are ready.

Metadata-only boundary and capture records let M0024 validate the safety rule
and diagnostics while keeping source syntax and runtime scheduling out of
scope.

Full deferral preserves optionality, but leaves the thread-safety milestone
without testable progress.

## Recommended Choice

Use a metadata-only bootstrap thread-capability model.

M0024 defines exactly two capability names:

- `Send`: a value may be transferred across an approved concurrent boundary.
- `Share`: a value may be shared across an approved concurrent boundary without
  exclusive transfer.

Capability derivation in M0024 is type-category based:

- `Bool`, `Int`, `Unit`, and `Null` satisfy `Send` and `Share`.
- `String` satisfies `Send` but not `Share`.
- A nullable type satisfies a capability only when its non-null base type
  satisfies that capability.
- Current-module nominal user-defined types satisfy neither `Send` nor `Share`.
- Generic parameter types satisfy neither capability in M0024.
- Unsupported, unresolved, or absent type information satisfies no capability.

M0024 boundary inputs are compiler side-table records, not source syntax. A
boundary record has a boundary node and ordered capture records. A capture
record has a capture node, captured local binding, captured type, and required
capability (`Send` or `Share`).

The capability analysis reports `missing_thread_capability` when a capture's
type does not satisfy the required capability. The primary diagnostic span is
the capture node. The secondary span is the boundary node. The diagnostic must
name the missing capability and the captured type category when available.

No source-level task spawning, detached threads, closures, async blocks,
coroutine bodies, synchronization primitives, atomics, locks, generic
capability satisfaction, user-declared capability implementations, or unsafe
capability overrides are introduced by M0024.

Because there are no approved synchronization abstractions in M0024, shared
mutable state is not accepted through a `Share` capture. Mutable captures may
only be modeled as `Send` transfers, and only when the captured type satisfies
`Send`; share-by-mutable-reference semantics are deferred.

Unspecified concurrency forms remain rejected or unsupported by the existing
syntax and semantic layers. M0024 must not add parser support for concurrency
constructs.

## Downstream Consequences

- M0024 can implement a capability representation and boundary checker without
  source-level concurrency syntax.
- M0025 may consume the same capability facts when coroutine or task-scope
  semantics are accepted.
- ADR-0032 remains in force: generic capability bounds are recorded but not
  enforced by M0024.
- User-declared `Send` or `Share`, synchronization APIs, and unsafe capability
  assertions require later ADRs.

## Dependencies

- ADR-0014
- ADR-0023
- ADR-0032
- ADR-0035
- ADR-0036

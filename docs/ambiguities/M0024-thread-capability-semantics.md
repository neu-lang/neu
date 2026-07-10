# Ambiguity Report: M0024 Thread Capability Semantics

## Metadata

- Report ID: `M0024-thread-capability-semantics`
- Related Task: `M0024-001`
- Related Milestone: `M0024`
- Filed By: `main task`
- Date: `2026-07-11`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0014-thread-safety-and-data-race-freedom.md`
- Milestone:
  - `docs/milestones/M0024-thread-safety-capability-analysis.md`

## Exact Ambiguous Text Or Missing Rule

```text
The language uses compile-time send/share capabilities, derived where sound and
explicitly declared where necessary. Shared mutable state requires safe
synchronization abstractions.
```

Missing rules:

- Which capability names exist in the bootstrap subset.
- Which accepted type identities satisfy `Send`, `Share`, both, or neither.
- Whether capability derivation depends on ownership, borrowing, mutability, or
  type identity in M0024.
- Which records represent approved concurrent boundaries.
- Which records represent captured values crossing a boundary.
- What missing-capability diagnostics are named and where they point.
- How shared mutable state is represented before synchronization abstractions
  exist.
- Whether task spawning, detached work, async, closure captures, function calls,
  or generic bounds participate in M0024.

## Competing Interpretations

1. Define a provisional capability catalog and classify all current primitive
   and nominal identities immediately.
2. Treat capability bounds from generic syntax as actual `Send`/`Share`
   requirements now.
3. Use a metadata-only boundary and capture model with explicit required
   capabilities and type-category satisfaction.
4. Defer all thread-safety analysis until coroutine/task syntax exists.

## Why Guessing Is Unsafe

- Capability satisfaction controls whether values can cross concurrency
  boundaries, directly affecting data-race freedom.
- Treating nominal user-defined values as sendable or shareable without
  authority can make unsafe cross-task sharing appear accepted.
- Treating generic bounds as capability enforcement would contradict ADR-0032's
  explicit deferral.
- Diagnostics must name missing capabilities and capture sites under M0024.

## Affected Work

- Tasks blocked:
  - `M0024-002` and later implementation tasks.
- Milestones affected:
  - `M0024`
  - `M0025`
- Tests blocked:
  - Positive capability fixtures.
  - Negative cross-boundary capture fixtures.
  - Diagnostic snapshots for missing capability and capture site.
- Implementation areas blocked:
  - Capability representation.
  - Boundary checking.
  - Cross-boundary capture diagnostics.

## Recommended Resolution Path

- [x] main-task language review determines whether existing text resolves it.
- [x] main-task semantic design drafts ADR or spec revision if new semantics are required.
- [x] main-task adversarial check reviews soundness risk.
- [x] main-task diagnostics check reviews diagnostic consequences.
- [x] main-task simplicity check reviews complexity.
- [x] main task approves final resolution.

## Temporary Rule

Implementation may proceed on M0024 thread-safety capability analysis against
ADR-0037's metadata-only bootstrap capability and boundary-checking subset.

## Resolution

- Decision:
  - Accepted `docs/adr/ADR-0037-bootstrap-thread-capability-analysis.md`.
- Source of truth updated:
  - `docs/adr/ADR-0037-bootstrap-thread-capability-analysis.md`
  - `docs/SPEC.md`
- Date resolved:
  - 2026-07-11

# Ambiguity Report: M0023 Borrow Lifetime Semantics

## Metadata

- Report ID: `M0023-borrow-lifetime-semantics`
- Related Task: `M0023-001`
- Related Milestone: `M0023`
- Filed By: `main task`
- Date: `2026-07-11`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0002-borrowing-semantics.md`
  - `docs/adr/ADR-0003-lifetime-model.md`
- Milestone:
  - `docs/milestones/M0023-borrow-and-lifetime-analysis.md`

## Exact Ambiguous Text Or Missing Rule

```text
The language allows either shared immutable borrows or one exclusive mutable
borrow, using Kotlin-like surface syntax and strong inference.

Lifetimes are inferred by default. Explicit lifetime parameters are required
only where needed for public generic APIs and unsafe-adjacent abstractions.
```

Missing rules:

- Which syntax or expression forms create shared borrows.
- Which syntax or expression forms create exclusive mutable borrows.
- Whether M0023 uses explicit borrow operators, inferred borrow sites, or both.
- Which local values can be borrowed in the bootstrap subset.
- What conflict windows are used for synchronous local borrow analysis.
- What constitutes a lifetime escape in the bootstrap subset.
- Which diagnostics, primary spans, secondary spans, and recovery behavior are
  required for borrow conflicts and lifetime escape.
- Whether method calls, member access, returns, assignment, `when`, closures,
  and function arguments participate in the initial borrow subset.

## Competing Interpretations

1. Import Rust-like `&` and `&mut` expression syntax for M0023.
2. Infer borrows only at existing expression uses with no explicit syntax.
3. Define a narrow metadata-only borrow model over future accepted borrow nodes
   and block syntax implementation.
4. Defer all borrow analysis until HIR and function calls exist.

## Why Guessing Is Unsafe

- Borrow creation and conflict windows determine memory safety.
- A wrong lifetime-escape rule can accept dangling references or reject valid
  safe programs.
- Diagnostics are semantic obligations under ADR-0015 and must identify the
  conflicting borrow sites.
- Choosing syntax now without authority can conflict with the Kotlin-like
  surface grammar.

## Affected Work

- Tasks blocked:
  - `M0023-002` and later implementation tasks.
- Milestones affected:
  - `M0023`
  - `M0024`
  - `M0025`
- Tests blocked:
  - Positive shared-borrow fixtures.
  - Negative mutable/shared conflict fixtures.
  - Negative lifetime escape fixtures.
  - Diagnostic snapshots for conflicting borrow sites.
- Implementation areas blocked:
  - Borrow analysis pass.
  - Lifetime validity checks.
  - Borrow conflict diagnostics.

## Recommended Resolution Path

- [x] main-task language review determines whether existing text resolves it.
- [x] main-task semantic design drafts ADR or spec revision if new semantics are required.
- [x] main-task adversarial check reviews soundness risk.
- [x] main-task diagnostics check reviews diagnostic consequences.
- [x] main-task simplicity check reviews complexity.
- [x] main task approves final resolution.

## Temporary Rule

No implementation may proceed on M0023 borrow or lifetime analysis until the
source of truth defines a bootstrap borrow and lifetime subset.

## Resolution

- Decision:
  - ADR-0036 accepted. M0023 has a metadata-only bootstrap borrow model with
    shared/exclusive borrow records, exact region-equality overlap,
    lifetime-escape records, `borrow_conflict`, and `lifetime_escape`
    diagnostics. Source-level borrow syntax and reference types remain
    deferred.
- Source of truth updated:
  - `docs/adr/ADR-0036-bootstrap-borrow-and-lifetime-analysis.md`
  - `docs/SPEC.md`
- Date resolved:
  - `2026-07-11`

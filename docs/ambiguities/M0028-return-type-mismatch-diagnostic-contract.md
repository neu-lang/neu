# Ambiguity Report: M0028 Return-Type Mismatch Diagnostic Contract

## Metadata

- Report ID: `M0028-RETURN-TYPE-MISMATCH-DIAGNOSTIC-CONTRACT`
- Related Task: `M0028-016`
- Related Milestone: `M0028`
- Filed By: `main-task diagnostics review`
- Date: `2026-07-11`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`, ADR-0015 and ADR-0041 summaries.
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0041-bootstrap-function-call-and-return-semantics.md`
  - `docs/adr/ADR-0050-bootstrap-straight-line-return-diagnostics.md`
- Milestone:
  - `docs/milestones/M0028-executable-expression-frontend-completion.md`

## Exact Ambiguous Text Or Missing Rule

```text
ADR-0041 requires return_type_mismatch when an explicit return value does not
match the declared return type. It does not define the primary span, recovery,
or whether an unresolved/deferred return expression also receives this
diagnostic. ADR-0050 defines only missing_return and unreachable_return.
```

## Competing Interpretations

1. Attach return_type_mismatch to the return statement and report it even when
   the expression has no type.
2. Attach it to the return expression and emit it only when both types are
   known.
3. Attach it to the function declaration because the signature is violated.

## Why Guessing Is Unsafe

- Statement or declaration attachment obscures the incompatible value.
- Diagnosing unknown expression types as mismatches can cascade an unresolved
  or unsupported expression into a false type error.
- Recovery determines whether HIR may receive a typed return terminator.

## Affected Work

- Tasks blocked:
  - `M0028-016`
- Milestones affected:
  - `M0028`
- Tests blocked:
  - return expression mismatch and recovery fixtures.
- Implementation areas blocked:
  - `crates/compiler/src/type_check.rs`

## Recommended Resolution Path

- [x] main-task language review determines whether existing text resolves it.
- [x] main-task semantic design drafts an ADR defining provenance and recovery.
- [x] main-task adversarial check reviews recovery into HIR.
- [x] main-task diagnostics check reviews interaction with unresolved values.
- [x] main-task simplicity check reviews the smallest type-checker boundary.
- [x] main task approves the final resolution.

## Temporary Rule

No implementation may proceed on return expression type validation until
accepted authority defines the diagnostic contract.

## Resolution

- Decision:
  - ADR-0054 defines expression provenance, known-types-only emission, and
    no-typed-return recovery.
- Source of truth updated:
  - `docs/adr/ADR-0054-bootstrap-return-type-mismatch-diagnostics.md`
  - `docs/SPEC.md`
- Date resolved:
  - `2026-07-11`

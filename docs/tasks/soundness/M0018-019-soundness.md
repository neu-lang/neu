# Soundness Report: M0018-019

## Metadata

- Task ID: `M0018-019`
- Milestone: `M0018`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-019-accepted-local-initializer-checks.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/newlang/src/type_check.rs`
  - `crates/newlang/tests/type_check.rs`
  - `docs/tasks/M0018-019-accepted-local-initializer-checks.md`
- Ordinary test results:
  - `cargo test --workspace --all-targets`: pass, 163 tests

## Safety Invariants Checked

- [x] Ownership cannot be bypassed.
- [x] Moved values cannot be reused.
- [x] Shared and exclusive borrows cannot conflict.
- [x] Borrowed data cannot outlive its owner.
- [x] Nullability refinements cannot be used after invalidation.
- [x] Thread send/share capabilities are enforced.
- [x] Coroutine scopes cannot outlive allowed ownership or borrow lifetimes.
- [x] Borrows across suspension are rejected unless proven safe by accepted semantics.
- [x] Unsafe and FFI boundaries do not weaken safe-code guarantees.
- [x] Diagnostics do not hide or misstate safety failures.

## Attacks Attempted

```text
Attack: Local initializer uses a resolved name supplied by name resolution and known-symbol typing.
Expected result: Compatible annotation and initializer types record an assignment check.
Actual result: Positive test records assignment checks for literal, resolved-name, and grouped resolved-name initializers.
Source of truth: ADR-0027 includes resolved name expression typing and local declarations with known explicit annotations.
Outcome: pass

Attack: Local initializer has a known accepted expression type that conflicts with the explicit annotation.
Expected result: A type_mismatch diagnostic is reported on the initializer expression.
Actual result: Negative test reports one TypeMismatch diagnostic on the bad initializer with expected String and actual Int.
Source of truth: ADR-0027 assignment compatibility and diagnostic primary span rules.
Outcome: pass

Attack: Unknown annotation or unresolved initializer pressures the helper into inference.
Expected result: The declaration or initializer is skipped; no assignment check is synthesized.
Actual result: Negative test confirms unknown annotation and untyped initializer declarations have no assignment checks.
Source of truth: ADR-0027 limits M0018 to known explicit annotation types and known expression types.
Outcome: pass

Attack: Helper silently runs name resolution, derives symbol types, or checks assignment statements.
Expected result: It consumes only supplied side tables and declaration metadata.
Actual result: Implementation accepts ResolutionTable and KnownSymbolType inputs, delegates accepted expression typing, and never invokes name-resolution builders or assignment statement checking.
Source of truth: Task out-of-scope list, ADR-0026 separation of name resolution, ADR-0027 typed side-table model.
Outcome: pass

Attack: Helper broadens M0018 expression typing to unsupported calls, members, binary, unary, or value-producing if expressions.
Expected result: Unsupported expression forms remain untyped unless represented by accepted metadata.
Actual result: Implementation consumes only parsed literals, grouped expressions, resolved names, and primitive annotations.
Source of truth: ADR-0027 explicit deferrals.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/type_check.rs`
- Tests run:
  - `cargo test --workspace --all-targets`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-019-accepted-local-initializer-checks.md`
- Result:
  - Pass.

## Findings

No findings.

## Ambiguities

- None.

## Decision

Pass. The change checks local declaration initializers only when both the explicit primitive annotation and accepted initializer expression type are known, and it does not invent inference, resolution, or unsupported expression semantics.

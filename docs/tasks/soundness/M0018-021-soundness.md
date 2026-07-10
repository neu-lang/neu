# Soundness Report: M0018-021

## Metadata

- Task ID: `M0018-021`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-021-unresolved-type-rule-diagnostics.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Changed files:
  - `crates/newlang/src/type_check.rs`
  - `crates/newlang/tests/type_check.rs`
- Ordinary test results:
  - `cargo test --workspace --all-targets`: pass, 166 tests

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
Attack: A local declaration without accepted annotation authority receives a synthesized type.
Expected result: No declaration signature or assignment check is produced; missing_annotation_type is reported.
Actual result: Tests confirm missing annotation and unknown primitive annotation declarations produce unresolved diagnostics and no successful entries.
Source of truth: ADR-0027 unresolved_type_rule and local declaration known explicit annotation requirement.
Outcome: pass

Attack: A resolved name with no supplied known type is silently treated as typed.
Expected result: No expression type is produced; missing_resolved_name_type is reported.
Actual result: Tests confirm unknown resolved symbols produce unresolved diagnostics and no expression type.
Source of truth: ADR-0027 name expression typing requires known type metadata.
Outcome: pass

Attack: Missing type authority causes the checker to run name resolution or infer types.
Expected result: The checker records unresolved diagnostics only.
Actual result: Implementation consumes existing ResolutionTable, KnownSymbolType, and parser metadata; it does not call name-resolution builders or infer types.
Source of truth: ADR-0026 separates name resolution, and ADR-0027 forbids inference for M0018.
Outcome: pass

Attack: This task begins typing unsupported expressions.
Expected result: Unsupported expression forms remain out of scope.
Actual result: Implementation touches only accepted helper paths for local declarations and resolved names.
Source of truth: ADR-0027 explicit deferrals.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/type_check.rs`
- Tests run:
  - `cargo test --workspace --all-targets`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-021-unresolved-type-rule-diagnostics.md`
- Result:
  - Pass.

## Findings

No findings.

## Ambiguities

- None.

## Decision

Pass. The change reports missing type authority for accepted M0018 constructs without adding inference, resolution, or unsupported expression typing.

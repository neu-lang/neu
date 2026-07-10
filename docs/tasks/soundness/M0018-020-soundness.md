# Soundness Report: M0018-020

## Metadata

- Task ID: `M0018-020`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-020-type-rule-diagnostic-contract.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
- Changed files:
  - `crates/compiler/src/type_check.rs`
  - `crates/compiler/tests/type_check.rs`
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
Attack: Unsupported expression diagnostics are added by typing unsupported expressions.
Expected result: This task only represents unsupported diagnostics and does not emit them from traversal.
Actual result: Implementation adds enum variants and constructors only; no expression traversal or typing behavior was added.
Source of truth: ADR-0027 explicit deferrals and task out-of-scope list.
Outcome: pass

Attack: Unresolved diagnostics synthesize expected or actual types.
Expected result: Unresolved diagnostics carry a stable rule identifier and node without expected or actual type payloads.
Actual result: Tests confirm expected_type and actual_type are None.
Source of truth: ADR-0027 unresolved_type_rule diagnostic contract.
Outcome: pass

Attack: Unsupported diagnostics synthesize expected or actual types.
Expected result: Unsupported diagnostics carry a stable rule identifier and node without expected or actual type payloads.
Actual result: Tests confirm expected_type and actual_type are None.
Source of truth: ADR-0027 unsupported_type_rule diagnostic contract.
Outcome: pass

Attack: Existing ambiguity and mismatch diagnostics are weakened.
Expected result: Ambiguity and mismatch diagnostics remain distinct and existing tests continue to pass.
Actual result: Existing M0018 tests pass; ambiguity rules remain comparable through the stable type-rule diagnostic wrapper.
Source of truth: ADR-0027 diagnostic list.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/type_check.rs`
- Tests run:
  - `cargo test --workspace --all-targets`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-020-type-rule-diagnostic-contract.md`
- Result:
  - Pass.

## Findings

No findings.

## Ambiguities

- None.

## Decision

Pass. The task adds diagnostic representation only and does not make unsupported or unresolved constructs type-check.

# Soundness Report: M0018-023

## Metadata

- Task ID: `M0018-023`
- Milestone: `M0018`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-023-type-check-core-orchestration.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/newlang/src/type_check.rs`
  - `crates/newlang/tests/type_check.rs`
- Ordinary test results:
  - `cargo test --workspace --all-targets`: pass, 171 tests

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
Attack: Orchestration silently runs name resolution or constructs local bindings.
Expected result: It consumes already-resolved names and already-built local bindings only.
Actual result: type_m0018_core takes ResolutionTable and LocalBinding slices and does not call name-resolution builders.
Source of truth: ADR-0026 separation and task out-of-scope list.
Outcome: pass

Attack: Orchestration infers missing declaration or expression types.
Expected result: Missing annotation authority and missing known name types remain diagnostics.
Actual result: Negative fixture reports missing_annotation_type and missing_resolved_name_type.
Source of truth: ADR-0027 unresolved_type_rule and no-inference rule.
Outcome: pass

Attack: Unsupported expressions become typed through orchestration.
Expected result: Unsupported nodes produce unsupported_type_rule diagnostics and no expression type entries.
Actual result: Orchestration merges unsupported-expression diagnostics from existing AST nodes only.
Source of truth: ADR-0027 explicit deferrals.
Outcome: pass

Attack: Well-typed accepted fixtures miss checks because helper order is wrong.
Expected result: Declaration signatures are recorded before known local symbol derivation, then name/grouped expressions and assignments can be checked.
Actual result: Positive fixture records declaration signatures and both declaration-initializer and assignment-statement checks with no diagnostics.
Source of truth: ADR-0027 concrete type checking model.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/type_check.rs`
- Tests run:
  - `cargo test --workspace --all-targets`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-023-type-check-core-orchestration.md`
- Result:
  - Pass.

## Findings

No findings.

## Ambiguities

- None.

## Decision

Pass. The orchestrator composes accepted M0018 side-table outputs without inventing missing semantics.

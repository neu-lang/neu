# Soundness Report: M0018-016

## Metadata

- Task ID: `M0018-016`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-016-parser-assignment-statement-metadata.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/compiler/src/parser.rs`
  - `crates/compiler/tests/parser.rs`
- Ordinary test results:
  - `cargo test --workspace --all-targets`: pass, 153 tests

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
Attack: Malformed assignment attempts to synthesize assignment metadata.
Expected result: No metadata is recorded for the malformed assignment.
Actual result: Test confirms only the valid assignment is recorded.
Source of truth: Parser metadata must reflect accepted parsed constructs only.
Outcome: pass

Attack: Expression statement or local declaration attempts to appear as assignment metadata.
Expected result: No assignment metadata is synthesized.
Actual result: Test includes expression and local declaration statements and records only the valid assignment.
Source of truth: Task scope restricts metadata to accepted assignment statements.
Outcome: pass

Attack: Metadata addition checks assignment compatibility or target legality.
Expected result: No type-checking behavior or legality rules are added.
Actual result: Changed code only records statement, target expression, and value expression node IDs.
Source of truth: ADR-0027 side-table model and task out-of-scope list.
Outcome: pass

Attack: Metadata addition changes ownership, borrow, coroutine, unsafe, HIR, MIR, or backend behavior.
Expected result: No safety or backend phase behavior changes.
Actual result: Changed files are limited to parser metadata and parser tests.
Source of truth: ADR-0027 explicit deferrals.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/parser.rs`
- Tests run:
  - `cargo test --workspace --all-targets`
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-016-parser-assignment-statement-metadata.md`
- Result:
  - Pass.

## Findings

No findings.

## Ambiguities

- None.

## Decision

Pass. The change exposes parser metadata for later assignment checking and does not introduce safety-relevant semantics.

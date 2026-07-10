# Soundness Report: M0028-009

## Metadata

- Task ID: `M0028-009`
- Milestone: `M0028`
- Filed By: `main-task adversarial check`
- Date: `2026-07-11`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0028-009-straight-line-return-validation.md`
- Milestone file: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0050-bootstrap-straight-line-return-diagnostics.md`
- Changed files:
  - `crates/compiler/src/type_check.rs`
  - `crates/compiler/tests/type_check.rs`
- Ordinary test results:
  - `cargo test --workspace --all-targets` (285 passed, 14 suites)

## Safety Invariants Checked

- [x] This task does not alter ownership or other safety boundaries.
- [x] Nested branch returns cannot satisfy a direct return path.

## Attacks Attempted

Attack: a nested return satisfies a deferred branch as if it were direct.

Expected result: it does not prevent `missing_return`.

Actual result: the focused test reports missing return for the nested-only
function and unreachable return only for the later direct return.

Source of truth: ADR-0050.

Outcome: pass.

## Adversarial Tests

- Tests added: `crates/compiler/tests/type_check.rs`
- Tests run: `cargo test -p compiler --test type_check m0028_straight_line_return`
- Result: 1 passed.

## Findings

- None.

## Ambiguities

- None. ADR-0050 resolved the diagnostic contract.

## Decision

Pass.

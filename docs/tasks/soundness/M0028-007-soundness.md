# Soundness Report: M0028-007

## Metadata

- Task ID: `M0028-007`
- Milestone: `M0028`
- Filed By: `main-task adversarial check`
- Date: `2026-07-11`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0028-007-entry-point-candidate-validation.md`
- Milestone file: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0040-bootstrap-program-entry-point.md`
  - `docs/adr/ADR-0049-bootstrap-entry-point-diagnostic-provenance.md`
- Changed files:
  - `crates/compiler/src/type_check.rs`
  - `crates/compiler/tests/type_check.rs`
- Ordinary test results:
  - `cargo test --workspace --all-targets` (283 passed, 14 suites)

## Safety Invariants Checked

- [x] This task does not alter ownership, borrowing, threading, coroutine,
  unsafe, or FFI behavior.
- [x] Entry candidates are selected only from the explicit package input.
- [x] Multi-source diagnostics preserve source-or-external-input provenance.

## Attacks Attempted

Attack: a `main` in another package satisfies the selected entry package.

Expected result: ignored.

Actual result: the selected package's valid `main` is the sole entry.

Source of truth: ADR-0025 and ADR-0040.

Outcome: pass.

Attack: duplicate source files hide one candidate through arena-local IDs.

Expected result: every duplicate has a source-qualified diagnostic.

Actual result: two duplicate diagnostics are emitted and no entry is selected.

Source of truth: ADR-0049.

Outcome: pass.

Attack: nested `main` satisfies the top-level entry contract.

Expected result: missing-entry diagnostic.

Actual result: nested candidate is ignored.

Source of truth: ADR-0040.

Outcome: pass.

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/type_check.rs`
- Tests run:
  - `cargo test -p compiler --test type_check m0028_entry_point`
- Result:
  - 3 passed.

## Findings

- None.

## Ambiguities

- None. ADR-0049 resolved the diagnostic-provenance ambiguity.

## Decision

Pass. The checker neither broadens entry semantics nor loses cross-source
provenance.

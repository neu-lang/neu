# Soundness Report: M0028-010

## Metadata

- Task ID: `M0028-010`
- Milestone: `M0028`
- Filed By: `main-task adversarial check`
- Date: `2026-07-11`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0028-010-direct-call-validation.md`
- Milestone file: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0041-bootstrap-function-call-and-return-semantics.md`
  - `docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md`
- Changed files:
  - `crates/compiler/src/type_check.rs`
  - `crates/compiler/tests/type_check.rs`
- Ordinary test results:
  - `cargo test --workspace --all-targets` (286 passed, 14 suites)

## Safety Invariants Checked

- [x] This task adds typed signature metadata only and alters no safety boundary.
- [x] Non-Int annotations do not acquire inferred executable signatures.

## Attacks Attempted

Attack: a non-Int parameter or return type is treated as an executable Int
signature.

Expected result: no bootstrap signature is produced.

Actual result: signature construction requires every explicit annotation to be
`Int`.

Source of truth: ADR-0027 and ADR-0042.

Outcome: pass.

## Adversarial Tests

- Tests added: `crates/compiler/tests/type_check.rs`
- Tests run: `cargo test -p compiler --test type_check m0028_function_signatures`
- Result: 1 passed.

## Findings

- None.

## Ambiguities

- None.

## Decision

Pass.

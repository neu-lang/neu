# Task: M0028-007 Entry-Point Candidate Validation

## Task Metadata

- Task ID: `M0028-007`
- Milestone: `M0028`
- Milestone File: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- Status: `completed`
- Owner main task: `main-task test work`
- Created By: `main-task task planning`
- Created Date: `2026-07-11`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
  - `docs/adr/ADR-0040-bootstrap-program-entry-point.md`
  - `docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md`
- Project Rules: `AGENTS.md`

## Goal

Validate the bootstrap entry-point candidate set in the explicit entry package:
there must be exactly one top-level `main` with a body, no parameters, and a
declared `Int` return type.

## Motivation

ADR-0040 requires a source-backed executable entry contract before HIR and
backend work. M0028-006 now supplies the parser relationships needed to check
candidate identity and signature without inferring from host paths.

## Scope

- Accept the entry package as explicit module/test-harness input using
  `PackageNamespace`.
- Check function declarations across supplied source files for top-level
  `main` candidates in that package.
- Diagnose missing candidates, duplicate candidates, and invalid body,
  parameter, or declared-return-type signatures using ADR-0040 identifiers.
- Record the one accepted entry declaration for later executable analysis.

## Out Of Scope

- Return-path reachability and `missing_return` analysis.
- Call checking, recursion checking, argument checking, and return-expression
  type checking.
- Compiler CLI, host path, manifest, package-manager, HIR, MIR, backend,
  runtime, and linker work.
- New entry syntax or any change to ADR-0040.

## Authority Extract

- ADR-0025 supplies an explicit module/package mapping from compiler
  invocation or test harness, without host-path identity.
- ADR-0040 accepts exactly one top-level `main` in the selected root module
  entry package, with no parameters, declared `Int`, and a body.
- ADR-0040 defines `missing_entry_point`, `duplicate_entry_point`, and
  `invalid_entry_point_signature` for the rules in this task.
- ADR-0040's return-path requirement is deliberately deferred to the next
  task because parser metadata does not decide reachability.

## Required Tests

- Positive tests:
  - Exactly one valid `main` in the selected package is recorded as entry.
  - A same-named function outside the selected package is ignored.
- Negative tests:
  - Missing and duplicate candidate sets diagnose.
  - Parameterized, no-body, missing-return-type, and non-`Int` candidates
    diagnose as invalid signatures.
- Diagnostic tests:
  - Verify the three ADR-0040 identifiers and primary candidate nodes.
- Adversarial tests:
  - Nested/member `main` declarations and cross-package `main` declarations
    cannot satisfy or duplicate the entry contract.

## Test-First Gate

- Test files to create before implementation:
  - `crates/compiler/tests/type_check.rs`
  - `docs/tests/m0028-entry-point-candidate-validation.sh`
- Expected pre-implementation result: `fail`.
- Failure reason expected before implementation:
  - The type-checker exposes no explicit package-scoped entry-candidate API or
    ADR-0040 entry diagnostics.
- main-task review approval required to modify/delete failing tests: `yes`

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Entry candidate selection uses explicit package input, not host paths.
- [x] ADR-0040 missing, duplicate, and invalid-signature diagnostics are
  emitted on the appropriate source nodes.
- [x] One accepted entry declaration is recorded only for a valid candidate.
- [x] Ordinary tests, adversarial checks, review, and CI pass.

## Execution Commands

- Generate tests: `cargo test -p compiler --test type_check`
- Verify tests fail: `cargo test -p compiler --test type_check`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `sh docs/scripts/adversarial-check.sh docs/tasks/M0028-007-entry-point-candidate-validation.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0028-007-entry-point-candidate-validation.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/type_check.rs`
  - `docs/tests/m0028-entry-point-candidate-validation.sh`
- Implementation files:
  - `crates/compiler/src/type_check.rs`
- Documentation files:
  - `docs/tasks/M0028-007-entry-point-candidate-validation.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md` or accepted ADRs.
- Do not infer an entry package from host paths or source-file ordering.
- Do not perform return-path, call, recursion, argument, or expression-type
  validation in this task.
- Do not weaken or delete failing tests without main-task review approval.

## Ambiguities And Dependencies

- ADR-0049 resolves entry-diagnostic source-or-external-input provenance.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0025 and ADR-0040 define explicit package-scoped candidate selection. handoff=main-task test work
- 2026-07-11 main_task=main phase=authority-review result=blocked evidence=ADR-0040 lacks required multi-source diagnostic provenance; ambiguity report filed before tests or implementation. handoff=main-task semantic design
- 2026-07-11 main_task=main phase=semantic-resolution result=pass evidence=ADR-0049 accepted source-or-external-input provenance for ADR-0040 diagnostics. handoff=main-task test work
- 2026-07-11 main_task=main phase=tests-created result=pass evidence=package-scoped valid, missing, duplicate, invalid-signature, and nested-main tests added. handoff=main-task test work
- 2026-07-11 main_task=main phase=pre-implementation-test result=fail evidence=`cargo test -p compiler --test type_check m0028_entry_point` failed because entry checker API and diagnostic kinds were absent. handoff=main-task implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=checker aggregates selected-package candidates and returns source-or-external-input diagnostics under ADR-0049. handoff=main-task validation
- 2026-07-11 main_task=main phase=focused-validation result=pass evidence=`cargo test -p compiler --test type_check m0028_entry_point` (3 passed). handoff=main-task validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=`cargo test --workspace --all-targets` (283 passed, 14 suites). handoff=main-task adversarial check
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=package isolation, duplicate provenance, and nested-main attacks pass in `docs/tasks/soundness/M0028-007-soundness.md`. handoff=main-task review
- 2026-07-11 main_task=main phase=task-review result=pass evidence=`docs/tasks/reviews/M0028-007-review.md` approves source-of-truth compliance and bounded checker scope. handoff=main-task CI
- 2026-07-11 main_task=main phase=ci result=pass evidence=`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace --all-targets` (283 passed, 14 suites); relevant M0028 validators passed. handoff=commit

## Files Changed

- `crates/compiler/src/type_check.rs`
- `crates/compiler/tests/type_check.rs`
- `docs/SPEC.md`
- `docs/adr/ADR-0049-bootstrap-entry-point-diagnostic-provenance.md`
- `docs/adr/proposals/reviews/ADR-0049-*.md`
- `docs/ambiguities/M0028-entry-point-diagnostic-provenance.md`
- `docs/tasks/M0028-007-entry-point-candidate-validation.md`
- `docs/tasks/soundness/M0028-007-soundness.md`
- `docs/tasks/reviews/M0028-007-review.md`
- `docs/tests/m0028-entry-point-candidate-validation.sh`
- `examples/current/README.md`
- `examples/current/executable_entry_point.neu`

## Open Questions

- None.

## Remaining Risk And Next Action

- Remaining risk: no return-path, direct-call, recursion, argument, or
  return-expression type checking exists yet.
- Next action: commit this task, then create the return-path validation task.

## Handoff

- Next main task: `main-task test work`
- Reason: entry-point tests must first establish the absent checker API and
  diagnostic identifiers.

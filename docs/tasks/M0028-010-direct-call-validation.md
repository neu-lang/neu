# Task: M0028-010 Function Signature Typing

## Task Metadata

- Task ID: `M0028-010`
- Milestone: `M0028`
- Milestone File: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- Status: `completed`
- Owner main task: `main-task semantic design`
- Created By: `main-task task planning`
- Created Date: `2026-07-11`

## Goal

Establish typed bootstrap function signatures and argument-expression inputs
needed by direct-call validation under ADR-0041.

## Scope

- Type explicit `Int` parameter and return annotations for parsed function
  declarations using ADR-0027 primitive identities.
- Expose ordered function signature metadata keyed by declaration identity.
- Preserve typed executable-subset argument expression facts for the follow-up
  direct-call checker.

## Out Of Scope

- Call target resolution, argument diagnostics, recursion diagnostics, methods,
  overloads, function values, closures, generic calls, defaults, named
  arguments, varargs, HIR, MIR, backend, runtime, and linking.

## Authority Extract

- ADR-0041 defines direct-call parameter and result typing requirements.
- ADR-0027 supplies primitive `Int` identity and annotation typing.
- ADR-0042 limits the first executable subset to `Int` parameters and returns.

## Required Tests

- `Int` parameter and return annotations produce ordered function signatures.
- Missing/non-`Int` executable signature components remain untyped for later
  call checking rather than being inferred.

## Test-First Gate

- Test files: `crates/compiler/tests/type_check.rs` and
  `docs/tests/m0028-function-signature-typing.sh`.
- Expected pre-implementation result: `fail`.
- Failure reason: no typed function signature metadata exists.

## Ambiguities And Dependencies

- ADR-0051 resolves direct-call diagnostic provenance and recovery.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=blocked evidence=ADR-0041 lacks required direct-call diagnostic provenance and recovery. handoff=main-task semantic design
- 2026-07-11 main_task=main phase=semantic-resolution result=pass evidence=ADR-0051 accepted direct-call diagnostic contract. handoff=main-task test work
- 2026-07-11 main_task=main phase=dependency-refinement result=pass evidence=typed function signatures are required before call validation; task narrowed to that prerequisite. handoff=main-task test work
- 2026-07-11 main_task=main phase=tests-created result=pass evidence=explicit Int function signature test added. handoff=main-task test work
- 2026-07-11 main_task=main phase=pre-implementation-test result=fail evidence=type_m0028_function_signatures was absent. handoff=main-task implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=ordered Int parameter and return signature metadata is retained. handoff=main-task validation
- 2026-07-11 main_task=main phase=focused-validation result=pass evidence=`cargo test -p compiler --test type_check m0028_function_signatures` passed. handoff=main-task validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=`cargo test --workspace --all-targets` (286 passed, 14 suites). handoff=main-task adversarial check
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=non-Int signature attack passes in `docs/tasks/soundness/M0028-010-soundness.md`. handoff=main-task review
- 2026-07-11 main_task=main phase=task-review result=pass evidence=`docs/tasks/reviews/M0028-010-review.md` approves narrow signature scope. handoff=main-task CI
- 2026-07-11 main_task=main phase=ci result=pass evidence=format, Clippy, 286 workspace tests, and signature validator passed. handoff=commit

## Files Changed

- `crates/compiler/src/type_check.rs`
- `crates/compiler/tests/type_check.rs`
- `docs/SPEC.md`
- `docs/adr/ADR-0051-bootstrap-direct-call-diagnostics.md`
- `docs/adr/proposals/reviews/ADR-0051-*.md`
- `docs/ambiguities/M0028-direct-call-diagnostic-contract.md`
- `docs/tasks/M0028-010-direct-call-validation.md`
- `docs/tasks/soundness/M0028-010-soundness.md`
- `docs/tasks/reviews/M0028-010-review.md`
- `docs/tests/m0028-function-signature-typing.sh`

## Open Questions

- None.

## Remaining Risk And Next Action

- Direct call resolution, arguments, recursion, and call result typing remain
  for the follow-up task.
- Next action: commit this signature prerequisite, then create direct-call
  checker implementation work.

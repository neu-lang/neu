# Task: M0028-009 Straight-Line Return Validation

## Task Metadata

- Task ID: `M0028-009`
- Milestone: `M0028`
- Milestone File: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- Status: `completed`
- Owner main task: `main-task semantic design`
- Created By: `main-task task planning`
- Created Date: `2026-07-11`

## Goal

Validate explicit return presence and unreachable direct returns in the
straight-line bootstrap subset.

## Authority Extract

- ADR-0041 requires explicit `return expression;` values and names
  `missing_return` and `unreachable_return`.
- ADR-0042 defers branches beyond straight-line return analysis.
- ADR-0015 and ADR-0024 require a primary source span or external location,
  recovery, and safe suggestion for diagnostics.

## Scope

- Direct returns in the function body block only.
- `missing_return` and `unreachable_return` diagnostic behavior after accepted
  provenance is available.

## Out Of Scope

- Return-expression type checking, `return;`/`Unit`, branches, loops, calls,
  recursion, HIR, MIR, backend, runtime, and linking.

## Required Tests

- A direct `Int` return satisfies a function's straight-line return path.
- No direct return reports `missing_return`.
- A later direct return reports `unreachable_return`.
- Nested-block returns do not satisfy a direct function-body path.

## Test-First Gate

- Test files: `crates/compiler/tests/type_check.rs` and
  `docs/tests/m0028-straight-line-return-validation.sh`.
- Expected pre-implementation result: `fail`.
- Failure reason: no accepted return diagnostic provenance or checker API.

## Ambiguities And Dependencies

- ADR-0050 resolves the return diagnostic contract.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=blocked evidence=ADR-0041 lacks diagnostic provenance, recovery, and safe-suggestion rules. handoff=main-task semantic design
- 2026-07-11 main_task=main phase=semantic-resolution result=pass evidence=ADR-0050 accepted direct-return and diagnostic rules. handoff=main-task test work
- 2026-07-11 main_task=main phase=tests-created result=pass evidence=missing, unreachable, and nested-block return test added. handoff=main-task test work
- 2026-07-11 main_task=main phase=pre-implementation-test result=fail evidence=return checker API and diagnostics were absent. handoff=main-task implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=checker considers only direct returns in Int function body blocks. handoff=main-task validation
- 2026-07-11 main_task=main phase=focused-validation result=pass evidence=`cargo test -p compiler --test type_check m0028_straight_line_return` passed. handoff=main-task validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=`cargo test --workspace --all-targets` (285 passed, 14 suites). handoff=main-task adversarial check
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=nested-return attack passes in `docs/tasks/soundness/M0028-009-soundness.md`. handoff=main-task review
- 2026-07-11 main_task=main phase=task-review result=pass evidence=`docs/tasks/reviews/M0028-009-review.md` approves scope and authority. handoff=main-task CI
- 2026-07-11 main_task=main phase=ci result=pass evidence=format, Clippy, 285 workspace tests, and M0028-009 validator passed. handoff=commit

## Files Changed

- `crates/compiler/src/type_check.rs`
- `crates/compiler/tests/type_check.rs`
- `docs/SPEC.md`
- `docs/adr/ADR-0050-bootstrap-straight-line-return-diagnostics.md`
- `docs/adr/proposals/reviews/ADR-0050-*.md`
- `docs/ambiguities/M0028-return-diagnostic-contract.md`
- `docs/tasks/M0028-009-straight-line-return-validation.md`
- `docs/tasks/soundness/M0028-009-soundness.md`
- `docs/tasks/reviews/M0028-009-review.md`
- `docs/tests/m0028-straight-line-return-validation.sh`
- `examples/current/README.md`

## Open Questions

- None.

## Remaining Risk And Next Action

- Return-expression types and direct-call semantics remain unchecked.
- Next action: commit this task, then create direct-call validation.

# Task: M0028-006 Executable Function Body Metadata

## Task Metadata

- Task ID: `M0028-006`
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
  - `docs/adr/ADR-0040-bootstrap-program-entry-point.md`
  - `docs/adr/ADR-0041-bootstrap-function-call-and-return-semantics.md`
  - `docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Project Rules: `AGENTS.md`

## Goal

Record parser metadata that associates each accepted function declaration with
its body and declared return type, each explicit return with its value and
enclosing function, and each direct-call syntax node with its callee and
source-ordered arguments.

## Motivation

ADR-0040 and ADR-0041 checks require declaration, body, return, callee, and
argument identities. Existing parser output records function parameters but
does not preserve the remaining executable-body relationships.

## Scope

- Add parser-output records for accepted function declaration body and return
  annotation facts.
- Add parser-output records for explicit return statements, their optional
  value expression, and containing function.
- Add parser-output records for call expressions, their syntactic callee, and
  ordered argument expressions.
- Preserve source spans and avoid creating semantic resolution or type results.

## Out Of Scope

- Entry-point validation or diagnostics.
- Call target resolution, recursion detection, argument type checking, or
  return type checking.
- Reachability or unsupported-executable-form checking.
- HIR, MIR, backend, runtime, or linker work.
- Changes to accepted syntax or language semantics.

## Required Inputs

- Milestone: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Spec sections:
  - `ADR-0040: Bootstrap Program Entry Point`
  - `ADR-0041: Bootstrap Function Call And Return Semantics`
  - `ADR-0042: Bootstrap Minimal Executable Subset`
- Existing files:
  - `crates/compiler/src/parser.rs`
  - `crates/compiler/src/ast.rs`
  - `crates/compiler/tests/parser.rs`

## Authority Extract

- ADR-0040 requires a top-level `main` with no parameters, declared `Int`
  return type, and a body.
- ADR-0041 requires explicit return expressions and direct same-module calls
  with left-to-right argument order.
- ADR-0042 limits the executable subset to function bodies, explicit `Int`
  return types, direct calls, and explicit returns.
- Parser metadata must not decide whether a function is an entry point, whether
  a call resolves, or whether a return is well-typed.

## Required Tests

- Positive tests:
  - Function records preserve declaration, body, return-type annotation, and
    parameter association.
  - Return records preserve the value expression and enclosing function.
  - Direct-call records preserve the callee expression and ordered arguments.
- Negative tests:
  - Malformed functions, calls, and returns do not create complete records.
- Diagnostic tests:
  - `not applicable`; this task preserves parser metadata only.
- Adversarial tests:
  - Nested calls and returns in separate functions cannot be associated with
    the wrong enclosing function or reordered arguments.

## Test-First Gate

- Test files to create before implementation:
  - `crates/compiler/tests/parser.rs`
  - `docs/tests/m0028-executable-function-body-metadata.sh`
- Expected pre-implementation result: `fail`.
- Failure reason expected before implementation:
  - `ParseOutput` does not expose complete executable function, return, or
    call metadata.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add only parser-side metadata records and parser plumbing necessary to retain
accepted executable-body relationships. Keep validation and semantic decisions
for follow-up M0028 tasks.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Complete records exist only for accepted function, return, and call
  parses.
- [x] Call argument order and enclosing-function association are preserved.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] Main-task review compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] Milestone checklist remains unchanged because the checker task owns its
  corresponding semantic completion item.

## Review Routing

- Base review: `main-task review`
- main-task test work: required
- main-task specification check: not required; this is metadata-only work
- main-task diagnostics check: not required
- main-task build check: not required
- main-task simplicity check: required; parser metadata must remain minimal
- main-task adversarial check: required; function association and evaluation
  order feed executable semantics

## Execution Commands

- Generate tests: `cargo test -p compiler --test parser`
- Verify tests fail: `cargo test -p compiler --test parser`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `sh docs/scripts/adversarial-check.sh docs/tasks/M0028-006-executable-function-body-metadata.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0028-006-executable-function-body-metadata.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets`

## Files Expected To Change

- Test files:
  - `crates/compiler/tests/parser.rs`
  - `docs/tests/m0028-executable-function-body-metadata.sh`
- Implementation files:
  - `crates/compiler/src/parser.rs`
- Documentation or checklist files:
  - `docs/tasks/M0028-006-executable-function-body-metadata.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md` or accepted ADRs.
- Do not add entry, call, recursion, argument, return, or reachability
  diagnostics.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement HIR, MIR, backend, runtime, or linker behavior.

## Ambiguities And Dependencies

- None. The task retains only syntactic relationships explicitly required by
  ADR-0040 through ADR-0042. Any need to interpret a relationship is deferred
  to the checker task.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=bounded parser-metadata task created from ADR-0040 through ADR-0042. handoff=main-task test work
- 2026-07-11 main_task=main phase=tests-created result=pass evidence=parser tests define function, return, and call metadata including malformed-record exclusion. handoff=main-task test work
- 2026-07-11 main_task=main phase=pre-implementation-test result=fail evidence=`cargo test -p compiler --test parser m0028_` failed because ParseOutput lacked function_declarations, return_statements, and call_expressions. handoff=main-task implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=parser retains function body/return annotations, enclosing-function returns, and complete ordered call arguments without semantic validation. handoff=main-task validation
- 2026-07-11 main_task=main phase=focused-validation result=pass evidence=`cargo test -p compiler --test parser` (51 passed). handoff=main-task validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=`cargo test --workspace --all-targets` (280 passed, 14 suites). handoff=main-task adversarial check
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=return containment, call argument ordering, and malformed-call exclusion approved in `docs/tasks/soundness/M0028-006-soundness.md`. handoff=main-task review
- 2026-07-11 main_task=main phase=task-review result=pass evidence=`docs/tasks/reviews/M0028-006-review.md` approves bounded parser-only scope and source-of-truth compliance. handoff=main-task CI
- 2026-07-11 main_task=main phase=ci result=pass evidence=`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace --all-targets` (280 passed, 14 suites); M0028 metadata validator passed. handoff=commit

## Files Changed

- `crates/compiler/src/parser.rs`
- `crates/compiler/tests/parser.rs`
- `docs/tasks/M0028-006-executable-function-body-metadata.md`
- `docs/tests/m0028-executable-function-body-metadata.sh`
- `docs/tasks/soundness/M0028-006-soundness.md`
- `docs/tasks/reviews/M0028-006-review.md`

## Open Questions

- None.

## Remaining Risk And Next Action

- Remaining risk: records do not yet validate entry signatures, call targets,
  argument types, returns, or reachability.
- Next action: commit this parser-metadata task, then create the semantic
  executable entry/call/return checker task.

## Handoff

- Next main task: `main-task test work`
- Reason: parser tests must demonstrate missing executable-body metadata before
  implementation begins.
- Required Context:
  - This task's Authority Extract
  - `crates/compiler/src/parser.rs`
  - `crates/compiler/tests/parser.rs`

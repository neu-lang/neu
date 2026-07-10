# Task: M0028-008 Return Statement Block Metadata

## Task Metadata

- Task ID: `M0028-008`
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
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
  - `docs/adr/ADR-0041-bootstrap-function-call-and-return-semantics.md`
  - `docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md`
- Project Rules: `AGENTS.md`

## Goal

Record the enclosing parsed block for every accepted return statement, while
preserving its existing enclosing-function and optional-value metadata.

## Motivation

ADR-0042 permits only straight-line return analysis. The next M0028 checker
must identify returns directly in a function body and avoid treating returns in
deferred branch blocks as proof of a reachable function return.

## Scope

- Add the enclosing block identity to accepted return parser metadata.
- Preserve return source order within each enclosing block.
- Retain existing malformed-return recovery without producing complete records.

## Out Of Scope

- `missing_return`, `unreachable_return`, and return-type diagnostics.
- Branch reachability, `if` semantics, loops, calls, HIR, MIR, backend, and
  runtime work.
- New source syntax or changes to accepted return semantics.

## Authority Extract

- ADR-0041 accepts only explicit `return expression;` for bootstrap values and
  defines missing/unreachable return diagnostics.
- ADR-0042 defers branches beyond straight-line return analysis.
- This task records containment only; it must not decide reachability.

## Required Tests

- Positive tests:
  - Direct function-body returns retain their function body block.
  - Returns in an `if` block retain the nested block, not the function body.
  - Ordered returns in one block preserve source order.
- Negative tests:
  - Malformed returns create no complete return record.
- Adversarial tests:
  - A nested branch return cannot masquerade as a direct function return.

## Test-First Gate

- Test files to create before implementation:
  - `crates/compiler/tests/parser.rs`
  - `docs/tests/m0028-return-statement-block-metadata.sh`
- Expected pre-implementation result: `fail`.
- Failure reason expected before implementation:
  - Return metadata has no enclosing block identity.
- main-task review approval required to modify/delete failing tests: `yes`

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation and fail for the expected reason.
- [x] Every complete return record has its enclosing block.
- [x] Nested block returns cannot claim the function body block.
- [x] Ordinary tests, adversarial checks, review, and CI pass.

## Execution Commands

- Generate tests: `cargo test -p compiler --test parser`
- Verify tests fail: `cargo test -p compiler --test parser`
- Ordinary tests: `cargo test --workspace --all-targets`
- Adversarial tests: `sh docs/scripts/adversarial-check.sh docs/tasks/M0028-008-return-statement-block-metadata.md`
- Review: `docs/scripts/review-task.sh docs/tasks/M0028-008-return-statement-block-metadata.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets`

## Files Expected To Change

- `crates/compiler/src/parser.rs`
- `crates/compiler/tests/parser.rs`
- `docs/tests/m0028-return-statement-block-metadata.sh`
- This task file and its generated review evidence.

## Forbidden Changes

- Do not modify `docs/SPEC.md` or accepted ADRs.
- Do not add reachability, return-type, call, or branch diagnostics.
- Do not weaken or delete failing tests without main-task review approval.

## Ambiguities And Dependencies

- None. The task preserves parser containment facts only.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=return block containment is required before ADR-0042 straight-line analysis. handoff=main-task test work
- 2026-07-11 main_task=main phase=tests-created result=pass evidence=direct and nested return block containment test added. handoff=main-task test work
- 2026-07-11 main_task=main phase=pre-implementation-test result=fail evidence=parser test failed because ParsedReturnStatement lacked block metadata. handoff=main-task implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=parser block frames assign only direct returns when each parsed block closes. handoff=main-task validation
- 2026-07-11 main_task=main phase=focused-validation result=pass evidence=`cargo test -p compiler --test parser m0028_records_return_statement_enclosing_blocks_in_source_order` passed. handoff=main-task validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=`cargo test --workspace --all-targets` (284 passed, 14 suites). handoff=main-task adversarial check
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=nested return containment attack passes in `docs/tasks/soundness/M0028-008-soundness.md`. handoff=main-task review
- 2026-07-11 main_task=main phase=task-review result=pass evidence=`docs/tasks/reviews/M0028-008-review.md` approves parser-only scope. handoff=main-task CI
- 2026-07-11 main_task=main phase=ci result=pass evidence=`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace --all-targets` (284 passed, 14 suites); task validator passed. handoff=commit

## Files Changed

- `crates/compiler/src/parser.rs`
- `crates/compiler/tests/parser.rs`
- `docs/tasks/M0028-008-return-statement-block-metadata.md`
- `docs/tasks/soundness/M0028-008-soundness.md`
- `docs/tasks/reviews/M0028-008-review.md`
- `docs/tests/m0028-return-statement-block-metadata.sh`

## Open Questions

- None.

## Remaining Risk And Next Action

- Remaining risk: reachability and return-type semantics remain unchecked.
- Next action: commit this metadata task, then create the straight-line return
  validation task.

## Handoff

- Next main task: `main-task test work`
- Reason: parser tests must demonstrate the missing return block metadata before
  implementation starts.

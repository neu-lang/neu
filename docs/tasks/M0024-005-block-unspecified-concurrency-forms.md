# Task: M0024-005 Block Unspecified Concurrency Forms

## Task Metadata

- Task ID: `M0024-005`
- Milestone: `M0024`
- Milestone File: `docs/milestones/M0024-thread-safety-capability-analysis.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner main task: `main-task implementation`

## Objective

Record and validate that M0024 does not add source-level concurrency forms and
that currently unspecified coroutine-like forms remain blocked by parser
diagnostics.

## Authority Extract

- `docs/adr/ADR-0037-bootstrap-thread-capability-analysis.md`, “Recommended Choice”.
- `docs/SPEC.md`, “ADR-0037: Bootstrap Thread Capability Analysis”.
- `docs/milestones/M0024-thread-safety-capability-analysis.md`.
- `crates/compiler/src/parser.rs`.

## Scope

- Add parser regression coverage for deferred coroutine-like syntax.
- Add docs validation that M0024 has no source-level concurrency parser APIs.
- Complete the M0024 checklist item for unspecified concurrency forms.

## Out Of Scope

- Adding task-spawn syntax.
- Adding coroutine syntax.
- Adding runtime scheduling.
- Adding synchronization APIs.

## Required Tests

- `crates/compiler/tests/parser.rs`
- `docs/tests/m0024-unspecified-concurrency-forms-blocked.sh`

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Parser diagnostics reject coroutine-like `async` syntax.
- [x] Parser diagnostics continue rejecting deferred statement forms.
- [x] No parser API for thread/task spawning exists.
- [x] No examples update is required because no user-written syntax changes.

## Execution Log

- 2026-07-11 agent=Main phase=create-task result=pass evidence=Task and parser/docs tests created from ADR-0037. handoff=Test
- 2026-07-11 agent=Main phase=test-first result=fail evidence=cargo test -p compiler --test parser m0024_unspecified_concurrency_forms_remain_blocked failed because async did not report malformed coroutine construct. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=parse_statement now reports malformed coroutine construct for async at statement start. handoff=Review
- 2026-07-11 agent=Main phase=validation result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; all M0024 docs validators. handoff=Commit

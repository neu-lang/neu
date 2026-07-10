# Task: M0028-003 Executable Operator Core Integration

## Task Metadata

- Task ID: `M0028-003`
- Milestone: `M0028`
- Milestone File: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner main task: `main-task test work`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md`
  - `docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md`
- Project Rules: `AGENTS.md`

## Objective

Integrate accepted executable `Int` operator typing into a dedicated M0028 core
type-checking entry point.

## Authority Extract

- `docs/SPEC.md`, “ADR-0042: Bootstrap Minimal Executable Subset” and
  “ADR-0043: Bootstrap Integer Runtime Semantics”.
- ADR-0042 and ADR-0043, “Recommended Choice”.
- `docs/milestones/M0028-executable-expression-frontend-completion.md`.
- `crates/compiler/src/type_check.rs`, M0018 core and M0028 operator helper.
- `crates/compiler/tests/type_check.rs`, M0018 core fixtures.

## Scope

- Add a dedicated M0028 core type-checking entry point that accepts parser unary
  and binary metadata.
- Run executable-operator typing before local-initializer and assignment checks.
- Preserve M0018 core behavior for callers that do not opt into M0028 inputs.
- Suppress generic unary/binary deferral diagnostics only for accepted M0028
  executable operators.
- Preserve deferral diagnostics for logical unary and non-executable binary
  forms.

## Out Of Scope

- New operator syntax or parser metadata.
- Static arithmetic diagnostics, calls, returns, entry points, and unsupported
  executable-form diagnostics.
- Runtime semantics, HIR, MIR, backend, objects, and linking.

## Required Tests

- Positive: M0028 core accepts typed local initializers and assignments using
  nested executable operators without generic operator deferrals.
- Negative: `Bool` operands report `TypeMismatch` and do not produce an `Int`
  initializer result.
- Regression: `!` and `==` remain deferred in the M0028 core.
- `docs/tests/m0028-executable-operator-core-integration.sh` validates the
  focused test names and core API.

## Test-First Gate

- Test files to create before implementation:
  - `crates/compiler/tests/type_check.rs`
  - `docs/tests/m0028-executable-operator-core-integration.sh`
- Expected pre-implementation result: `fail`.
- Failure reason: the M0028 core integration API does not exist.
- Reviewer approval required to modify/delete failing tests: `yes`

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Accepted executable operators type before initializer and assignment checks.
- [x] Accepted executable operators do not receive generic deferral diagnostics.
- [x] `!` and non-executable binary forms remain deferred.
- [x] M0018 core behavior remains covered without M0028 inputs.
- [x] Ordinary tests pass before adversarial tests run.
- [x] Adversarial tests pass after ordinary tests.
- [x] Main-task review compares output against `docs/SPEC.md` and M0028.
- [x] CI passes as final gate.

## Review Routing

- Base review: `main-task review`.
- Test review: required.
- Specification-compliance review: required.
- Diagnostics review: required.
- Simplicity review: required because the core boundary is extended.
- Adversarial review: required because incorrect expression types can undermine
  later safety analyses.

## Execution Commands

- Generate tests: `cargo test -p compiler --test type_check`.
- Verify tests fail: `cargo test -p compiler --test type_check`.
- Ordinary tests: `cargo test --workspace --all-targets`.
- Adversarial tests: `sh docs/scripts/adversarial-check.sh docs/tasks/M0028-003-executable-operator-core-integration.md`.
- Review: `docs/scripts/review-task.sh docs/tasks/M0028-003-executable-operator-core-integration.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets`.

## Files Expected To Change

- Tests: `crates/compiler/tests/type_check.rs` and
  `docs/tests/m0028-executable-operator-core-integration.sh`.
- Implementation: `crates/compiler/src/type_check.rs`.
- Documentation: this task file and M0028 milestone checklist.

## Forbidden Changes

- Do not modify `docs/SPEC.md` or `docs/adr/`.
- Do not weaken or delete failing tests without reviewer approval.
- Do not broaden support beyond the ADR-0042/0043 operator set.

## Ambiguities And Dependencies

- Static arithmetic diagnostics and executable-form rejection require later
  M0028 tasks; this task only routes already accepted operator typing through
  the core.

## Execution Log

- 2026-07-11 agent=Main phase=create-task result=pass evidence=bounded M0028 core-integration task created from ADR-0042 and ADR-0043. handoff=Test
- 2026-07-11 agent=Main phase=test-first result=pass evidence=focused tests failed because the M0028 core entry point was absent. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=shared core path preserves M0018 behavior and M0028 routes accepted operators before checks. handoff=Validation
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=cargo test --workspace --all-targets passed 274 tests in 14 suites. handoff=Adversarial
- 2026-07-11 agent=Main phase=adversarial result=pass evidence=accepted filtering cannot type Bool operands or erase ! and == deferrals; report in docs/tasks/soundness/M0028-003-soundness.md. handoff=Review
- 2026-07-11 agent=Main phase=review result=pass evidence=scope, source-of-truth compliance, diagnostics, tests, and core boundary approved; report in docs/tasks/reviews/M0028-003-review.md. handoff=CI
- 2026-07-11 agent=Main phase=ci result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; M0028 docs validators; agent-configs. handoff=Commit

## Handoff

- Next main task: `main-task implementation`.
- Reason: focused tests must show the absent M0028 core entry point.

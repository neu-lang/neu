# Task: M0028-002 Executable Operator Type Checking

## Task Metadata

- Task ID: `M0028-002`
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

Type the complete bootstrap arithmetic, bitwise, shift, and unary operator
surface when its operands are `Int`.

## Authority Extract

- `docs/SPEC.md`, “ADR-0042: Bootstrap Minimal Executable Subset” and
  “ADR-0043: Bootstrap Integer Runtime Semantics”.
- `docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md`, “Recommended
  Choice”.
- `docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md`, “Recommended
  Choice”.
- `docs/milestones/M0028-executable-expression-frontend-completion.md`.
- `crates/compiler/src/parser.rs` executable-expression metadata.
- `crates/compiler/src/type_check.rs` expression-type and diagnostic APIs.

## Scope

- Preserve unary operator, operand, expression, and span metadata in parser
  output for `+`, `-`, and `~`.
- Type unary `+`, `-`, and `~` only for `Int`, with `Int` result types.
- Type binary `+`, `-`, `*`, `/`, `%`, `**`, `&`, `|`, `^`, `<<`, and `>>` only
  for `Int` operands, with `Int` result types.
- Record `TypeMismatch` diagnostics for known non-`Int` operands without
  inventing a new diagnostic category.
- Add a fixed-point type-checking helper so nested executable operators type
  after their operands and existing grouped-expression metadata propagates.

## Out Of Scope

- Integer-literal parsing and constant-expression evaluation.
- Static overflow, zero division/modulo, negative exponent, or invalid shift
  diagnostics.
- Runtime trap behavior.
- Type-check core orchestration and removal of the existing generic
  unary/binary deferral diagnostics.
- Function calls, returns, entry-point checks, HIR, MIR, backend, objects, and
  linking.

## Required Tests

- Positive parser test: unary parser metadata records all executable unary
  operators and their operands.
- Positive type-check test: every executable unary and binary operator produces
  `Int` from `Int` operands, including nested expressions.
- Negative type-check test: a known `Bool` operand reports `TypeMismatch` with
  expected `Int` and actual `Bool`.
- Adversarial test: an operator with an untyped operand must not synthesize an
  `Int` result or a false success.
- `docs/tests/m0028-executable-operator-type-checking.sh` validates the task
  contract and focused Rust tests.

## Test-First Gate

- Test files to create before implementation:
  - `crates/compiler/tests/parser.rs`
  - `crates/compiler/tests/type_check.rs`
  - `docs/tests/m0028-executable-operator-type-checking.sh`
- Expected pre-implementation result: `fail`.
- Failure reason: unary parser metadata and executable-operator type-checking
  API do not exist.
- Reviewer approval required to modify/delete failing tests: `yes`

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] The complete executable unary and binary operator set has `Int` typing.
- [x] Nested executable operators type after typed operands.
- [x] Known non-`Int` operands receive `TypeMismatch` diagnostics.
- [x] Unknown operands do not receive invented result types.
- [x] Ordinary tests pass before adversarial tests run.
- [x] Adversarial tests pass after ordinary tests.
- [x] Main-task review compares output against `docs/SPEC.md` and M0028.
- [x] CI passes as final gate.

## Review Routing

- Base review: `main-task review`.
- Test review: required.
- Specification-compliance review: required because accepted operator behavior
  changes from deferred to typed.
- Diagnostics review: required because type diagnostics change.
- Simplicity review: required because a new fixed-point helper is introduced.
- Adversarial review: required because false expression typing can undermine
  later safety analyses.

## Execution Commands

- Generate tests: `cargo test -p compiler --test parser --test type_check`.
- Verify tests fail: `cargo test -p compiler --test parser --test type_check`.
- Ordinary tests: `cargo test --workspace --all-targets`.
- Adversarial tests: `sh docs/scripts/adversarial-check.sh docs/tasks/M0028-002-executable-operator-type-checking.md`.
- Review: `docs/scripts/review-task.sh docs/tasks/M0028-002-executable-operator-type-checking.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets`.

## Files Expected To Change

- Tests: `crates/compiler/tests/parser.rs`, `crates/compiler/tests/type_check.rs`,
  and `docs/tests/m0028-executable-operator-type-checking.sh`.
- Implementation: `crates/compiler/src/parser.rs` and
  `crates/compiler/src/type_check.rs`.
- Documentation: this task file and the M0028 milestone checklist.

## Forbidden Changes

- Do not modify `docs/SPEC.md` or `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement static arithmetic diagnostics, call checking, or runtime
  behavior.
- Do not introduce semantics beyond ADR-0042 and ADR-0043.

## Ambiguities And Dependencies

- The source of truth specifies static arithmetic diagnostics but does not
  require them for this type-only task. A later M0028 task must carry literal
  values and constant-expression facts before implementing those diagnostics.

## Execution Log

- 2026-07-11 agent=Main phase=create-task result=pass evidence=bounded M0028 operator typing task created from ADR-0042 and ADR-0043. handoff=Test
- 2026-07-11 agent=Main phase=test-first result=pass evidence=focused tests failed before implementation because unary metadata and the type-checking API were absent. handoff=Implementer
- 2026-07-11 agent=Main phase=test-correction-review result=approved evidence=positive fixture moved unary prefix expressions into accepted local initializers without reducing operator coverage. handoff=Implementer
- 2026-07-11 agent=Main phase=test-correction-review result=approved evidence=fixed-point helper includes existing grouped-expression propagation required by the accepted parenthesized subset. handoff=Implementer
- 2026-07-11 agent=Main phase=implementation result=pass evidence=unary metadata and fixed-point executable Int operator typing added. handoff=Validation
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=cargo test --workspace --all-targets passed 271 tests in 14 suites. handoff=Adversarial
- 2026-07-11 agent=Main phase=adversarial result=pass evidence=Bool and unresolved operands cannot gain false Int results; report in docs/tasks/soundness/M0028-002-soundness.md. handoff=Review
- 2026-07-11 agent=Main phase=review result=pass evidence=scope, ADR-0042/0043 compliance, diagnostics, tests, and simplicity approved; report in docs/tasks/reviews/M0028-002-review.md. handoff=CI
- 2026-07-11 agent=Main phase=ci result=pass evidence=cargo fmt --all --check; cargo clippy --workspace --all-targets -- -D warnings; cargo test --workspace --all-targets; M0028 docs validators; agent-configs. handoff=Commit

## Handoff

- Next main task: `main-task implementation`.
- Reason: tests must first demonstrate the absent metadata and type-checking
  capability.

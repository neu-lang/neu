# Task: M0028-005 Static Integer Diagnostics

## Task Metadata

- Task ID: `M0028-005`
- Milestone: `M0028`
- Milestone File: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- Status: `completed`
- Owner main task: `main-task test work`

## Authority Extract

- ADR-0043, “Recommended Choice”.
- ADR-0048, “Recommended Choice”.
- `docs/SPEC.md`, “ADR-0043: Bootstrap Integer Runtime Semantics”.
- `docs/ambiguities/M0028-static-integer-constant-expressions.md`.

## Authority Read

- `AGENTS.md`.
- `docs/SPEC.md`, ADR-0043 and ADR-0048 sections.
- `docs/adr/ADR-0042-bootstrap-executable-operators.md`.
- `docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md`.
- `docs/adr/ADR-0048-bootstrap-integer-constant-expressions.md`.
- `docs/milestones/M0028-executable-expression-frontend-completion.md`.

## Scope

- Implement accepted ADR-0043 static integer diagnostics for ADR-0048 constant
  expression trees.

## Out Of Scope

- Runtime trap behavior, calls, returns, entry points, HIR, MIR, and backend.

## Required Tests

- Positive and negative diagnostics for literal-tree arithmetic.

## Test-First Gate

- Expected pre-implementation result: `fail`.
- Reason: no constant-expression evaluator or ADR-0043 diagnostic API exists.
- Reviewer approval required to modify/delete failing tests: `yes`

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Ambiguity report filed before implementation.
- [x] Accepted constant-expression authority exists.
- [x] Tests are created before implementation.
- [x] Static diagnostics follow accepted authority.

## Execution Commands

- Generate tests: `cargo test -p compiler --test type_check`.
- Verify tests fail: `cargo test -p compiler --test type_check`.
- Ordinary tests: `cargo test --workspace --all-targets`.
- Adversarial tests: `sh docs/scripts/adversarial-check.sh docs/tasks/M0028-005-static-integer-diagnostics.md`.
- Review: `docs/scripts/review-task.sh docs/tasks/M0028-005-static-integer-diagnostics.md`
- CI: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets`.

## Forbidden Changes

- Do not modify `docs/SPEC.md` or accepted ADRs without the semantic-change workflow.
- Do not infer a constant-expression rule from Kotlin, Rust, or current implementation.

## Execution Log

- 2026-07-11 agent=Main phase=create-task result=blocked evidence=constant-expression authority missing; ambiguity report filed. handoff=Language-Designer
- 2026-07-11 agent=Main phase=semantic-resolution result=pass evidence=ADR-0048 accepted literal-tree constant boundary. handoff=Test
- 2026-07-11 agent=Main phase=tests-created result=pass evidence=range, overflow, division-by-zero, negative-exponent, invalid-shift, minimum-Int, nonconstant-tree, and full-operator tests added. handoff=Test
- 2026-07-11 agent=Main phase=pre-implementation-test result=fail evidence=minimum-Int/nonconstant-tree test reported four range diagnostics instead of the expected one. handoff=Implement
- 2026-07-11 agent=Main phase=implementation result=pass evidence=maximal constant trees are evaluated once; unary minimum-Int magnitude is preserved; exponentiation avoids narrowing its exponent. handoff=Validation
- 2026-07-11 agent=Main phase=focused-validation result=pass evidence=`cargo test -p compiler --test type_check` (83 passed); `docs/tests/m0028-static-integer-diagnostics.sh` passed. handoff=CI
- 2026-07-11 agent=Main phase=ordinary-tests result=pass evidence=`cargo test --workspace --all-targets` (278 passed, 14 suites). handoff=Adversarial
- 2026-07-11 agent=Main phase=adversarial-review result=pass evidence=minimum-Int, arithmetic failures, nonconstant-tree boundary, and non-cascading diagnostics reviewed in `docs/tasks/soundness/M0028-005-soundness.md`. handoff=Review
- 2026-07-11 agent=Main phase=task-review result=pass evidence=`docs/tasks/reviews/M0028-005-review.md` approves scope, authority, maintainability, and test-first integrity. handoff=CI
- 2026-07-11 agent=Main phase=ci result=pass evidence=`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace --all-targets` (278 passed, 14 suites); relevant M0028 and roadmap docs validators passed. handoff=Commit

## Files Changed

- `crates/compiler/src/type_check.rs`
- `crates/compiler/tests/type_check.rs`
- `docs/SPEC.md`
- `docs/adr/ADR-0048-bootstrap-integer-constant-expressions.md`
- `docs/adr/proposals/reviews/ADR-0048-*.md`
- `docs/ambiguities/M0028-static-integer-constant-expressions.md`
- `docs/tests/m0028-static-integer-diagnostics.sh`
- `docs/tasks/soundness/M0028-005-soundness.md`
- `docs/tasks/reviews/M0028-005-review.md`
- `examples/current/README.md`
- `examples/current/executable_int_arithmetic.neu`
- `examples/current/static_integer_diagnostics.neu`

## Open Questions

- None.

## Remaining Risk And Next Action

- Remaining risk: no compiler-driver integration exists yet; M0028's later
  entry, call, and return work owns that boundary.
- Next action: commit this completed task, then select the next M0028 task.

## Handoff

- Next main task: `main-task test work`.
- Reason: tests can now encode ADR-0043 static diagnostics against ADR-0048.

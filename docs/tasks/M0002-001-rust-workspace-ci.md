# Task: M0002-001 Create Rust Workspace And CI Skeleton

## Task Metadata

- Task ID: `M0002-001`
- Milestone: `M0002`
- Milestone File: `docs/milestones/M0002-rust-workspace-and-ci-skeleton.md`
- Status: `complete`
- Owner main task: `main-task build check`
- Created By: `main-task task planning`
- Created Date: `2026-07-09`
- Branch: `task/M0002-001-rust-workspace-ci`

## Source Of Truth

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`
- Project Rules: `docs/main task rules`
- main task Prompts:
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`
  - `main task rules`

## Goal

Create a minimal Rust workspace, documented build commands, and CI skeleton that run formatting, lint, and tests without implementing compiler features.

## Motivation

M0002 requires repeatable validation commands before lexer, parser, semantic analysis, or backend work begins.

## Scope

- Add root Rust workspace metadata.
- Add a placeholder Rust crate that contains no compiler semantics.
- Add formatting, lint, and test command documentation.
- Add CI configuration that runs the documented gates.
- Add a validation script for this milestone task.

## Out Of Scope

- Lexer, parser, AST, HIR, MIR, semantic analysis, ownership checking, borrow checking, or backend behavior.
- Cranelift or LLVM dependencies.
- Target-pack implementation.
- Language syntax or semantic decisions.

## Required Inputs

- Milestone: `docs/milestones/M0002-rust-workspace-and-ci-skeleton.md`
- Spec sections:
  - M0002 milestone acceptance criteria.
  - Project source-of-truth path alignment from M0001.
- ADRs:
  - `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`
- Existing files:
  - `docs/ROADMAP.md`
  - `docs/main task rules`
  - `main task rules`

## Required Tests

Tests must be created before implementation.

- Positive tests:
  - `docs/tests/m0002-workspace-ci.sh` verifies that workspace files exist and documented gates run.
- Negative tests:
  - The validation script must fail before implementation because `Cargo.toml` and CI files are absent.
- Diagnostic tests:
  - Not applicable; this task introduces no compiler diagnostics.
- Adversarial tests:
  - Confirm no lexer, parser, AST, semantic, or backend files are introduced by this task.

## Test-First Gate

- Test files to create before implementation:
  - `docs/tests/m0002-workspace-ci.sh`
- Expected pre-implementation result: `fail`
- Failure reason expected before implementation:
  - The Rust workspace manifest and CI skeleton do not exist yet.
- main-task review approval required to modify/delete failing tests: `yes`

## Implementation Plan

Add the minimum workspace, placeholder crate, CI workflow, and build documentation needed for `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace --all-targets` to run successfully.

## Acceptance Criteria

- [x] Task references exactly one milestone.
- [x] Scope and out-of-scope are explicit.
- [x] Tests are created before implementation.
- [x] Tests fail before implementation for the expected reason.
- [x] Implementation is the smallest passing change.
- [x] Ordinary tests pass.
- [x] Adversarial tests pass after ordinary tests.
- [x] main-task review compares output against `docs/SPEC.md` and the milestone.
- [x] CI passes as final gate.
- [x] Milestone checklist is updated.

## Execution Commands

- Generate tests: `create docs/tests/m0002-workspace-ci.sh`
- Verify tests fail: `docs/tests/m0002-workspace-ci.sh`
- Ordinary tests: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets`
- Adversarial tests: `docs/tests/m0002-workspace-ci.sh`
- Review: `docs/scripts/review-task.sh docs/tasks/M0002-001-rust-workspace-ci.md`
- CI: `docs/tests/m0002-workspace-ci.sh`

## Files Expected To Change

- Test files:
  - `docs/tests/m0002-workspace-ci.sh`
- Implementation files:
  - `Cargo.toml`
  - `Cargo.lock`
  - `rust-toolchain.toml`
  - `crates/compiler/Cargo.toml`
  - `crates/compiler/src/lib.rs`
  - `.github/workflows/ci.yml`
- Documentation or checklist files:
  - `docs/build.md`
  - `docs/milestones/M0002-rust-workspace-and-ci-skeleton.md`
  - `docs/tasks/M0002-001-rust-workspace-ci.md`

## Forbidden Changes

- Do not modify `docs/SPEC.md`.
- Do not modify `docs/adr/`.
- Do not weaken or delete failing tests without main-task review approval.
- Do not implement work outside this task scope.
- Do not introduce language semantics not present in `docs/SPEC.md` or `docs/adr/`.

## Ambiguities And Dependencies

- No semantic ambiguity blocks this task because it intentionally creates no compiler behavior.
- Build command details are owned by main-task build check and do not define language semantics.

## Execution Log

Append entries as the task progresses.

```text
2026-07-09 main_task=Task-Decomposer phase=create-task result=pass notes=Created first M0002 task and narrowed it to workspace plus CI skeleton.
2026-07-09 main_task=main-task test work phase=generate-tests result=pass notes=Created docs/tests/m0002-workspace-ci.sh before implementation.
2026-07-09 main_task=main-task test work phase=verify-tests-fail result=pass notes=Validation failed as expected: missing required file Cargo.toml.
2026-07-09 main_task=main-task implementation phase=ordinary-tests result=pass notes=docs/tests/m0002-workspace-ci.sh passed after adding the minimal Rust workspace and CI skeleton.
2026-07-09 main_task=Adversarial-Engineer phase=adversarial-tests result=pass notes=Soundness report approved; no compiler-feature paths or unsafe code introduced.
2026-07-09 main_task=main-task review phase=review result=pass notes=Review approved against docs/SPEC.md and M0002.
2026-07-09 main_task=Build-Engineer phase=ci result=pass notes=Final CI-equivalent gate docs/tests/m0002-workspace-ci.sh passed.
2026-07-09 main_task=Build-Engineer phase=commit result=blocked notes=Repo root is /Users/c16a/projects with unrelated untracked directories; no scoped commit attempted.
```

## Handoff

- Next main task: `main-task roadmap planning`
- Reason: `M0002 is complete; select M0003 next.`
- Required Context:
  - This task file
  - `docs/SPEC.md`
  - `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`
  - `docs/milestones/M0002-rust-workspace-and-ci-skeleton.md`

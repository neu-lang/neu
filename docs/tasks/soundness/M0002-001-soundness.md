# Soundness Report: M0002-001

## Metadata

- Task ID: `M0002-001`
- Milestone: `M0002`
- Filed By: `main-task adversarial check`
- Date: `2026-07-09`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0002-001-rust-workspace-ci.md`
- Milestone file: `docs/milestones/M0002-rust-workspace-and-ci-skeleton.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`
- Changed files:
  - `Cargo.toml`
  - `rust-toolchain.toml`
  - `crates/newlang/Cargo.toml`
  - `crates/newlang/src/lib.rs`
  - `.github/workflows/ci.yml`
  - `docs/build.md`
  - `docs/tests/m0002-workspace-ci.sh`
- Ordinary test results:
  - `docs/tests/m0002-workspace-ci.sh` passed after implementation.

## Safety Invariants Checked

- [x] Ownership cannot be bypassed by this task because no ownership semantics are implemented.
- [x] Moved values cannot be reused by this task because no move semantics are implemented.
- [x] Shared and exclusive borrows cannot conflict by this task because no borrow semantics are implemented.
- [x] Borrowed data cannot outlive its owner by this task because no lifetime semantics are implemented.
- [x] Nullability refinements cannot be used after invalidation by this task because no nullability semantics are implemented.
- [x] Thread send/share capabilities are not weakened because no concurrency semantics are implemented.
- [x] Coroutine scopes are not affected because no coroutine semantics are implemented.
- [x] Borrows across suspension are not affected because no async semantics are implemented.
- [x] Unsafe and FFI boundaries do not weaken safe-code guarantees because the placeholder crate forbids unsafe code.
- [x] Diagnostics do not hide or misstate safety failures because no compiler diagnostics are implemented.

## Attacks Attempted

```text
Attack: Check whether M0002 introduced lexer/parser/AST/HIR/MIR/backend files.
Expected result: No such files exist.
Actual result: docs/tests/m0002-workspace-ci.sh passed absence checks.
Source of truth: docs/milestones/M0002-rust-workspace-and-ci-skeleton.md
Outcome: pass

Attack: Check whether unsafe code is allowed in the placeholder crate.
Expected result: Unsafe code is forbidden.
Actual result: crates/newlang/src/lib.rs uses #![forbid(unsafe_code)].
Source of truth: M0002 no compiler behavior scope plus safe-code project constraints.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0002-workspace-ci.sh`
- Tests run:
  - `docs/tests/m0002-workspace-ci.sh`
- Result:
  - pass

## Findings

No blocking findings.

## Ambiguities

- None.

## Decision

Pass.


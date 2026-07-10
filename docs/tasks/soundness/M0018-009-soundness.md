# Soundness Report: M0018-009

## Metadata

- Task ID: `M0018-009`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-009-local-declaration-type-metadata.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/newlang/src/parser.rs`
  - `crates/newlang/tests/parser.rs`
  - `docs/tasks/M0018-009-local-declaration-type-metadata.md`
- Ordinary test results:
  - `cargo fmt --all --check`: pass
  - `cargo test --workspace --all-targets`: pass, 138 tests
  - `cargo clippy --workspace --all-targets -- -D warnings`: pass
  - `sh docs/tests/m0018-type-checking-core-accepted.sh`: pass
  - `sh docs/tests/m0002-workspace-ci.sh`: pass

## Safety Invariants Checked

- [x] Ownership cannot be bypassed.
- [x] Moved values cannot be reused.
- [x] Shared and exclusive borrows cannot conflict.
- [x] Borrowed data cannot outlive its owner.
- [x] Nullability refinements cannot be used after invalidation.
- [x] Thread send/share capabilities are enforced.
- [x] Coroutine scopes cannot outlive allowed ownership or borrow lifetimes.
- [x] Borrows across suspension are rejected unless proven safe by accepted semantics.
- [x] Unsafe and FFI boundaries do not weaken safe-code guarantees.
- [x] Diagnostics do not hide or misstate safety failures.

## Attacks Attempted

Attack: Treat local declaration metadata as declaration type checking.
Expected result: Metadata records node relationships only.
Actual result: The change adds `ParsedLocalDeclaration` with declaration, annotation, and initializer node ids; no type-checking APIs changed.
Source of truth: `docs/tasks/M0018-009-local-declaration-type-metadata.md` Scope and Out Of Scope.
Outcome: pass

Attack: Emit metadata for malformed declarations.
Expected result: Malformed declarations do not produce local declaration metadata.
Actual result: Tests assert only the valid `var ok: Int = 1;` declaration is recorded after a malformed `val`.
Source of truth: `docs/tasks/M0018-009-local-declaration-type-metadata.md` Negative tests.
Outcome: pass

Attack: Use metadata to perform assignment compatibility, conversions, ownership, borrow checking, HIR, MIR, or backend behavior.
Expected result: No such behavior is added.
Actual result: Changes are limited to parser metadata and parser tests.
Source of truth: `docs/adr/ADR-0027-type-checking-core.md` Explicit Deferrals.
Outcome: pass

## Adversarial Tests

- Tests added:
  - No separate adversarial Rust tests were needed beyond parser metadata unit tests.
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-009-local-declaration-type-metadata.md`
  - `cargo test --workspace --all-targets`
  - `sh docs/tests/m0018-type-checking-core-accepted.sh`
- Result:
  - pass

## Findings

No findings.

## Ambiguities

No new ambiguity found. Type resolution and declaration mismatch diagnostics remain later M0018 tasks.

## Decision

Pass.

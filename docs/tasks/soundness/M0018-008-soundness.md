# Soundness Report: M0018-008

## Metadata

- Task ID: `M0018-008`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-008-parser-literal-type-inputs.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/newlang/src/parser.rs`
  - `crates/newlang/src/type_check.rs`
  - `crates/newlang/tests/parser.rs`
  - `crates/newlang/tests/type_check.rs`
  - `docs/tasks/M0018-008-parser-literal-type-inputs.md`
- Ordinary test results:
  - `cargo fmt --all --check`: pass
  - `cargo test --workspace --all-targets`: pass, 136 tests
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

Attack: Treat parser literal metadata as new syntax acceptance.
Expected result: Only already-accepted literal tokens produce metadata.
Actual result: Parser changes only classify token kinds already handled by `parse_primary_expression`.
Source of truth: `docs/adr/ADR-0024-expression-statement-pattern-syntax.md` and `docs/adr/ADR-0027-type-checking-core.md`.
Outcome: pass

Attack: Record non-literal expressions as literal metadata.
Expected result: Name and call expressions do not become literal inputs.
Actual result: Tests assert parser output for `compute()` and `item` contains no literal expression metadata.
Source of truth: `docs/tasks/M0018-008-parser-literal-type-inputs.md` Negative tests.
Outcome: pass

Attack: Use parser literal mapping to infer missing expression types.
Expected result: Type checker creates entries only for parser literal metadata records.
Actual result: Type-check tests assert a missing expression node lookup returns `None`.
Source of truth: `docs/adr/ADR-0027-type-checking-core.md` Typed Output Shape section.
Outcome: pass

Attack: Smuggle assignment compatibility, type mismatch diagnostics, calls, ownership, borrow checking, HIR, MIR, or backend behavior into the mapping.
Expected result: Mapping is limited to literal metadata conversion and existing literal typing.
Actual result: Changes add no assignment compatibility, mismatch diagnostics, call typing, safety analysis, lowering, or backend code.
Source of truth: `docs/adr/ADR-0027-type-checking-core.md` Explicit Deferrals.
Outcome: pass

## Adversarial Tests

- Tests added:
  - No separate adversarial Rust tests were needed beyond parser metadata and type-check wrapper unit tests.
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-008-parser-literal-type-inputs.md`
  - `cargo test --workspace --all-targets`
  - `sh docs/tests/m0018-type-checking-core-accepted.sh`
- Result:
  - pass

## Findings

No findings.

## Ambiguities

No new ambiguity found. Assignment compatibility and mismatch diagnostics remain later M0018 tasks.

## Decision

Pass.

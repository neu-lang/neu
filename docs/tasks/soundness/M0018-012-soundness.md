# Soundness Report: M0018-012

## Metadata

- Task ID: `M0018-012`
- Milestone: `M0018`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-012-resolved-local-name-expression-typing.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0027-type-checking-core.md`
  - `docs/adr/ADR-0026-name-resolution-policy.md`
- Changed files:
  - `crates/compiler/src/type_check.rs`
  - `crates/compiler/tests/type_check.rs`
  - `docs/tasks/M0018-012-resolved-local-name-expression-typing.md`
- Ordinary test results:
  - `cargo fmt --all --check`: pass
  - `cargo test --workspace --all-targets`: pass, 144 tests
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

Attack: Infer a name expression type from resolution alone.
Expected result: Resolved names without a `KnownSymbolType` input are skipped.
Actual result: Tests assert unknown symbols and missing resolution entries do not synthesize expression type entries.
Source of truth: `docs/adr/ADR-0027-type-checking-core.md` Typed Output Shape.
Outcome: pass

Attack: Change name-resolution semantics from the type checker.
Expected result: Type checker consumes an existing `ResolutionTable` only.
Actual result: No name-resolution implementation was changed; the function reads ordered `resolved_names()`.
Source of truth: `docs/adr/ADR-0026-name-resolution-policy.md`.
Outcome: pass

Attack: Smuggle assignment checks, declaration signatures, member lookup, calls, ownership, borrow checking, HIR, MIR, or backend behavior into name typing.
Expected result: Name expression typing records expression type entries only.
Actual result: Tests assert diagnostics, declaration signatures, and assignment checks remain empty.
Source of truth: `docs/tasks/M0018-012-resolved-local-name-expression-typing.md` Out Of Scope.
Outcome: pass

## Adversarial Tests

- Tests added:
  - No separate adversarial Rust tests were needed beyond known-symbol and unknown-symbol unit tests.
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0018-012-resolved-local-name-expression-typing.md`
  - `cargo test --workspace --all-targets`
  - `sh docs/tests/m0018-type-checking-core-accepted.sh`
- Result:
  - pass

## Findings

No findings.

## Ambiguities

No new ambiguity found. Integrating declaration signatures into known symbol type inputs remains a later M0018 task.

## Decision

Pass.

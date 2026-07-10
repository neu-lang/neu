# Soundness Report: M0019-007

## Metadata

- Task ID: `M0019-007`
- Milestone: `M0019`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0019-007-parser-flow-metadata.md`
- Milestone file: `docs/milestones/M0019-nullability-and-flow-typing.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0028-nullability-and-flow-typing.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/newlang/src/parser.rs`
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0019-parser-flow-metadata.sh`
- Ordinary test results:
  - `cargo test -p newlang --test parser`: pass
  - `sh docs/tests/m0019-parser-flow-metadata.sh`: pass

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

```text
Attack: Treat parser metadata as semantic null-test recognition.
Expected result: Rejected.
Actual result: Metadata records operands, operators, and branch nodes only; validator rejects recognize_null_test and apply_smart_cast symbols.
Source of truth: ADR-0028 Null-Test Recognition.
Outcome: pass
```

```text
Attack: Type check binary expressions while collecting operands.
Expected result: Rejected.
Actual result: Parser records syntax only; no type_check behavior was added.
Source of truth: ADR-0024 parser syntax and ADR-0028 flow-specific recognizer requirement.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0019-parser-flow-metadata.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0019-007-parser-flow-metadata.md`
  - `cargo test -p newlang --test parser`
  - `sh docs/tests/m0019-parser-flow-metadata.sh`
- Result:
  - pass

## Findings

None.

## Ambiguities

- None for parser metadata. Null-test recognition remains a later semantic task.

## Decision

Pass. The change is syntax metadata only.

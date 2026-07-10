# Soundness Report: M0018-001

## Metadata

- Task ID: `M0018-001`
- Milestone: `M0018`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0018-001-type-checking-ambiguity-blocker.md`
- Milestone file: `docs/milestones/M0018-type-checking-core.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/newlang/src/type_check.rs`
  - `crates/newlang/tests/type_check.rs`
  - `docs/ambiguities/M0018-type-checking-core.md`
  - `docs/tests/m0018-type-checking-ambiguity-blocker.sh`
- Ordinary test results:
  - `cargo test -p newlang --test type_check`: pass
  - `sh docs/tests/m0018-type-checking-ambiguity-blocker.sh`: pass

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
Attack: Smuggle type inference or expression checking into the ambiguity blocker.
Expected result: No checking or inference functions exist.
Actual result: Validator rejects check_expression, check_declaration, infer_type, literal_type, resolve_call, check_assignment, TypedExpression, TypedProgram, and WellTyped patterns.
Source of truth: M0018 acceptance criteria and ambiguity report.
Outcome: pass
```

```text
Attack: Use the blocker to define primitive, literal, assignment, call, or function application semantics.
Expected result: Only blocked rule categories are recorded.
Actual result: Tests verify diagnostics; no typed output is produced.
Source of truth: docs/ambiguities/M0018-type-checking-core.md.
Outcome: pass
```

```text
Attack: Mix later safety or backend phases into type checking core.
Expected result: No ownership, borrow, flow typing, HIR, MIR, or backend behavior is introduced.
Actual result: Validator rejects Ownership, Borrow, FlowType, Hir, Mir, Cranelift, and LLVM patterns.
Source of truth: M0018 out-of-scope list.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0018-type-checking-ambiguity-blocker.sh`
- Tests run:
  - `sh docs/tests/m0018-type-checking-ambiguity-blocker.sh`
- Result:
  - pass

## Findings

No soundness findings.

## Ambiguities

- `docs/ambiguities/M0018-type-checking-core.md` remains open and blocks literal typing, primitive scalar categories, assignment compatibility, call resolution, and function type application.

## Decision

Pass. The task blocks ambiguous type checking rules without implementing them.

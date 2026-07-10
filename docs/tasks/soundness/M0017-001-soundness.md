# Soundness Report: M0017-001

## Metadata

- Task ID: `M0017-001`
- Milestone: `M0017`
- Filed By: `Adversarial Engineer`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0017-001-type-identity-model.md`
- Milestone file: `docs/milestones/M0017-type-representation.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0010-type-system-shape.md`
  - `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`
- Changed files:
  - `crates/newlang/src/lib.rs`
  - `crates/newlang/src/types.rs`
  - `crates/newlang/tests/types.rs`
  - `docs/tests/m0017-type-identity-model.sh`
- Ordinary test results:
  - `cargo test -p newlang --test types`: pass
  - `sh docs/tests/m0017-type-identity-model.sh`: pass

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
Attack: Look for type inference or constraint solving introduced by the task.
Expected result: No inference or solver logic exists.
Actual result: Validator rejects infer_type, solve_constraints, and ConstraintSolver patterns.
Source of truth: docs/milestones/M0017-type-representation.md out-of-scope list.
Outcome: pass
```

```text
Attack: Look for ownership, layout, ABI, HIR, MIR, or backend behavior hidden in the type model.
Expected result: No such behavior exists.
Actual result: Validator rejects OwnershipCapability, Layout, Abi, Hir, Mir, Cranelift, and LLVM patterns.
Source of truth: docs/milestones/M0017-type-representation.md out-of-scope list.
Outcome: pass
```

```text
Attack: Look for guessed primitive scalar or nullable representation in this identity-only task.
Expected result: No primitive catalog or nullable wrapper is introduced.
Actual result: Validator rejects PrimitiveScalar, common primitive type names, NullableType, OptionalType, and TypeKind::Nullable patterns.
Source of truth: M0017 risk notes and task scope.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0017-type-identity-model.sh`
- Tests run:
  - `sh docs/tests/m0017-type-identity-model.sh`
- Result:
  - pass

## Findings

No soundness findings.

## Ambiguities

- Primitive scalar categories remain unresolved for a later M0017 task.
- Nullable representation remains required for M0017 but is intentionally outside this identity task.

## Decision

Pass. This task adds identity representation only and does not weaken safety-critical semantics.

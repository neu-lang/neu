# Soundness Report: M0017-002

## Metadata

- Task ID: `M0017-002`
- Milestone: `M0017`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0017-002-nullable-type-representation.md`
- Milestone file: `docs/milestones/M0017-type-representation.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
- Changed files:
  - `crates/newlang/src/types.rs`
  - `crates/newlang/tests/types.rs`
  - `docs/tests/m0017-nullable-type-representation.sh`
- Ordinary test results:
  - `cargo test -p newlang --test types`: pass
  - `sh docs/tests/m0017-nullable-type-representation.sh`: pass

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
Attack: Treat nullable representation as implicit nullability checking.
Expected result: No null checking, null literal typing, smart casts, or flow typing are introduced.
Actual result: Validator rejects smart_cast, flow_typ, FlowType, MutationInvalidation, NullCheck, and NullLiteral patterns.
Source of truth: ADR-0006 downstream consequences and M0017 out-of-scope boundary.
Outcome: pass
```

```text
Attack: Use nullable representation to add platform or FFI null behavior.
Expected result: No FFI or platform nullability mapping is introduced.
Actual result: Validator rejects FfiNull and PlatformNull patterns.
Source of truth: ADR-0006 says FFI null mapping must be defined separately.
Outcome: pass
```

```text
Attack: Hide inference, constraint solving, ownership, layout, ABI, HIR, MIR, or backend behavior in the type model.
Expected result: None of those systems appear in this task.
Actual result: Validator rejects infer_type, solve_constraints, ConstraintSolver, OwnershipCapability, Layout, Abi, Hir, Mir, Cranelift, and LLVM patterns.
Source of truth: M0017 out-of-scope list.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0017-nullable-type-representation.sh`
- Tests run:
  - `sh docs/tests/m0017-nullable-type-representation.sh`
- Result:
  - pass

## Findings

No soundness findings.

## Ambiguities

- Generic nullability constraints remain unspecified and deferred.
- Null literal typing is deferred to type checking.
- Repeated nullable marker diagnostics remain parser-owned under ADR-0023.

## Decision

Pass. This task adds only a distinguishable nullable wrapper representation.

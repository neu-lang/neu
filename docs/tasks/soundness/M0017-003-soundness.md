# Soundness Report: M0017-003

## Metadata

- Task ID: `M0017-003`
- Milestone: `M0017`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0017-003-unsupported-type-form-blocking.md`
- Milestone file: `docs/milestones/M0017-type-representation.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0023-type-and-generic-syntax.md`
- Changed files:
  - `crates/compiler/src/types.rs`
  - `crates/compiler/tests/types.rs`
  - `docs/tests/m0017-unsupported-type-form-blocking.sh`
- Ordinary test results:
  - `cargo test -p compiler --test types`: pass
  - `sh docs/tests/m0017-unsupported-type-form-blocking.sh`: pass

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
Attack: Smuggle deferred type forms into the usable type model.
Expected result: Unsupported forms are diagnostics only, with no TypeKind::Unsupported.
Actual result: Validator rejects TypeKind::Unsupported and Unsupported(UnsupportedTypeForm).
Source of truth: M0017 acceptance criteria and task forbidden changes.
Outcome: pass
```

```text
Attack: Add type lowering or type checking behavior while blocking forms.
Expected result: No lowering, inference, solver, or primitive catalog behavior is introduced.
Actual result: Validator rejects lower_type, type_lower, infer_type, solve_constraints, ConstraintSolver, and PrimitiveScalar patterns.
Source of truth: M0017 out-of-scope list.
Outcome: pass
```

```text
Attack: Hide ownership, backend, or platform behavior in type diagnostics.
Expected result: No ownership, backend, or code generation terms appear in the type model.
Actual result: Validator rejects OwnershipCapability, Cranelift, and LLVM patterns.
Source of truth: M0017 out-of-scope list.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0017-unsupported-type-form-blocking.sh`
- Tests run:
  - `sh docs/tests/m0017-unsupported-type-form-blocking.sh`
- Result:
  - pass

## Findings

No soundness findings.

## Ambiguities

- Primitive scalar categories remain unresolved for a separate task before type checking relies on them.
- Lowering parsed unsupported syntax into `TypeDiagnostic` is deferred.

## Decision

Pass. Unsupported forms are explicitly blocked as diagnostics and do not become usable types.

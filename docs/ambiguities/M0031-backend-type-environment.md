# Ambiguity Report: M0031 Backend Type Environment

## Metadata

- Report ID: `M0031-BACKEND-TYPE-ENVIRONMENT`
- Related Task: `M0031-001`
- Related Milestone: `M0031`
- Filed By: `main-task architecture review`
- Date: `2026-07-11`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`, ADR-0044 through ADR-0046 summaries.
- ADRs:
  - `docs/adr/ADR-0045-bootstrap-mir-runtime-contract.md`
  - `docs/adr/ADR-0046-bootstrap-abi-and-calling-convention.md`
  - `docs/adr/ADR-0052-bootstrap-module-type-identity.md`
- Milestone:
  - `docs/milestones/M0031-cranelift-backend-smoke.md`

## Exact Ambiguous Text Or Missing Rule

```text
ADR-0052 says a TypeId is meaningful only with its owning module TypeArena.
ADR-0045 requires MIR function and expression type facts, and ADR-0046 requires
the backend to lower Int to a signed 64-bit value. Neither ADR defines how the
owning TypeArena reaches HIR-to-MIR or MIR-to-backend lowering.
```

## Competing Interpretations

1. Backend lowering treats a raw TypeId numeric value as the bootstrap `Int`.
2. HIR and MIR own copied TypeArena instances.
3. Lowering APIs receive the owning module TypeArena as explicit companion
   input and resolve every runtime TypeId through it.
4. MIR replaces TypeId with a backend runtime-type enum immediately.

## Why Guessing Is Unsafe

- A raw TypeId has no meaning without its arena and can identify a different
  type under another insertion order.
- Copying the arena risks divergent identity domains and hidden duplication.
- A backend runtime-type enum would prematurely replace typed representation
  facts and obscure diagnostics.

## Affected Work

- Tasks blocked: `M0030-004`, `M0031-001`.
- Milestones affected: `M0030`, `M0031`.
- Tests blocked: typed HIR-to-MIR and MIR-to-Cranelift lowering tests.
- Implementation areas blocked: `crates/compiler/src/mir.rs`,
  `crates/compiler/src/backend.rs`.

## Recommended Resolution Path

- [x] main-task language review determines whether existing text resolves it.
- [x] main-task semantic design drafts ADR or spec revision if new semantics are required.
- [x] main-task adversarial check reviews soundness risk.
- [x] main-task diagnostics check reviews diagnostic consequences.
- [x] main-task simplicity check reviews complexity.
- [x] main task approves final resolution.

## Temporary Rule

No backend lowering may infer a runtime type from the numeric representation of
a TypeId.

## Resolution

- Decision: ADR-0055 requires the owning module TypeArena as explicit
  companion input to typed lowering boundaries.
- Source of truth updated: `docs/adr/ADR-0055-bootstrap-type-environment-transport.md`
  and `docs/SPEC.md`.
- Date resolved: `2026-07-11`

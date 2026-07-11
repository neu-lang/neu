# M0035: Primitive Runtime Support

## Title

M0035: Bool, Unit, Float, And Byte Runtime Support

## Identifier

M0035

## Goal

Extend the accepted bootstrap executable pipeline beyond `Int` to support
`Bool`, `Unit`, `Float`, and `Byte` end to end.

## Motivation

The compiler currently carries several primitive identities only through type
checking while HIR, MIR, and Cranelift accept `Int` runtime values. Systems
programs need the additional primitive values with explicit representations and
without garbage collection or manual memory management.

## Background

ADR-0059 defines the primitive syntax, operations, representations, diagnostics,
and intermediate-representation contracts. Existing entry-point and target-pack
contracts remain unchanged.

## Prerequisites

- M0034
- ADR-0059

## Inputs

- `docs/SPEC.md`
- Accepted ADRs 0027, 0042, 0043, 0044, 0045, 0046, 0058, and 0059.
- Existing lexer, parser, type checker, HIR, MIR, Cranelift, and target-pack
  tests.

## Outputs

- Primitive-aware frontend and type checking.
- Primitive-aware HIR and MIR.
- Cranelift lowering and object/link smoke coverage.
- Updated current examples containing only accepted runnable forms.

## Scope

- Bool, Unit, Float, and Byte only as defined by ADR-0059.
- Exact typing, diagnostics, ownership/thread capability preservation, and
  target-aware primitive lowering.

## Out Of Scope

- Implicit numeric conversions, casts, strings, heap allocation, standard
  library, printing, FFI, public ABI, LLVM, or new control-flow features.

## Deliverables

- Ordered implementation tasks M0035-001 through M0035-006.
- Tests and negative diagnostics for each layer.
- Primitive backend and cross-target smoke evidence.

## Acceptance Criteria

- Each requested primitive is lexed/parsed, typed, represented in HIR/MIR, and
  lowered by Cranelift according to ADR-0059.
- Unit produces no ABI result and does not require fabricated storage.
- Bool and Byte representations are normalized/checked at boundaries.
- Float uses `f64` IEEE behavior without integer-style traps.
- Full CI and primitive-specific validators pass.

## Test Strategy

- Test-first unit and integration tests for each ordered layer.
- Negative literal, type, operation, and runtime-trap tests.
- Host and cross-target object-format smoke tests.
- Current-example backend-surface validation.

## Risks

- Existing executable subset assumptions are Int-specific.
- Unit no-result calls may expose MIR and backend assumptions about value IDs.
- Float literal lexing and IEEE behavior need precise diagnostics and tests.

## Estimated Effort

10-18 working days across ordered small tasks.

## Expected Files Changed

- `docs/SPEC.md`, `docs/adr/`, and `docs/tasks/`.
- Compiler lexer/parser/type/HIR/MIR/backend modules and focused tests.
- `examples/current/` only when accepted source syntax becomes runnable.

## Completion Checklist

- [ ] Primitive frontend support passes.
- [ ] Primitive HIR/MIR support passes.
- [ ] Primitive Cranelift lowering passes.
- [ ] Host and cross-target smoke tests pass.
- [ ] Full CI and release documentation are updated.

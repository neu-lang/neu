# Task: M0030-005 Unary Integer MIR Lowering

## Task Metadata

- Task ID: `M0030-005`
- Milestone: `M0030`
- Milestone File: `docs/milestones/M0030-mir-design-and-lowering.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Preserve ADR-0043 unary `Int` operations in backend-independent MIR.

## Authority Extract

- ADR-0043 unary plus, negation, complement, and negation-overflow rules.
- ADR-0044 typed HIR runtime contract.
- ADR-0045 checked/trapping `Int` operations in MIR.
- ADR-0055 TypeArena transport.

## Scope

- Add source-mapped MIR unary operation records.
- Lower HIR unary plus, negation, and complement into those records.
- Keep typed input validation at the HIR-to-MIR boundary.

## Out Of Scope

- Cranelift backend lowering, exponentiation, locals, control flow, calls, or
  language-semantic changes.

## Test-First Gate

- Test: typed HIR unary plus, negation, and complement lower to ordered MIR
  unary operations with their input values and spans.
- Expected initial result: `fail`; MIR has no unary operation model.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0043
  semantics exist in HIR but are missing at the HIR-to-MIR boundary. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=unary HIR
  lowering returned `UnsupportedExpression` because MIR had no unary model.
  handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=typed
  unary HIR operations now preserve operand identity and spans in MIR.
  handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  unary MIR test and validator passed; formatter, Clippy, and workspace tests passed.
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=no
  raw type IDs, ownership, borrow, or unsafe facts are altered.
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0043,
  ADR-0044, ADR-0045, and ADR-0055 compliance confirmed.
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; focused unary MIR validator. handoff=commit

## Required Outputs

- Authority read: ADR-0043, ADR-0044, ADR-0045, ADR-0055.
- Files changed: MIR, MIR tests, task evidence, review, soundness report, and
  validator.
- Tests written before implementation and expected failure: unary HIR lowering
  initially returned `UnsupportedExpression`.
- Validation commands and results: all required gates passed.
- Open questions: none.
- Remaining risk and next main-task action: lower accepted unary MIR in M0031.

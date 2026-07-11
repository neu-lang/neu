# Task: M0030-003 MIR Function Return Type

## Task Metadata

- Task ID: `M0030-003`
- Milestone: `M0030`
- Milestone File: `docs/milestones/M0030-mir-design-and-lowering.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Make the MIR function contract preserve the declared result type required by
ADR-0045, so backend lowering can form a source-authorized function signature.

## Authority Extract

- ADR-0041 direct-call result types.
- ADR-0044 typed HIR contract.
- ADR-0045 ordered parameters and a return type in MIR function definitions.
- ADR-0046 bootstrap `Int` ABI lowering.

## Scope

- Store and expose each MIR function's declared return type.
- Propagate the HIR function return type during HIR-to-MIR lowering.
- Cover the model and lowering paths with focused tests.

## Out Of Scope

- Backend code, ABI lowering, local lowering, control-flow lowering, runtime
  traps, or language-semantic changes.

## Test-First Gate

- Test: direct MIR construction and lowered HIR expose the declared `Int`
  return type.
- Expected initial result: `fail`; `MirFunction` has no return-type field or
  accessor.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0045
  requires a MIR return type and the existing model omits it. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the MIR
  constructor has six arguments and `MirFunction::return_type` is absent.
  handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the
  MIR model and HIR-to-MIR lowering preserve the declared result type.
  handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=`cargo
  fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace --all-targets`; focused validator passed.
  handoff=adversarial
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=no
  safety facts are altered and the declared result type is preserved from HIR;
  `docs/tasks/soundness/M0030-003-soundness.md`. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0045 and
  M0030 contract compliance confirmed; `docs/tasks/reviews/M0030-003-review.md`.
  handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=`git diff --check`;
  `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D
  warnings`; `cargo test --workspace --all-targets`; focused validator.
  handoff=commit

## Required Outputs

- Authority read: ADR-0041, ADR-0044, ADR-0045, ADR-0046.
- Files changed: `crates/compiler/src/mir.rs`, `crates/compiler/tests/mir.rs`,
  `docs/tests/m0030-mir-function-return-type.sh`, task evidence.
- Tests written before implementation and expected failure: focused MIR test
  failed because the field, constructor argument, and accessor were absent.
- Validation commands and results: all required ordinary, adversarial, review,
  CI, and focused documentation-validator gates passed.
- Open questions: none.
- Remaining risk and next main-task action: validate backend task prerequisites.

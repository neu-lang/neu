# Task: M0032-001 Function Symbol Identity Transport

## Task Metadata

- Task ID: `M0032-001`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Preserve the structured bootstrap function identity required by ADR-0056 from
parsed declarations through HIR and MIR.

## Authority Extract

- ADR-0044 requires HIR to preserve function and module/package identity.
- ADR-0045 requires MIR function definitions with stable identity.
- ADR-0046 requires object symbols to derive from module, package, and source
  function name.
- ADR-0056 requires structured identity transport and forbids numeric-ID
  substitution.

## Scope

- Model module, package, and source-function identity for executable functions.
- Populate identity from accepted parsed declaration metadata in HIR.
- Carry the same identity from HIR into MIR.
- Add focused model and transport tests.

## Out Of Scope

- Object emission, linker selection or invocation, runtime startup, and process
  entry.
- Symbol escaping or target-specific object encoding.
- New language syntax or semantics.

## Test-First Gate

- Test: source-lowered HIR records `helper` and `main` identities, and HIR to
  MIR preserves the structured identity.
- Expected initial result: `fail`; HIR and MIR currently omit source function
  names.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0056
  resolves the M0032 object-symbol identity gap. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=identity
  accessors and transport assertions are not yet implemented. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=parsed
  declaration names populate HIR identity and HIR identity is copied into MIR.
  handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  HIR and MIR identity tests plus the task validator passed.
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  transport preserves module, package, and source name as structured values;
  no numeric ID or backend side table is used.
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0056 and
  ADR-0044 through ADR-0046 compliance confirmed.
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; M0032 identity validator; diff check. handoff=commit

## Required Outputs

- Authority read: ADR-0044, ADR-0045, ADR-0046, ADR-0056, and M0032.
- Files changed: `crates/compiler/src/module.rs`, `crates/compiler/src/hir.rs`,
  `crates/compiler/src/mir.rs`, HIR/MIR tests, this task record, the review and
  soundness reports, the validator, and the M0032 milestone input list.
- Tests written before implementation and expected failure: HIR source and
  HIR-to-MIR identity assertions fail until the transport is implemented.
- Validation commands and results: all required gates passed.
- Open questions: none; object encoding is a later task.
- Remaining risk and next main-task action: object emission must consume this
  identity in M0032-002.

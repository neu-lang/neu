# Task: M0032-002 Host Object Emission

## Task Metadata

- Task ID: `M0032-002`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `completed`

## Goal

Emit a parseable native host object for the currently supported bootstrap MIR
function shape using the ADR-0056 function identity.

## Authority Extract

- ADR-0046 selects the current host target and signed `Int` `I64` ABI for the
  initial backend smoke.
- ADR-0047 selects native host object output before linking and defers
  cross-target formats to M0033.
- ADR-0056 requires structured module, package, and source-function identity
  for object symbols.

## Scope

- Add Cranelift object emission for the existing verified one-block, no-parameter
  bootstrap MIR subset.
- Derive the internal object symbol from the supplied structured function
  identity.
- Verify that the emitted bytes parse as a native object and contain the
  function symbol.

## Out Of Scope

- Linker selection or invocation, runtime startup, process entry, and exit-code
  mapping.
- Direct calls, locals, multiple MIR blocks, cross-target packs, LLVM, and
  executable smoke tests.
- New language semantics or public ABI guarantees.

## Test-First Gate

- Test: an `Int` MIR function with identity `app/demo/main` emits a parseable
  native object containing its internal function symbol.
- Expected initial result: `fail`; object emission is not yet exposed.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0032-001
  now transports ADR-0056 identity into MIR. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=object
  emission API is not yet implemented. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=verified
  Cranelift lowering is defined through `cranelift-object`, with a deterministic
  length-prefixed hex symbol derived from ADR-0056 identity. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=object
  parser test, missing-identity negative test, and task validator passed.
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  emitter rejects missing identity before target/object work and does not use
  MIR-local IDs as symbol identity.
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0046,
  ADR-0047, and ADR-0056 compliance confirmed; no linker or runtime behavior
  was introduced.
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; object validator; current-example validator; diff check.
  handoff=commit

## Required Outputs

- Authority read: ADR-0046, ADR-0047, ADR-0056, and M0032.
- Files changed: compiler dependencies and backend, object tests, this task
  record, review and soundness reports, object/example validators, and the
  current backend-compatible example set.
- Tests written before implementation and expected failure: object integration
  test initially fails because the emission API is absent.
- Validation commands and results: all required gates passed.
- Open questions: none for object emission; linker/runtime remain later tasks.
- Remaining risk and next main-task action: linker, startup shim, and executable
  smoke remain for later M0032 tasks.

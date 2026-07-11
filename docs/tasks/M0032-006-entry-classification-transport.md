# Task: M0032-006 Entry Classification Transport

## Task Metadata

- Task ID: `M0032-006`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Preserve the accepted selected-entry classification from HIR into MIR for
later canonical entry-symbol and startup-shim lowering.

## Authority Extract

- ADR-0040 defines the selected top-level `main` function.
- ADR-0044 requires HIR entry classification.
- ADR-0046 and ADR-0057 require a language-entry symbol distinct from the raw
  platform entry.

## Scope

- Add an entry-classification fact to MIR functions.
- Copy HIR entry classification during HIR-to-MIR lowering.
- Add focused source-to-MIR coverage.

## Out Of Scope

- Canonical symbol emission, startup-shim implementation, linker invocation,
  process entry, or exit-code mapping.
- Entry-point selection or new source semantics.

## Tests

- A source-lowered HIR entry function remains marked as an entry function in
  MIR.
- A non-entry helper remains non-entry.

## Acceptance Criteria

- MIR preserves the HIR entry bit without deriving it from a numeric ID or
  function name at the backend.
- Existing non-entry MIR construction remains valid.
- No linker or runtime process is launched.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0032-005
  produces an entry argument plan but MIR does not yet carry HIR entry
  classification. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=MIR entry
  accessor and transport assertion are not implemented. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=HIR
  entry classification is copied into MIR without changing function identity or
  numeric-ID behavior. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=entry
  transport validator passed for entry and non-entry functions.
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  backend receives an explicit MIR fact and does not infer entry from names or
  IDs. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0040,
  ADR-0044, ADR-0046, and ADR-0057 compliance confirmed. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; example audit; entry validator; diff check. handoff=commit

## Required Outputs

- Authority read: ADR-0040, ADR-0044, ADR-0046, ADR-0057, and M0032.
- Files changed: `crates/compiler/src/mir.rs`, MIR tests, this task, review and
  soundness reports, and the validator.
- Tests written before implementation and expected failure: entry transport
  assertion fails because MIR has no entry fact.
- Validation commands and results: all required gates passed.
- Open questions: canonical symbol emission remains a later task.
- Remaining risk and next main-task action: canonical symbol emission and
  startup-shim linking remain later tasks.

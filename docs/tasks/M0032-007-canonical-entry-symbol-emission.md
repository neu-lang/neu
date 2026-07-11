# Task: M0032-007 Canonical Entry-Symbol Emission

## Task Metadata

- Task ID: `M0032-007`
- Milestone: `M0032`
- Milestone File: `docs/milestones/M0032-object-and-bundled-linker-pipeline.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Emit the selected MIR entry function under the target-pack canonical
language-entry symbol required by the startup shim.

## Authority Extract

- ADR-0040 selects exactly one top-level `main`.
- ADR-0046 separates language `main` from the raw platform entry symbol.
- ADR-0056 defines structured symbols for ordinary functions.
- ADR-0057 defines the pack-supplied canonical language-entry symbol.

## Scope

- Add an object-emission API accepting the pack canonical language-entry
  symbol.
- Use that symbol only for a MIR function explicitly marked as entry.
- Preserve ADR-0056-derived symbols for non-entry functions.
- Reject an entry emission request without a non-empty canonical symbol.

## Out Of Scope

- Startup-shim implementation, linker process execution, exit-code mapping,
  runtime traps, or target-pack artifact distribution.
- Entry selection, duplicate-main diagnostics, or new source semantics.

## Tests

- Entry object contains the supplied canonical symbol.
- Empty canonical symbol is rejected.
- Existing non-entry object emission retains its structured identity symbol.

## Acceptance Criteria

- Canonical symbol emission depends on explicit MIR entry classification.
- The target-pack symbol is passed as data rather than hardcoded in backend
  lowering.
- No raw platform entry symbol is treated as the language `main` symbol.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0032-006
  now transports MIR entry classification and M0032-005 exposes the pack
  language-entry symbol. handoff=test
- 2026-07-11 main_task=main phase=test-first result=fail evidence=canonical
  entry-symbol API is not yet implemented. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=entry
  MIR functions use the supplied pack canonical symbol only when explicitly
  marked as entry; helpers retain ADR-0056 symbols. handoff=validation
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=canonical
  symbol and missing-symbol tests plus validator passed.
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=empty
  canonical symbols are rejected and non-entry functions cannot silently use
  the entry alias. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0040,
  ADR-0046, ADR-0056, and ADR-0057 compliance confirmed. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=formatter; Clippy;
  all workspace tests; example audit; canonical-symbol validator; diff check.
  handoff=commit

## Required Outputs

- Authority read: ADR-0040, ADR-0046, ADR-0056, ADR-0057, and M0032.
- Files changed: backend object emission, object tests, this task, review and
  soundness reports, and the validator.
- Tests written before implementation and expected failure: entry symbol test
  fails because the emission API is absent.
- Validation commands and results: all required gates passed.
- Open questions: startup-shim implementation and actual link execution remain
  later tasks.
- Remaining risk and next main-task action: run full CI; startup-shim
  implementation remains later.

# Task: M0035-014 Byte Contextual Literal Typing

## Task Metadata

- Task ID: `M0035-014`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Apply the accepted expected-type rule for integer literals assigned to `Byte`
inside executable-core type checking.

## Authority Extract

- ADR-0059 permits an integer literal to initialize `Byte` only when it is in
  range and the expected type is `Byte`.
- M0035-003 defines Byte range diagnostics.
- M0035-013 defines executable-core primitive operator integration.

## Scope

- Thread parsed integer literals into executable-core assignment checking.
- Accept in-range `Byte` literal initializers contextually.
- Emit `byte_literal_out_of_range` for out-of-range or unrepresentable values.
- Preserve exact-type rejection for ordinary `Int`/`Byte` expressions.

## Out Of Scope

- Implicit conversions or casts.
- Byte local-name resolution and parameter typing.
- MIR/backend changes.

## Tests

- Source `Byte` declarations accept `0` and `255`.
- Source `Byte` declarations reject `256`.
- Source `Byte` declarations reject non-literal `Int` expressions without a
  conversion.
- Existing Int and primitive operator typing remains passing.

## Acceptance Criteria

- Contextual acceptance applies only to a direct integer literal initializer.
- Only values in `0..255` are accepted.
- Out-of-range values produce the specified static diagnostic.
- No implicit `Int` to `Byte` conversion is introduced for expressions.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0035-013
  identified executable-core Byte literals as the remaining frontend gap.
  handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=pending evidence=tests
  are being added before expected-type propagation. handoff=implementation
- 2026-07-11 main_task=main phase=test-first result=fail evidence=in-range
  Byte source initializers were rejected as Int/Byte mismatches. handoff=
  implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the
  executable-core assignment path now receives integer literals and applies
  the direct Byte contextual rule. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=in-range,
  out-of-range, and primitive source regression tests passed. handoff=
  adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=no
  implicit conversion was added and contextual acceptance is literal-only.
  handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0059,
  M0035-003, M0035-013, and task scope are aligned. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=cargo fmt --all
  --check; M0035-013 and M0035-014 validators; cargo clippy --workspace
  --all-targets -- -D warnings; cargo test --workspace --all-targets (374
  passed); git diff --check. handoff=commit

## Open Questions

none

## Remaining Risk

Byte locals/parameters and short-circuit CFG lowering remain subsequent M0035
tasks. Next main-task action: commit locally.

## Required Outputs

- Authority read: ADR-0059, M0035-003, M0035-013, type-checker source, and
  tests.
- Files changed: this task, type-checker source, focused tests, validator,
  review, and soundness evidence.
- Tests written before implementation and expected pre-implementation failure:
  in-range Byte source initializers fail as `Int`/`Byte` mismatches.
- Validation commands and results recorded in this task's execution log.
- Open questions or `none`.
- Remaining risk and the next main-task action.

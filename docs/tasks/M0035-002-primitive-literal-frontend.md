# Task: M0035-002 Primitive Literal Frontend

## Task Metadata

- Task ID: `M0035-002`
- Milestone: `M0035`
- Milestone File: `docs/milestones/M0035-primitive-runtime-support.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Extend lexical and parser metadata for ADR-0059 Float literals and the `()`
Unit literal without changing existing integer, boolean, string, or null forms.

## Authority Extract

- ADR-0059 accepts decimal/exponent Float literals and `()` as Unit.
- Existing parser metadata preserves literal identity and source spans.
- Byte literals remain existing integer tokens and are handled contextually by
  the type checker in a later task.

## Scope

- Add decimal/exponent Float tokenization with malformed-form diagnostics.
- Preserve Float literal metadata and source spans through parsing.
- Recognize an empty parenthesized expression as the Unit literal.
- Add focused lexer and parser tests.

## Out Of Scope

- Float value parsing or IEEE validation in the type checker.
- Byte range checking or implicit conversion.
- HIR, MIR, Cranelift, ABI, or executable smoke support.
- General grouping, calls, or control-flow changes.

## Tests

- Lexer recognizes decimal and exponent Float forms.
- Lexer rejects malformed exponent forms with a Float diagnostic.
- Parser records Float literal metadata.
- Parser records `()` as a Unit literal, not a grouped expression.

## Acceptance Criteria

- Existing integer lexing and parser metadata tests remain unchanged and pass.
- Float literal spans cover the complete source spelling.
- Unit literal spans cover both parentheses.
- Malformed float input does not produce an accepted Float token.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=ADR-0059
  requires Float and Unit literal syntax that the current frontend does not
  represent. handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=fail evidence=Float token,
  malformed-float diagnostic, and Float/Unit literal metadata variants did not
  exist. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=decimal
  and exponent Float lexing, malformed exponent diagnostics, and Unit literal
  parsing are implemented. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=focused
  lexer and parser literal tests passed. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=
  malformed exponent input is rejected and Unit is not represented as a group.
  handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0059,
  SPEC.md, M0035, and task scope are aligned. handoff=ci

## Required Outputs

- Authority read: ADR-0059, M0035, SPEC.md, lexer, parser, and existing tests.
- Files changed: this task, lexer/parser literal metadata, and focused tests.
- Tests written before implementation and expected pre-implementation failure:
  Float token and Unit metadata tests must fail because their token/metadata
  variants do not exist.
- Validation commands and results: formatter, primitive planning and literal
  validators, Clippy with warnings denied, workspace tests, and diff checks
  passed.
- Open questions or `none`.
- Remaining risk and the next main-task action.
- Remaining risk: Float value validation and Byte contextual typing remain
  subsequent M0035 tasks. Next main-task action: commit locally.

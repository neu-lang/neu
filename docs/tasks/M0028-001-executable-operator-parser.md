# Task: M0028-001 Executable Operator Parser

## Task Metadata

- Task ID: `M0028-001`
- Milestone: `M0028`
- Milestone File: `docs/milestones/M0028-executable-expression-frontend-completion.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner main task: `main-task implementer`

## Objective

Add lexer and parser coverage for the complete ADR-0042 executable operator
surface.

## Authority Extract

- `docs/SPEC.md`, `ADR-0042: Bootstrap Minimal Executable Subset`.
- `docs/adr/ADR-0042-bootstrap-minimal-executable-subset.md`.
- `docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md`.
- `docs/milestones/M0028-executable-expression-frontend-completion.md`.

## Scope

- Lex executable operator tokens for `**`, `~`, `^`, `<<`, and `>>`.
- Parse unary executable operators `+`, `-`, and `~`.
- Parse binary executable operators `+`, `-`, `*`, `/`, `%`, `**`, `&`, `|`,
  `^`, `<<`, and `>>`.
- Preserve ADR-0042 precedence and right-associative exponentiation.
- Add focused lexer, parser, and docs validator coverage.

## Out Of Scope

- Type checking executable operators.
- Integer overflow, division, exponent, or shift diagnostics.
- Entry-point checking.
- Function call and return checking.
- HIR, MIR, backend, object, or linker implementation.

## Required Tests

- `crates/compiler/tests/lexer.rs`
- `crates/compiler/tests/parser.rs`
- `docs/tests/m0028-executable-operator-parser.sh`

## Acceptance Criteria

- [x] Tests are added before implementation.
- [x] The complete executable operator token set lexes.
- [x] The complete executable binary operator set records parser metadata.
- [x] Unary `+`, unary `-`, and unary `~` parse as unary expressions.
- [x] Exponentiation parses right-associatively.
- [x] No type-checker, HIR, MIR, backend, object, or linker implementation is
  added.

## Execution Log

- 2026-07-11 agent=Main phase=test-design result=in-progress evidence=operator parser tests prepared before implementation. handoff=main-task implementer
- 2026-07-11 agent=Main phase=implementation result=complete evidence=executable operator lexer and parser coverage implemented and validated. handoff=main-task reviewer

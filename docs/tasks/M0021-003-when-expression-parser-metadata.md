# Task: M0021-003 When Expression Parser Metadata

## Task Metadata

- Task ID: `M0021-003`
- Milestone: `M0021`
- Milestone File: `docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`
- Specification: `docs/SPEC.md`
- Status: `complete`
- Owner: `Test Engineer`, then `Implementer`

## Objective

Parse ADR-0033 `when` expressions and expose subject, arm, pattern, and body
metadata without resolving variants or checking coverage.

## Authority Extract

- `docs/SPEC.md`, “ADR-0033: Bootstrap Sealed Sums And Exhaustive Match”.
- `docs/adr/ADR-0033-bootstrap-sealed-sums-and-exhaustive-match.md`,
  “Decision” and “Diagnostics And Recovery”.
- `crates/compiler/src/parser.rs`: expression parsing and pattern parsing.
- `crates/compiler/src/ast.rs`: expression and pattern node conventions.
- Validation: `cargo test -p compiler --test parser m0021_when_expression`;
  `cargo fmt --all --check`; `git diff --check`.

## Scope

- Add `when` expression and match-arm AST nodes.
- Parse parenthesized subject expressions and braced arms with qualified
  variant or wildcard patterns, `->`, accepted arm expressions, and optional
  semicolons.
- Record source-ordered side-table metadata for subject, arms, patterns, and
  arm bodies.
- Preserve ordinary parser diagnostics/recovery for malformed arms.

## Out Of Scope

- Enum or variant resolution, duplicate detection, exhaustiveness,
  `invalid_match_subject`, arm-result type checking, payload/destructuring
  patterns, and smart casts.

## Required Tests Before Implementation

- Qualified-variant and wildcard arms record ordered metadata and exact nodes.
- An arm without a semicolon before `}` is accepted.
- Missing arrow/body and unsupported pattern shapes diagnose without emitting
  complete arm metadata.
- Parsing alone does not produce coverage diagnostics.

## Acceptance Criteria

- [x] Tests fail before parser metadata APIs exist.
- [x] Only the ADR-0033 arm syntax records complete arm metadata.
- [x] Arm order, subject, pattern, body, and spans are preserved.
- [x] No resolution or exhaustiveness semantics are added.
- [x] Focused tests, formatter, review, adversarial check, and CI pass.

## Execution Log

- 2026-07-10 agent=Main phase=create-task result=pass evidence=task bounded to ADR-0033 syntax and parser metadata only. handoff=Test-Engineer
- 2026-07-10 agent=Main phase=test-first result=fail evidence=`cargo test -p compiler --test parser m0021_when_expression` failed because when-expression and match-arm metadata APIs were absent. handoff=Implementer
- 2026-07-10 agent=Main phase=implementation result=pass evidence=parser records expression subjects and ordered qualified/wildcard arms without resolution or coverage checks. handoff=Reviewer
- 2026-07-10 agent=Main phase=ordinary-tests result=pass evidence=focused parser test and validator passed. handoff=Reviewer
- 2026-07-10 agent=Main phase=adversarial-check result=pass evidence=incomplete and unsupported arms cannot create complete metadata; no semantic checks were introduced. handoff=Reviewer
- 2026-07-10 agent=Main phase=review result=approve evidence=focused parser tests, formatter, strict clippy, and full workspace tests passed; parser-only scope verified against ADR-0033 and M0021. handoff=none

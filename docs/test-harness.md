# Test Harness And Fixtures

The test harness is organized around fixture categories that later milestones can extend after the relevant `docs/SPEC.md` or `docs/adr/` authority exists.

## Fixture Categories

- `tests/fixtures/positive/`: accepted programs or inputs for a specific compiler stage.
- `tests/fixtures/negative/`: rejected programs or inputs for a specific compiler stage.
- `tests/fixtures/diagnostics/`: inputs whose diagnostic messages, spans, notes, or suggestions are part of the expected result.
- `tests/golden/diagnostics/`: golden diagnostic outputs paired with diagnostic fixtures.

## Source Of Truth

Every non-inert fixture must cite one of:

- `docs/SPEC.md`
- a specific file under `docs/adr/`
- a milestone file that explicitly authorizes non-semantic harness behavior

Fixtures must not encode guessed language syntax, semantics, or compiler behavior. If the source of truth is ambiguous, the agent must file an ambiguity report instead of adding the fixture.

## Inert Fixtures

An inert fixture proves fixture discovery or harness plumbing only. It must declare `kind = "inert"` and `compiler_behavior = "none"`. It must not contain source text, expected output, expected errors, token streams, parser expectations, AST expectations, HIR expectations, MIR expectations, or backend expectations.

## Diagnostic Fixtures

Diagnostic fixtures are governed by ADR-0015. They must eventually identify the diagnostic source location, expected message, relevant notes, and safe suggestions when applicable. Until diagnostic infrastructure exists, diagnostic fixture directories may contain only placeholders and documentation.

## Golden Files

Golden files are expected outputs reviewed by Test Engineer and Diagnostics Engineer. Golden diagnostics must be updated only when the expected diagnostic behavior is intentionally changed and reviewed against `docs/SPEC.md` and relevant ADRs.


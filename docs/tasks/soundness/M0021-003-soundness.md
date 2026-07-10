# Soundness Report: M0021-003

- Task: `M0021-003`
- Milestone: `M0021`
- Decision: `pass`

## Inputs

- `docs/SPEC.md`, "ADR-0033: Bootstrap Sealed Sums And Exhaustive Match".
- `docs/adr/ADR-0033-bootstrap-sealed-sums-and-exhaustive-match.md`, "Decision" and "Diagnostics And Recovery".
- Parser and AST changes in `crates/newlang/src/`.
- Parser tests and the task's ordinary-test evidence.

## Attacks And Results

- Incomplete arms (missing arrow or body) and binding patterns were parsed as
  invalid arms and did not create complete match-arm metadata.
- Qualified case and wildcard arms retain source ordering and spans.
- The parser performs no variant resolution, duplicate detection, coverage,
  type checking, ownership analysis, or unsafe-boundary handling. It therefore
  cannot claim, or accidentally bypass, those later semantic checks.

## Evidence

- `cargo test -p newlang --test parser`
- `sh docs/tests/m0021-when-expression-parser-metadata.sh`

## Findings

None. The remaining risk is intentionally deferred semantic enforcement:
variant identity, duplicate arms, exhaustiveness, subject validity, and arm
result typing.

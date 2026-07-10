# ADR-0048 Simplicity Review

## Metadata

- Proposal: `ADR-0048`
- Milestone: `M0028`
- Review: `main-task simplicity review`
- Date: `2026-07-11`
- Decision: `approve`

## Inputs Read

- `docs/adr/ADR-0048-bootstrap-integer-constant-expressions.md`
- `docs/milestones/M0028-executable-expression-frontend-completion.md`

## Review

The literal-tree rule is the smallest boundary that supports ADR-0043's
required static arithmetic diagnostics. It deliberately excludes an evaluator
for locals, calls, or control flow and requires no new AST form, symbol table,
or runtime contract.

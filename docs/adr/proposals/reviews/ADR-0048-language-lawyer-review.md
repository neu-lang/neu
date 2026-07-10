# ADR-0048 Language Lawyer Review

## Metadata

- Proposal: `ADR-0048`
- Milestone: `M0028`
- Review: `main-task language-lawyer review`
- Date: `2026-07-11`
- Decision: `approve`

## Inputs Read

- `docs/adr/ADR-0042-bootstrap-executable-operators.md`
- `docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md`
- `docs/ambiguities/M0028-static-integer-constant-expressions.md`
- `docs/milestones/M0028-executable-expression-frontend-completion.md`

## Review

ADR-0048 resolves ADR-0043's undefined constant-expression boundary without
adding binding, call, evaluation-order, ownership, or control-flow semantics.
Its recursive literal-tree definition precisely limits static diagnostics to
the forms already accepted for bootstrap integer operators.

## Open Questions

- None blocking acceptance.

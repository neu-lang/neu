# ADR-0048 Diagnostics Review

## Metadata

- Proposal: `ADR-0048`
- Milestone: `M0028`
- Review: `main-task diagnostics review`
- Date: `2026-07-11`
- Decision: `approve`

## Inputs Read

- `docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md`
- `docs/adr/ADR-0048-bootstrap-integer-constant-expressions.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`

## Review

The boundary makes ADR-0043 diagnostics deterministic: each maximal
bootstrap constant tree can produce one primary static-integer diagnostic.
Nonconstant forms are not partially evaluated, avoiding misleading
diagnostics based on unapproved inference. Existing ADR-0043 identifiers
remain the diagnostic vocabulary.

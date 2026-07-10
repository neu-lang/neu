# ADR-0048 Adversarial Review

## Metadata

- Proposal: `ADR-0048`
- Milestone: `M0028`
- Review: `main-task adversarial review`
- Date: `2026-07-11`
- Decision: `approve`

## Inputs Read

- `docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md`
- `docs/adr/ADR-0048-bootstrap-integer-constant-expressions.md`

## Attacks

Attack: Treat a local binding or direct call as statically evaluable.

Expected result: no ADR-0043 static arithmetic diagnostic is derived merely
from inferred runtime values.

Review result: pass.

Attack: Use the valid unary spelling of the minimum signed bootstrap `Int`.

Expected result: it is accepted without an out-of-range diagnostic.

Review result: pass.

Attack: Nest an invalid literal tree under multiple operators.

Expected result: one primary static diagnostic for the maximal constant tree,
not cascaded duplicates.

Review result: pass.

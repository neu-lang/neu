# ADR-0036 Adversarial Review

## Metadata

- Proposal: `ADR-0036`
- Milestone: `M0023`
- Review: `main-task adversarial check`
- Date: `2026-07-11`
- Decision: `approve`

## Inputs Read

- `docs/adr/proposals/ADR-0036-bootstrap-borrow-and-lifetime-analysis.md`
- `docs/adr/ADR-0002-borrowing-semantics.md`
- `docs/adr/ADR-0003-lifetime-model.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`

## Attacks

Attack: Two shared borrows of the same local in the same region.

Expected result: accepted.

Review result: pass.

Attack: Exclusive borrow overlaps a shared borrow in the same region.

Expected result: `borrow_conflict` on the later borrow with the earlier borrow
as secondary origin.

Review result: pass.

Attack: Borrow is used outside its owning region.

Expected result: `lifetime_escape` on the escape node with the original borrow
as secondary origin.

Review result: pass.

Attack: Treat source syntax such as `&x` or reference types as accepted.

Expected result: not accepted by ADR-0036.

Review result: pass.

## Soundness Notes

The proposal is conservative because it only checks explicit side-table facts
and exact region identity. It does not claim broad borrow soundness for syntax,
calls, returns, members, async, unsafe, or FFI.

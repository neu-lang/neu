# ADR-0035 Adversarial Review

## Metadata

- Proposal: `ADR-0035`
- Milestone: `M0022`
- Review: `main-task adversarial check`
- Date: `2026-07-11`
- Decision: `approve for owner acceptance`

## Inputs Read

- `docs/adr/proposals/ADR-0035-bootstrap-ownership-and-move-analysis.md`
- `docs/ambiguities/M0022-ownership-value-categories.md`
- `docs/adr/ADR-0001-ownership-model.md`
- `docs/adr/ADR-0005-copy-move-and-value-categories.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/milestones/M0022-ownership-and-move-analysis.md`

## Attacks

Attack: Move a `String` local into another local and use the original name.

Expected result: `use_after_move` on the later use, with the transfer
expression as secondary span.

Review result: pass.

Attack: Copy an `Int` local into another local and use the original name.

Expected result: no moved state because `Int` is copyable.

Review result: pass.

Attack: Move inside an `if` branch and use the value after the branch.

Expected result: M0022 must not invent branch join semantics.

Review result: pass. Branch propagation is deferred.

Attack: Treat a call, return, capture, or `when` subject as moving ownership.

Expected result: deferred unless a later accepted rule defines it.

Review result: pass.

## Soundness Notes

The proposal is conservative for M0022. Its main safety value is preventing a
known moved local from being reused in the linear lexical subset. It does not
claim global ownership soundness; M0023-M0026 still need borrow, lifetime,
thread, coroutine, unsafe, and destructor semantics.

## Handoff

Chief Architect for final owner-acceptance decision.

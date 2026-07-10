# ADR-0027 main-task language review Review

## Metadata

- ADR: `ADR-0027`
- Milestone: `M0018`
- main-task review: `main-task language review`
- Date: `2026-07-10`
- Decision: `request revision before acceptance`

## Findings

ADR-0027 should not be accepted until the typed output shape is made explicit. The proposal names typed output as required accepted content, but it does not define whether M0018 produces expression-to-type tables, declaration signatures, typed AST nodes, or another phase boundary artifact.

Primitive scalar categories must be separated from ABI and layout commitments in the accepted text. The draft says `Int` is abstract, which is a good direction, but the accepted ADR must state what identity those primitive categories use in the M0017 type representation.

Assignment compatibility needs a precise rule for nullable targets. The draft says a non-null base type may satisfy its nullable wrapper "if accepted by review"; acceptance cannot leave that conditional text unresolved.

Function call rules depend on function signature representation that is not present in parser metadata today. The accepted ADR must either define a signature source or block direct function calls until a later parser/signature task.

## Required Revisions

- Define the M0018 typed output shape.
- Define primitive category identity without ABI or layout meaning.
- Decide nullable assignment compatibility.
- Define whether direct function calls are in scope for M0018 or deferred.
- Replace conditional draft wording with accepted rules or explicit deferrals.

## Boundary

This review is not accepted source of truth. Do not implement M0018 type checking from this proposal or this review.

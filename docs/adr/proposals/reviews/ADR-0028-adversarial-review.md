# ADR-0028 Adversarial Review

## Metadata

- Proposal: `ADR-0028`
- Milestone: `M0019`
- main-task review: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `request revision before acceptance`

## Inputs Read

- `docs/adr/proposals/ADR-0028-nullability-and-flow-typing.md`
- `docs/ambiguities/M0019-nullability-and-flow-typing.md`
- `docs/SPEC.md`
- `docs/adr/ADR-0002-borrowing-semantics.md`
- `docs/adr/ADR-0006-nullability-and-absence.md`
- `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
- `docs/adr/ADR-0013-mutability-model.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`

## Attacks

Attack: Preserve a non-null refinement after mutation through an alias.

Expected result: must not implement this behavior in M0019.

Review result: pass for draft status. The proposal excludes alias analysis and mutable binding refinements.

Attack: Treat calls as not mutating nullable state and preserve refinements across calls.

Expected result: must not implement this behavior in M0019.

Review result: pass for draft status. The proposal defers function call effects.

Attack: Preserve refinements across coroutine suspension.

Expected result: must not implement this behavior in M0019.

Review result: pass for draft status. The proposal defers coroutine suspension effects.

Attack: Use broad Kotlin property stability rules without accepted ownership and borrowing support.

Expected result: must not implement this behavior in M0019.

Review result: pass for draft status. The proposal excludes member and field refinements.

## Soundness Concerns

- M0019 soundness depends on non-null refinements never outliving the control-flow fact that justifies them.
- The proposal must define branch region boundaries precisely before acceptance; otherwise a refinement may accidentally outlive its guard.
- The proposal must define nested scope and shadowing behavior before acceptance.
- The proposal must make unsupported calls, members, aliasing, suspension, unsafe, and FFI cases diagnostic rather than silently dropping facts if a nullable value is used as non-null.

## Decision

Request revision before acceptance. The narrow subset is sound as a direction, but accepted text must make the lifetime of each refinement fact unambiguous.

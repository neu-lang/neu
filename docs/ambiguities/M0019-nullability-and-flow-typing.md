# Ambiguity Report: M0019 Nullability And Flow Typing

## Metadata

- Report ID: `M0019-nullability-and-flow-typing`
- Related Task: `M0019-001`
- Related Milestone: `M0019`
- Filed By: `Language Lawyer`
- Date: `2026-07-10`
- Status: `open`
- Required Owner: `Language Designer`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0006-nullability-and-absence.md`
  - `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
  - `docs/adr/ADR-0013-mutability-model.md`
  - `docs/adr/ADR-0015-diagnostics-as-semantics.md`
  - `docs/adr/ADR-0027-type-checking-core.md`
- Milestone:
  - `docs/milestones/M0019-nullability-and-flow-typing.md`

## Exact Ambiguous Text Or Missing Rule

```text
ADR-0006 says nullable types use Kotlin-style surface syntax and are
semantically modeled as explicit optional values. Non-nullable types never
implicitly contain null.

ADR-0011 says the language supports flow-sensitive smart casts for immutable or
exclusively borrowed values, and mutation invalidates refinements.

M0019 requires "Nullability checking", "Flow refinement tracking",
"Diagnostics for invalid nullable use and invalidated refinement", and
"Smart casts for approved immutable and exclusive cases."
```

Missing concrete rules:

- Which expression forms count as null tests and create a non-null refinement.
- Which expression forms count as nullable misuse.
- Whether M0019 supports nullable member access, safe calls, force unwraps, equality tests, pattern tests, or only assignment compatibility.
- What exact smart-cast eligibility means for immutable bindings, mutable bindings, parameters, fields, temporaries, aliases, and exclusively borrowed values.
- What exact operations invalidate refinements: assignment, mutation through an alias, member assignment, function calls, coroutine suspension, or borrow creation.
- Which stable diagnostic identifiers, primary spans, recovery actions, source-of-truth citations, and safe suggestions are required.

## Competing Interpretations

1. Implement Kotlin-like null tests and smart casts broadly, including equality-based null checks, nullable member access, and force unwrap behavior.
2. Restrict M0019 to local immutable binding refinements after explicit `x != null` tests and block all mutable, member, aliasing, call, and suspension cases.
3. Implement only M0018 nullable assignment compatibility and treat all other M0019 nullability and flow behavior as unsupported until a concrete ADR is accepted.
4. Add diagnostic-only blockers for every nullable use and flow construct that is not already accepted by ADR-0027.

## Why Guessing Is Unsafe

- Smart casts interact with ownership, borrowing, mutation authority, aliasing, and later coroutine suspension rules.
- An overly broad refinement rule can make nullable values appear non-null after mutation or aliasing.
- An overly narrow rule can freeze the language into unnecessary explicit unwrapping and conflict with Kotlin-like ergonomics.
- Nullable member access and force unwrap semantics affect diagnostics, control-flow facts, panic policy, and future unsafe boundaries.
- Diagnostic identifiers and recovery actions are semantic obligations under ADR-0015 and cannot be invented ad hoc.

## Affected Work

- Tasks blocked:
  - `M0019-001`
- Milestones affected:
  - `M0019`
  - `M0020`
  - `M0021`
  - `M0022`
  - `M0023`
- Tests blocked:
  - Positive smart-cast fixtures.
  - Negative nullable misuse fixtures.
  - Negative refinement invalidation fixtures.
  - Diagnostic snapshots for invalid nullable use and invalidated refinement.
- Implementation areas blocked:
  - Type checker nullability checks.
  - Flow refinement tracking.
  - Smart-cast fact storage.
  - Mutation invalidation.
  - M0019 diagnostic model.

## Recommended Resolution Path

- [ ] Language Lawyer determines whether existing text resolves it.
- [ ] Language Designer drafts ADR or spec revision if new semantics are required.
- [ ] Adversarial Engineer reviews soundness risk.
- [ ] Diagnostics Engineer reviews diagnostic consequences.
- [ ] Simplicity Guardian reviews complexity.
- [ ] Chief Architect approves final resolution.

## Temporary Rule

No implementation may proceed on M0019 nullability checks, flow refinement tracking, smart-cast eligibility, mutation invalidation, or related diagnostics until the source of truth is updated or the ambiguity is ruled non-blocking by Chief Architect.

M0018 nullable assignment compatibility remains valid because it is explicitly accepted by ADR-0027.

## Resolution

- Decision:
  - Pending.
- Source of truth updated:
  - Pending.
- Date resolved:
  - Pending.

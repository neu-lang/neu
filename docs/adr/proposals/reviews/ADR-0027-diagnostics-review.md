# ADR-0027 Diagnostics Review

## Metadata

- ADR: `ADR-0027`
- Milestone: `M0018`
- main-task review: `main-task diagnostics check`
- Date: `2026-07-10`
- Decision: `request revision before acceptance`

## Findings

The proposed diagnostic categories are directionally sound, but acceptance needs a concrete diagnostic contract for each category. Every M0018 diagnostic must define primary span, recovery action, source-of-truth citation, and safe suggestion policy.

`type_mismatch` needs separate span guidance for expression mismatch, assignment mismatch, call argument mismatch, and declaration annotation mismatch. These contexts should not all be collapsed if that would obscure the user-facing error.

`unresolved_type_rule`, `unsupported_type_rule`, and `ambiguous_type_rule` need machine-stable rule identifiers so implementation main tasks can test that unsupported behavior is blocked for the right reason.

Recovery must describe what downstream phases receive. If typed output is blocked, the accepted ADR must say whether later phases stop entirely or receive error markers that cannot satisfy safety checks.

## Required Revisions

- Define primary span rules for each diagnostic context.
- Define recovery action precisely, including downstream typed output behavior.
- Define source-of-truth citation requirements.
- Define safe suggestion policies that do not suggest unaccepted casts, conversions, imports, overloads, or annotations.
- Define stable rule identifiers for unresolved, unsupported, and ambiguous type rules.

## Boundary

This review is not accepted source of truth. Do not implement M0018 diagnostics from this review.

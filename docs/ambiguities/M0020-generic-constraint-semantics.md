# M0020 Generic Constraint Semantics Blocker

## Exact Source Text

`docs/SPEC.md`, “ADR-0023: Type And Generic Syntax” states that it does not
define generic constraint solving or capability semantics. `docs/SPEC.md`,
“ADR-0027: Type Checking Core” separately defers generic constraint solving.

M0020 requires generic constraint checking and diagnostics for violated
specified bounds. No accepted source defines a capability catalog, how bound
names resolve, which types satisfy a capability, generic-argument substitution,
or diagnostics for violations.

## Competing Interpretations

1. Treat arbitrary parsed names such as `Send` and `Share` as built-in
   constraints and implement checks.
2. Treat capability bounds as nominal interface requirements.
3. Preserve the accepted deferral until a semantic ADR defines the model.

## Decision Required

Only option 3 is currently authorized. Options 1 and 2 would invent language
semantics. Language Designer must provide an ADR or `docs/SPEC.md` revision
before M0020 can implement constraint checking, bound diagnostics, or generic
use validation.

## Current Safe Progress

M0020-001 through M0020-003 preserve accepted syntax, generic parameter type
identity, and opaque bound occurrences. They do not interpret a capability.

## Resolution

Resolved by accepted ADR-0032. M0020 is limited to representation; generic
constraint enforcement remains a post-M0024 semantic milestone requiring a
separate accepted ADR.

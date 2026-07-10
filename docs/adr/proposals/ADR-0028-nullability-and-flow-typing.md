# ADR-0028: Nullability And Flow Typing

Status: Draft proposal - not accepted source of truth

## Non-Authority Notice

This file is a draft proposal only. It is not accepted language semantics, not an accepted ADR, and not a valid basis for implementation.

No implementation may depend on this proposal until accepted by Chief Architect and moved into the accepted ADR set or incorporated into `docs/SPEC.md`.

The active blocker remains `docs/ambiguities/M0019-nullability-and-flow-typing.md`.

## Question

What nullability checks, flow-sensitive smart-cast eligibility rules, mutation invalidation rules, and diagnostic obligations should M0019 implement before ownership and borrow analysis?

## Competing Designs

1. Full Kotlin-like nullability and smart casts across locals, properties, calls, type tests, and boolean control flow.
2. Nullability-only smart casts for immutable local bindings after explicit null comparisons.
3. No smart casts; require explicit unwrap operations for nullable values.
4. Diagnostic-only blocking for all M0019 behavior beyond ADR-0027 assignment compatibility.

## Trade-offs

Full smart casts maximize ergonomics, but they require aliasing, property stability, call effects, suspension effects, and type-test semantics that are not yet accepted.

Local immutable nullability-only refinements provide useful M0019 behavior while keeping mutation and alias invalidation small enough to specify before the borrow checker exists.

Requiring explicit unwraps is simple, but it conflicts with ADR-0011 and the Kotlin-like ergonomics goal.

Diagnostic-only blocking is safest, but it prevents M0019 from delivering its milestone goal once concrete rules are accepted.

## Recommended Draft Choice

Define a narrow M0019 nullability and flow-typing subset:

- retain ADR-0027 nullable assignment compatibility
- reject implicit `Null` in non-nullable targets
- refine immutable local bindings from `T?` to `T` only inside control-flow regions guarded by explicit `x != null`
- refine immutable local bindings to an unreachable non-null path only for the false branch of `x == null`
- reject refinements for mutable bindings, fields, temporaries, member expressions, call results, ambiguous names, and unsupported expression forms
- invalidate refinements on any assignment to the refined binding or any mutation authority that is not proven immutable by accepted earlier phases

The accepted version does not rely on Kotlin, Rust, current parser behavior, current test behavior, or current type_check behavior as implicit authority.

## Draft Concrete Nullability And Flow Model

This section is a draft direction, not accepted semantics.

M0019 should extend the M0018 side-table type-check report with flow facts rather than rewriting the AST. The original AST remains the syntactic source.

The draft model starts with only local binding facts because M0016 already defines local binding identity and M0018 already records known local binding types.

Initial facts:

- A binding with type `T?` has nullable base type `T`.
- A binding with type `T` is already non-nullable.
- The primitive `Null` value is not a value of any non-nullable type.
- Missing, unresolved, or unsupported type information creates no flow fact.

Refinement facts:

- `x != null` in an `if` condition refines `x: T?` to `x: T` in the then branch when `x` is an immutable local binding.
- `x == null` in an `if` condition refines `x: T?` to `x: T` in the else branch when `x` is an immutable local binding.
- The same rules apply when the null literal appears on the left side, such as `null != x` or `null == x`.
- The refined type is a view of the existing binding in the guarded region, not a new binding and not an AST rewrite.

Draft exclusions:

- boolean combinations such as `&&` and `||`
- negated null tests such as `!(x == null)`
- nested condition propagation
- pattern-based refinements
- member or field refinements
- call-result refinements
- generic nullable constraints
- exclusive-borrow refinements

Excluded forms should produce `unsupported_flow_rule` or preserve the existing unsupported type-rule diagnostic until the accepted ADR defines a narrower behavior.

## Draft Nullable Use Rules

This section is a draft direction, not accepted semantics.

M0019 should diagnose nullable misuse only where a typed expression is required to be non-null under already accepted M0018 rules.

Included nullable-use checks:

- assigning `Null` to a non-nullable target remains `type_mismatch` under ADR-0027
- assigning `T?` to `T` without an active non-null refinement reports `invalid_nullable_use`
- using a name expression of type `T?` where `T` is expected without an active non-null refinement reports `invalid_nullable_use`
- using a refined name expression of type `T` where `T` is expected succeeds inside the guarded region

Excluded nullable-use checks:

- member access on nullable receivers
- safe-call operators
- force unwrap operators
- Elvis or defaulting operators
- platform nullability
- nullable generic constraints
- nullable pattern matching

## Draft Smart-Cast Eligibility

This section is a draft direction, not accepted semantics.

A binding is eligible for M0019 smart-cast refinement only when all of the following are true:

- the expression is a simple unqualified local name
- M0016 resolves the name to exactly one local binding
- the local binding is immutable
- M0018 knows the binding type
- the binding type is a nullable wrapper `T?`
- the checked value is compared directly with `null`
- the guarded region is a syntactic `if` branch accepted by ADR-0024

Ineligible values:

- mutable local bindings
- top-level declarations
- parameters until parameter mutability and signature metadata are accepted
- fields and member expressions
- package-qualified names
- temporaries
- call results
- generic type parameters
- values whose type is missing, unsupported, or ambiguous
- values accessed through a borrow until exclusive-borrow refinement rules are accepted

Ineligible values should report `unsupported_flow_rule` if a refinement is requested, or `invalid_nullable_use` if a nullable value is used where non-null is required.

## Draft Mutation Invalidation

This section is a draft direction, not accepted semantics.

M0019 should be conservative because the borrow checker has not yet accepted alias and lifetime rules.

Refinements are invalidated by:

- assignment to the refined local binding
- any operation that requires treating the binding as mutable
- leaving the guarded branch region
- entering a nested region where accepted metadata cannot prove the binding remains immutable

Because the initial eligible subset is immutable local bindings only, ordinary assignment to the same binding is already illegal or ineligible. The accepted ADR must still define `invalidated_refinement` for future mutable or exclusive-borrow cases so diagnostics do not need to be redesigned later.

M0019 must not preserve a refinement across function calls, coroutine suspension points, member mutation, aliasing operations, unsafe blocks, or FFI boundaries until accepted effect and borrow semantics define those cases.

## Draft Flow Output Shape

This section is a draft direction, not accepted semantics.

M0019 should extend type-check output with:

- refinement table keyed by expression or branch `AstNodeId`
- refined expression type entries for eligible name expressions inside guarded regions
- diagnostics list entries for invalid nullable use, invalidated refinement, unsupported flow rule, and ambiguous flow rule

The flow output must not lower to HIR, rewrite the AST, or erase the original nullable type of the binding.

## Required Accepted Content

The accepted ADR must define:

- concrete null-test forms
- concrete nullable misuse rules
- smart-cast eligibility
- branch and region boundaries for refinements
- mutation invalidation
- how refinements affect expression type side tables
- interaction with ADR-0027 assignment compatibility
- diagnostic primary spans
- diagnostic recovery actions
- diagnostic source-of-truth citations
- diagnostic safe suggestion policy
- explicit deferrals for calls, members, generics, patterns, exclusive borrows, coroutines, unsafe, FFI, HIR, MIR, and backend behavior

## Required Diagnostics

The accepted ADR must define diagnostic obligations for:

- `invalid_nullable_use`
- `invalidated_refinement`
- `unsupported_flow_rule`
- `ambiguous_flow_rule`

Diagnostic: `invalid_nullable_use`

- Primary span: the nullable expression used where a non-null value is required.
- Recovery action: omit the successful refined type entry for that use and continue checking independent constructs.
- Source-of-truth citation: accepted ADR-0028 nullable-use section.
- Safe suggestion policy: mention the required non-null type and actual nullable type; do not suggest force unwraps, unsafe operations, API redesigns, or ownership changes.

Diagnostic: `invalidated_refinement`

- Primary span: the use of a refinement after the invalidating operation.
- Recovery action: treat the value as its original nullable type after invalidation and continue checking independent constructs.
- Source-of-truth citation: accepted ADR-0028 mutation-invalidation section.
- Safe suggestion policy: name the invalidating operation when known; do not suggest hidden copies or unsafe casts.

Diagnostic: `unsupported_flow_rule`

- Primary span: the expression or control-flow construct requesting a flow rule outside the accepted subset.
- Recovery action: produce no refinement fact for that construct and continue checking independent constructs.
- Source-of-truth citation: accepted ADR-0028 explicit deferrals.
- Safe suggestion policy: no fix-it unless an accepted equivalent exists.

Diagnostic: `ambiguous_flow_rule`

- Primary span: the expression or branch where multiple flow interpretations are possible and no accepted rule selects one.
- Recovery action: produce no refinement fact for that construct and continue checking independent constructs.
- Source-of-truth citation: accepted ADR-0028 ambiguity-handling section.
- Safe suggestion policy: no fix-it.

## Explicit Draft Deferrals

This proposal defers:

- member access on nullable receivers
- safe-call operators
- force unwrap operators
- Elvis or defaulting operators
- boolean-combination refinement through `&&` and `||`
- negated-condition refinement
- pattern-based refinement
- type-test smart casts
- parameter refinements
- top-level declaration refinements
- mutable binding refinements
- exclusive-borrow refinements
- alias analysis
- function call effects
- member mutation effects
- coroutine suspension effects
- unsafe and FFI nullability
- generic nullable constraints
- platform nullability
- HIR lowering
- MIR lowering
- backend code generation

## Downstream Consequences

M0020 depends on nullable generic constraints remaining explicit deferrals unless accepted before generic constraint solving.

M0022 and M0023 depend on flow facts being conservative so move and borrow checking cannot rely on guessed non-null states.

Coroutine milestones depend on refinements not crossing suspension points until suspension and borrow interactions are accepted.

Diagnostics infrastructure must support stable rule identifiers for unsupported and ambiguous flow rules.

## Dependencies

- ADR-0002
- ADR-0006
- ADR-0011
- ADR-0013
- ADR-0015
- ADR-0024
- ADR-0026
- ADR-0027

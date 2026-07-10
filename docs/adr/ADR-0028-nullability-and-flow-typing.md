# ADR-0028: Nullability And Flow Typing

Status: Accepted

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

## Decision

Define a narrow M0019 nullability and flow-typing subset:

- retain ADR-0027 nullable assignment compatibility
- reject implicit `Null` in non-nullable targets
- refine immutable local bindings from `T?` to `T` only inside control-flow regions guarded by explicit `x != null`
- refine immutable local bindings to an unreachable non-null path only for the false branch of `x == null`
- reject refinements for mutable bindings, fields, temporaries, member expressions, call results, ambiguous names, and unsupported expression forms
- invalidate refinements on any assignment to the refined binding or any mutation authority that is not proven immutable by accepted earlier phases

The language definition does not rely on Kotlin, Rust, current parser behavior, current test behavior, or current type_check behavior as implicit authority.

## Concrete Nullability And Flow Model

M0019 extends the M0018 side-table type-check report with flow facts rather than rewriting the AST. The original AST remains the syntactic source.

The model starts with only local binding facts because M0016 already defines local binding identity and M0018 already records known local binding types.

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

Excluded refinements:

- boolean combinations such as `&&` and `||`
- negated null tests such as `!(x == null)`
- nested condition propagation
- pattern-based refinements
- member or field refinements
- call-result refinements
- generic nullable constraints
- exclusive-borrow refinements

Excluded forms produce `unsupported_flow_rule` or preserve the existing unsupported type-rule diagnostic until the accepted ADR defines a narrower behavior.

## Null-Test Recognition

M0019 recognizes only direct equality comparisons between one simple local name expression and the `null` literal as refinement conditions.

Accepted null-test shapes:

- `x != null`
- `null != x`
- `x == null`
- `null == x`

The recognizer is a flow-specific condition recognizer. It does not require general binary expression type checking, overload resolution, user-defined equality, implicit conversion, or Boolean operator typing.

The recognizer may inspect the parsed AST shape accepted by ADR-0024 and the literal metadata accepted by ADR-0027 only to identify the comparison form. It must not assign a general type to binary expressions or make other binary operators type-check.

Unsupported condition shapes:

- boolean combinations such as `x != null && y != null`
- negated expressions such as `!(x == null)`
- comparisons against non-null literals
- comparisons involving member expressions
- comparisons involving calls
- comparisons involving package-qualified names
- comparisons where both sides are `null`
- comparisons where neither side resolves to an eligible local binding

Unsupported condition shapes do not create refinement facts. If a nullable value is later used where non-null is required, that use is diagnosed independently.

## Branch Region Boundaries

M0019 branch regions are syntactic regions from ADR-0024 `if` expressions.

For `x != null`, the refinement is active only inside the then branch block.

For `null != x`, the refinement is active only inside the then branch block.

For `x == null`, the non-null refinement is active only inside the else branch block when an else branch exists.

For `null == x`, the non-null refinement is active only inside the else branch block when an else branch exists.

The then branch region is the statements and optional trailing expression contained directly inside the then branch block.

The else branch region is the statements and optional trailing expression contained directly inside the else branch block.

The refinement starts at the first statement or optional trailing expression in the refined branch and ends at the closing brace of that branch.

No refinement is active in the condition expression itself, before the `if`, after the `if`, in sibling branches, or in a missing else branch.

Nested blocks inherit an active refinement only while they remain within the refined branch region and do not shadow the refined local binding.

## Refined Output Shape

M0019 keeps the M0018 side-table model and adds flow output without rewriting the AST.

The flow output contains:

- refinement table keyed by the `AstNodeId` of the `if` expression or branch block that introduces the fact
- refinement records identifying the original local binding, original nullable type, refined non-null type, branch region, and originating null-test expression
- refined expression type entries for eligible name expressions inside guarded regions
- diagnostics list entries for invalid nullable use, invalidated refinement, unsupported flow rule, and ambiguous flow rule

The original nullable type of the binding remains unchanged in declaration signature tables and local binding type facts.

Refined expression type entries are per-use views. A use of `x` inside the active branch may receive refined type `T`, while the binding declaration still has original type `T?`.

Assignment compatibility checks inside a refined branch may use the refined expression type for the assigned expression use only. They must not mutate the target declaration type or create a lasting non-null binding.

If refinement output is missing because the condition is unsupported, ambiguous, or ineligible, later phases must treat the expression as its original nullable type.

## Shadowing And Nested Scope Rules

M0019 identifies refinements by local binding identity, not by textual name alone.

Shadowing rules for shadowing declarations:

- A nested declaration that introduces a new local binding with the same name hides the outer refinement for uses that resolve to the nested binding.
- Uses before the nested declaration and still inside the refined branch keep the outer refinement when M0016 resolves them to the outer binding.
- Uses after the nested declaration resolve according to M0016 local lookup and receive the refinement only if they still resolve to the original refined binding.

Nested block rules:

- A nested block inside the refined branch inherits the active refinement for the same local binding.
- The inherited refinement ends when control leaves the refined branch.
- A nested block outside the refined branch never receives the refinement.

Duplicate local bindings and ambiguous local binding cases:

- If M0016 reports duplicate local bindings in the same scope, M0019 must not create a refinement for the duplicate binding site.
- If a name expression does not resolve to exactly one local binding, M0019 must not create a refinement.
- If a condition references an ambiguous local binding, M0019 reports `ambiguous_flow_rule` with a stable rule identifier.

## Nullable Use Rules

M0019 diagnoses nullable misuse only where a typed expression is required to be non-null under already accepted M0018 rules.

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

## Smart-Cast Eligibility

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

Ineligible values report `unsupported_flow_rule` if a refinement is requested, or `invalid_nullable_use` if a nullable value is used where non-null is required.

## Mutation Invalidation

M0019 is conservative because the borrow checker has not yet accepted alias and lifetime rules.

Refinements are invalidated by:

- assignment to the refined local binding
- any operation that requires treating the binding as mutable
- leaving the guarded branch region
- entering a nested region where accepted metadata cannot prove the binding remains immutable

Because the initial eligible subset is immutable local bindings only, ordinary assignment to the same binding is already illegal or ineligible. The accepted ADR must still define `invalidated_refinement` for future mutable or exclusive-borrow cases so diagnostics do not need to be redesigned later.

M0019 must not preserve a refinement across function calls, coroutine suspension points, member mutation, aliasing operations, unsafe blocks, or FFI boundaries until accepted effect and borrow semantics define those cases.

## Flow Diagnostics

ADR-0028 defines diagnostic obligations for:

- `invalid_nullable_use`
- `invalidated_refinement`
- `unsupported_flow_rule`
- `ambiguous_flow_rule`

Diagnostic: `invalid_nullable_use`

- Primary span: the nullable expression used where a non-null value is required. For grouped expressions, use the innermost nullable expression span when known and the grouped expression span otherwise. For assignment values, use the assigned value expression span.
- Recovery action: omit the successful refined type entry for that use and continue checking independent constructs.
- Source-of-truth citation: accepted ADR-0028 nullable-use section.
- Safe suggestion policy: mention the required non-null type and actual nullable type; do not suggest force unwraps, unsafe operations, API redesigns, or ownership changes.
- Required stable rule identifier examples: `nullable_value_without_refinement`, `nullable_assignment_without_refinement`.

Diagnostic: `invalidated_refinement`

- Primary span: the use of a refinement after the invalidating operation. The invalidating operation should be a secondary span when available.
- Recovery action: treat the value as its original nullable type after invalidation and continue checking independent constructs.
- Source-of-truth citation: accepted ADR-0028 mutation-invalidation section.
- Safe suggestion policy: name the invalidating operation when known; do not suggest hidden copies or unsafe casts.
- Required stable rule identifier examples: `assignment_invalidated_refinement`, `region_exit_invalidated_refinement`.

Diagnostic: `unsupported_flow_rule`

- Primary span: the expression or control-flow construct requesting a flow rule outside the accepted subset.
- Recovery action: produce no refinement fact for that construct and continue checking independent constructs.
- Source-of-truth citation: accepted ADR-0028 explicit deferrals.
- Safe suggestion policy: no fix-it unless an accepted equivalent exists.
- Required stable rule identifier examples: `mutable_local_refinement_deferred`, `boolean_combination_refinement_deferred`, `member_refinement_deferred`, `call_result_refinement_deferred`, `exclusive_borrow_refinement_deferred`.

Diagnostic: `ambiguous_flow_rule`

- Primary span: the expression or branch where multiple flow interpretations are possible and no accepted rule selects one.
- Recovery action: produce no refinement fact for that construct and continue checking independent constructs.
- Source-of-truth citation: accepted ADR-0028 ambiguity-handling section.
- Safe suggestion policy: no fix-it.
- Required stable rule identifier examples: `ambiguous local binding`, `ambiguous_null_test_region`.

## Required Diagnostics

ADR-0028 defines diagnostic obligations for:

- `invalid_nullable_use`
- `invalidated_refinement`
- `unsupported_flow_rule`
- `ambiguous_flow_rule`

Each diagnostic defines primary span, recovery action, source-of-truth citation, safe suggestion policy, and stable rule identifier requirements where the diagnostic represents a blocked or ambiguous rule.

## Explicit Deferrals

ADR-0028 defers:

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

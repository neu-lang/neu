# ADR-0027: Type Checking Core

Status: Draft proposal - not accepted source of truth

## Non-Authority Notice

This file is a draft proposal only. It is not accepted language semantics, not an accepted ADR, and not a valid basis for implementation.

No implementation may depend on this proposal until accepted by Chief Architect and moved into the accepted ADR set or incorporated into `docs/SPEC.md`.

The active blocker remains `docs/ambiguities/M0018-type-checking-core.md`.

## Question

What bootstrap type-checking subset, literal typing model, primitive scalar category model, assignment compatibility rule, call resolution rule, function type application rule, and type diagnostic contract should M0018 implement before ownership and borrow analysis?

## Competing Designs

1. Full expression type checking with inferred literal defaults and direct primitive categories.
2. Explicit-annotation-only nominal checking with all expression typing deferred.
3. Small bootstrap expression checking with accepted primitive categories and no overloads.
4. Constraint-based checking from the beginning.

## Trade-offs

Full expression type checking exercises more frontend code but risks inventing literal defaults, conversions, overload behavior, and primitive scalar categories.

Explicit-annotation-only nominal checking is safest but may not satisfy M0018's well-typed and ill-typed fixture requirements unless the accepted subset is clearly defined.

A small bootstrap checker can support useful fixtures while keeping overloads, numeric conversions, and generic solving deferred, but it requires accepted rules for literals, assignment compatibility, call resolution, and function type application.

Constraint-based checking may scale to later features, but it adds complexity before the language has accepted constraints, overloads, and inference behavior.

## Recommended Draft Choice

Define a small bootstrap type checker with explicit, accepted primitive scalar categories, literal typing rules, exact nominal assignment compatibility, direct function declaration calls, and structural function type application.

The accepted version should specify only the model needed by the near-term frontend pipeline:

- primitive scalar categories available to M0018
- literal typing rules
- expression forms included in M0018
- declaration forms included in M0018
- assignment compatibility
- direct call resolution
- structural function type application
- type mismatch diagnostics
- unresolved and unsupported type-rule diagnostics
- explicit deferrals for overloads, implicit numeric conversions, generic constraint solving, flow typing, ownership, borrowing, and member lookup

The accepted version must not rely on Kotlin, Rust, Go, current parser behavior, current test behavior, or current type_check behavior as implicit authority.

## Draft Bootstrap Type Checking Subset

This section is a draft direction, not accepted semantics.

M0018 should type check only already parsed and name-resolved constructs whose rules are accepted in the final ADR.

The draft bootstrap subset should include:

- integer, string, boolean, and null literal expressions if primitive categories are accepted
- name expressions that resolve to local values or top-level declarations from M0016
- explicit nullable wrappers from M0017 where surface syntax requested nullable type
- local `val` and `var` declarations with explicit type annotations if parser metadata supports them
- assignments where both sides have known types and assignment compatibility is exact
- direct calls to resolved function declarations with known parameter and return types if parser metadata supports function signatures
- structural function type application for values known to have function type

The subset should not include inference beyond local propagation from explicit annotations and literal rules.

## Draft Primitive Scalar Categories

This section is a draft direction, not accepted semantics.

The accepted ADR should either define a minimal primitive category set or explicitly block primitive-dependent fixtures.

If primitive categories are accepted for M0018, the draft candidate set is:

- `Bool`
- `Int`
- `String`
- `Unit`
- `Null`

`Int` should be treated as an abstract bootstrap integer category for type checking only, not as an ABI or layout commitment.

`Null` should not be assignable to non-nullable types. Nullable assignment rules should use the M0017 explicit nullable representation.

## Draft Literal Typing

This section is a draft direction, not accepted semantics.

The accepted ADR should define literal typing before implementation.

Draft literal typing:

- `true` and `false` have type `Bool`.
- decimal, binary, and hexadecimal integer literals have type `Int` in M0018.
- string literals have type `String`.
- `null` has type `Null` and may only satisfy a nullable target type.

Integer overflow, suffixes, ABI width, signedness, and numeric conversions remain outside M0018.

## Draft Assignment Compatibility

This section is a draft direction, not accepted semantics.

Assignment compatibility should be exact type identity for M0018, except that `Null` may satisfy a nullable target type and a non-null base type may satisfy its nullable wrapper if accepted by review.

No implicit numeric conversions, subtyping, protocol conformance, variance, ownership move rules, or borrow rules should be implemented in M0018.

## Draft Call Resolution

This section is a draft direction, not accepted semantics.

Direct function calls should require a single resolved function declaration from M0016 and an accepted function signature representation.

The checker should compare argument count and exact argument types against parameter types. Return type should be the declared return type.

Overload resolution, default arguments, named arguments, varargs, methods, constructors, extension calls, member calls, generic instantiation, and type-directed lookup remain deferred.

## Draft Function Type Application

This section is a draft direction, not accepted semantics.

Function type values should be callable only when their type is a structural function type accepted by M0017 or a follow-up type representation ADR.

Application should compare arity and exact argument types. Result type should be the function type's result.

Suspending function types, receiver function types, effects, unsafe boundaries, and ownership behavior remain deferred.

## Required Accepted Content

The accepted ADR must define:

- primitive scalar categories or an explicit primitive-blocking rule
- literal typing rules
- included expression and declaration forms
- assignment compatibility
- direct function call rules
- function type application rules
- typed output shape
- type mismatch diagnostic requirements
- unresolved type-rule diagnostic requirements
- unsupported type-rule diagnostic requirements
- ambiguous type-rule diagnostic requirements
- deferrals for overloads, numeric conversion, member lookup, generic constraint solving, flow typing, ownership, borrowing, HIR, MIR, and backend behavior

## Required Diagnostics

Diagnostic: `type_mismatch`

- Primary span: expression, assignment target, argument, or declaration annotation where the mismatch is observed.
- Recovery action: keep analyzing using an error type marker that cannot satisfy later safety checks.
- Source-of-truth citation: accepted ADR-0027 section defining assignment or expression compatibility.
- Safe suggestion policy: suggestions may mention expected and actual types only; they must not propose conversions or casts not accepted by source of truth.

Diagnostic: `unresolved_type_rule`

- Primary span: the expression or declaration requiring a missing type rule.
- Recovery action: block the current typed output and continue collecting independent diagnostics.
- Source-of-truth citation: accepted ADR-0027 deferral or ambiguity section.
- Safe suggestion policy: no fix-it.

Diagnostic: `unsupported_type_rule`

- Primary span: the unsupported syntax or semantic construct.
- Recovery action: block the construct and continue independent analysis.
- Source-of-truth citation: accepted ADR-0027 explicit deferrals.
- Safe suggestion policy: no fix-it unless an accepted equivalent exists.

Diagnostic: `ambiguous_type_rule`

- Primary span: the expression, declaration, call, or assignment whose rule is ambiguous.
- Recovery action: block typed output for the construct.
- Source-of-truth citation: accepted ADR-0027 ambiguity-handling section.
- Safe suggestion policy: no fix-it.

## Explicit Draft Deferrals

This proposal defers:

- overload resolution
- implicit numeric conversion
- integer width and layout
- floating-point types
- character types
- generic constraint solving
- generic specialization
- protocol or interface conformance
- subtyping
- variance
- member lookup
- constructor lookup
- methods and extensions
- default arguments
- named arguments
- varargs
- smart casts and flow typing
- ownership and move analysis
- borrow checking
- coroutine suspension typing
- unsafe and FFI typing
- HIR lowering
- MIR lowering
- backend code generation

## Downstream Consequences

M0019 depends on M0018 distinguishing nullable from non-nullable types without implementing smart casts prematurely.

M0020 depends on generic placeholders being blocked where constraint solving is not accepted.

M0022 and M0023 depend on typed expressions being reliable enough that move and borrow diagnostics are not built on guessed types.

Backend milestones must not rely on bootstrap primitive categories as ABI commitments unless a later ABI or layout ADR accepts that meaning.

## Dependencies

- ADR-0005
- ADR-0006
- ADR-0010
- ADR-0011
- ADR-0015
- ADR-0016
- ADR-0023
- ADR-0024
- ADR-0026

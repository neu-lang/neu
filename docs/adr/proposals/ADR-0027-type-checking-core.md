# ADR-0027: Type Checking Core

Status: Draft proposal - not accepted source of truth

## Non-Authority Notice

This file is a draft proposal only. It is not accepted language semantics, not an accepted ADR, and not a valid basis for implementation.

No implementation may depend on this proposal until accepted by main task and moved into the accepted ADR set or incorporated into `docs/SPEC.md`.

The active blocker remains `docs/ambiguities/M0018-type-checking-core.md`.

## Question

What bootstrap type-checking subset, typed output shape, primitive type identity model, literal typing model, assignment compatibility rule, and type diagnostic contract should M0018 implement before ownership and borrow analysis?

## Competing Designs

1. Full expression type checking with inferred literal defaults, calls, and primitive categories.
2. Explicit-annotation-only nominal checking with expression typing deferred.
3. Small bootstrap expression checking with primitive type-checking identities, literals, names, and exact assignment compatibility.
4. Constraint-based checking from the beginning.

## Trade-offs

Full expression type checking exercises more frontend code but risks inventing overload behavior, conversions, function signatures, and primitive scalar semantics beyond M0018.

Explicit-annotation-only nominal checking is safest but does not create useful well-typed and ill-typed expression fixtures.

A small bootstrap checker gives M0018 testable behavior while keeping direct calls, structural function type application, overloads, numeric conversion, generic solving, and member lookup deferred.

Constraint-based checking may scale later, but it adds complexity before accepted constraints, overloads, and inference behavior exist.

## Recommended Draft Choice

Define a small bootstrap type checker with primitive type-checking identities, literal typing, resolved name expression typing, explicit nullable wrappers, and exact assignment compatibility.

Direct function declaration calls and structural function type application are deferred for M0018 because accepted function signature metadata and function type representation are not yet sufficient implementation authority.

The accepted version must not rely on Kotlin, Rust, Go, current parser behavior, current test behavior, or current type_check behavior as implicit authority.

## Draft Concrete Type Checking Model

This section is a draft direction, not accepted semantics.

M0018 should type check only constructs whose input identities are already available from accepted earlier milestones:

- parsed AST node identities from M0009 through M0013
- module, package, and visibility metadata from M0014
- symbol and name-resolution results from M0015 and M0016
- type identities, nullable wrappers, and unsupported type-form diagnostics from M0017

M0018 should not rewrite the AST, lower to HIR, infer missing declarations, resolve calls, or perform ownership analysis.

The draft concrete model includes:

- literal expressions for `true`, `false`, accepted integer literals, accepted string literals, and `null`
- name expressions whose M0016 resolution points to a local binding or declaration with a known type supplied by a future explicit annotation/signature input
- assignment statements where both sides have known types
- local declarations with known explicit annotation types when parser metadata provides those annotations
- block expressions only as containers for checking contained expressions and statements, not as value-producing expressions

## Draft Typed Output Shape

This section is a draft direction, not accepted semantics.

M0018 should produce a type-check report containing:

- expression type table keyed by `AstNodeId`
- declaration signature table keyed by `AstNodeId`
- assignment check table keyed by assignment statement `AstNodeId`
- diagnostics list

The type checker should perform no typed AST rewrite. The original AST remains the syntactic source, and typed output is side-table metadata for later phases.

If a construct cannot be typed, the report records a diagnostic and no successful type table entry for that construct. Later safety phases must not treat missing entries or error markers as satisfying safety checks.

## Draft Primitive Type Identity

This section is a draft direction, not accepted semantics.

M0018 may introduce `PrimitiveType` identities for type checking only:

- `Bool`
- `Int`
- `String`
- `Unit`
- `Null`

Each primitive is a type-checking identity only. It has no ABI or layout meaning and does not imply runtime representation, integer width, signedness, calling convention, object layout, or backend lowering.

Primitive identities should be represented in the type model as bootstrap type identities or a dedicated primitive type record only after ADR-0027 is accepted.

## Draft Included Expression Forms

This section is a draft direction, not accepted semantics.

Included expression forms for M0018:

- boolean literals type to `Bool`
- accepted integer literals type to `Int`
- accepted string literals type to `String`
- `null` types to `Null`
- name expressions type to the type of their resolved local binding or declaration when that type is known from accepted metadata
- grouped expressions type to the inner expression type

Excluded expression forms for M0018:

- call expressions
- member expressions
- binary expressions
- unary expressions
- `if` expressions as value-producing expressions
- block expressions as value-producing expressions

Excluded forms should report `unsupported_type_rule` with a stable rule identifier rather than guessing.

## Draft Assignment Compatibility

This section is a draft direction, not accepted semantics.

Assignment compatibility is exact type identity for M0018, with two nullable exceptions:

- `Null` is assignment-compatible only with nullable target types.
- Non-null base values are assignment-compatible with their nullable wrapper.

`Null` is never assignment-compatible with non-nullable targets.

No implicit numeric conversion, subtyping, protocol conformance, variance, ownership move rule, borrow rule, dereference rule, or user-defined conversion participates in M0018 assignment compatibility.

## Draft Direct Call Deferral

This section is a draft direction, not accepted semantics.

Direct function declaration calls are deferred for M0018.

Reason: accepted parser and declaration metadata do not yet provide complete parameter and return signature authority for a type checker, and M0016 explicitly does not implement overload resolution, member lookup, generic instantiation, constructor lookup, extension lookup, or type-directed lookup.

Call expressions should report `unsupported_type_rule` with stable rule identifier `direct_call_deferred`.

## Draft Function Type Application Deferral

This section is a draft direction, not accepted semantics.

Structural function type application is deferred for M0018.

Reason: M0017 records nominal, generic placeholder, nullable, and unsupported type forms, but does not yet provide a first-class function type representation suitable for checking arity, argument types, result types, suspension, effects, or ownership behavior.

Function type application should report `unsupported_type_rule` with stable rule identifier `function_type_application_deferred`.

## Draft Type Checking Diagnostics

This section is a draft direction, not accepted semantics.

Diagnostic: `type_mismatch`

- Primary span: for expression mismatch, the expression whose actual type cannot satisfy the expected type; for assignment mismatch, the assigned value expression; for declaration annotation mismatch, the initializer expression.
- Recovery action: omit the successful type table entry for the construct and continue checking independent constructs.
- Source-of-truth citation: accepted ADR-0027 section defining assignment compatibility or included expression typing.
- Safe suggestion policy: report expected and actual type display strings only; do not suggest casts, conversions, overload choices, imports, annotations, or ownership changes.

Diagnostic: `unresolved_type_rule`

- Primary span: the expression or declaration requiring a type rule not present in accepted source of truth.
- Recovery action: omit the successful type table entry for the construct and continue checking independent constructs.
- Source-of-truth citation: accepted ADR-0027 section defining unresolved type-rule handling.
- Safe suggestion policy: no fix-it.
- Required stable rule identifier examples: `missing_annotation_type`, `missing_resolved_name_type`.

Diagnostic: `unsupported_type_rule`

- Primary span: the unsupported expression, statement, declaration, or type node.
- Recovery action: omit the successful type table entry for the construct and continue checking independent constructs.
- Source-of-truth citation: accepted ADR-0027 explicit deferrals.
- Safe suggestion policy: no fix-it unless an accepted equivalent exists.
- Required stable rule identifier examples: `direct_call_deferred`, `function_type_application_deferred`, `member_expression_deferred`, `binary_expression_deferred`, `if_value_deferred`.

Diagnostic: `ambiguous_type_rule`

- Primary span: the expression, declaration, assignment, or type node whose applicable rule is ambiguous.
- Recovery action: omit the successful type table entry for the construct and continue checking independent constructs.
- Source-of-truth citation: accepted ADR-0027 ambiguity-handling section.
- Safe suggestion policy: no fix-it.
- Required stable rule identifier examples: `ambiguous_literal_target`, `ambiguous_assignment_target`.

## Draft Unsupported And Ambiguous Rules

This section is a draft direction, not accepted semantics.

M0018 should reject unsupported or ambiguous type rules explicitly. It must not choose a behavior by source order, parser traversal order, external language precedent, or current test convenience.

Unsupported and ambiguous diagnostics must carry a stable rule identifier so fixtures can assert the exact blocked rule.

## Required Accepted Content

The accepted ADR must define:

- concrete type-checking model
- typed output shape
- primitive type identity and primitive scalar categories
- included expression forms
- assignment compatibility
- direct call deferral or accepted direct call rules
- function type application deferral or accepted function type application rules
- type_mismatch diagnostics
- unresolved_type_rule diagnostics
- unsupported_type_rule diagnostics
- ambiguous_type_rule diagnostics
- explicit deferrals for overloads, numeric conversion, member lookup, generic constraint solving, flow typing, ownership, borrowing, HIR, MIR, and backend behavior

## Required Diagnostics

The accepted ADR must define diagnostic obligations for:

- `type_mismatch`
- `unresolved_type_rule`
- `unsupported_type_rule`
- `ambiguous_type_rule`

Each diagnostic must define primary span, recovery action, source-of-truth citation, safe suggestion policy, and stable rule identifier requirements where the diagnostic represents a blocked or ambiguous rule.

## Explicit Draft Deferrals

This proposal defers:

- direct function declaration calls
- structural function type application
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

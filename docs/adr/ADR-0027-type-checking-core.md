# ADR-0027: Type Checking Core

Status: Accepted

## Decision

The compiler defines a small bootstrap type checker with primitive type-checking identities, literal typing, resolved name expression typing, explicit nullable wrappers, and exact assignment compatibility.

Direct function declaration calls and structural function type application are deferred for this implementation because accepted function signature metadata and function type representation are not yet sufficient implementation authority.

The type checker must not rely on Kotlin, Rust, Go, current parser behavior, current test behavior, or current type_check behavior as implicit authority.

## Question

What bootstrap type-checking subset, typed output shape, primitive type identity model, literal typing model, assignment compatibility rule, and type diagnostic contract should the compiler implement before ownership and borrow analysis?

## Competing Designs

1. Full expression type checking with inferred literal defaults, calls, and primitive categories.
2. Explicit-annotation-only nominal checking with expression typing deferred.
3. Small bootstrap expression checking with primitive type-checking identities, literals, names, and exact assignment compatibility.
4. Constraint-based checking from the beginning.

## Trade-offs

Full expression type checking exercises more frontend code but risks inventing overload behavior, conversions, function signatures, and primitive scalar semantics beyond the compiler.

Explicit-annotation-only nominal checking is safest but does not create useful well-typed and ill-typed expression fixtures.

A small bootstrap checker gives the compiler testable behavior while keeping direct calls, structural function type application, overloads, numeric conversion, generic solving, and member lookup deferred.

Constraint-based checking may scale later, but it adds complexity before accepted constraints, overloads, and inference behavior exist.

## Concrete Type Checking Model

The type checker checks only constructs whose input identities are already available from accepted parser and frontend work:

- parsed AST node identities from the parser
- module, package, and visibility metadata from the frontend
- symbol and name-resolution results from the frontend
- type identities, nullable wrappers, and unsupported type-form diagnostics from accepted type metadata

The compiler does not rewrite the AST, lower to HIR, infer missing declarations, resolve calls, or perform ownership analysis.

The concrete model includes:

- literal expressions for `true`, `false`, accepted integer literals, accepted string literals, and `null`
- name expressions whose resolution points to a local binding or declaration with a known type supplied by accepted explicit annotation or signature metadata
- assignment statements where both sides have known types
- local declarations with known explicit annotation types when parser metadata provides those annotations
- block expressions only as containers for checking contained expressions and statements, not as value-producing expressions

## Typed Output Shape

The compiler produces a type-check report containing:

- expression type table keyed by `AstNodeId`
- declaration signature table keyed by `AstNodeId`
- assignment check table keyed by assignment statement `AstNodeId`
- diagnostics list

The type checker performs no typed AST rewrite. The original AST remains the syntactic source, and typed output is side-table metadata for later phases.

If a construct cannot be typed, the report records a diagnostic and no successful type table entry for that construct. Later safety phases must not treat missing entries or error markers as satisfying safety checks.

## Primitive Type Identity

The compiler may introduce `PrimitiveType` identities for type checking only:

- `Bool`
- `Int`
- `String`
- `Unit`
- `Null`

Each primitive is a type-checking identity only. It has no ABI or layout meaning and does not imply runtime representation, integer width, signedness, calling convention, object layout, or backend lowering.

Primitive identities are represented in the type model as bootstrap type identities or a dedicated primitive type record only according to accepted ADR-0027 semantics.

## Included Expression Forms

Included expression forms for this implementation:

- boolean literals type to `Bool`
- accepted integer literals type to `Int`
- accepted string literals type to `String`
- `null` types to `Null`
- name expressions type to the type of their resolved local binding or declaration when that type is known from accepted metadata
- grouped expressions type to the inner expression type

Excluded expression forms for this implementation:

- call expressions
- member expressions
- binary expressions
- unary expressions
- `if` expressions as value-producing expressions
- block expressions as value-producing expressions

Excluded forms report `unsupported_type_rule` with a stable rule identifier rather than guessing.

## Assignment Compatibility

Assignment compatibility is exact type identity for this implementation, with two nullable exceptions:

- `Null` is assignment-compatible only with nullable target types.
- Non-null base values are assignment-compatible with their nullable wrapper.

`Null` is never assignment-compatible with non-nullable targets.

No implicit numeric conversion, subtyping, protocol conformance, variance, ownership move rule, borrow rule, dereference rule, or user-defined conversion participates in this implementation assignment compatibility.

## Direct Call Deferral

Direct function declaration calls are deferred for this implementation.

Reason: accepted parser and declaration metadata do not yet provide complete parameter and return signature authority for a type checker, and the compiler explicitly does not implement overload resolution, member lookup, generic instantiation, constructor lookup, extension lookup, or type-directed lookup.

Call expressions report `unsupported_type_rule` with stable rule identifier `direct_call_deferred`.

## Function Type Application Deferral

Structural function type application is deferred for this implementation.

Reason: the compiler records nominal, generic placeholder, nullable, and unsupported type forms, but does not yet provide a first-class function type representation suitable for checking arity, argument types, result types, suspension, effects, or ownership behavior.

Function type application reports `unsupported_type_rule` with stable rule identifier `function_type_application_deferred`.

## Type Checking Diagnostics

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

## Unsupported And Ambiguous Rules

The compiler rejects unsupported or ambiguous type rules explicitly. It must not choose a behavior by source order, parser traversal order, external language precedent, or current test convenience.

Unsupported and ambiguous diagnostics carry a stable rule identifier so fixtures can assert the exact blocked rule.

## Explicit Deferrals

ADR-0027 defers:

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

The compiler depends on this implementation distinguishing nullable from non-nullable types without implementing smart casts prematurely.

The compiler depends on generic placeholders being blocked where constraint solving is not accepted.

Ownership and borrow diagnostics depend on typed expressions being reliable enough that they are not built on guessed types.

Backend future work must not rely on bootstrap primitive categories as ABI commitments unless a later ABI or layout ADR accepts that meaning.

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

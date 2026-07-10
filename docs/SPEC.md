# New Systems Language Specification

Status: Draft seed

This file records the initial semantic decisions accepted from ADR-0001 through
ADR-0020. These are language-level choices, not compiler implementation details.

## ADR-0001: Ownership Model

The language uses single-owner affine ownership with borrowing and deterministic
destruction.

## ADR-0002: Borrowing Semantics

The language allows either shared immutable borrows or one exclusive mutable
borrow, using Kotlin-like surface syntax and strong inference.

## ADR-0003: Lifetime Model

Lifetimes are inferred by default. Explicit lifetime parameters are required
only where needed for public generic APIs and unsafe-adjacent abstractions.

## ADR-0004: Destruction And Resource Finalization

Resources are finalized through deterministic destruction tied to ownership,
with structured resource scopes available as ergonomic sugar where useful.

## ADR-0005: Copy, Move, And Value Categories

Primitive scalar types copy. User-defined types move by default unless
explicitly declared copyable under strict language rules.

## ADR-0006: Nullability And Absence

The language uses Kotlin-style nullable types as surface syntax, semantically
modeled as explicit optional values. Non-nullable types never implicitly contain
null.

## ADR-0007: Error Handling

Recoverable errors are represented as result-style error values with lightweight
propagation syntax. Panics are reserved for unrecoverable programmer faults.

## ADR-0008: Structured Concurrency Semantics

Structured concurrency is the default concurrency model. Detached work is
explicit and constrained.

## ADR-0009: Async Suspension And Borrowing

Borrows may cross suspension points only when the compiler proves the suspended
frame cannot be concurrently accessed or outlive the borrowed data. Advanced
cases require explicit annotations.

## ADR-0010: Type System Shape

The language uses nominal user-defined types, interfaces or protocols for
behavior, and generic constraints capable of static dispatch where required.

## ADR-0011: Flow Typing And Smart Casts

The language supports flow-sensitive smart casts for immutable or exclusively
borrowed values. Mutation invalidates refinements.

## ADR-0012: Pattern Matching And Algebraic Data

The language supports sealed sum types with exhaustive pattern matching,
integrated with smart casts.

## ADR-0013: Mutability Model

Bindings are immutable by default. Mutable bindings are explicit. Mutation
authority is controlled by exclusive mutable borrows.

## ADR-0014: Thread Safety And Data-Race Freedom

The language uses compile-time send/share capabilities, derived where sound and
explicitly declared where necessary. Shared mutable state requires safe
synchronization abstractions.

## ADR-0015: Diagnostics As Semantics

The language defines diagnostic obligations for all core safety systems,
including ownership, borrowing, lifetime, nullability, move, and concurrency
errors.

## ADR-0016: Generics And Parametric Polymorphism

The language uses constrained nominal generics with explicit capability bounds.
Static specialization is permitted without exposing template metaprogramming as
the primary model.

## ADR-0017: Modules, Visibility, And API Evolution

Modules are explicit compilation and visibility units. Packages or namespaces
organize declarations within modules.

## ADR-0018: Unsafe, FFI, And Trust Boundaries

The language has explicit unsafe functions and blocks, with module-level audit
boundaries. Ordinary use is expected to go through safe wrappers.

## ADR-0019: Compile-Time Evaluation And Metaprogramming

The language starts with deterministic, bounded compile-time evaluation for
constants, layout-relevant values, and simple generic support. Macros are
deferred until the core language proves insufficient.

## ADR-0020: Portability, Targets, And Platform Semantics

The language provides Go-like bundled target packs with explicit target triples,
standard layout rules, platform capability declarations, and no hidden host
dependency for ordinary builds.

## ADR-0021: Lexical Grammar

The language uses an accepted small Kotlin-like custom lexical grammar for the
bootstrap compiler. The initial lexer accepts ASCII identifiers, a fixed
reserved keyword set, decimal, binary, and hexadecimal integer literals,
double-quoted strings with a minimal escape set, nested block comments, line
comments, and an explicit operator and delimiter set. Unicode identifiers,
string interpolation, raw strings, character literals, and numeric suffixes are
deferred until future accepted ADRs or spec updates.

As superseded by ADR-0029, the fixed reserved keyword set reserves `const` and
does not reserve `val`. Under ADR-0021's exact-match identifier rule, `val`
lexes as an ordinary identifier.

## ADR-0022: Declaration Syntax

The language uses a small Kotlin-like custom declaration grammar for the
bootstrap compiler. The accepted declaration shell covers source-file ordering,
package declarations, import declarations, visibility modifiers, functions,
structs, enums or sealed sums, interfaces, and declaration bodies. Function
parameter contents, concrete return type syntax, fields, properties,
constructors, expression bodies, and statement bodies are deferred until future
accepted ADRs or spec updates. ADR-0033 accepts its narrow enum-variant subset.

## ADR-0023: Type And Generic Syntax

The language uses a small Kotlin-like custom type grammar for the bootstrap
compiler. The accepted type syntax covers named type references, postfix
nullable type syntax, generic parameter syntax, generic argument syntax,
`&`-joined capability-bound syntax, parenthesized function type syntax, explicit
type grouping, recovery boundaries, and ADR-0015 diagnostic obligations.

ADR-0023 does not define type checking, generic constraint solving, capability
semantics, variance, wildcard types, receiver function types, type aliases,
associated types, higher-kinded types, dependent types, intersection or union
type syntax, inferred placeholder types, layout or effect types, coroutine
suspension markers, expression syntax, statement syntax, or pattern syntax.

## ADR-0024: Expression Statement And Pattern Syntax

The language uses a small Kotlin-like custom body grammar for the bootstrap
compiler. The accepted body syntax covers block bodies, explicit semicolon
statement separators, local `const` and `var` declaration statements,
assignment statements, return statements, expression statements, expression
grammar, operator precedence and associativity, call syntax, member access,
grouped expressions, `if` expressions, and pattern grammar for wildcards,
literals, bindings, qualified cases, and grouped patterns.

ADR-0024 defines parser recovery boundaries and parser diagnostic obligations
for expression grammar, statement grammar, block grammar, and pattern grammar.
As superseded by ADR-0029, `const` is the immutable-local statement starter and
controls directly spelling-dependent parser dispatch and recovery. `val` is an
ordinary identifier, including as the binding name after `const` or `var`, and
old declaration-introducer use receives only ordinary parser diagnostics and
recovery.
Unsafe block syntax, coroutine syntax, loops,
`break` and `continue`, indexing, lambdas, destructuring declarations, labels,
error propagation syntax, and advanced pattern forms remain deferred. ADR-0033
accepts its narrow expression-level `when` subset.

## ADR-0025: Module Package And Visibility Model

The bootstrap frontend uses explicit module names supplied by compiler invocation or tests. A module name is a dot-separated sequence of ADR-0021 identifiers.
Host paths are not module identity. Each parsed source file belongs to exactly
one module per frontend invocation.

Packages are namespaces inside modules. Package declarations use ADR-0022
qualified names. Files without package declarations belong to the root package,
represented by the empty package path. Imports remain syntax only for M0014 and
do not create module dependencies.

Visibility categories are `public`, `internal`, and `private`. Default visibility is `internal`. `public` means visible to other modules subject to
later dependency and name resolution rules. `internal` means visible within the
same module. `private` means visible only within the declaring source file.
Package and import declarations have no visibility metadata.

Each declaration with visibility scope has exactly one effective visibility
category and records whether that category was explicit or defaulted.

M0014 module metadata includes module name, ordered source file identities, package namespace per source file, and effective visibility metadata. It does
not include module dependencies, target triples, package manager metadata,
manifest paths, artifact hashes, resolved symbols, or imported names.

ADR-0025 defines required diagnostics for missing module identity, invalid
module identity, ambiguous source-module assignment, invalid package namespace,
unsupported visibility categories, and duplicate visibility metadata.

## ADR-0026: Name Resolution Policy

M0016 resolves a bootstrap subset using local lexical scope plus same-module package top-level declarations. Included name references are simple identifier expressions, package-qualified name expressions, and type name nodes in accepted declaration, local binding, or explicit annotation positions.

Function declaration names, type declaration names, local `const` statements,
and local `var` statements introduce names. Function parameters, pattern
bindings, import aliases, member declarations, and fields remain excluded from
M0016 name binding. ADR-0029 changes only the immutable-local binding-position
spelling; binding identity, scope, declaration order, lookup, shadowing,
duplicate, and ambiguity rules remain unchanged.

Declaration bodies and block expressions introduce lexical scopes. Top-level declarations in the same module and package namespace are visible throughout that module/package namespace regardless of source-file order. Local bindings are not visible before their declaration statement and remain visible through the end of their containing lexical block unless shadowed.

Unqualified lookup searches innermost local lexical scope outward, then the current source file's package namespace in the current module. Package-qualified lookup uses the explicitly named package namespace in the current module only. Imports remain syntax-only and do not add lookup candidates.

Duplicate local bindings in the same scope and duplicate top-level declarations with the same module, package namespace, declaration name, and declaration kind are rejected. Ambiguous lookup reports ambiguity instead of choosing by insertion order, source-file order, or parser traversal order.

M0016 records same-module visibility metadata but does not enforce visibility. Cross-module lookup, member lookup, overload resolution, extension lookup, and type-directed lookup remain unsupported.

Resolution diagnostics include `unresolved_name`, `duplicate_name`, `ambiguous_name`, `unsupported_import_resolution`, `unsupported_cross_module_lookup`, and `unsupported_member_resolution`. Each diagnostic follows ADR-0015 and ADR-0026 primary span, recovery action, source-of-truth citation, and safe suggestion policy requirements.

## ADR-0027: Type Checking Core

M0018 defines a small bootstrap type checker with primitive type-checking identities, literal typing, resolved name expression typing, explicit nullable wrappers, and exact assignment compatibility.

Typed output is side-table metadata: an expression type table, declaration signature table, assignment check table, and diagnostics list. The type checker does not rewrite the AST.

Primitive identities `Bool`, `Int`, `String`, `Unit`, and `Null` are type-checking identities only and have no ABI or layout meaning.

Assignment compatibility is exact type identity, except `Null` is assignment-compatible only with nullable target types and non-null base values are assignment-compatible with their nullable wrapper.

Direct function declaration calls and structural function type application are deferred for M0018. Overload resolution, implicit numeric conversion, member lookup, generic constraint solving, ownership and move analysis, borrow checking, HIR lowering, MIR lowering, and backend behavior remain deferred.

Type checking diagnostics include `type_mismatch`, `unresolved_type_rule`, `unsupported_type_rule`, and `ambiguous_type_rule`. Diagnostics define primary spans, recovery actions, source-of-truth citations, safe suggestion policies, and stable rule identifiers where required.

## ADR-0028: Nullability And Flow Typing

M0019 defines a narrow nullability and flow-typing subset for local immutable null refinements.

Null-test recognition is a flow-specific condition recognizer for direct comparisons between one simple local name expression and `null`; it does not require general binary expression type checking, overload resolution, user-defined equality, implicit conversion, or Boolean operator typing.

Refinements apply only to eligible immutable local bindings with known nullable wrapper types. `x != null` and `null != x` refine inside the then branch block. `x == null` and `null == x` refine inside the else branch block when an else branch exists. Refinements start at the first statement or optional trailing expression in the refined branch and end at that branch's closing brace.

Refined output remains side-table metadata. Declaration signatures and original local binding types preserve the original nullable type; refined expression type entries are per-use views inside guarded regions.

M0019 diagnoses nullable use where a nullable expression is required to be non-null without an active refinement. A simple, unqualified name expression resolving to the same eligible immutable local whose refinement ended at the closing brace of its guarded `if` branch is a region-exit use only when it occurs in a later statement or trailing expression directly contained by that `if`'s enclosing block and is required to have base type `T`. That exact use reports `invalidated_refinement` with stable rule identifier `region_exit_invalidated_refinement`; its primary span is the later name expression, it has no secondary span, and recovery treats it as its original `T?`. Where it matches an annotated-local initializer shape, this region-exit mapping takes precedence over ADR-0030. Otherwise, for an annotated local whose initializer is exactly a bare resolved name of type `T?` and whose annotation is its base type `T`, `invalid_nullable_use` uses the stable rule identifier `nullable_assignment_without_refinement`; ADR-0027 keeps the initializer expression as the primary span. This mapping is limited to that case. Flow diagnostics include `invalid_nullable_use`, `invalidated_refinement`, `unsupported_flow_rule`, and `ambiguous_flow_rule`.

Member nullable access, safe-call operators, force unwrap operators, boolean-combination refinement, negated-condition refinement, patterns, type-test smart casts, parameter refinements, top-level declaration refinements, mutable binding refinements, exclusive-borrow refinements, alias analysis, function call effects, member mutation effects, coroutine suspension effects, unsafe and FFI nullability, generic nullable constraints, HIR, MIR, and backend behavior remain deferred.

## ADR-0029: Immutable Local `const` Keyword

The immutable-local declaration introducer is `const`, replacing `val` as a
hard lexical and grammar change. A valid local `const` maps to the existing
immutable-local semantic category and follows the initializer rules already
applicable to that category. The spelling has no compile-time-constant,
evaluator, storage, copyability, ownership, destruction, borrow, lifetime,
send/share, type-position, or layout meaning.

`val` is not reserved and lexes as an ordinary identifier. It is excluded only
from the immutable-local declaration-introducer position and remains valid in
ordinary identifier positions, including as the binding name in
`const val: Int = 1;` and `var val = 1;`. Removed declaration-introducer use has
no alias or special legacy diagnostic and receives only ordinary parser
diagnostics and recovery.

Reserving the formerly ordinary identifier `const` is source-breaking for old
uses of that spelling as an identifier. Any future rule that gives local
`const` compile-time meaning must explicitly supersede ADR-0029; a separate
compile-time-evaluation feature does not reinterpret it implicitly.

## ADR-0032: Generic Constraint Enforcement Sequencing

M0020 represents generic parameter identity and explicit capability-bound
occurrences, but does not enforce a bound. The language still selects
constrained nominal generics; however, capability identity or resolution,
satisfaction rules, generic substitution, and bound-violation diagnostics are
deferred until ownership and thread-capability semantics provide their required
inputs. M0021 depends only on the representation boundary. A later
post-M0024 milestone must introduce enforcement through a separate accepted
semantic decision.

## ADR-0033: Bootstrap Sealed Sums And Exhaustive Match

M0021 accepts closed no-payload `enum` variants and expression-level `when`.
Enum variants are identifier-only and scoped to their declaring module/package.
`when (subject) { Enum.Variant -> expression; _ -> expression; }` uses only
qualified variant patterns or a wildcard. A match is exhaustive only when it
covers each declared variant once or has exactly one wildcard. Duplicate,
unknown, and missing variants, duplicate wildcards, and non-enum subjects
diagnose under ADR-0033; payloads, destructuring, generic enums, nullable
coverage, implicit smart casts, and arm-result type unification remain deferred.

## ADR-0034: Bootstrap Enum Subject Typing

For functions with bodies, parameters use `identifier : named-type` entries in
a comma-separated parameter list. M0021 accepts an ADR-0033 `when` subject
only as a bare reference to one such parameter when its named annotation
resolves in the declaring module/package to exactly one bootstrap enum. Other
subject shapes, unresolved types, and non-enum types report
`invalid_match_subject` on the subject. Parameters are immutable local bindings
visible throughout their function body; general parameter typing, enum value
expressions, constructors, member lookup, nullable/generic parameters, calls,
and cross-module lookup remain deferred.

## ADR-0035: Bootstrap Ownership And Move Analysis

M0022 classifies `Bool`, `Int`, `Unit`, and `Null` as copyable primitive
identities, `String` as move-only, and all current-module user-defined nominal
identities, including bootstrap enums, as move-only. Explicitly copyable
user-defined types remain deferred.

Only a local `const` or `var` initializer, or an assignment statement, whose
value is a bare resolved local name of move-only type is an M0022 ownership
transfer site. A later bare local-name expression using that moved binding
reports `use_after_move` on the later use with the transfer expression as the
move-origin secondary span. Copyable values do not enter the moved state.

M0022 records ownership facts in side tables and does not rewrite the AST or
lower to HIR. Calls, returns, captures, `when` subject evaluation, branch move
joins, destructuring, member or partial moves, destructors, borrowing,
coroutine frames, FFI, layout, cloning, generic copyability, and user-declared
copy remain deferred.

## ADR-0036: Bootstrap Borrow And Lifetime Analysis

M0023 uses a metadata-only bootstrap borrow model. It adds no source-level
borrow syntax, reference types, dereference operators, function parameter
borrowing, method receivers, member borrows, closure captures, coroutine
borrows, unsafe references, or FFI references.

Borrow input records contain a borrow node, borrowed local binding, borrow kind
(`shared` or `exclusive`), and region node. Shared borrows may overlap other
shared borrows of the same local in the same region. An exclusive borrow
conflicts with any other shared or exclusive borrow of the same local in the
same region. M0023 overlap is exact region-node equality only; nested, sibling,
non-lexical, loop, path-sensitive, and control-flow-sensitive overlap rules are
deferred.

Lifetime escape input records contain an escape node, borrowed local binding,
borrow node, borrow region, and use region. A `lifetime_escape` diagnostic is
reported when the use region differs from the borrow region. `borrow_conflict`
diagnoses on the later conflicting borrow with the earlier borrow as secondary
span. `lifetime_escape` diagnoses on the escape node with the original borrow
as secondary span.

## ADR-0037: Bootstrap Thread Capability Analysis

M0024 uses a metadata-only bootstrap thread-capability model. It adds no
source-level task spawning, detached threads, async blocks, coroutine bodies,
closures, synchronization primitives, atomics, locks, generic capability
enforcement, user-declared capability implementations, or unsafe capability
overrides.

M0024 defines `Send` for values that may transfer across an approved concurrent
boundary and `Share` for values that may be shared across an approved
concurrent boundary without exclusive transfer. `Bool`, `Int`, `Unit`, and
`Null` satisfy both capabilities. `String` satisfies `Send` but not `Share`.
Nullable types satisfy a capability only when their non-null base type satisfies
that capability. Current-module nominal user-defined types, generic parameter
types, unsupported types, unresolved types, and absent type information satisfy
neither capability in M0024.

Boundary input records contain a boundary node and ordered capture records.
Capture records contain a capture node, captured local binding, captured type,
and required capability. A `missing_thread_capability` diagnostic is reported
when a capture's type does not satisfy the required capability. The capture node
is primary and the boundary node is secondary. Because M0024 has no approved
synchronization abstractions, shared mutable state is not accepted through a
`Share` capture; mutable captures may only be modeled as `Send` transfers when
the type satisfies `Send`.

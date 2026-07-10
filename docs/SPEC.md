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

## ADR-0022: Declaration Syntax

The language uses a small Kotlin-like custom declaration grammar for the
bootstrap compiler. The accepted declaration shell covers source-file ordering,
package declarations, import declarations, visibility modifiers, functions,
structs, enums or sealed sums, interfaces, and declaration bodies. Function
parameter contents, concrete return type syntax, fields, properties,
constructors, enum variants, expression bodies, and statement bodies are
deferred until future accepted ADRs or spec updates.

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
statement separators, local `val` and `var` declaration statements, assignment
statements, return statements, expression statements, expression grammar,
operator precedence and associativity, call syntax, member access, grouped
expressions, `if` expressions, and pattern grammar for wildcards, literals,
bindings, qualified cases, and grouped patterns.

ADR-0024 defines parser recovery boundaries and parser diagnostic obligations
for expression grammar, statement grammar, block grammar, and pattern grammar.
Unsafe block syntax, coroutine syntax, match or `when` syntax, loops,
`break` and `continue`, indexing, lambdas, destructuring declarations, labels,
error propagation syntax, and advanced pattern forms remain deferred.

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

Function declaration names, type declaration names, local `val` statements, and local `var` statements introduce names. Function parameters, pattern bindings, import aliases, member declarations, and fields remain excluded from M0016 name binding.

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

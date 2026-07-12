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

As superseded by ADR-0061, the fixed reserved keyword set reserves both `val`
and `const`. `val` introduces an immutable runtime binding and `const`
introduces a compile-time local constant.

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
statement separators, local `val`, `const`, and `var` declaration statements,
assignment statements, return statements, expression statements, expression
grammar, operator precedence and associativity, call syntax, member access,
grouped expressions, `if` expressions, and pattern grammar for wildcards,
literals, bindings, qualified cases, and grouped patterns.

ADR-0024 defines parser recovery boundaries and parser diagnostic obligations
for expression grammar, statement grammar, block grammar, and pattern grammar.
As superseded by ADR-0061, `val` is the immutable-local statement starter and
`const` is the compile-time local statement starter. Both are reserved in
keyword positions.
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

Function declaration names, type declaration names, local `val`, `const`, and
`var` statements introduce names. Function parameters, pattern
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

Primitive identities `Bool`, `Int`, `String`, `Unit`, and `Null` are type-checking
identities. `String` receives the compiler-owned opaque runtime representation
defined by ADR-0064; it has no stable public layout or FFI meaning.

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

## ADR-0029 And ADR-0061: Local Binding Keywords

ADR-0061 supersedes ADR-0029's keyword and compile-time-meaning decisions.
`val` is the reserved immutable-local declaration introducer. It preserves the
existing immutable binding category and has no compile-time evaluation meaning.
`const` remains reserved, is valid only for local declarations, requires an
initializer, and is a compile-time constant.

`const` initializers accept primitive literals and pure primitive operators for
`Bool`, `Int`, `Float`, `Byte`, and `Unit`. Calls, local reads, allocation, I/O,
control flow, strings, nullable values, user-defined values, and unsupported
operators are rejected. Typed constant facts may be consumed by runtime
expressions and future fixed-array length expressions.

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

Only a local `val`, `const`, or `var` initializer, or an assignment statement, whose
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

## ADR-0038: Bootstrap Coroutine Scope And Suspension Analysis

M0025 uses a metadata-only bootstrap coroutine scope and suspension model. It
adds no source-level coroutine, `async`, `await`, task-spawn, detached-task,
cancellation, pinned-frame, closure, channel, synchronization, or scheduler
syntax. Existing unsupported concurrency-like source forms remain rejected or
unsupported.

Structured scope input records contain a scope node and ordered child-task
records. Child-task records contain a task node, the containing scope node, and
the scope node in which the child is proven to complete or be cancelled. A child
task is valid only when its completion-or-cancellation scope is the same scope
as its containing structured scope. Other task escape rules are deferred.

M0025 reports `task_scope_escape` when a child task is not proven to complete or
be cancelled in its containing structured scope. The child task node is primary,
the containing scope node is secondary, and the diagnostic identifies that the
child task would outlive its structured scope.

Suspension point records contain a suspension node and suspended-frame scope
node. Suspended-borrow records contain a suspension node, borrowed local
binding, borrow node, borrow kind, borrowed-value lifetime scope,
suspended-frame scope, and whether the suspended frame may be concurrently
accessed. A borrow crossing suspension is valid only when the frame is not
concurrently accessible and the suspended-frame scope is the same scope as the
borrowed value's lifetime scope.

M0025 reports `borrow_across_suspension` when a suspended borrow may be
concurrently accessed, may outlive the borrowed value, or both. The suspension
node is primary, the borrow node is secondary, and the diagnostic identifies the
borrowed local and rejection reason.

Cancellation resource-safety in M0025 is limited to the structured-scope
completion-or-cancellation check. Runtime cancellation propagation, destructor
execution during cancellation, cancellation handlers, cancellation masking, and
async drop are deferred. ADR-0037 remains the authority for any supplied
thread-capability capture records.

## ADR-0039: Bootstrap Unsafe FFI Boundary Analysis

M0026 uses a metadata-only bootstrap unsafe and FFI boundary model. It adds no
source-level unsafe block, unsafe function, extern block, foreign declaration,
ABI string, link attribute, target attribute, safe-wrapper syntax, or module
audit syntax. Existing unsupported unsafe-like and FFI-like source forms remain
rejected or unsupported.

M0026 defines `ProvenSafe` for operations proven safe by accepted compiler
analyses and `TrustedUnsafe` for operations relying on explicit programmer or
binding assertions. Unsafe context records contain a context node and context
kind: `block`, `function`, or `module_audit`. Unsafe operation records contain
an operation node, operation kind, safety basis, and optional containing unsafe
context node.

A `ProvenSafe` operation is accepted without an unsafe context. A
`TrustedUnsafe` operation is accepted only when its containing context node
matches a supplied unsafe context record. M0026 reports
`unsafe_operation_outside_context` when a trusted unsafe operation has no
matching unsafe context. The operation node is primary. A non-matching supplied
context node is secondary when present. The diagnostic identifies trusted
assertion rather than compiler-proven safety.

FFI declaration records contain a declaration node, declaration kind, safety
basis, safe-wrapper status, and metadata presence for target contract, calling
convention, nullability, ownership transfer, lifetime validity, and
thread-safety or send/share guarantees. M0026 validates metadata presence only;
target triples, layout, calling convention compatibility, symbols, linker
inputs, generated bindings, dynamic loading, platform APIs, and ABI lowering
are deferred.

M0026 reports `missing_ffi_safety_metadata` when an FFI declaration lacks one
or more required metadata categories. The FFI declaration node is primary, and
the diagnostic lists the missing categories. Safe-wrapper status is metadata
only in M0026 and does not affect source visibility, type checking, or call
lowering.

## ADR-0040: Bootstrap Program Entry Point

The first executable subset accepts exactly one top-level `main` function in
the compiler-selected root module entry package. It has no parameters, declares
return type `Int`, has a body, and returns an `Int` on every reachable path.
The language-level result of `main` maps to the process exit code for bootstrap
executables. The first executable smoke must use a non-negative `Int` in
`0..255`; other result mappings are deferred. CLI arguments are deferred.

Entry diagnostics include `missing_entry_point`, `duplicate_entry_point`,
`invalid_entry_point_signature`, and `missing_return`.

## ADR-0041: Bootstrap Function Call And Return Semantics

The first executable subset accepts direct calls to resolved top-level
functions in the same module/package. A call is well-typed when the callee
resolves to exactly one function, argument count matches parameter count, each
argument is assignment-compatible with its parameter type, and the call
expression type is the callee return type. Direct and mutual recursion are
deferred and rejected.

Arguments evaluate left-to-right before callee entry. Bootstrap parameter
passing is by value. Only explicit `return expression;` returns a value in the
first executable subset. Implicit trailing-expression returns are deferred for
runtime semantics. Functions returning `Int` must explicitly return an `Int` on
every reachable accepted path.

Call and return diagnostics include `invalid_call_target`,
`argument_count_mismatch`, `argument_type_mismatch`,
`recursive_call_unsupported`, `return_type_mismatch`, `missing_return`, and
`unreachable_return`.

## ADR-0042: Bootstrap Minimal Executable Subset

The first runnable smoke subset includes package declarations, top-level
functions with explicit `Int` parameter and return types, the ADR-0040 `main`
form, `Int` locals, local `val`, `const`, and `var` declarations, assignments to local
`var` bindings, integer literals, bare local-name expressions, parenthesized
expressions, unary arithmetic and bitwise operations `+`, `-`, and `~` on
`Int`, binary arithmetic operations `+`, `-`, `*`, `/`, `%`, and `**` on
`Int`, binary bitwise operations `&`, `|`, and `^` on `Int`, binary shift
operations `<<` and `>>` on `Int`, direct same-module top-level function
calls, and explicit `return expression;`.

The required runnable shape uses helper functions that exercise `Int`
arithmetic and bitwise operations and `main` returning the helper result as the
process exit code.
Classes, structs at runtime, interfaces, enums at runtime, generics, nullable
runtime representation, heap allocation, destructuring, pattern matching,
loops, broader branching, coroutines, unsafe, FFI, printing, string operations
outside ADR-0064, standard
library calls, scheduler/runtime work, exceptions or panics as language
constructs, closures, methods, and general member access remain deferred.
Fixed-size inline arrays are accepted only as defined by ADR-0063. Owned UTF-8
strings are accepted only as defined by ADR-0064. Dynamic arrays, slices, and
target-pack APIs remain deferred. Unsupported parsed forms must fail before
backend lowering with
`unsupported_executable_form` or a more specific existing diagnostic. M0028
must add parser and type-checker support for executable operators not already
covered by ADR-0024 before HIR lowering consumes executable fixtures.

## ADR-0043: Bootstrap Integer Runtime Semantics

For the bootstrap executable subset, `Int` is a signed 64-bit two's-complement
integer with range `-9223372036854775808..9223372036854775807`. Integer
literals in executable code must fit this range or report
`integer_literal_out_of_range`.

Unary `+` returns its `Int` operand. Unary `-` produces arithmetic negation and
overflows for `-9223372036854775808`. Unary `~` produces bitwise complement.
Binary arithmetic operations `+`, `-`, `*`, `/`, `%`, and `**` evaluate the
left operand before the right operand and produce `Int`. Division truncates
toward zero. Remainder has the same sign as the dividend. `**` requires a
non-negative exponent. Bitwise `&`, `|`, and `^` operate on the
two's-complement representation. Shifts `<<` and `>>` require a shift count in
`0..63`, and `>>` is arithmetic right shift. Statically provable overflow
reports `integer_overflow`; known division or modulo by zero reports
`division_by_zero`; known negative exponent reports `negative_exponent`; known
invalid shift count reports `invalid_shift_count`. Equivalent runtime failures
trap and must not silently wrap or continue. Wrapping, saturating, unchecked
arithmetic, numeric casts, unsigned integers, fixed-width aliases, floats,
machine-word integers, rotates, population count, and bit-scan intrinsics are
deferred.

## ADR-0048: Bootstrap Integer Constant Expressions

For ADR-0043 diagnostics, a bootstrap integer constant expression is an
integer literal or a grouped, unary `+`/`-`/`~`, or ADR-0042 integer binary
operator expression whose operands are bootstrap integer constant expressions.
Local bindings, names, assignments, calls, member access, `if`, `when`, and
other forms are not bootstrap integer constant expressions. They must not
receive a static arithmetic diagnostic solely from an inferred runtime value.

## ADR-0049: Bootstrap Entry-Point Diagnostic Provenance

`missing_entry_point` uses the explicit selected entry-package invocation input
as its primary location. Every duplicate top-level selected-package `main`
candidate receives `duplicate_entry_point` at its declaration. An invalid
candidate receives `invalid_entry_point_signature` at its declaration. Entry
diagnostics carry either a source-file-qualified span or the explicit external
input location, never a host path or arena-local node identity alone; recovery
selects no entry point.

## ADR-0050: Bootstrap Straight-Line Return Diagnostics

Only explicit returns directly contained by an `Int` function's body block
participate in bootstrap straight-line analysis. No direct return reports
`missing_return` at the declaration; every direct return after the first
reports `unreachable_return` at that return. Nested-block returns are deferred,
neither satisfying nor producing these diagnostics.

## ADR-0051: Bootstrap Direct Call Diagnostics

`invalid_call_target` attaches to the callee, `argument_count_mismatch` to the
call, `argument_type_mismatch` to the mismatching argument, and
`recursive_call_unsupported` to the recursive call. Each recovery produces no
successful call target or result type.

## ADR-0052: Bootstrap Module Type Identity

Every source file in one module compilation shares one TypeArena. TypeId is
meaningful only inside that module arena; same-module direct-call compatibility
uses those shared identities, while cross-module calls remain deferred.

## ADR-0053: Bootstrap Unsupported Executable-Form Diagnostics

Every parsed form outside ADR-0042 receives `unsupported_executable_form`
unless a more-specific accepted diagnostic already applies. The diagnostic
attaches to the source-file-qualified outermost unsupported form; unsupported
descendants in that form are suppressed. Recovery emits no executable type,
control-flow, ownership, or lowering fact for the rejected region, and HIR
must not receive it. Unrelated sibling forms continue checking.

## ADR-0054: Bootstrap Return-Type Mismatch Diagnostics

`return_type_mismatch` attaches to the explicit return expression and emits
only when that expression and the enclosing function's declared return type
are known and incompatible. Recovery records no typed executable return fact;
an unresolved or deferred expression retains its original diagnostic without a
second mismatch error.

## ADR-0055: Bootstrap Type Environment Transport

The owning module `TypeArena` accompanies typed lowering boundaries that need
to interpret `TypeId`, including HIR-to-MIR and MIR-to-backend lowering. HIR
and MIR preserve type identities without owning, duplicating, or reinterpreting
the arena. A runtime lowering resolves each identity through that exact arena;
for the bootstrap executable subset, bootstrap primitives and recursively
supported inline arrays from ADR-0063 are supported runtime values and lower
according to ADR-0043, ADR-0046, and ADR-0063. Missing, foreign, or unsupported
identities are explicit unsupported-lowering conditions and must not be
inferred from raw ID values.

## ADR-0044: Bootstrap HIR Runtime Contract

Bootstrap HIR is typed, source-mapped, and backend-independent. It preserves
function identity, package/module identity, entry classification, parameter
order and types, return type, local binding identity and mutability, expression
types, direct callee identity, left-to-right operand and argument order,
explicit returns, source spans, required ownership and safety facts, already
produced or proven-absent safety diagnostics, and unsupported-form markers for
constructs excluded by ADR-0042. HIR must reject unchecked, unresolved, or
unsupported AST input rather than represent it as executable HIR.

## ADR-0045: Bootstrap MIR Runtime Contract

Bootstrap MIR is backend-independent and contains function definitions, ordered
parameters, return types, local slots, temporaries, basic blocks, ordered
instructions, one terminator per block, source mapping, `Int` constants, local
load/store, checked or trapping `Int` arithmetic, exponentiation, bitwise, and
shift operations, direct calls, unconditional branches, conditional branches
only where already needed, return terminators, and trap terminators.

Cleanup and destruction are a bootstrap boundary. The first executable subset
has only bootstrap primitives and supported inline arrays as runtime values, so
there are no user-defined destructors, heap resources, async cancellation
cleanups, or FFI cleanup edges. MIR must reserve
a later cleanup insertion boundary without inventing cleanup semantics.

## ADR-0046: Bootstrap ABI And Calling Convention

The bootstrap backend assumes the current host target for the initial smoke.
Cross-target behavior is deferred to M0033. `Int` lowers to a signed 64-bit
integer value. Unsupported runtime types must not reach ABI lowering.

Bootstrap language functions use an internal Neu calling convention that may be
implemented with Cranelift's platform-default call convention for the initial
host target. This is not a stable external ABI. Bootstrap symbol names are
deterministic internal symbols derived from module identity, package namespace,
and function name and must be collision-free for the bootstrap subset.

Language `main` is not the raw platform entry symbol. The object/link pipeline
must arrange a bootstrap executable entry path that calls language `main`.

## ADR-0047: Bootstrap Object Link Runtime Model

The first object/link pipeline targets the current host object's native format
only. Cross-object formats and target packs are deferred to M0033.

The first runnable program requires no Neu standard library. It may use a tiny
bootstrap runtime shim whose only responsibilities are participating in the
entry path, calling compiled language `main`, mapping a non-negative `Int` in
`0..255` to the process exit status, and trapping on bootstrap runtime traps
such as checked integer overflow, division/modulo by zero, negative exponent,
or invalid shift count. The shim is not a language standard library and must
not provide printing, allocation, CLI arguments, scheduling, panics as a
language feature, exceptions, FFI helpers, or platform APIs.

M0032 must use the planned bundled linker path for the initial host target. Any
temporary host-tool dependency must be documented as a blocker or explicit
limitation and must not be presented as satisfying Go-like target-pack
semantics.

## ADR-0056: Bootstrap Function Symbol Identity

HIR and MIR preserve structured bootstrap function identity containing the
accepted module identity, package namespace, and source function name. The
identity is carried from parsed declaration metadata through HIR and MIR to
object emission. Missing identity is an explicit lowering failure; a numeric
MIR function ID is not a substitute. The backend derives a deterministic,
collision-free internal object symbol from those components. Exact escaping
and encoding are compiler implementation details and do not define a public
ABI or new language semantics.

## ADR-0057: Bootstrap Target-Pack Linker Contract

The initial host target pack owns a pinned executable `lld` linker artifact and
a target-specific startup-shim object. Its manifest identifies the exact target
triple, native object and executable formats, linker path, startup-shim path,
platform entry symbol, canonical language-entry symbol, and test-visible
non-success trap status. Paths are pack-relative; absolute paths, traversal,
missing artifacts, and target mismatches are rejected.

The compiler receives an explicit target-pack root and never searches `PATH` or
falls back to a host linker. The startup shim calls the selected language
`main`, maps an `Int` in `0..255` to process exit status, and exits
unsuccessfully for bootstrap traps or unsupported exit values. It is not a
standard library and provides no printing, allocation, scheduling, CLI
arguments, or panic formatting. M0032 covers the current host pack; additional
target-pack distribution remains M0033 work.

## ADR-0058: Bootstrap Target Capability Profile

Every bundled target pack declares a typed `[capabilities]` profile containing
`int_width_bits`, `pointer_width_bits`, `endianness`, `alignment_model`,
`calling_convention`, `atomic_model`, and `platform_apis`. The initial host
profile declares signed 64-bit `Int`, 64-bit pointers, little-endian layout,
the `platform-default` bootstrap calling convention, deferred alignment and
atomic models, and an empty platform API list.

Target-pack resolution validates these declarations. The compiler never infers
capabilities from the host or silently substitutes values from a target triple.
Deferred capabilities are unavailable to executable forms, and an empty
platform API list means no platform API or standard library is provided. Future
target packs must declare their own profile and require accepted ABI, layout,
atomic, or platform API semantics before using non-deferred values.

## ADR-0059: Bootstrap Primitive Runtime Support

The bootstrap runtime supports `Bool`, `Unit`, `Float`, and `Byte` in addition
to `Int`. `Bool` uses `true` and `false`, one-byte `0`/`1` representation,
logical `!`, short-circuit `&&`/`||`, and equality. `Unit` has the single value
`()`, no payload, and no ABI return value. `Float` is IEEE 754 binary64 with
decimal/exponent literals, arithmetic, comparisons, and IEEE NaN behavior.
`Byte` is unsigned eight-bit `0..255`, has checked arithmetic and bitwise
operations, and has no implicit conversion to or from `Int`.

Primitive operations require exact operand types. HIR and MIR preserve literal
kind, value, type, source mapping, and safety facts. Cranelift lowers `Bool` to
`i8` normalized to `0` or `1`, `Byte` to unsigned `i8`, `Float` to `f64`, and
`Unit` without an ABI result. Byte range, overflow, division, shift, malformed
float, and exact primitive mismatch diagnostics follow ADR-0059. The entry
point remains an `Int`-returning `main`; additional primitives are supported in
helpers, locals, parameters, returns, calls, and backend smokes.

## ADR-0060: Bootstrap Control Flow

The bootstrap executable subset accepts Boolean `if`/`else` conditionals and
inclusive integer range loops written as `for (name in start..end)`. `if` and
`else` are control-flow statements; value-producing conditional expressions
are deferred, and an `if` without an `else` has `Unit` behavior. The loop
binding is immutable, `Int`, scoped to the loop body, and advances by one.
`break` leaves the
innermost loop and `continue` advances it; neither carries a value or label.
`while` remains unsupported. Branches and loop back-edges preserve ownership,
borrowing, initialization, cleanup, and source-mapping obligations. No runtime,
stdlib, printing, or scheduler behavior is added.

## ADR-0062: Inferred Ownership Effects

Neu infers read-only, exclusive-mutating, consuming, storing, and
returned-ownership effects from function bodies and call sites. Read-only uses
create temporary shared borrows and preserve the source binding. Mutating,
consuming, and storing uses consume move-only values; copyable values remain
available under existing copy rules. Both `val` and `var` may be consumed, but
only `var` supports atomic consume-and-rebind when a consuming call returns a
compatible owned value.

Effect contracts preserve per-parameter projections, implicit borrow regions,
conditional ownership states, consumption origins, and returned ownership.
Bindings are usable after branches and loops only when every reachable path
proves them available. Separate compilation requires exported effect metadata;
missing or stale metadata is an error. No explicit reference, dereference,
lifetime, or move syntax is introduced. Methods, fields, closures, coroutines,
move-only nominal runtime values, slices, allocation outside ADR-0064, and FFI
remain deferred where their existing frontend or backend contracts are absent;
fixed-size inline arrays follow ADR-0063 and owned UTF-8 strings follow
ADR-0064.

## ADR-0065: Class And Interface Foundation

Classes and interfaces are nominal identities consisting of their declaring
module, package, declaration, and later generic arguments. A class has at most
one direct superclass and may implement multiple interfaces. Interfaces may
extend interfaces. Structural conformance, multiple class inheritance, traits,
mixins, nested classes, companion objects, and generic classes/interfaces are
deferred.

Classes may declare typed `val` or `var` fields and `func` methods. Visibility is
`public`, `internal`, or `private` under ADR-0025; `protected` is deferred.
Interfaces declare required methods. Default methods, interface state,
extension methods, and operator overloading are deferred. Implementations must
provide exactly one compatible method and use explicit `override` when
replacing an inherited method. Field hiding and ambiguous interface methods are
diagnostics.

Instance methods have an implicit non-null `this`; `this.` is available for
shadowing. Local names shadow members. `this` cannot be rebound or escape before
construction completes. Class/interface values are nullable only with the
accepted postfix nullable type form; safe access, casts, type tests, and
downcasts are deferred. Null checks use existing flow typing before dispatch.

Instances and fields are move-only by default. Fields own their values. Read
receiver use borrows implicitly, mutation requires an exclusive inferred
effect, and methods do not implicitly consume `this`. `Send` and `Share` are
derived from owned fields and immutable state under ADR-0037. Cyclic owning
graphs, tracing collection, reflection, serialization, and FFI remain
deferred.

## ADR-0066: Inheritance And Dispatch

Neu uses one direct superclass and multiple interfaces. The `open`-gating and
default-final wording in this historical section is superseded by ADR-0070;
classes and methods are overridable by default and `final` opts out. Overrides
require explicit `override` and compatible parameter types, return type,
visibility, receiver effects, and capabilities. Private methods are not
virtual. Field hiding is rejected, and there is no `protected` visibility.

`super.method(...)` is valid only in a derived instance method or constructor,
selects the immediate superclass implementation, and cannot be stored,
returned, or used to bypass visibility. Unqualified lookup uses the current
receiver and local bindings shadow members; `this.member` qualifies receiver
lookup. A class must satisfy every required interface method exactly once.
Default-method conflict rules are deferred because default methods are
deferred.

Non-overridden or final methods use compiler-private direct dispatch.
Overridable class methods use compiler-private virtual metadata and interface calls use
compiler-private interface tables. No object, vtable, interface-table, method,
or symbol layout is a stable public ABI or FFI contract. Separate compilation
exports nominal method identity, visibility, signatures, override relationships,
capabilities, and ownership-effect metadata, not target-specific table
layouts. Downcasts, runtime type tests, nullable dispatch beyond existing flow
typing, reflection,
multiple dispatch, dynamic loading, and FFI vtables are deferred.

## ADR-0067: Object Lifecycle And ABI

Classes use one primary constructor form and have no implicit default
constructor. Secondary constructors and overload resolution are deferred.
Fields require declared types and exactly-once initialization before the object
is observable. Initialization follows declaration order and explicit
superclass construction order. `this` cannot escape or be used as a fully
initialized receiver before construction completes.

Instances are compiler-managed owned values. The compiler may use local or
target-pack-managed heap storage, but source code cannot request or observe
placement. There is no `free`, allocator primitive, stable pointer, tracing
collector, or user-visible deallocation API. Objects move by default and are
not copied unless a later accepted copyability decision proves all owned state
copyable. Receiver and field effects follow ADR-0062.

Fields are destroyed recursively in reverse declaration order, with derived
fields before inherited superclass fields. Partially initialized objects are
never observable; initialized fields are cleaned in reverse order on rejected
construction or runtime allocation failure, and allocation failure traps
non-successfully. Exceptions, `Result` construction APIs, and catch behavior
are deferred. Fields have no implicit null or zero state, and reads before
initialization are diagnostics. Cyclic owning graphs are rejected or deferred.

Object fields, offsets, alignment, padding, allocation headers, vtables, and
interface tables are compiler-private target-pack contracts. Separate
compilation carries nominal identity, field types, visibility, lifecycle,
capability, and ownership metadata, never raw offsets. Public object layout,
stable object ABI, FFI, and standard-library allocation remain deferred.

## ADR-0069: Primary Constructors And Construction

A class may have one primary constructor parameter list after its name:
`class Name(val field: T, var other: U) { ... }`. Constructor parameters
marked `val` or `var` are fields initialized left to right. Unmarked parameters
are temporary constructor parameters. Names must be unique; duplicate fields
are diagnostics. The parameter list may be empty.

The only construction expression is `new Name(argument1, argument2)`. Arguments
evaluate left to right and match the primary constructor parameter types exactly.
Secondary constructors, overloads, default arguments, implicit default
constructors, and conversions are deferred. A superclass constructor is called
explicitly with `super(...)` in the class header and completes before derived
fields initialize. Interfaces have no constructors.

Construction transfers owned arguments into fields, preserves inferred receiver
and field effects, and provides no user-visible allocation, deallocation,
pointer, or layout API. Missing or duplicate initialization is diagnosed before
lowering; allocation failure and construction failure trap non-successfully,
with reverse cleanup of fields initialized so far. Constructor bodies,
secondary construction, reflection, exceptions, and FFI remain deferred until
their own accepted contracts.

## ADR-0068: Class And Field Surface Syntax

`class` and `interface` are reserved declaration keywords. A class header is
`class Name` optionally followed by `: Base(), InterfaceName, ...`; an
interface may extend interfaces after `:`. Generic class and interface headers
remain deferred.

Class fields use an optional `public`, `internal`, or `private` modifier,
followed by `val` or `var`, a name, `:`, and a declared type, terminated by
`;`. `protected` is rejected. Fields have no declaration-time default
initializer in this foundation; the primary constructor initializes every
field under ADR-0067. Interface bodies contain required method declarations;
default methods and interface fields are deferred.

Field access is `receiver.field`. In an instance method or constructor, a bare
field name means `this.field`; explicit `this.field` disambiguates a shadowed
field. Task-008 records nominal and field metadata and type-checks projections
only where an accepted receiver context exists. `new`, constructor calls,
allocation, initialization, and runtime object access are deferred to task-009.

## ADR-0070: Final-Only Runtime Dispatch

Classes and methods are overridable by default. `final class` prevents
subclassing, `final func` prevents overriding, and `override func` remains
required for inherited replacements. `open` is not accepted in declaration
positions and is diagnosed rather than silently migrated. Interface methods and
constructors cannot be `final` or `override`; private methods are non-virtual.

Class calls preserve compiler-private direct, virtual-class, interface, and
static-super dispatch facts. Base-typed and interface-typed receivers select
the runtime object's most-derived implementation. `super.method(...)` always
selects the immediate superclass implementation statically. Method identity,
override target, dispatch slot, receiver ownership/effects, and source spans
are preserved through HIR, MIR, Cranelift, object emission, and linking.

Vtables and interface tables are compiler-generated, object-owned,
target-specific metadata. Their layouts, pointers, slots, symbols, and ABI are
not public, stable, or available to FFI. Separate compilation exchanges
nominal method and dispatch metadata rather than raw table layouts. Nullable
receivers must be flow-refined before dispatch under existing rules. Multiple
class inheritance, default interface methods, reflection, dynamic loading,
runtime type tests and downcasts, FFI tables, and new reference/move/lifetime
syntax remain deferred.

## ADR-0071: `func` Function Keyword

`func` is the only accepted source keyword for top-level functions, class
methods, and interface methods. It is reserved in identifier positions. The
historical `fun` spelling is lexically recognized only to produce the
source-mapped `ObsoleteFunctionKeyword` diagnostic; it is not a compatibility
alias and never declares a function. This spelling change does not alter
function typing, ownership, dispatch, constructors, ABI, symbols, or runtime
behavior. Active examples and fixtures use `func`; historical ADR text retains
its original wording as evidence.

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

The initial implementation is host-only. It emits a Cranelift object for the
host triple and links it with the host system linker. Non-host targets are
explicitly rejected. Cross compilation, bundled linkers, and target-specific
startup artifacts require a future accepted portability and ABI decision.

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
represented by the empty package path. Imports remain syntax-only and
do not create module dependencies.

Visibility categories are `public`, `internal`, and `private`. Default visibility is `internal`. `public` means visible to other modules subject to
later dependency and name resolution rules. `internal` means visible within the
same module. `private` means visible only within the declaring source file.
Package and import declarations have no visibility metadata.

Each declaration with visibility scope has exactly one effective visibility
category and records whether that category was explicit or defaulted.

Module metadata includes module name, ordered source file identities, package namespace per source file, and effective visibility metadata. It does
not include module dependencies, target triples, package manager metadata,
manifest paths, artifact hashes, resolved symbols, or imported names.

ADR-0025 defines required diagnostics for missing module identity, invalid
module identity, ambiguous source-module assignment, invalid package namespace,
unsupported visibility categories, and duplicate visibility metadata.

## ADR-0026: Name Resolution Policy

The compiler resolves a bootstrap subset using local lexical scope plus same-module package top-level declarations. Included name references are simple identifier expressions, package-qualified name expressions, and type name nodes in accepted declaration, local binding, or explicit annotation positions.

Function declaration names, type declaration names, local `val`, `const`, and
`var` statements introduce names. Function parameters, pattern
bindings, import aliases, member declarations, and fields remain excluded from
Name binding follows ADR-0029 only in its immutable-local spelling; binding
identity, scope, declaration order, lookup, shadowing,
duplicate, and ambiguity rules remain unchanged.

Declaration bodies and block expressions introduce lexical scopes. Top-level declarations in the same module and package namespace are visible throughout that module/package namespace regardless of source-file order. Local bindings are not visible before their declaration statement and remain visible through the end of their containing lexical block unless shadowed.

Unqualified lookup searches innermost local lexical scope outward, then the current source file's package namespace in the current module. Package-qualified lookup uses the explicitly named package namespace in the current module only. Imports remain syntax-only and do not add lookup candidates.

Duplicate local bindings in the same scope and duplicate top-level declarations with the same module, package namespace, declaration name, and declaration kind are rejected. Ambiguous lookup reports ambiguity instead of choosing by insertion order, source-file order, or parser traversal order.

The compiler records same-module visibility metadata but does not enforce visibility. Cross-module lookup, member lookup, overload resolution, extension lookup, and type-directed lookup remain unsupported.

Resolution diagnostics include `unresolved_name`, `duplicate_name`, `ambiguous_name`, `unsupported_import_resolution`, `unsupported_cross_module_lookup`, and `unsupported_member_resolution`. Each diagnostic follows ADR-0015 and ADR-0026 primary span, recovery action, source-of-truth citation, and safe suggestion policy requirements.

## ADR-0027: Type Checking Core

The compiler defines a small bootstrap type checker with primitive type-checking identities, literal typing, resolved name expression typing, explicit nullable wrappers, and exact assignment compatibility.

Typed output is side-table metadata: an expression type table, declaration signature table, assignment check table, and diagnostics list. The type checker does not rewrite the AST.

Primitive identities `Bool`, `Int`, `String`, `Unit`, and `Null` are type-checking
identities. `String` receives the compiler-owned opaque runtime representation
defined by ADR-0064; it has no stable public layout or FFI meaning.

Assignment compatibility is exact type identity, except `Null` is assignment-compatible only with nullable target types and non-null base values are assignment-compatible with their nullable wrapper.

Direct function declaration calls and structural function type application are deferred for this implementation. Overload resolution, implicit numeric conversion, member lookup, generic constraint solving, ownership and move analysis, borrow checking, HIR lowering, MIR lowering, and backend behavior remain deferred.

Type checking diagnostics include `type_mismatch`, `unresolved_type_rule`, `unsupported_type_rule`, and `ambiguous_type_rule`. Diagnostics define primary spans, recovery actions, source-of-truth citations, safe suggestion policies, and stable rule identifiers where required.

## ADR-0028: Nullability And Flow Typing

The compiler defines a narrow nullability and flow-typing subset for local immutable null refinements.

Null-test recognition is a flow-specific condition recognizer for direct comparisons between one simple local name expression and `null`; it does not require general binary expression type checking, overload resolution, user-defined equality, implicit conversion, or Boolean operator typing.

Refinements apply only to eligible immutable local bindings with known nullable wrapper types. `x != null` and `null != x` refine inside the then branch block. `x == null` and `null == x` refine inside the else branch block when an else branch exists. Refinements start at the first statement or optional trailing expression in the refined branch and end at that branch's closing brace.

Refined output remains side-table metadata. Declaration signatures and original local binding types preserve the original nullable type; refined expression type entries are per-use views inside guarded regions.

The compiler diagnoses nullable use where a nullable expression is required to be non-null without an active refinement. A simple, unqualified name expression resolving to the same eligible immutable local whose refinement ended at the closing brace of its guarded `if` branch is a region-exit use only when it occurs in a later statement or trailing expression directly contained by that `if`'s enclosing block and is required to have base type `T`. That exact use reports `invalidated_refinement` with stable rule identifier `region_exit_invalidated_refinement`; its primary span is the later name expression, it has no secondary span, and recovery treats it as its original `T?`. Where it matches an annotated-local initializer shape, this region-exit mapping takes precedence over ADR-0030. Otherwise, for an annotated local whose initializer is exactly a bare resolved name of type `T?` and whose annotation is its base type `T`, `invalid_nullable_use` uses the stable rule identifier `nullable_assignment_without_refinement`; ADR-0027 keeps the initializer expression as the primary span. This mapping is limited to that case. Flow diagnostics include `invalid_nullable_use`, `invalidated_refinement`, `unsupported_flow_rule`, and `ambiguous_flow_rule`.

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

The compiler represents generic parameter identity and explicit capability-bound
occurrences, but does not enforce a bound. The language still selects
constrained nominal generics; however, capability identity or resolution,
satisfaction rules, generic substitution, and bound-violation diagnostics are
deferred until ownership and thread-capability semantics provide their required
inputs. The implementation depends only on the representation boundary. A later
later implementation must introduce enforcement through a separate accepted
semantic decision.

## ADR-0033: Bootstrap Sealed Sums And Exhaustive Match

The compiler accepts closed no-payload `enum` variants and expression-level `when`.
Enum variants are identifier-only and scoped to their declaring module/package.
`when (subject) { Enum.Variant -> expression; _ -> expression; }` uses only
qualified variant patterns or a wildcard. A match is exhaustive only when it
covers each declared variant once or has exactly one wildcard. Duplicate,
unknown, and missing variants, duplicate wildcards, and non-enum subjects
diagnose under ADR-0033; payloads, destructuring, generic enums, nullable
coverage, implicit smart casts, and arm-result type unification remain deferred.

## ADR-0034: Bootstrap Enum Subject Typing

For functions with bodies, parameters use `identifier : named-type` entries in
a comma-separated parameter list. The compiler accepts an ADR-0033 `when` subject
only as a bare reference to one such parameter when its named annotation
resolves in the declaring module/package to exactly one bootstrap enum. Other
subject shapes, unresolved types, and non-enum types report
`invalid_match_subject` on the subject. Parameters are immutable local bindings
visible throughout their function body; general parameter typing, enum value
expressions, constructors, member lookup, nullable/generic parameters, calls,
and cross-module lookup remain deferred.

## ADR-0035: Bootstrap Ownership And Move Analysis

The compiler classifies `Bool`, `Int`, `Unit`, and `Null` as copyable primitive
identities, `String` as move-only, and all current-module user-defined nominal
identities, including bootstrap enums, as move-only. Explicitly copyable
user-defined types remain deferred.

Only a local `val`, `const`, or `var` initializer, or an assignment statement, whose
value is a bare resolved local name of move-only type is an ownership
transfer site. A later bare local-name expression using that moved binding
reports `use_after_move` on the later use with the transfer expression as the
move-origin secondary span. Copyable values do not enter the moved state.

The compiler records ownership facts in side tables and does not rewrite the AST or
lower to HIR. Calls, returns, captures, `when` subject evaluation, branch move
joins, destructuring, member or partial moves, destructors, borrowing,
coroutine frames, FFI, layout, cloning, generic copyability, and user-declared
copy remain deferred.

## ADR-0036: Bootstrap Borrow And Lifetime Analysis

The compiler uses a metadata-only bootstrap borrow model. It adds no source-level
borrow syntax, reference types, dereference operators, function parameter
borrowing, method receivers, member borrows, closure captures, coroutine
borrows, unsafe references, or FFI references.

Borrow input records contain a borrow node, borrowed local binding, borrow kind
(`shared` or `exclusive`), and region node. Shared borrows may overlap other
shared borrows of the same local in the same region. An exclusive borrow
conflicts with any other shared or exclusive borrow of the same local in the
same region. Overlap is exact region-node equality only; nested, sibling,
non-lexical, loop, path-sensitive, and control-flow-sensitive overlap rules are
deferred.

Lifetime escape input records contain an escape node, borrowed local binding,
borrow node, borrow region, and use region. A `lifetime_escape` diagnostic is
reported when the use region differs from the borrow region. `borrow_conflict`
diagnoses on the later conflicting borrow with the earlier borrow as secondary
span. `lifetime_escape` diagnoses on the escape node with the original borrow
as secondary span.

## ADR-0037: Bootstrap Thread Capability Analysis

The compiler uses a metadata-only bootstrap thread-capability model. It adds no
source-level task spawning, detached threads, async blocks, coroutine bodies,
closures, synchronization primitives, atomics, locks, generic capability
enforcement, user-declared capability implementations, or unsafe capability
overrides.

The compiler defines `Send` for values that may transfer across an approved concurrent
boundary and `Share` for values that may be shared across an approved
concurrent boundary without exclusive transfer. `Bool`, `Int`, `Unit`, and
`Null` satisfy both capabilities. `String` satisfies `Send` but not `Share`.
Nullable types satisfy a capability only when their non-null base type satisfies
that capability. Current-module nominal user-defined types, generic parameter
types, unsupported types, unresolved types, and absent type information satisfy
neither capability in this implementation.

Boundary input records contain a boundary node and ordered capture records.
Capture records contain a capture node, captured local binding, captured type,
and required capability. A `missing_thread_capability` diagnostic is reported
when a capture's type does not satisfy the required capability. The capture node
is primary and the boundary node is secondary. Because the compiler has no approved
synchronization abstractions, shared mutable state is not accepted through a
`Share` capture; mutable captures may only be modeled as `Send` transfers when
the type satisfies `Send`.

## ADR-0038: Bootstrap Coroutine Scope And Suspension Analysis

The compiler uses a metadata-only bootstrap coroutine scope and suspension model. It
adds no source-level coroutine, `async`, `await`, task-spawn, detached-task,
cancellation, pinned-frame, closure, channel, synchronization, or scheduler
syntax. Existing unsupported concurrency-like source forms remain rejected or
unsupported.

Structured scope input records contain a scope node and ordered child-task
records. Child-task records contain a task node, the containing scope node, and
the scope node in which the child is proven to complete or be cancelled. A child
task is valid only when its completion-or-cancellation scope is the same scope
as its containing structured scope. Other task escape rules are deferred.

The compiler reports `task_scope_escape` when a child task is not proven to complete or
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

The compiler reports `borrow_across_suspension` when a suspended borrow may be
concurrently accessed, may outlive the borrowed value, or both. The suspension
node is primary, the borrow node is secondary, and the diagnostic identifies the
borrowed local and rejection reason.

Cancellation resource-safety in this implementation is limited to the structured-scope
completion-or-cancellation check. Runtime cancellation propagation, destructor
execution during cancellation, cancellation handlers, cancellation masking, and
async drop are deferred. ADR-0037 remains the authority for any supplied
thread-capability capture records.

## ADR-0039: Bootstrap Unsafe FFI Boundary Analysis

The compiler uses a metadata-only bootstrap unsafe and FFI boundary model. It adds no
source-level unsafe block, unsafe function, extern block, foreign declaration,
ABI string, link attribute, target attribute, safe-wrapper syntax, or module
audit syntax. Existing unsupported unsafe-like and FFI-like source forms remain
rejected or unsupported.

The compiler defines `ProvenSafe` for operations proven safe by accepted compiler
analyses and `TrustedUnsafe` for operations relying on explicit programmer or
binding assertions. Unsafe context records contain a context node and context
kind: `block`, `function`, or `module_audit`. Unsafe operation records contain
an operation node, operation kind, safety basis, and optional containing unsafe
context node.

A `ProvenSafe` operation is accepted without an unsafe context. A
`TrustedUnsafe` operation is accepted only when its containing context node
matches a supplied unsafe context record. The compiler reports
`unsafe_operation_outside_context` when a trusted unsafe operation has no
matching unsafe context. The operation node is primary. A non-matching supplied
context node is secondary when present. The diagnostic identifies trusted
assertion rather than compiler-proven safety.

FFI declaration records contain a declaration node, declaration kind, safety
basis, safe-wrapper status, and metadata presence for target contract, calling
convention, nullability, ownership transfer, lifetime validity, and
thread-safety or send/share guarantees. The compiler validates metadata presence only;
target triples, layout, calling convention compatibility, symbols, linker
inputs, generated bindings, dynamic loading, platform APIs, and ABI lowering
are deferred.

The compiler reports `missing_ffi_safety_metadata` when an FFI declaration lacks one
or more required metadata categories. The FFI declaration node is primary, and
the diagnostic lists the missing categories. Safe-wrapper status is metadata
only in this implementation and does not affect source visibility, type checking, or call
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
cross-target APIs remain deferred. Unsupported parsed forms must fail before
backend lowering with
`unsupported_executable_form` or a more specific existing diagnostic. The compiler
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
Non-host targets are rejected before lowering. `Int` lowers to a signed 64-bit
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
only. Non-host object formats and cross compilation are deferred.

The first runnable program requires no Neu standard library. Host `cc` links the
Cranelift object, and `NEU_LINKER` may override that command. The compiler-owned
runtime boundary handles the accepted entry and trap contracts without exposing
printing, allocation, CLI arguments, scheduling, panics as a language feature,
exceptions, FFI helpers, or platform APIs. There is no bundled startup object.

## ADR-0056: Bootstrap Function Symbol Identity

HIR and MIR preserve structured bootstrap function identity containing the
accepted module identity, package namespace, and source function name. The
identity is carried from parsed declaration metadata through HIR and MIR to
object emission. Missing identity is an explicit lowering failure; a numeric
MIR function ID is not a substitute. The backend derives a deterministic,
collision-free internal object symbol from those components. Exact escaping
and encoding are compiler implementation details and do not define a public
ABI or new language semantics.

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

## ADR-0092: Array Element Iteration

`for (value in array)` and the equivalent unparenthesized header iterate fixed
`T[N]` and compiler-managed `Array<T>` values exactly once, in ascending index
order. Empty arrays execute no iterations. The loop binding is an immutable
compiler-inferred read-only binding: copyable elements are copied, while
move-only elements are implicitly shared-borrowed and do not consume the array.
The binding cannot be assigned, and the array remains usable after the loop.

Indexed writes continue to use existing `var`, ownership, and projection rules;
structural dynamic-array mutation during iteration is rejected. `break` and
`continue` retain ADR-0060 behavior. HIR and MIR preserve the target,
fixed/dynamic identity, element projection, ownership, cleanup, and loop
back-edge facts. Iterator objects, slices, views, generators, custom steps,
labels, and structural mutation during iteration remain deferred.

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
host-linking-managed heap storage, but source code cannot request or observe
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
interface tables are compiler-private host-linking contracts. Separate
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
field. The compiler records nominal and field metadata and type-checks projections
only where an accepted receiver context exists. `new`, constructor calls,
allocation, initialization, and runtime object access are deferred to later
lifecycle work.

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

## ADR-0072: Fixed-Array Type Suffixes

Fixed-size arrays use `T[N]`; the historical `[T; N]` spelling is rejected.
Lengths retain ADR-0063's literal and named compile-time `const Int` rules.
Nested suffixes are written outer-to-inner, so `Int[2][3]` is two arrays of
three integers each. Array literals, expression indexing, ownership, inline
representation, parameter/return transport, and dynamic-array deferrals are
unchanged. The old spelling is retained only in historical ADR text.

## ADR-0073: Bootstrap Dynamic Arrays

`Array<T>` is a compiler-managed dynamic array distinct from fixed `T[N]`.
`new T[]` creates an empty array when `T` is determined by the declared type.
The bootstrap scalar element set is `Bool`, `Int`, `Float`, and `Byte`.
Mutable `var` bindings support `add`, indexed `add`, `remove`, and `size`; `val`
rejects mutation. Dynamic arrays are move-only, opaque, compiler-managed
values with host-linking allocation and deterministic traps. Strings, nominal
elements, nested dynamic arrays, indexing, slices, iterators, public layout,
FFI, and user allocation APIs remain deferred.

## ADR-0074: Nominal Fixed-Array Elements

Fixed inline arrays may contain accepted class and interface values, including
nested fixed arrays. They are structural, move-only aggregates with left-to-
right initialization, reverse destruction, existing indexed ownership rules,
and existing class/interface dispatch. Their compiler-private inline ABI is not
public or available to FFI. Nullable nominal elements, dynamic-array nominal
elements, slices, and generic variance remain deferred.

## ADR-0075: Bootstrap Value ABI Extension

The internal same-module ABI transports accepted primitive, string, fixed-array,
class, interface, and scalar dynamic-array values with typed signatures.
Dynamic scalar arrays use opaque compiler-managed pointers; move-only ownership
and return cleanup remain enforced. Dynamic arrays containing strings, nominal
values, fixed arrays, nested dynamic arrays, or nullable values are rejected
before lowering until their element ABI and destruction contracts are accepted.

## ADR-0076: Function And Method Overloading

Same-module top-level functions, class methods, and interface methods may form
overload sets. Identity is owner, name, and ordered parameter type identity;
return type is not part of identity. Exact matches are preferred, already
accepted assignment compatibility may be used only for one unique candidate,
and numeric conversions, defaults, named arguments, varargs, generic overloads,
and cross-module lookup remain deferred. Ambiguous and missing matches are
source-mapped diagnostics. Overload selection is compile-time; the selected
method identity then preserves direct, virtual, interface, or static-super
dispatch and existing ownership/effect facts through HIR, MIR, Cranelift, and
the compiler-private ABI. Constructors remain single-primary-constructor only.

## ADR-0077: Value-Producing Conditional Expressions

An `if` in expression position requires an exact `Bool` condition and an
`else` branch. Exactly one branch executes, and every reachable non-terminating
branch must produce the same exact result type; `Unit` follows the same rule.
No truthiness, common-supertype inference, numeric conversion, or nullable
widening is added. Statement conditionals remain unchanged. Branch-local flow,
ownership, initialization, cleanup, source mappings, and CFG joins are
preserved through HIR, MIR, Cranelift, and the existing private ABI. Missing
results, invalid conditions, incompatible branches, and branch-dependent
consumption are diagnosed before lowering.

## ADR-0079: Zero-Payload Enum Values And Typed Transport

Enums are closed nominal types. Bare variants are declared in deterministic
order and are constructed with `EnumName.VariantName`. The expression has the
enclosing enum type and transports through same-module function and method
parameters and returns using a compiler-private typed scalar tag. The tag,
layout, symbols, and representation are not public ABI or FFI contracts.

The initial runtime slice accepts zero-payload variants only. Duplicate
variants are diagnostics. Zero-payload values are immutable copyable tags;
payload variants, fields, argument-bearing constructors, enum-associated
functions, inheritance, generic enums, reflection, serialization, and FFI are
deferred. HIR, MIR, Cranelift, object emission, and linking preserve nominal
enum and variant identity and source mappings without exposing layout.

## ADR-0081: Payload-Bearing Enums And Enum Functions

Enums may declare immutable payload fields in the header and variants must
provide exact, left-to-right constructor arguments. Payload patterns bind one
name per field, are scoped to their arm, and require exact arity. Existing
ownership and reverse destruction rules apply. Instance `func` declarations
inside an enum use an implicit non-consuming `this`; `static func` declarations
are associated functions without a receiver. Enum functions are final and use
the existing overload, effect, capability, and ABI rules.

Payload and enum-function representations remain compiler-private tagged
aggregates with semantic HIR/MIR/source mappings and no public layout or FFI
contract. Generic enum behavior waits for the generic declaration and
constraint decisions. Inheritance, reflection, serialization, dynamic
loading, FFI, implicit conversions, and user allocation remain deferred.

## ADR-0093: Static Class Functions

Classes may declare associated functions with `static func`. They have no
implicit receiver and are callable through a nominal class name only. Static
functions cannot access `this`, `super`, instance fields, or instance methods;
interfaces do not declare them. They use existing visibility, ownership,
capability, generic, overload, and value ABI rules without a receiver
parameter.

Derived classes inherit an unhidden static function for class-name lookup, but
a same-named declaration hides it and is not an override. `override` is
rejected for static functions; `final` is accepted as redundant metadata.
Static calls preserve class and function identity as direct calls through HIR,
MIR, Cranelift, object emission, and linking. Class objects, static fields,
reflection, dynamic lookup, public symbol/layout ABI, and FFI remain deferred.

## ADR-0094: Abstract Classes And Functions

`abstract class` declares a non-instantiable class. It may contain concrete
fields, the primary constructor, concrete methods, and declaration-only
`abstract func` instance methods. Abstract classes may inherit and implement
interfaces, and may remain abstract through further subclasses. A concrete
class is constructible only when every inherited abstract method identity has a
compatible concrete `override`; incomplete classes and abstract construction
are diagnosed before HIR lowering.

Abstract methods have no bodies, no static form, and retain ordinary receiver,
ownership, capability, overload, visibility, and dispatch rules. Abstract
static methods and `abstract final` are rejected. Concrete completions preserve
virtual/interface identity through HIR, MIR, Cranelift, object emission, and
linking. Public abstract-object layouts, reflection, dynamic loading, and FFI
remain deferred.

## ADR-0080: Zero-Payload Enum `when` Patterns

`when` supports qualified zero-payload enum variant patterns and `_`. The
subject is evaluated once, arms are considered in source order, and only the
selected arm executes. Statement arms need not produce values. Expression-form
`when` requires exhaustive coverage and exact result type equality across all
reachable arms; a wildcard covers remaining variants. Duplicate and
unreachable arms are diagnostics. Payload, destructuring, guards, nullable
matching, and generic patterns remain deferred.

## ADR-0078: Optional Semicolons And Newline Termination

Neu preserves line-boundary metadata on lexer tokens. Ordinary declarations,
assignments, expression statements, `return`, `break`, and `continue` may end
at a newline when the preceding token completes a statement and the next token
does not continue the expression. Explicit semicolons remain valid and may
separate multiple same-line statements. Closing braces and end of input also
terminate completed statements.

Newlines after operators, dots, commas, opening delimiters, and other accepted
continuation points do not terminate expressions. Parentheses and brackets
preserve their existing list and expression structure. `else` attaches to the
preceding `if` across line breaks. A `return` expression must begin on the same
logical line as `return`; a newline immediately after it is a bare return.
Comments and blank lines preserve line boundaries, while newlines in strings
remain rejected. Recovery uses line boundaries, semicolons, braces, and
declaration starters. This syntax decision changes no type, ownership, HIR,
MIR, ABI, backend, or runtime semantics.

## ADR-0082: Generic Type Identity And Substitution

Generic arguments are explicit type-only arguments with exact arity and
invariant identity. A generic parameter is identified by its declaring generic
declaration and parameter node, not by its spelling. A generic instance is
identified by the nominal declaration and ordered substituted type identities;
nested generic, nullable, fixed-array, dynamic-array, and accepted nominal
arguments retain their structural identities.

The compiler performs explicit recursive substitution through accepted type
constructors. It does not infer arguments, add conversions, support wildcards
or variance, solve constraints, erase types, specialize code, or define a
runtime generic layout. Generic metadata is source-mapped and compiler-private;
generic declarations, constraints, specialization, and cross-module lookup
remain governed by later accepted decisions.

## ADR-0084: Generic Constraints And Capabilities

Generic bounds are explicit conjunctions checked in source order after
substitution and before HIR lowering. `Send` and `Share` use the existing
thread-capability rules; ownership, nullability, and nominal class/interface
bounds retain their existing contracts. A concrete argument that fails any
bound is diagnosed with source mapping, while an uninstantiated parameter
retains a proof obligation rather than being guessed as valid.

Unknown, unresolved, recursive, and stale bounds are diagnostics. Constraints
do not change overload identity and do not add inference, conversions,
variance, wildcards, higher-kinded types, or public ABI behavior.

## ADR-0085: Generic Specialization And Private ABI

Concrete generic calls use explicit type arguments before their argument list.
Specialization identity is the generic declaration plus ordered concrete type
identities. Equal identities are deduplicated, recursive expansion is rejected,
and specialization occurs after constraint validation before MIR lowering.

Substituted HIR/MIR preserves ownership, cleanup, aggregate, dispatch, and
source facts. Generated symbols and layouts are compiler-private; unsupported
bootstrap or host-linking representations are diagnosed before object emission.
Erasure, inference, reflection, public generic ABI, and separate-compilation
caches remain deferred.

## ADR-0086: Top-Level Function Values

Function types use structural `(T1, T2) -> R` identity. Named top-level
functions with one unambiguous compatible signature may become non-null,
non-capturing opaque compiler-managed values; they are copyable and satisfy
`Send` and `Share`. Function values may be stored, passed, returned, and
applied through compiler-private indirect-call facts. Overload ambiguity,
closures, captures, bound methods, nullable function values, equality, public
function-pointer layout, and FFI remain deferred.

## ADR-0087: Higher-Order Calls

Indirect calls require an exact non-null function type. The callee evaluates
before left-to-right arguments, and the referenced function's parameter,
return, ownership, and effect contract is preserved. HIR/MIR distinguish
function references and indirect calls from direct calls; Cranelift uses a
compiler-private host-linking signature and address. No implicit conversions,
nullable function values, callback boundary, public pointer layout, or FFI
representation is added.

## ADR-0088: Owned Lambdas And Closures

Lambda expressions use Kotlin-like `{ parameter: Type -> expression }` syntax.
Captures are inferred; copyable values copy, move-only values transfer, and
mutable captures require an exclusive inferred effect. Borrowed captures may
not outlive their source or cross a transfer/suspension boundary. Closure
identity and environment layout are compiler-private, values are immutable and
non-comparable, and cleanup/capture facts remain source-mapped through HIR and
MIR. Public closure ABI, FFI, reflection, detached execution, and coroutine
suspension remain deferred.

## ADR-0089: Concurrent Closure Captures

Moved or mutable closure captures crossing an approved thread or structured-task
boundary require `Send`; shared captures require `Share`. Borrowed captures and
mutable shared captures are rejected at the boundary. Existing task-scope,
borrow-suspension, capability, and compiler-managed cleanup rules apply through
completion and cancellation. No detached task, scheduler API, public closure
layout, FFI ABI, or explicit transfer syntax is added.

## ADR-0090: Implicit Local Type Inference

Local `val` and `var` declarations may omit an annotation only when an
initializer provides an already-resolved exact type. Inference is local-only,
does not widen concrete classes or perform numeric conversion, and rejects a
bare `null`. Inferred `var` bindings retain their type and use the existing
assignment, ownership, nullability, capability, dispatch, cleanup, HIR, MIR,
and ABI rules. Parameters, returns, fields, constructors, and public
signatures remain explicitly typed.

## ADR-0091: Optional Control-Header Parentheses

`if`, `for`, and `when` headers may omit their surrounding parentheses while
retaining the parenthesized forms. The opening body or arm brace terminates an
unparenthesized header. Existing range and array iteration, `when` matching,
grouping, multiline, optional-semicolon, ownership, cleanup, HIR, MIR, and
backend behavior are unchanged; ambiguous or malformed headers remain
diagnostics.

## ADR-0083: Generic Declaration Environments

Top-level functions, classes, interfaces, enums, and accepted members may
declare explicit generic parameters. A member environment is its enclosing
type parameters followed by its own parameters. Duplicate names in one list
are rejected; member-level shadowing is allowed because parameter identity is
the declaration node, not the spelling.

Generic fields, constructors, parameters, returns, enum payloads, and member
signatures preserve visible parameter identity and source spans. Generic
inheritance and interface implementation use exact invariant substituted
arguments. No inference, default arguments, variance, implicit conversion,
cross-module lookup, or executable lowering of unspecialized declarations is
added; specialization is a later accepted decision.

## ADR-0095: Directory Packages And Import Aliases

Imports resolve normalized directories, never individual `.neu` files. The
canonical syntax is `import "./relative/directory"`, optionally followed by
`as alias`. Every `.neu` file directly in that directory belongs to the package;
subdirectories are separate packages. Files in the current package share an
unqualified namespace, while imported declarations are reached through the
alias or deterministic package qualifier.

Package headers are optional for bootstrap. Present headers must agree across
all direct files; omitted headers use deterministic normalized-directory
identity. Relative imports resolve from the importing file within a virtual
project source root and may not escape it. Missing or empty packages, file
imports, malformed or duplicate aliases, header disagreement, duplicate
package identity, ambiguous qualified names, and cycles are diagnostics.

The driver accepts normalized virtual paths paired with raw source strings and
assigns deterministic file IDs. Paths, package identities, and source mappings
remain attached through name resolution, ownership, HIR, MIR, object emission,
and linking. No manifest, registry, dependency download, wildcard import,
re-export, precompiled package format, public package ABI, or cyclic package
semantics is introduced. Visibility remains governed by ADR-0025 and its
follow-up decision.

## ADR-0096: Public, Private, And Protected Visibility

`internal` is not an accepted source modifier and produces a declaration
diagnostic. The accepted modifiers are `public`, `private`, and `protected`;
omitted visibility defaults to `public`.

Top-level private declarations are file-scoped. Top-level protected
declarations are invalid because there is no enclosing type. Public
declarations are importable through directory-package aliases. For class
members, private is declaring-class scoped, protected is available to the
declaring class and subclasses, and public follows the visible declaring type.
Interfaces may declare public or private members, but not protected members.
Constructors, fields, methods, static functions, enum functions, and accepted
type members use these rules in their declaring context. Overrides retain the
inherited visibility; private members are not inherited override targets.

Visibility is enforced during lookup and is preserved as compiler-private
declaration provenance through ownership/effect facts, HIR, MIR, dispatch,
object emission, and linking. It does not alter symbol naming, slots, layout,
calling convention, object format, or ABI. Aliases do not bypass access checks,
and missing or stale separate-compilation visibility metadata is an error.
Friend packages, re-exports, wildcard imports, reflection, dynamic loading,
FFI visibility, and package registries remain deferred.

## ADR-0097: Neu Project Manifest

`neu.json` is rooted at the project directory and contains required `name`,
`entrypoint`, and `srcs` fields plus optional `description` and `dependencies`.
The name is the module identity under existing dotted-identifier rules;
description is metadata only. Unknown fields, duplicate keys, malformed JSON,
wrong types, and invalid paths are diagnostics.

The entrypoint is a normalized relative `.neu` path and must be included in the
authoritative `srcs` allowlist. Source patterns support `*`, `**`, and `?`, use
slash separators, match `.neu` files, exclude hidden components, reject
absolute paths and `..` escapes, reject symlink escapes, sort normalized paths,
deduplicate matches, and diagnose empty matches. The selected files feed the
directory-package graph from ADR-0095.

Dependency descriptors default to Git and contain a URL and non-empty tag;
other dependency types are rejected. Manifest validation does not fetch Git or
create lockfiles; recursive cache and lockfile behavior is reserved for the
dependency-resolution decision. Raw-source driver APIs remain available.

## ADR-0098: Import Qualifier Collisions

After path normalization and package-header resolution, an explicit `as` alias
is the local qualifier; otherwise the resolved package identity is used.
Distinct packages may not receive the same qualifier in one source file.
Equivalent paths to one package are deduplicated. Explicit aliases must be
unique and cannot shadow another import, the current package qualifier, or a
local declaration. Such failures are `import_qualifier_collision` diagnostics
with both import origins and an alias suggestion, before qualified lookup or
later compiler stages.

## ADR-0099: Git Dependency Resolution And Lockfiles

Neu accepts only `https://` Git dependencies with non-empty tags. The cache is
`NEU_PATH` or `$HOME/.neu`, under `pkg/<host>/<owner>/<repository>`; the cache
path never becomes module identity. A repository must contain root `neu.json`,
whose `name` supplies the dependency module identity. Tags resolve to full
commit hashes and builds use detached cached checkouts without executing hooks
or build scripts.

Dependencies resolve recursively and deterministically. Cycles, duplicate
module identities, conflicting URLs/tags, unsupported schemes/types, missing
manifests, submodules, and symlink escapes are diagnostics. Projects with Git
dependencies create or update `neu.lock.json` atomically after complete
resolution; entries are sorted and record module, URL, type, requested tag, and
resolved commit. Locked builds reject moved tags and offline misses. Registries,
archives, binaries, branches, features, and automatic updates remain deferred.

## ADR-0100: Host-Only System Linking

The initial compiler emits Cranelift object code for the host triple and links
it with the host system C linker, `cc`. `NEU_LINKER` may override the linker
command. Non-host target requests are rejected explicitly before frontend or
backend compilation. No bundled linker, startup object, target registry, or
foreign-target executable contract is provided. Compiler-owned runtime support
remains private and does not define a public allocation, object-layout, or FFI
ABI. `main(): Int` remains the executable entry contract.

## ADR-0101: Structured Concurrency Runtime Semantics

Neu provides cooperative, single-threaded structured concurrency through
compiler-owned `suspend func`, `scope { ... }`, `spawn { ... }`, and
`await(task)` forms. `spawn` is valid only inside an active lexical scope and
returns an opaque move-only `Task<T>`. `await` consumes the handle once and
returns its owned result. Scope exit waits for all registered children and
performs cleanup; child failure cancels unfinished siblings and reports the
first failure in deterministic creation order. The scheduler is FIFO at
accepted suspension points and does not imply OS threads or parallel execution.

Ownership and capability checks apply to captures: moved captures require
`Send`, shared captures require `Share`, and borrowed or mutable-shared captures
are rejected. Borrows may cross suspension only under the proof already
defined by ADR-0009. HIR and MIR retain scope, task, suspension, result,
ownership, cleanup, and source-mapping facts. Task handles, frames, and runtime
symbols have no public layout or FFI ABI. Explicit cancellation, channels,
timers, I/O, detached tasks, OS threads, parallel execution, and concurrency
standard-library APIs remain deferred.

`Task<T>` and `Channel<T>` may appear in public Neu interface method signatures
when their source-level ownership and capability contracts are satisfied. Their
runtime handles, frames, queue state, dispatch metadata, and layouts remain
compiler-private and are never part of a public ABI or FFI contract.

## ADR-0104: Member Task Cancellation

`task.cancel()` requests idempotent cancellation of an owned child task without
forceful termination. The receiver must be an owned `Task<T>` and the call has
no arguments. The former free `cancel(task)` spelling is rejected. Cancellation
is observed at accepted suspension and
cleanup points, propagates from scopes to unfinished descendants, and runs
ownership cleanup before completion. A cancelled task may be awaited once to
observe its deterministic cancellation result. Cancellation does not introduce
tokens, callbacks, timeouts, OS signals, or public runtime types.

## ADR-0103: Bounded Channel Contract

`channel<T>(capacity)` creates an opaque bounded channel. `send`, `receive`, and
`close` provide FIFO message transfer, suspension while full or empty, and
idempotent closure. `receive` returns the compiler-provided
`ChannelResult<T>` type defined by ADR-0105: `ChannelResult.Message(value)` for
a queued message and `ChannelResult.Closed` after closure and draining. This
keeps end-of-stream distinct from every message value, including nullable or
otherwise empty values. Capacity zero is a
rendezvous channel. Multiple senders and one logical receiver are supported;
concurrent receiver use is diagnosed. Sent values must satisfy `Send`, and
move-only values are consumed when a send completes. Channel identity, element
type, capacity, ownership, suspension, closure, cancellation, cleanup, and
source spans remain in HIR/MIR without a public layout or FFI ABI.

## ADR-0105: Channel Receive Result

`ChannelResult<T>` is a compiler-provided nominal result with exactly the
payload-bearing `Message(T)` and zero-payload `Closed` variants. Existing
payload-bearing `when` matching from ADR-0081 inspects the result; no new
pattern, nullable-EOF convention, public result library, or allocation syntax
is introduced. Ownership follows `T`: receiving a move-only message transfers
ownership through the `Message` payload, while copyable messages follow normal
copy rules. The result representation, channel state, and runtime operations
remain compiler-private with no stable ABI or FFI layout.

## ADR-0106: Shared Channel Handles

`Channel<T>` is a copyable compiler-owned handle to shared channel state. It
may be captured by multiple structured tasks and is not consumed by `send`,
`receive`, or `close`; message ownership still follows ADR-0103 and ADR-0062.
Channel handles satisfy the accepted `Send` and `Share` capability checks.
Private state lifetime, queue storage, and cleanup are compiler/runtime facts,
not source-visible layout or allocation APIs.

## Proposed ADR-0109: Generic Calls And Enum Runtime

Generic calls use explicit type arguments in declaration order. The argument
count must match exactly; generic parameters are invariant and nominal, and no
inference, variance conversion, wildcard argument, or implicit conversion is
performed. The compiler substitutes concrete arguments through parameter,
return, and capability-bound types before lowering and diagnoses an unsatisfied
or unsupported bound at the call site.

`Option<T>`, `Result<T, E>`, and `Ordering` are closed nominal enums. Their
variants are `Some(T)`/`None`, `Ok(T)`/`Err(E)`, and `Less`/`Equal`/`Greater`.
Instantiation identity is the declaring enum plus ordered concrete arguments.
Representation, discriminant encoding, allocation, and cross-module generic
ABI remain compiler-private and are not source or FFI contracts.

## Proposed ADR-0111: Recoverable Errors And Panic Boundaries

Expected operation failures are explicit `Result<T, E>` values, or `Option<T>`
when absence is the complete outcome. They do not trap, return nullable
sentinels, or use undocumented integer codes. Error propagation is explicit;
there is no implicit propagation operator, exception handler, or conversion
between error types without an accepted library operation. Panic and abort are
reserved for unrecoverable invariant faults, not checked indexing, parsing, or
conversion failures. Assertion and panic helper APIs require separate
evaluation, diagnostic, source-span, optimization, and termination contracts.

## Proposed ADR-0112: Explicit Numeric Utilities

The initial numeric utility surface is limited to `Int` helpers `min`, `max`,
`clamp`, `abs`, and `sign`. They perform no implicit conversion and preserve
ADR-0043 evaluation and overflow behavior; `abs(Int.MIN)` overflows, and an
invalid `clamp` range is a diagnosed programmer fault. Checked, saturating, and
wrapping arithmetic, numeric casts, unsigned and floating-point types, and
other numeric intrinsics require separate accepted contracts.

## Proposed ADR-0113: Collection Protocol Contracts

`Eq<T>`, `Ord<T>`, `Hash<T>`, `Clone<T>`, and `Default<T>` are ordinary
library protocols, not compiler-recognized capabilities. Protocol use is
explicit and nominal; there is no structural duck typing, reflection, blanket
implementation, or representation-derived fallback. `Hash<T>` must preserve
equal-value/equal-hash consistency. `HashMap<K, V>` and `HashSet<T>` require
`Eq` and `Hash`; `BTreeMap<K, V>` and `BTreeSet<T>` require `Ord`.

Sorting, search, deduplication, and lexicographic operations state their exact
protocol prerequisites and explicit failure results. None of these protocols
implies `Copy`, `Send`, or `Share`, and protocol dispatch does not expose a
public runtime or ABI representation.

## Proposed ADR-0114: Collection Capacity And Allocation Failure

Collection capacity is queried through ordinary APIs; allocators, pointers,
layouts, and capacity tokens are not public. `reserve`, `shrink`, and
growth-sensitive insertion have explicit `Result` contracts. Failed growth is
atomic: it leaves logical contents and ownership unchanged, rather than
partially mutating, discarding elements, or returning a nullable sentinel.
Growth policy, relocation, alignment, and physical allocation remain
compiler/runtime facts.

## Project Build Command

The `neu` workspace binary exposes only `neu build` initially. It discovers
`neu.json` from the current directory or an explicit project/manifest path,
defaults the target to the host, and writes to
`<manifest-root>/target/<safe-manifest-name>` unless `--output` is supplied.
`--target` accepts an explicit target triple. Only the host triple is supported;
other values are rejected before compilation. The command creates output
directories, never executes the produced binary, and reports compiler,
manifest, dependency, target, linker, and I/O failures with a non-zero
exit status. Raw-source APIs remain library interfaces; `run`, `check`, `test`,
package-manager, registry, and dependency-update commands are deferred.

# ADR-0067: Object Lifecycle And ABI

Status: Accepted

## Question

How are accepted class instances constructed, stored, moved, destroyed, and
represented without a garbage collector, user-visible deallocation, or public
layout promise?

## Competing Designs

1. Compiler-managed move-only objects with required primary construction,
   opaque host-linking layout, and deterministic destruction.
2. Inline value objects with copying and no allocation.
3. Traced heap objects with automatic reclamation.
4. Public C-compatible object layout and allocator ABI.

Inline copying conflicts with move-only owned fields. Tracing violates the
no-garbage-collector constraint. A public layout exposes target and runtime
choices prematurely. The first design preserves safety and ergonomics while
keeping layout internal.

## Decision

A class has one primary constructor form associated with its class declaration;
secondary constructors and overload resolution are deferred. There is no
implicit default constructor. Every declared field must be initialized exactly
once before construction completes. Field initialization follows declaration
order, then superclass construction occurs according to the accepted primary
constructor contract; construction order and superclass chaining must be
explicit in source once constructor syntax is implemented.

An object is a compiler-managed owned value. The compiler may place an object
in a local or host-linking-managed heap storage according to escape and ABI
needs, but source code cannot observe or request placement. The first runtime
implementation uses host-linking allocation artifacts for escaping or required
heap objects. No `free`, allocator primitive, stable pointer, or user-visible
deallocation API exists.

Class values move by default. Copying is unavailable unless a later accepted
copyability decision proves every owned field copyable. Read-only receiver use
creates an implicit shared borrow; mutation requires an exclusive inferred
effect; storing, returning, and consuming parameter passing transfer ownership.
Field destruction is recursive and occurs in reverse declaration order. The
derived object fields are destroyed before inherited superclass fields.

Partially initialized objects are never observable. If construction fails or a
compile-time initialization check rejects a path, already initialized fields
are destroyed in reverse order and the allocation is reclaimed by the target
pack. Runtime allocation failure traps non-successfully under the existing
bootstrap trap contract. Exceptions, `Result` construction APIs, and user
catching are deferred.

Fields have no default null or zero state unless their declared initializer
explicitly produces that value. Reads before initialization are compile-time
diagnostics. Cyclic owning graphs are rejected because no reference or weak
reference syntax is introduced; non-owning cycle support is deferred.

Object and interface layout, field offsets, alignment, padding, vtable and
interface-table representation, and allocation headers are compiler-private
host-linking contracts. They may vary by target, compiler version, and module
boundary. Separate compilation exchanges nominal identities, field types,
visibility, lifecycle requirements, capabilities, and ownership-effect
metadata, never raw offsets or stable layout. FFI and public object ABI are
deferred.

Constructors cannot publish `this`, return it, store it into an externally
reachable field, or invoke methods that require a fully initialized receiver
before initialization completes. Superclass construction must complete before
derived fields become observable. `super` construction and cleanup are
deterministic and follow the inherited-field order from ADR-0066.

## Diagnostics

Required diagnostics include missing or duplicate field initialization,
invalid constructor chaining, use of `this` before initialization, reads of
uninitialized fields, illegal ownership escape, cyclic owning declarations,
unsupported allocation context, and stale lifecycle or layout metadata. All
construction and cleanup diagnostics preserve field or constructor spans.

## Consequences

HIR and MIR must preserve field declaration order, initialization state,
ownership transitions, cleanup boundaries, and source mappings. Host linking
must provide compiler-only allocation/deallocation artifacts for executable
class programs. No standard library, public allocator, stable layout, or FFI
support is implied.

## Dependencies

- ADR-0001
- ADR-0004
- ADR-0005
- ADR-0014
- ADR-0018
- ADR-0020
- ADR-0035
- ADR-0045
- ADR-0046
- ADR-0047
- ADR-0062
- ADR-0065
- ADR-0066

# ADR-0069: Primary Constructors And Construction

Status: Accepted

## Question

What constructor syntax and initialization contract makes the accepted
move-only class model executable without adding overload resolution or public
allocation APIs?

## Decision

A class may declare one primary constructor parameter list immediately after
its name: `class Name(val field: T, var other: U) { ... }`. Constructor
parameters marked `val` or `var` are fields and initialize those fields in
left-to-right declaration order. A parameter without `val` or `var` is a
temporary constructor parameter and is not a field. Parameter names must be
unique and cannot duplicate body fields. The parameter list may be empty.

Construction uses `new Name(argument1, argument2)`. Arguments evaluate
left-to-right and must match the primary constructor parameter types exactly.
There are no secondary constructors, overloads, default arguments, implicit
default constructors, or inferred conversions. A constructor is not a normal
callable function and cannot be used as a value.

If a superclass exists, its constructor arguments are supplied by an explicit
`super(argument1, ...)` construction clause in the class header. The
superclass completes before derived fields initialize. Interface constructors
do not exist. A constructor body, when later accepted, may read initialized
superclass state and previously initialized fields but may not publish `this`
or read a later field.

The first implementation supports primitive and already-supported owned values
as fields only where the target-pack runtime contract can represent them. Class
instances are move-only. Field reads borrow implicitly, field writes require
exclusive ownership, and construction transfers argument ownership into owned
fields. No user-visible allocation, deallocation, pointer, or layout API is
introduced.

Missing, duplicate, or out-of-order initialization is a compile-time
diagnostic. Allocation failure and invalid construction paths trap
non-successfully. Already initialized fields are destroyed in reverse order on
construction failure. Constructor calls cannot occur before all required class
and field metadata is available.

## Consequences

Parser and type checking must distinguish constructor declarations from normal
functions and preserve parameter-to-field identity. HIR and MIR must preserve
argument evaluation order, field initialization order, ownership transfers,
and cleanup boundaries. Backend object storage remains compiler-private and
target-pack supplied. Secondary constructors, default values, exceptions,
reflection, and FFI remain deferred.

## Dependencies

- ADR-0001
- ADR-0004
- ADR-0041
- ADR-0047
- ADR-0065
- ADR-0066
- ADR-0067
- ADR-0068

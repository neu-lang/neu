# ADR-0068: Class And Field Surface Syntax

Status: Accepted

## Question

What concrete source forms make the accepted class, interface, and field
foundation implementable without prematurely defining constructors or object
allocation?

## Decision

`class` and `interface` are reserved declaration keywords. A class header is
`class Name` optionally followed by `: Base(), InterfaceName, ...`; it names at
most one direct superclass and any number of interfaces. An interface header
may list interfaces after `:`. Generic class and interface headers remain
deferred.

Class bodies contain field declarations and, once method syntax is implemented,
method declarations. A field is written as an optional `public`, `internal`,
or `private` modifier followed by `val` or `var`, a name, `:`, and a declared
type, terminated by `;`. `protected` is rejected as specified by ADR-0065.
Field declarations do not have default initializers in this foundation; every
field is initialized by the primary constructor under ADR-0067. Interface
bodies contain required method declarations only; default bodies and interface
fields are deferred.

Member access is `receiver.field`. Within an instance method or constructor,
`field` is shorthand for `this.field`; `this.field` is the disambiguating form
when a local or parameter shadows a field. No alternate property syntax,
reflection, extension-member syntax, or implicit global field lookup exists.

The parser may parse and record class/interface and field metadata, nominal type
identity, field visibility, mutability, and declared types. It may type-check
field projections only where a receiver type is already available from an
accepted method or constructor context. Object construction expressions,
constructor calls, field initialization, allocation, and runtime field access
are later lifecycle contracts and are not inferred by this ADR.

## Diagnostics

The parser reports missing class/interface names, malformed inheritance lists,
duplicate fields, unsupported `protected`, field declarations without a type,
invalid visibility, and malformed member access with source spans. The type
checker reports unknown fields, inaccessible fields, immutable writes, and
field hiding before HIR lowering.

## Consequences

The syntax is Kotlin-like without committing to a public object layout. ADR-0067
remains the authority for initialization and placement. `new`, constructor
overloads, secondary constructors, and allocation failure behavior remain
deferred to the lifecycle implementation task.

## Dependencies

- ADR-0021
- ADR-0022
- ADR-0024
- ADR-0025
- ADR-0065
- ADR-0066
- ADR-0067

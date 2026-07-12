# ADR-0083: Generic Declaration Environments

## Status

Accepted.

## Decision

Top-level functions, classes, interfaces, enums, and their accepted members
may declare explicit generic parameters. A declaration's generic environment
is the ordered list of its enclosing type declaration parameters followed by
its own parameters. A member may refer to enclosing parameters; a member-level
parameter with the same spelling shadows the enclosing spelling but has a
distinct declaration identity. Duplicate names within one parameter list are
diagnostics.

Generic classes, interfaces, and enums use invariant explicit type arguments in
their headers and constructed type positions. Generic functions and methods
use explicit type arguments when instantiated; no type inference, default
arguments, variance, or implicit conversions are introduced. Generic
inheritance and interface implementation preserve the substituted ordered
arguments and require exact invariant identity.

Generic fields, constructor parameters, function parameters, returns, enum
payloads, and accepted member signatures may reference visible generic
parameters. Their source spans and declaration ownership are preserved in
compiler metadata. Generic declarations without a concrete specialization do
not lower to executable backend code; specialization is defined by a later
ADR.

Ownership, nullability, capabilities, visibility, overload identity, and
dispatch are checked after constructing the ordered declaration environment.
Constraint solving beyond the existing bound metadata, recursive bounds,
cross-module lookup, generic closures, reflection, and public generic ABI
remain deferred.

## Consequences

Parser metadata records the declaration owning each generic parameter. Type
checking can build enclosing-plus-local environments deterministically, while
HIR/MIR and backend specialization remain separate later stages.

## Dependencies

ADR-0016, ADR-0023, ADR-0032, ADR-0035, ADR-0065, ADR-0066, ADR-0081,
ADR-0082.

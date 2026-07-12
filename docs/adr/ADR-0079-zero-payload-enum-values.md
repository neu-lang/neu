# ADR-0079: Zero-Payload Enum Values And Typed Transport

## Status

Accepted.

## Decision

An `enum` declaration is a closed nominal type. Its variants are declared as
bare identifiers in declaration order and have stable compiler-private
identity `(module, package, enum declaration, variant)`. A zero-payload
variant is constructed with the qualified expression `EnumName.VariantName`.
The expression produces the enclosing enum type; it is not a class object and
does not allocate through a user-visible API.

The first runtime enum slice supports zero-payload variants only. Each enum
has at least one variant, duplicate variant names are diagnostics, and variant
names are unique within their enum but may repeat in other enums. Values may
be passed to and returned from same-module functions and methods using a
compiler-private typed scalar tag representation. Variant declaration order is
the deterministic tag order. The representation, tag width, symbols, and
layout are not public ABI or FFI contracts.

Zero-payload enum values are immutable, copyable tags. Equality, pattern
matching lowering, nullability, capability derivation, destruction, and
ownership effects use the existing primitive/value contracts where applicable;
no user-visible identity or allocation is exposed. Nullable enum types remain
accepted only where the existing nullable type machinery accepts the nominal
type, but runtime nullable enum lowering is deferred until its representation
is separately accepted.

Enum declarations do not inherit classes or interfaces. Payload variants,
variant fields, constructors with arguments, enum-associated functions,
instance receivers, enum field access, generic enums, reflection,
serialization, standard-library helpers, and FFI are deferred. Unsupported
forms receive source-mapped diagnostics before HIR lowering.

HIR and MIR preserve enum declaration identity, variant identity, tag type,
source spans, parameter/return facts, and cleanup facts. Cranelift uses the
same typed scalar transport as the internal Neu ABI; no stable public enum
layout is introduced.

## Dependencies

ADR-0021, ADR-0027, ADR-0035, ADR-0055, ADR-0075, ADR-0078.

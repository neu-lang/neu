# Ambiguity: Payload Enum Functions

Status: Resolved by ADR-0081

## Affected Work

Task-011, enum payload parsing and construction, payload-aware `when`, enum
instance and associated functions, HIR/MIR representation, and enum ABI.

## Authority Gap

Task-011 mandates payload-bearing declarations such as
`enum Color(val rgb: Int)`, payload extraction in patterns, and enum functions.
Accepted ADR-0079 explicitly limits the accepted runtime enum slice to
zero-payload variants and explicitly defers payload fields, argument-bearing
constructors, enum functions, and payload patterns.

## Competing Interpretations

1. Supersede ADR-0079 with a payload representation and ownership/ABI contract.
2. Implement task-011's syntax using an internal packed scalar representation.
3. Keep ADR-0079 authoritative and defer task-011 until a new enum payload ADR
   is accepted.

## Why Guessing Is Unsafe

The choice changes enum layout, signed payload transport, construction and
destruction, pattern binding, receiver effects, ABI flattening, and target-pack
requirements. A packed scalar would silently restrict the required ABI type set
and could make ownership and nullable behavior unsound.

## Resolution

ADR-0081 defines payload fields, constructor arguments, payload pattern
bindings, enum instance and associated functions, ownership, cleanup, and the
compiler-private tagged aggregate ABI. Task-011 is back in the queue and may
implement those accepted semantics.

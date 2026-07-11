# ADR-0055: Bootstrap Type Environment Transport

Status: Accepted

## Question

How does a backend-facing typed representation resolve its `TypeId` values to
the module type identities required for bootstrap runtime lowering?

## Competing Designs

1. Treat raw `TypeId` numeric values as stable runtime-type codes.
2. Copy or own a `TypeArena` in every HIR and MIR module.
3. Pass the owning module `TypeArena` explicitly with typed lowering inputs.
4. Replace typed representations with a backend-specific runtime-type enum.

## Trade-offs

Raw IDs contradict ADR-0052 because identity depends on the owning arena.

Duplicated arena ownership risks divergence and adds unnecessary copying to
intermediate representations.

An explicit companion input preserves the existing module-wide identity domain
without making HIR or MIR backend-specific.

Replacing type identities with backend types too early loses semantic and
diagnostic information needed by later phases.

## Recommended Choice

The owning module `TypeArena` is an explicit companion input at every typed
lowering boundary that must inspect a `TypeId`, including HIR-to-MIR and
MIR-to-backend lowering. HIR and MIR continue to preserve `TypeId` values and
do not own, duplicate, or reinterpret the arena.

The lowering boundary must resolve a `TypeId` through that supplied arena
before selecting a bootstrap runtime representation. For the executable subset,
bootstrap primitives, recursively supported inline arrays from ADR-0063, and
owned UTF-8 strings from ADR-0064 are accepted runtime values and lower under
ADR-0043, ADR-0046, ADR-0063, and ADR-0064.
Missing, foreign, or unsupported type identities fail as explicit
unsupported-lowering conditions; they must not be interpreted from raw numeric
IDs.

The companion arena must be the same module arena that produced the HIR/MIR
type facts under ADR-0052. Passing a different arena is invalid lowering input.
The exact Rust ownership and API shape remain compiler implementation details.

## Downstream Consequences

- M0030 must preserve and validate the module type environment at HIR-to-MIR
  lowering boundaries.
- M0031 can map `Int` to Cranelift `I64` without hardcoding `TypeId` values.
- Backend unsupported-type failures remain internal lowering diagnostics until
  user-facing runtime types are accepted.
- This decision does not define additional language types, layouts, or ABI
  behavior.

## Dependencies

- ADR-0027
- ADR-0043
- ADR-0044
- ADR-0045
- ADR-0046
- ADR-0052

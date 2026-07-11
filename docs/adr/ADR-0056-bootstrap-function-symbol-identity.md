# ADR-0056: Bootstrap Function Symbol Identity

Status: Accepted

## Question

How must source function identity reach object emission so bootstrap symbols
follow ADR-0046 without relying on MIR-local numeric IDs?

## Competing Designs

1. Derive object symbols from `MirFunctionId` alone.
2. Flatten a symbol string before HIR lowering and carry only that string.
3. Preserve structured module, package, and source-function identity through HIR
   and MIR, then mangle it at the backend boundary.
4. Keep a separate backend side table keyed by MIR function IDs.

## Trade-offs

MIR-local IDs are not source identity and can collide across modules or change
when declaration order changes. An early flattened string loses the structured
facts needed for diagnostics and future target policies.

A separate side table can work, but it creates a second identity transport
that can drift from HIR and MIR. Preserving the structured identity keeps the
existing intermediate representations authoritative while leaving the exact
object encoding to the backend.

## Recommended Choice

HIR and MIR preserve a structured bootstrap function symbol identity containing
the accepted module identity, package namespace, and source function name. The
identity is carried from parsed declaration metadata into HIR, from HIR into
MIR, and from MIR into object emission. Missing identity is an explicit
lowering failure; a numeric MIR function ID must never substitute for it.

The backend derives a deterministic internal object symbol from those three
components. The exact escaping and encoding are compiler implementation
details, must be collision-free within one object/link invocation, and do not
define a stable public ABI or new language semantics.

## Downstream Consequences

- M0030/M0032 implementation must preserve function identity across the HIR,
  MIR, and object boundaries.
- Object emission can reject incomplete identity instead of inventing a symbol.
- Linker and runtime work may refer to the emitted internal symbol without
  changing source-level function semantics.
- Public symbol export, FFI names, and cross-target mangling remain deferred.

## Dependencies

- ADR-0020
- ADR-0044
- ADR-0045
- ADR-0046
- ADR-0052

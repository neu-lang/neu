# ADR-0039: Bootstrap Unsafe FFI Boundary Analysis

Status: Accepted

## Question

What bootstrap unsafe and FFI boundary semantics are sufficient for this implementation
without defining unsafe source syntax, FFI declaration syntax, host linking,
foreign binding generation, or ABI lowering?

## Competing Designs

1. Define concrete unsafe and FFI source syntax now.
2. Implement host-linking and ABI validation before unsafe/FFI checks.
3. Use metadata-only unsafe-context, unsafe-operation, and FFI-declaration
   records for this implementation.
4. Defer the compiler entirely until unsafe syntax and host linking exist.

## Trade-offs

Concrete source syntax would make unsafe and FFI user-facing, but ADR-0024
explicitly defers unsafe block syntax and the current grammar does not accept
FFI declarations.

Host-linking-first validation would improve ABI precision, but it would push
backend and packaging future work ahead of core safe-code boundary checks.

Metadata-only records let the compiler validate the trust-boundary rules selected by
ADR-0018 while keeping syntax, host linking, linking, and ABI lowering out of
scope.

Full deferral avoids premature decisions, but leaves safe-code guarantees
without an incremental unsafe/FFI boundary model.

## Recommended Choice

Use a metadata-only bootstrap unsafe and FFI boundary model.

The compiler introduces no source-level unsafe block, unsafe function, extern block,
foreign declaration, ABI string, link attribute, target attribute, safe-wrapper
syntax, or module audit syntax. Existing unsupported unsafe-like and FFI-like
source forms remain rejected or unsupported.

The compiler defines two safety bases:

- `ProvenSafe`: the compiler's accepted analyses prove the operation safe, so
  no unsafe context is required.
- `TrustedUnsafe`: the operation relies on an explicit programmer or binding
  assertion, so an approved unsafe context is required.

Unsafe context records are compiler side-table facts. A context record has a
context node and context kind: `block`, `function`, or `module_audit`. These
kinds are semantic metadata only in this implementation and do not imply source syntax.

Unsafe operation records are compiler side-table facts. An operation record has
an operation node, operation kind, safety basis, and optional containing unsafe
context node. A `ProvenSafe` operation is accepted without an unsafe context. A
`TrustedUnsafe` operation is accepted only when its containing context node
matches one of the supplied unsafe context records.

The compiler reports `unsafe_operation_outside_context` when a `TrustedUnsafe`
operation has no matching unsafe context. The primary diagnostic span is the
operation node. When a non-matching context node is supplied, that context node
is the secondary span; otherwise there is no secondary span. The diagnostic
must identify that the operation is a trusted assertion rather than compiler
proven safety.

FFI declaration records are compiler side-table facts. A declaration record has
a declaration node, declaration kind, safety basis, safe-wrapper status, and
metadata presence for these required categories:

- target contract;
- calling convention;
- nullability;
- ownership transfer;
- lifetime validity;
- thread-safety or send/share guarantees.

The compiler validates metadata presence only. It does not validate concrete target
triples, layout, calling convention compatibility, symbol names, linker inputs,
header parsing, generated bindings, dynamic loading, platform APIs, or ABI
lowering.

The compiler reports `missing_ffi_safety_metadata` when an FFI declaration lacks one
or more required metadata categories. The primary diagnostic span is the FFI
declaration node. The diagnostic must list the missing metadata categories.

A safe wrapper is valid in this implementation only as metadata that marks whether ordinary
safe use is intended to go through a wrapper. The compiler does not type-check wrapper
bodies, prove wrapper safety, lower wrapper calls, or expose FFI declarations
to source-level name resolution.

Safe-code guarantees remain intact because the compiler accepts trusted unsafe
operations only inside supplied unsafe contexts and treats FFI declarations
without required safety metadata as invalid. Existing ownership, borrowing,
thread-capability, coroutine, and nullability analyses remain authoritative for
compiler-proven facts; trusted unsafe records do not weaken those analyses for
ordinary safe code.

## Downstream Consequences

- The compiler can implement unsafe-context and FFI metadata checkers without source
  syntax.
- Later unsafe syntax must map into the same context and operation categories
  or supersede this ADR.
- Later host-linking future work must replace metadata-presence checks with
  target-specific validation.
- Diagnostics can rely on stable identifiers
  `unsafe_operation_outside_context` and `missing_ffi_safety_metadata`.
- Safe wrapper semantics require a later ADR before wrappers affect source
  visibility, type checking, or call lowering.

## Dependencies

- ADR-0001
- ADR-0003
- ADR-0006
- ADR-0014
- ADR-0015
- ADR-0018
- ADR-0020
- ADR-0024

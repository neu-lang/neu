# ADR-0075: Bootstrap Value ABI Extension

- Status: Accepted by the main-task Chief Architect
- Extends: ADR-0041, ADR-0055, ADR-0063, ADR-0064, ADR-0070, and ADR-0073

## Decision

The internal same-module Neu ABI transports every currently accepted primitive,
string, fixed-array, class, interface, and scalar dynamic-array value through
typed Cranelift signatures. Fixed arrays flatten recursively by element; class
and interface slots remain compiler-private pointer values. `Array<Bool>`,
`Array<Int>`, `Array<Float>`, and `Array<Byte>` cross calls as opaque
compiler-managed pointers carrying their private length/capacity/data header.

Ownership effects remain part of the call contract: move-only dynamic arrays
are transferred to consuming parameters and returned values are owned by the
caller. A dynamic-array return is not cleaned up in the callee before return.
The compiler inserts cleanup at the owning local boundary.

Dynamic arrays containing strings, classes, interfaces, fixed arrays, nested
dynamic arrays, or nullable values are rejected before HIR lowering until
their element-destruction and aggregate ABI contracts are accepted. This is a
specified bootstrap boundary, not a backend type guess. `main(): Int`, public
layout, FFI, slices, and generic function ABI remain unchanged or deferred.

## Consequences

- Fixed and dynamic arrays remain distinct at signatures and call sites.
- Accepted dynamic scalar values have executable parameter/return coverage.
- Unsupported aggregate element combinations receive deterministic frontend
  diagnostics before Cranelift.

## Dependencies

ADR-0041, ADR-0055, ADR-0063, ADR-0064, ADR-0067, ADR-0070, ADR-0072,
ADR-0073, and ADR-0074.

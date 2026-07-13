# ADR-0045: Bootstrap MIR Runtime Contract

Status: Accepted

## Question

What MIR model is required for the first executable subset?

## Competing Designs

1. Minimal SSA-like MIR with explicit blocks and instructions.
2. Stack-machine bytecode MIR.
3. Cranelift-shaped MIR.
4. Defer MIR and lower HIR directly to Cranelift.

## Trade-offs

A minimal SSA-like MIR gives clear values, control flow, and backend
independence without requiring optimization infrastructure.

Stack-machine MIR is compact but less aligned with Cranelift and future
analysis.

Cranelift-shaped MIR leaks backend constraints into the compiler middle end.

Direct HIR-to-Cranelift skips the architecture boundary selected by the
implementation plan.

## Recommended Choice

Bootstrap MIR is backend-independent and contains:

- function definitions with stable internal function identifiers;
- ordered parameters and a return type;
- local slots for source locals that need storage;
- temporaries for expression results;
- basic blocks with ordered instructions and one terminator;
- source mapping from MIR instructions/terminators back to HIR/source spans;
- constants for bootstrap `Int`;
- local load/store for executable-subset locals;
- checked or trapping `Int` arithmetic, exponentiation, bitwise, and shift
  operations with ADR-0043 semantics;
- direct calls with ordered arguments and one return value;
- unconditional branches;
- conditional branches only when needed by already accepted frontend forms;
- return terminators; and
- trap terminators for integer overflow and unsupported runtime traps;
- inline aggregate construction, ordered element initialization, indexed
  loads/stores, and bounds checks accepted by ADR-0063.

MIR cleanup/destruction is a bootstrap boundary. For the first executable
subset, only bootstrap primitives and supported inline arrays are runtime
values, so there are no user-defined destructors, heap resources, async
cancellation cleanups, or FFI cleanup edges.
MIR must reserve a representation boundary for later cleanup insertion but must
not invent cleanup semantics.

## Downstream Consequences

- The compiler must validate values, locals, temporaries, blocks, calls, returns,
  checked arithmetic, and source mapping.
- The compiler can lower the minimal MIR subset without accepting broader language
  features.
- Future destructor, ownership, coroutine, and FFI lowering must supersede or
  extend the cleanup boundary.

## Dependencies

- ADR-0035
- ADR-0041
- ADR-0042
- ADR-0043
- ADR-0044

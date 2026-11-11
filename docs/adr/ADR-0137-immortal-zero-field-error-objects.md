# ADR-0137: Immortal Zero-Field Error Objects

Status: Accepted by the project owner. Resolves the mechanism left open by ADR-0135 O2. Depends
on ADR-0136.

## Question

ADR-0135 requires `AllocationFailedError` to be constructed without heap allocation,
because allocation failure is the condition it reports. ADR-0067 and ADR-0073 make
runtime allocation failure trap, and ADR-0114 asks for a recoverable `Result` on growth.
Constructing the error object by allocating would therefore fail in the very case it is
meant to report. What mechanism lets the runtime produce these error values without
allocating?

## Decision

1. A class with no fields and no destructor side effects is eligible for immortal
   representation. This decision applies only to the stdlib error classes
   `IndexOutOfBoundsError` and `AllocationFailedError` (ADR-0135). Other classes allocate
   normally (ADR-0067).
2. The compiler emits one static, immutable instance of each such class, in the target's
   read-only data. Its layout is compiler-private (ADR-0067).
3. Constructing an immortal error is a reference to that static instance. It involves no
   allocation and therefore has no failure path.
4. Destroying an immortal instance does nothing. It is not reclaimed, because it was never
   allocated. This is the one stated exception to ADR-0135 decision 7 ("destroys the
   concrete object exactly once"). For immortal objects, destruction is observationally a
   no-op, so the exactly-once rule holds in effect.
5. An immortal instance has no fields, so nothing can mutate it. It satisfies `Send` and
   `Share` by the derivation rule of ADR-0136, and may be shared across tasks without
   synchronization.
6. The ownership rules of ADR-0135 decision 6 still apply at the language level. A `Result`
   holding an immortal `Error` is move-only and not copyable. Immortality is a runtime
   representation fact, not a language-visible copy.

## Non-goals

- Immortal representation for user-defined classes, or for classes with fields.
- A user-visible allocator, static-object API, or pointer (ADR-0067, ADR-0114).
- Pre-allocating errors at startup as an alternative to static emission.

## Consequences

- The allocation-failure path cannot itself fail. Reporting `AllocationFailed` never
  depends on the heap.
- The target pack must support a read-only data section for immortal instances. This is
  host-linking work (ADR-0047, ADR-0100) and needs a host-linking test.
- The rule of ADR-0135 decision 7 gains one stated exception, recorded here.

## Open questions

- **Q1. Portability. Resolved.** ADR-0020 is superseded by ADR-0100: the bootstrap compiler
  supports only the current host and uses the host linker. Read-only static data is
  therefore needed only for the host target. Any future cross-target work needs its own
  accepted decision.
- **Q2. Scope. Resolved: restricted.** Immortality applies only to `IndexOutOfBoundsError`
  and `AllocationFailedError`. User zero-field classes stay on the normal allocation path.

## Required follow-up

Tests, written first:

- constructing `AllocationFailedError` under simulated allocation failure succeeds without
  calling the allocator;
- constructing `IndexOutOfBoundsError` does not allocate;
- destroying an immortal error is a no-op and does not free memory;
- an immortal error satisfies `Send` and `Share` (ADR-0136);
- a host-linking test confirms the static instance links and is referenced correctly
  (CI host-linking requirement, AGENTS.md).

ADR-0020 must be checked (Q1) before this ADR is accepted, and `docs/SPEC.md` must carry
its section.

## Dependencies

ADR-0020, ADR-0047, ADR-0067, ADR-0073, ADR-0100, ADR-0114, ADR-0135, ADR-0136.

# ADR-0138: Dynamic Array Growth Results

Status: Accepted by the project owner. Revises ADR-0073 (growth and allocation failure),
ADR-0114 (growth scope), and ADR-0132 (the "operations unchanged" clause). Depends on
ADR-0135, ADR-0137. Q1 and Q2 are resolved as recommended. Q3 is deferred to a separate ADR.

## Question

ADR-0073 makes the growing operations of `T[]` return `Unit` and trap on allocation
failure. ADR-0114 requires any insertion that needs growth to return a recoverable
`Result` and to leave the collection unchanged on failure. The project owner chose the
`Result` behavior (A1). Which operations change, what the failure value is, and what the
atomicity rule requires?

## Decision

1. `add(value)` on a `T[]` returns `Result<Unit, Error>`. Growth is the only failure
   path.
2. Growth failure returns `Err` holding an `AllocationFailedError` (ADR-0135), which is
   an immortal instance (ADR-0137). Constructing it cannot itself fail.
3. The growth is atomic. The compiler allocates the new storage before any element is
   moved. On failure, the old storage, its length, and every element are unchanged, and
   the value passed to `add` is returned to the caller unchanged. The caller cannot observe
   a partial insertion.
4. The `Err` case requires the caller to handle the `Result` (ADR-0111). A `Result` value
   that is discarded as an expression statement is a diagnostic.
5. `add(value, index)` returns `Result<Unit, Error>` as well. An index outside `0..size`
   returns `Err` holding an `IndexOutOfBoundsError`, and the sequence is unchanged. This
   extends the owner's A1 answer, which covered growth only. It follows from ADR-0110,
   which forbids a trap for out-of-bounds access.
6. `size()` keeps its `Int` result. `val` bindings still reject both `add` forms (ADR-0073).
7. Non-growth traps stay in force. An unsupported target capability still traps. Failure
   to construct an ordinary class object still traps (ADR-0067). Only the growing forms of
   `T[]` change.

## Non-goals

- `remove` and element access. `remove` is decided by ADR-0139. Element reads and writes
  are unchanged here (see Open questions).
- Capacity queries, `reserve`, and `shrink` (ADR-0114 follow-up).
- Any user-visible allocator or capacity token (ADR-0114).

## Consequences

- `ArrayList<T>.add` (ADR-0132) inherits a recoverable failure with no change to its own
  design.
- Compiler growth becomes a primitive that reports failure. Its implementation needs
  tests that force failure and check that the old contents survive.
- The `Result` payload is an interface value (ADR-0135 O4). Its construction and
  destruction depend on ADR-0137's immortal instances.

## Open questions

- **Q1. Discarded `Result` in statement position. Resolved: rejected.** Decision 4 stands.
- **Q2. `add(value, index)` bounds. Resolved: `Err`, not a trap.** Decision 5 stands.
- **Q3. Element reads and writes. Deferred** to a separate ADR. ADR-0073 and ADR-0132 keep
  indexed reads and writes trapping on bounds, which ADR-0110 forbids for collections.
  `ArrayList.get` can check bounds before calling the primitive, so the library never
  reaches the trap. Until that ADR is accepted, the primitive's rule is open.

## Required follow-up

Tests, written first:

- `add` with room available returns `Ok(())` and the element is present;
- `add` under forced growth failure returns `Err(AllocationFailedError)` and the length,
  elements, and ownership are unchanged;
- the value passed to a failed `add` is returned to the caller intact (move-only element);
- `add(value, index)` with `index` outside `0..size` returns `Err(IndexOutOfBoundsError)`
  and leaves the sequence unchanged;
- a discarded `Result` from `add` is a diagnostic (Q1);
- `add` on a `val` binding is still rejected;
- a host-linking test confirms the growth failure path links and runs (AGENTS.md).

`docs/SPEC.md` needs this section and the ADR-0073 revision before acceptance.

## Dependencies

ADR-0067, ADR-0073, ADR-0110, ADR-0111, ADR-0114, ADR-0119, ADR-0132, ADR-0135,
ADR-0137.

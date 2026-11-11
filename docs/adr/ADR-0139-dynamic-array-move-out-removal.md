# ADR-0139: Dynamic Array Move-Out Removal

Status: Accepted by the project owner. Revises ADR-0073 (`remove` returns `Unit`) and
ADR-0132 (the "operations unchanged" clause). Depends on ADR-0110, ADR-0135, ADR-0137.
Q1 and Q2 are resolved as recommended.

## Question

ADR-0073 makes `remove(index)` return `Unit`. No accepted rule moves an element out of
`T[]`, so a removed element for move-only `T` is lost or cannot be returned. ADR-0110
requires removal to return an explicit recoverable result. The project owner chose
move-out removal returning the owned element inside a bounds `Result` (A2). What is the
exact rule?

## Decision

1. `remove(index)` on a `T[]` returns `Result<T, Error>`.
2. If `index` is in `0..size`, the element at that index is moved out and returned as
   `Ok(element)`. The elements after it shift down by one, preserving their order, and
   `size` decreases by one.
3. If `index` is outside `0..size`, the result is `Err` holding an `IndexOutOfBoundsError`
   (ADR-0135, ADR-0137). The sequence is unchanged.
4. Removal is a move. Ownership of the returned element transfers to the caller. The same
   rule applies to copyable `T`: the element leaves the sequence, and no implicit copy is
   made.
5. Removal does not allocate and cannot report allocation failure (ADR-0114). The
   `Result` error type is therefore only `IndexOutOfBoundsError` in practice.
6. The `Result` must be handled (ADR-0111). A discarded `remove` result is governed by the
   same diagnostic as ADR-0138 decision 4.
7. `val` bindings still reject `remove` (ADR-0073).

## Non-goals

- `pop`, `first`, `last`, or any other `ArrayList`-level operation. Those are library
  operations built on `remove`.
- Moving an element out by index without shifting. `T[]` has no hole representation.
- Changing `add` (ADR-0138) or element reads and writes.

## Consequences

- `ArrayList<T>.pop` can be implemented as `remove` on the last index, returning an owned
  element for move-only `T`.
- Removal from the middle is linear in the number of shifted elements, as ADR-0110 already
  states.
- The removal primitive's result type is an interface-payload `Result` (ADR-0135 O4), so
  it depends on ADR-0137 for its error value.

## Open questions

- **Q1. Static out-of-range index. Resolved: keep the compile-time diagnostic.** A constant
  out-of-range index is diagnosed before lowering, as ADR-0073 requires. It is not turned
  into a run-time `Err`.
- **Q2. Discarded result. Resolved: rejected**, as in ADR-0138 Q1.

## Required follow-up

Tests, written first:

- `remove(i)` for `0 <= i < size` returns `Ok(element)`, and the length drops by one;
- the elements after the removed one keep their order;
- `remove` with an index outside `0..size` returns `Err(IndexOutOfBoundsError)` and leaves
  the sequence unchanged;
- removing a move-only element transfers ownership, and the element is destroyed exactly
  once by its new owner;
- removing from an empty sequence returns `Err`;
- a statically known out-of-range index is diagnosed (Q1);
- `remove` on a `val` binding is rejected;
- a discarded `remove` result is a diagnostic (Q2).

`docs/SPEC.md` needs this section and the ADR-0073 revision before acceptance.

## Dependencies

ADR-0067, ADR-0073, ADR-0110, ADR-0111, ADR-0114, ADR-0124, ADR-0132, ADR-0135, ADR-0137,
ADR-0138.

# Soundness Report: M0028-004

## Decision

`pass`

## Attacks Attempted

- Decimal, binary, and hexadecimal values with separators preserve their exact
  `u64` magnitude: passed.
- The magnitude `9223372036854775808` remains available for the valid unary
  minimum-`Int` form: passed.
- A value above `u64` does not wrap or truncate: passed as `None` metadata.

## Findings

None. No type, ownership, borrow, thread, or runtime behavior changes.

## Residual Risk

ADR-0043 range and arithmetic diagnostics are intentionally deferred to the
next M0028 task.

# Soundness Report: M0028-003

## Decision

`pass`

## Attacks Attempted

- A `Bool` arithmetic operand must not receive an `Int` result through the core:
  passed. The M0028 core records `TypeMismatch` and suppresses no unrelated
  diagnostic.
- Unsupported `!` and `==` must not become accepted through broad filtering:
  passed. They retain their existing unary/binary deferral diagnostics.
- Nested arithmetic must reach local initializer and assignment checks only
  after operator typing: passed.

## Findings

None. This task changes type-checking metadata flow only; it does not alter
ownership, borrow, thread, coroutine, unsafe, or FFI rules.

## Residual Risk

Static arithmetic failures, call/return rules, and unsupported-executable-form
diagnostics remain later M0028 work.

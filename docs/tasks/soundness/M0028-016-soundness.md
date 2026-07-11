# Soundness Report: M0028-016

## Decision

Pass. `return true` in an `Int` function produces one expression-span
`return_type_mismatch`; an unresolved return expression produces no false
mismatch. Failed compatibility records no typed executable return fact, as
required by ADR-0054. Ownership, borrow, thread, coroutine, unsafe, and FFI
analysis are unchanged.

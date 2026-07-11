# Soundness Report: M0031-002

- Decision: `pass`
- Authority: ADR-0043, ADR-0045, ADR-0046, ADR-0055.
- Changed files: `crates/compiler/src/backend.rs`,
  `crates/compiler/tests/backend.rs`.
- Ordinary tests: formatter, Clippy, workspace tests, and focused validator
  passed before this check.

## Attack

An `iadd` wraps at machine width unless checked. The lowerer computes signed
overflow from both operand-to-result sign changes and emits `trapnz` with
Cranelift's `INTEGER_OVERFLOW` code. The backend IR test requires both `iadd`
and `int_ovf`; no wrapping-only path is accepted.

## Findings

None.

## Decision

Pass. The narrow operation preserves ADR-0043's trap-on-runtime-overflow
requirement and does not alter ownership, borrowing, concurrency, or unsafe
semantics.

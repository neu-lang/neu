# Soundness Report: M0031-003

Decision: pass.

ADR-0043 requires subtraction overflow to trap rather than wrap. The backend
emits `isub`, computes the signed overflow condition from operand and result
sign changes, and traps with `INTEGER_OVERFLOW`. The focused IR test requires
both `isub` and `int_ovf`; formatter, Clippy, workspace tests, and the focused
validator passed before this review. No ownership, borrow, concurrency, or
unsafe semantics change. Findings: none.

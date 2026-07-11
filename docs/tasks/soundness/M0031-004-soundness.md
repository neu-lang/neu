# Soundness Report: M0031-004

Decision: pass.

ADR-0043 requires multiplication overflow to trap. The backend compares
`smulhi` with the sign extension of `imul` and traps with `INTEGER_OVERFLOW` on
mismatch. The focused IR test requires `imul`, `smulhi`, and `int_ovf`; full CI
passed. Findings: none.

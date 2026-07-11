# Soundness Report: M0031-005

Decision: pass. Cranelift `sdiv` documents traps for a zero divisor and the
sole signed overflow case, `Int.MIN_VALUE / -1`, matching ADR-0043. The focused
IR test requires `sdiv`; full CI passed. Findings: none.

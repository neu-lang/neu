# Soundness Report: M0031-007

Decision: pass. Cranelift `band`, `bor`, and `bxor` operate directly on the
bootstrap signed 64-bit two's-complement representation, matching ADR-0043.
The chained IR fixture and full CI passed. Findings: none.

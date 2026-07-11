# Soundness Report: M0031-009

Decision: pass. Unary negation checks `Int.MIN_VALUE` before `ineg`, plus is
identity, and complement is a direct `bnot`. Full CI passed. Findings: none.

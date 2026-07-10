# Soundness Report: M0021-016

- Task ID: `M0021-016`
- Milestone: `M0021`
- Decision: `pass`

Inputs read: `docs/tasks/M0021-016-exhaustive-match-coverage.md`,
`docs/milestones/M0021-algebraic-data-and-exhaustiveness.md`, ADR-0033
authority excerpt, `crates/newlang/src/name_resolution.rs`, and
`crates/newlang/tests/name_resolution.rs`.

Attack checked: a `when` with duplicate arms could otherwise also report
missing coverage and obscure the first actionable error. The coverage analyzer
suppresses `non_exhaustive_match` when resolution or duplicate-arm diagnostics
already apply to the same `when`.

Attack checked: a wildcard arm could be mishandled as missing concrete
variants. The analyzer treats one wildcard arm as exhaustive for the ADR-0033
bootstrap subset.

No ownership, borrowing, lifetime, async, thread-safety, unsafe, or FFI
boundary is changed. Ordinary tests passed before the adversarial check.

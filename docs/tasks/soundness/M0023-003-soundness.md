# Soundness Report: M0023-003

- Task ID: `M0023-003`
- Milestone: `M0023`
- Decision: `pass`

Inputs read: `docs/tasks/M0023-003-lifetime-escape-diagnostics.md`,
`docs/adr/ADR-0036-bootstrap-borrow-and-lifetime-analysis.md`,
`crates/newlang/src/borrow.rs`, and `crates/newlang/tests/borrow.rs`.

Attack checked: same-region borrow use must not diagnose as an escape. The
test includes a same-region record that produces no diagnostic.

Attack checked: different-region borrow use must diagnose and preserve the
original borrow node as origin. The test checks both fields.

Attack checked: diagnostics must continue after one escape. The test expects
two independent lifetime escape diagnostics.

No source syntax, nested region overlap, async, unsafe, or FFI behavior is
implemented by this task.

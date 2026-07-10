# Soundness Report: M0022-003

- Task ID: `M0022-003`
- Milestone: `M0022`
- Decision: `pass`

Inputs read: `docs/tasks/M0022-003-ownership-transfer-sites.md`,
`docs/adr/ADR-0035-bootstrap-ownership-and-move-analysis.md`,
`crates/compiler/src/ownership.rs`, and `crates/compiler/tests/ownership.rs`.

Attack checked: copyable `Int` local sources must not create move origins. The
transfer collector uses the M0022 value-category classifier and ignores
copyable categories.

Attack checked: unsupported generic or nullable categories must not be guessed
as move-only. The collector ignores unsupported categories.

Attack checked: assignment values and local initializers must both be recorded
when their value expression is a resolved move-only local name. The regression
test covers both forms.

No invalid-use diagnostic, branch propagation, borrow, lifetime, destructor,
thread-safety, async, unsafe, or FFI behavior is implemented by this task.

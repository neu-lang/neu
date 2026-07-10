# Soundness Report: M0022-004

- Task ID: `M0022-004`
- Milestone: `M0022`
- Decision: `pass`

Inputs read: `docs/tasks/M0022-004-use-after-move-diagnostics.md`,
`docs/adr/ADR-0035-bootstrap-ownership-and-move-analysis.md`,
`crates/newlang/src/ownership.rs`, and `crates/newlang/tests/ownership.rs`.

Attack checked: the transfer source use itself must not be diagnosed. The
analyzer diagnoses only uses whose node id is later than the transfer source.

Attack checked: moved state must not be cleared after the first invalid use.
The regression test expects two later uses to produce two diagnostics.

Attack checked: a later use of another binding must not be diagnosed. The
analyzer compares local-binding identity before reporting.

No branch propagation, borrow, lifetime, destructor, thread-safety, async,
unsafe, or FFI behavior is implemented by this task.

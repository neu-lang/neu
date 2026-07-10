# Soundness Report: M0014-006

## Metadata

- Task ID: `M0014-006`
- Milestone: `M0014`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0014-006-module-identity-model.md`
- Milestone file: `docs/milestones/M0014-module-package-and-visibility-model.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0021-lexical-grammar.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Changed files:
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/src/module.rs`
  - `crates/compiler/tests/module.rs`
  - `docs/tests/m0014-module-identity-model.sh`
  - `docs/milestones/M0014-module-package-and-visibility-model.md`
  - M0014 historical validators updated to allow in-scope `module.rs`.
- Ordinary test results:
  - Focused Rust module tests plus M0014 module identity and accepted ADR validators passed.

## Safety Invariants Checked

- [x] Ownership cannot be bypassed.
- [x] Moved values cannot be reused.
- [x] Shared and exclusive borrows cannot conflict.
- [x] Borrowed data cannot outlive its owner.
- [x] Nullability refinements cannot be used after invalidation.
- [x] Thread send/share capabilities are enforced.
- [x] Coroutine scopes cannot outlive allowed ownership or borrow lifetimes.
- [x] Borrows across suspension are rejected unless proven safe by accepted semantics.
- [x] Unsafe and FFI boundaries do not weaken safe-code guarantees.
- [x] Diagnostics do not hide or misstate safety failures.

## Attacks Attempted

```text
Attack: Smuggle host path identity into module metadata through SourceDatabase paths.
Expected result: Module ID remains the exact validated module-name string.
Actual result: Rust test proves different source paths with the same explicit module name produce the same module ID.
Source of truth: docs/adr/ADR-0025-module-package-visibility-model.md
Outcome: pass

Attack: Accept package names, malformed dots, Unicode identifier segments, or filesystem-like names as module identities.
Expected result: ModuleName::parse rejects malformed names using ADR-0021 identifier spelling.
Actual result: Rust tests reject leading dots, trailing dots, repeated dots, non-identifier segments, and Unicode segments.
Source of truth: docs/adr/ADR-0021-lexical-grammar.md, docs/adr/ADR-0025-module-package-visibility-model.md
Outcome: pass

Attack: Use this task to introduce visibility metadata, module dependencies, name resolution, manifests, or target triples.
Expected result: Those remain absent and deferred.
Actual result: Documentation validator rejects those terms in `crates/compiler/src/module.rs`; parser and name-resolution files remain untouched.
Source of truth: docs/adr/ADR-0025-module-package-visibility-model.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/compiler/tests/module.rs`
  - `docs/tests/m0014-module-identity-model.sh`
- Tests run:
  - `cargo test --workspace --all-targets module -- --nocapture && docs/tests/m0014-module-identity-model.sh && docs/tests/m0014-module-package-visibility-model-accepted.sh`
- Result:
  - `pass`

## Findings

No findings.

## Ambiguities

- None.

## Decision

Pass.

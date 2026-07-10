# Soundness Report: M0014-008

## Metadata

- Task ID: `M0014-008`
- Milestone: `M0014`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0014-008-visibility-metadata.md`
- Milestone file: `docs/milestones/M0014-module-package-and-visibility-model.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Changed files:
  - `crates/newlang/src/module.rs`
  - `crates/newlang/tests/module.rs`
  - `docs/tests/m0014-visibility-metadata.sh`
  - `docs/milestones/M0014-module-package-and-visibility-model.md`
  - `docs/tasks/M0014-008-visibility-metadata.md`
- Ordinary test results:
  - Focused Rust module tests plus M0014 visibility, package namespace, and module identity validators passed.

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
Attack: Add unaccepted visibility categories such as protected, friend, or sealed scope.
Expected result: Only public, internal, and private exist.
Actual result: VisibilityCategory has Public, Internal, and Private only; validator rejects protected/friend/sealed terms.
Source of truth: docs/adr/ADR-0025-module-package-visibility-model.md
Outcome: pass

Attack: Turn metadata representation into access checking or dependency lookup.
Expected result: No access checking, name resolution, or dependency lookup appears.
Actual result: module.rs stores category and origin only; validator rejects access-check, name-resolution, and dependency-lookup terms.
Source of truth: docs/adr/ADR-0025-module-package-visibility-model.md
Outcome: pass

Attack: Attach visibility to package or import declarations contrary to ADR-0025.
Expected result: Package and import declarations are rejected for visibility metadata.
Actual result: Rust tests reject PackageDeclaration and ImportDeclaration visibility records with a module diagnostic.
Source of truth: docs/adr/ADR-0025-module-package-visibility-model.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0014-visibility-metadata.sh`
  - visibility metadata tests in `crates/newlang/tests/module.rs`
- Tests run:
  - `cargo test --workspace --all-targets module -- --nocapture && docs/tests/m0014-visibility-metadata.sh && docs/tests/m0014-package-namespace-metadata.sh && docs/tests/m0014-module-identity-model.sh`
- Result:
  - `pass`

## Findings

No findings.

## Ambiguities

- None.

## Decision

Pass.

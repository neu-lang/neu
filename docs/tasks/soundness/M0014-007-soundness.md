# Soundness Report: M0014-007

## Metadata

- Task ID: `M0014-007`
- Milestone: `M0014`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0014-007-package-namespace-metadata.md`
- Milestone file: `docs/milestones/M0014-module-package-and-visibility-model.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0021-lexical-grammar.md`
  - `docs/adr/ADR-0022-declaration-syntax.md`
  - `docs/adr/ADR-0025-module-package-visibility-model.md`
- Changed files:
  - `crates/newlang/src/module.rs`
  - `crates/newlang/tests/module.rs`
  - `docs/tests/m0014-package-namespace-metadata.sh`
  - `docs/milestones/M0014-module-package-and-visibility-model.md`
  - `docs/tasks/M0014-007-package-namespace-metadata.md`
- Ordinary test results:
  - Focused Rust module tests plus M0014 package namespace and module identity validators passed.

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
Attack: Treat package namespace as part of module identity.
Expected result: Module ID remains the explicit module name, independent of package namespace.
Actual result: Rust tests show root and nested package metadata keep the same module ID when module name is the same.
Source of truth: docs/adr/ADR-0025-module-package-visibility-model.md
Outcome: pass

Attack: Accept malformed package namespaces that would later poison name resolution.
Expected result: PackageNamespace::parse rejects malformed ADR-0021 identifier segments.
Actual result: Rust tests reject leading dots, trailing dots, repeated dots, non-identifier segments, and Unicode segments.
Source of truth: docs/adr/ADR-0021-lexical-grammar.md, docs/adr/ADR-0025-module-package-visibility-model.md
Outcome: pass

Attack: Use package metadata task to introduce package manager behavior, imports, dependencies, or name resolution.
Expected result: Those concepts remain absent.
Actual result: Documentation validator rejects package manager, manifest, target triple, dependency, name resolution, symbol table, and import resolver terms in module implementation.
Source of truth: docs/adr/ADR-0025-module-package-visibility-model.md
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `docs/tests/m0014-package-namespace-metadata.sh`
  - package namespace tests in `crates/newlang/tests/module.rs`
- Tests run:
  - `cargo test --workspace --all-targets module -- --nocapture && docs/tests/m0014-package-namespace-metadata.sh && docs/tests/m0014-module-identity-model.sh`
- Result:
  - `pass`

## Findings

No findings.

## Ambiguities

- None.

## Decision

Pass.

# Task: M0033-001 Target-Pack Registry

## Task Metadata

- Task ID: `M0033-001`
- Milestone: `M0033`
- Milestone File: `docs/milestones/M0033-target-packs-and-cross-compilation-smoke.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Provide one explicit target-pack registry entry point that maps a requested
target triple to its bundled pack directory and manifest, and reports an
unknown target without consulting host tools or `PATH`.

## Authority Extract

- ADR-0020 requires explicit target triples, bundled target packs, standard
  layout rules, and no hidden host dependency for ordinary builds.
- M0033 requires target capabilities to be read from bundled metadata and an
  unknown target to fail explicitly.
- Existing `TargetPack::resolve_toml` validates one caller-selected pack root;
  this task adds discovery around that existing contract without changing its
  manifest semantics.

## Scope

- Define the registry API and error for a missing target-pack directory or
  manifest.
- Resolve `target-packs/<requested-triple>/manifest.toml` beneath an explicit
  repository or installation root.
- Reuse `TargetPack::resolve_toml` for manifest and artifact validation.
- Add focused positive and negative tests.

## Out Of Scope

- Adding a second target pack or cross-target linker artifacts.
- Target capability semantics beyond the fields already in the manifest.
- Compiler language semantics, ABI changes, or backend lowering changes.
- PATH lookup, host linker discovery, or distribution packaging.

## Tests

- A registry resolves the existing host pack when given its explicit root and
  `Triple::host()`.
- A request for an unavailable triple returns the dedicated unknown-target
  error.
- A pack manifest mismatch remains rejected by the existing resolver.

## Acceptance Criteria

- Target selection is represented by an explicit `target_lexicon::Triple`.
- Registry resolution reads only the explicit bundled-pack root.
- Unknown targets produce a stable, testable error.
- Existing target-pack validation remains the single validation path.
- No source file invokes `PATH` search or host-toolchain discovery.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0033
  requires explicit target selection and an unknown-target negative path.
  handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the
  registry integration tests could not compile because the registry API and
  unknown-target error did not exist. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=the
  explicit-root registry delegates manifest and artifact validation to
  `TargetPack::resolve_toml`. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  focused registry suite passed two tests. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=the
  validator confirms no PATH or host-toolchain lookup and existing resolver
  security checks remain active. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=scope,
  maintainability, and M0033/ADR-0020 compliance reviewed. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=`cargo fmt --all
  --check`, registry validator, Clippy with warnings denied, workspace tests,
  and `git diff --check` passed. handoff=commit

## Required Outputs

- Authority read: `docs/SPEC.md`, ADR-0020, and M0033.
- Files changed: this task, target-pack registry implementation, focused tests,
  and any narrowly scoped validator required by the task.
- Tests written before implementation and expected pre-implementation failure:
  registry tests must fail because the registry API does not yet exist.
- Validation commands and results: `cargo fmt --all --check`,
  `docs/tests/m0033-target-pack-registry.sh`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo test --workspace --all-targets`, and
  `git diff --check` all passed.
- Open questions or `none`.
- Remaining risk: non-host target artifacts and cross-compilation execution
  remain future M0033 tasks. Next main-task action: commit locally without
  pushing.

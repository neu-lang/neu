# Task: M0033-003 Cross-Target Object Smoke

## Task Metadata

- Task ID: `M0033-003`
- Milestone: `M0033`
- Milestone File: `docs/milestones/M0033-target-packs-and-cross-compilation-smoke.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Prove that the compiler can lower the bootstrap `Int` subset for one explicit
non-host target and link the result with that target pack, without executing a
foreign-architecture binary on the host.

## Authority Extract

- ADR-0020 requires explicit bundled target packs and no hidden host
  dependency.
- ADR-0046 defines bootstrap `Int` as signed 64-bit and defers cross-target
  behavior until M0033.
- ADR-0057 requires pack-owned linker and startup artifacts.
- ADR-0058 requires an explicit validated capability profile.

## Scope

- Add target-aware Cranelift object emission for the existing bootstrap MIR
  subset.
- Add an `x86_64-unknown-linux-gnu` target pack with an ELF startup shim and
  pack-owned linker artifact.
- Add target-specific linker argument construction.
- Link a cross-target object and validate the resulting ELF format and entry
  symbols.

## Out Of Scope

- Executing the foreign target binary on the host.
- CLI arguments, standard library, platform APIs, or FFI.
- A complete target matrix or stable public ABI.
- Non-bootstrap runtime values or target-specific language semantics.

## Tests

- Target-aware Cranelift emission produces an ELF x86-64 object.
- The registry resolves the explicit non-host pack and validates its profile.
- The linker uses target-specific GNU-style arguments and emits an ELF file.
- Unknown targets and unsupported target architectures remain rejected.

## Acceptance Criteria

- The target triple is passed explicitly to backend and linker selection.
- The generated cross-target object is not a host object.
- Linker and startup artifacts are resolved only from the target pack.
- The linked output parses as the declared ELF executable format.
- The smoke does not claim foreign executable runtime success.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=toolchain
  probe confirmed Apple clang can assemble x86_64 ELF and the cross smoke can
  be validated without foreign execution. handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=fail evidence=the cross
  integration test first failed because target-aware emission was absent, then
  reached the expected missing-pack failure. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=Cranelift
  target selection, x86 GNU link arguments, ELF startup shim, and target pack
  artifacts are present. handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  cross-target smoke links and parses an x86-64 ELF output. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=
  foreign execution is deliberately excluded, target selection is explicit,
  and unsupported architectures remain rejected. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0020,
  ADR-0046, ADR-0057, ADR-0058, M0033, and task scope are aligned. handoff=ci
- 2026-07-11 main_task=main phase=ci result=pass evidence=the cross-target
  validator, formatter, Clippy with warnings denied, workspace tests, and diff
  checks passed. handoff=commit

## Required Outputs

- Authority read: ADR-0020, ADR-0046, ADR-0057, ADR-0058, M0033, and SPEC.md.
- Files changed: this task, target-aware backend/linker code, cross-target pack
  artifacts, tests, and validators.
- Tests written before implementation and expected pre-implementation failure:
  the cross-target integration test must fail because target-aware emission,
  the non-host pack, and GNU argument handling do not exist.
- Validation commands and results: `docs/tests/m0033-cross-target-pack.sh`,
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D
  warnings`, `cargo test --workspace --all-targets`, and `git diff --check`
  all passed.
- Open questions or `none`.
- Remaining risk: the foreign executable is format-validated but not executed;
  broader target matrices remain future work. Next main-task action: commit
  locally without pushing.

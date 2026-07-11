# Task: M0033-004 Target-Pack Inventory Validation

## Task Metadata

- Task ID: `M0033-004`
- Milestone: `M0033`
- Milestone File: `docs/milestones/M0033-target-packs-and-cross-compilation-smoke.md`
- Specification: `docs/SPEC.md`
- Status: `in_progress`

## Goal

Make bundled target-pack discovery deterministic and validate that every
enumerated pack directory agrees with its manifest target triple.

## Authority Extract

- ADR-0020 requires standard bundled target-pack layout and explicit target
  triples.
- ADR-0057 requires pack-relative validated artifacts.
- ADR-0058 requires validated per-pack capability profiles.
- M0033 requires target metadata to be read from bundled metadata.

## Scope

- Enumerate direct target-pack directories beneath an explicit root.
- Parse each manifest and reject directory/manifest target mismatches.
- Return deterministic target ordering for diagnostics and tooling.
- Add repository validation covering all checked-in target packs.

## Out Of Scope

- New target artifacts or target-specific ABI semantics.
- Host `PATH` discovery or implicit target selection.
- Executing foreign binaries.
- Distribution archives or release hosting.

## Tests

- The repository inventory contains the host and x86 target packs in stable
  order.
- A directory whose manifest declares another triple is rejected.
- Non-directory files beneath the explicit root are ignored.

## Acceptance Criteria

- Inventory reads only direct children of the explicit target-pack root.
- Every returned target has a manifest-declared matching triple.
- Ordering is deterministic and independent of filesystem iteration order.
- All checked-in packs pass manifest, capability, artifact, and startup-shim
  validation.

## Execution Log

- 2026-07-11 main_task=main phase=create-task result=pass evidence=M0033 now
  contains two bundled packs and needs deterministic inventory validation before
  release hardening. handoff=test-first
- 2026-07-11 main_task=main phase=test-first result=fail evidence=inventory
  tests failed because TargetPackRegistry had no enumeration API. handoff=implementation
- 2026-07-11 main_task=main phase=implementation result=pass evidence=direct
  target directories are sorted and manifest target identity is checked.
  handoff=ordinary-tests
- 2026-07-11 main_task=main phase=ordinary-tests result=pass evidence=the
  focused inventory suite passed two tests. handoff=adversarial-review
- 2026-07-11 main_task=main phase=adversarial-review result=pass evidence=
  mismatched directory and manifest identity is rejected and non-direct files
  are not enumerated. handoff=review
- 2026-07-11 main_task=main phase=review result=pass evidence=ADR-0020,
  ADR-0057, ADR-0058, M0033, and scope reviewed. handoff=ci

## Required Outputs

- Authority read: ADR-0020, ADR-0057, ADR-0058, M0033, and SPEC.md.
- Files changed: this task, registry implementation, focused tests, and the
  repository inventory validator.
- Tests written before implementation and expected pre-implementation failure:
  inventory tests must fail because no enumeration API exists.
- Validation commands and results: `docs/tests/m0033-target-pack-inventory.sh`,
  `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D
  warnings`, `cargo test --workspace --all-targets`, and `git diff --check`
  all passed.
- Open questions or `none`.
- Remaining risk: release packaging and broader target matrices remain future
  work. Next main-task action: complete CI evidence and commit locally.

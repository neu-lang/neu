# M0033: Target Packs And Cross Compilation Smoke

## Title

M0033: Target Packs And Cross Compilation Smoke

## Identifier

M0033

## Goal

Introduce bundled target-pack metadata and a cross-compilation smoke path.

## Motivation

Go-like cross compilation is a core project requirement and must be validated before release hardening.

## Background

ADR-0020 selects bundled target packs with explicit target triples, standard layout rules, platform capability declarations, and no hidden host dependency.

## Prerequisites

- M0032

## Inputs

- Object and linker pipeline from M0032.
- `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`
- Build system and CI from M0002.

## Outputs

- Target-pack metadata format.
- Initial target capability declarations.
- Cross-compilation smoke test for one non-host target where feasible.

## Scope

- Target-pack structure.
- Explicit target triple handling.
- Smoke-level cross build.

## Out of Scope

- Complete target matrix.
- Platform standard library.
- ABI stabilization.

## Deliverables

- Target-pack metadata files.
- Cross-compilation command.
- CI or local smoke test.

## Acceptance Criteria

- Target triple is explicit in the compile command.
- Target capabilities are read from bundled metadata.
- Cross-compilation smoke path does not rely on hidden host toolchain behavior.

## Test Strategy

- Target metadata validation tests.
- Cross-target smoke test.
- Negative test for unknown target triple.

## Risks

- Platform ABI and startup semantics may be under-specified.
- Non-host linker availability may require toolchain packaging decisions.

## Estimated Effort

3-5 working days.

## Expected Files Changed

- Target-pack metadata.
- Build configuration.
- Backend target-selection files.
- Tests.

## Completion Checklist

- [ ] Target metadata exists.
- [ ] Explicit target selection works.
- [ ] Cross-compilation smoke path is tested.

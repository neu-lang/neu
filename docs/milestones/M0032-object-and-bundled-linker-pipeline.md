# M0032: Object And Bundled Linker Pipeline

## Title

M0032: Object And Bundled Linker Pipeline

## Identifier

M0032

## Goal

Produce object files and link them through the planned bundled linker path for the initial host target.

## Motivation

The architecture requires object files and a bundled linker before executable production can be validated.

## Background

ADR-0020 requires no hidden host dependency for ordinary builds.

## Prerequisites

- M0031

## Inputs

- Cranelift backend from M0031.
- Build system from M0002.
- `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`
- `docs/adr/ADR-0047-bootstrap-object-link-runtime-model.md`
- `docs/adr/ADR-0056-bootstrap-function-symbol-identity.md`

## Outputs

- Object emission pipeline.
- Bundled linker integration for initial host target.
- Executable smoke test.

## Scope

- Initial host target object and executable path.
- Build integration.

## Out of Scope

- Multi-target packs.
- Full platform ABI matrix.
- Release packaging.

## Deliverables

- Object emission command.
- Link command through bundled linker.
- Executable smoke test.

## Acceptance Criteria

- Compiler can produce an object file for the initial smoke program.
- Compiler can produce an executable for the initial host target without hidden host linker dependency.
- Executable smoke returns the ADR-0040 `main` result as the process exit code.
- main-task build check confirms documented toolchain inputs.

## Test Strategy

- Object file smoke test.
- Executable smoke test.
- CI build gate for host target.

## Risks

- Bundled linker choice may require separate toolchain decision.
- Entry point and runtime startup semantics may be under-specified.

## Estimated Effort

3-5 working days.

## Expected Files Changed

- Backend object pipeline.
- Linker integration files.
- Build scripts.
- Tests.

## Completion Checklist

- [ ] Object emission works.
- [ ] Bundled linker path works for host smoke.
- [ ] Hidden host dependencies are documented or eliminated.

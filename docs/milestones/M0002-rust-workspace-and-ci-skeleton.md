# M0002: Rust Workspace And CI Skeleton

## Title

M0002: Rust Workspace And CI Skeleton

## Identifier

M0002

## Goal

Create the Rust compiler workspace and minimal CI skeleton without implementing compiler behavior.

## Motivation

Autonomous implementation main tasks need a stable project layout and repeatable validation commands before adding compiler stages.

## Background

The compiler is planned to be implemented in Rust with Cranelift initially. Build mechanics must not introduce language semantics.

## Prerequisites

- M0001

## Inputs

- `docs/SPEC.md`
- `docs/adr/ADR-0020-portability-targets-and-platform-semantics.md`
- `docs/main task rules`
- `main task rules`

## Outputs

- Rust workspace layout.
- CI configuration for formatting, linting, and tests.
- Empty or placeholder crates with no compiler semantics.

## Scope

- Workspace metadata.
- CI command definitions.
- Build documentation.

## Out of Scope

- Lexer, parser, semantic analysis, or backend code.
- Target-pack implementation.
- Optimization.

## Deliverables

- Workspace manifest.
- Build and CI documentation.
- CI gate list matching `main task rules`.

## Acceptance Criteria

- Formatting, lint, and empty test commands run successfully.
- CI file exists and invokes the documented commands.
- No compiler behavior is implemented.

## Test Strategy

- Run formatting check.
- Run lint check.
- Run test command.

## Risks

- Choosing crate boundaries that prematurely encode architecture.
- Adding backend dependencies before they are needed.

## Estimated Effort

2-3 working days.

## Expected Files Changed

- Rust workspace manifests.
- CI configuration.
- Build documentation.

## Completion Checklist

- [x] Workspace commands are documented.
- [x] CI gates exist.
- [x] No compiler semantics are implemented.

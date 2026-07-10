# M0017: Type Representation

## Title

M0017: Type Representation

## Identifier

M0017

## Goal

Create the internal type representation needed by type checking without implementing full checking.

## Motivation

Type checking, nullability, generics, ownership, and capabilities need a shared type model.

## Background

ADR-0010 selects nominal types and interfaces or protocols. ADR-0006 defines nullable surface types as explicit optional values semantically.

## Prerequisites

- M0016

## Inputs

- Resolved names from M0016.
- `docs/adr/ADR-0006-nullability-and-absence.md`
- `docs/adr/ADR-0010-type-system-shape.md`
- `docs/adr/ADR-0016-generics-and-parametric-polymorphism.md`

## Outputs

- Type representation for approved type forms.
- Type identity and display rules for diagnostics.

## Scope

- Nominal type identity.
- Primitive scalar type categories.
- Nullable type representation where specified.
- Generic placeholder representation.

## Out of Scope

- Type inference.
- Constraint solving.
- Ownership capabilities.
- Layout or ABI.

## Deliverables

- Type model.
- Tests for type identity and display.

## Acceptance Criteria

- Named types map to nominal identities.
- Nullable type representation is distinguishable from non-nullable type representation.
- Unsupported type forms are rejected or marked blocked.

## Test Strategy

- Unit tests for type identity.
- Snapshot tests for diagnostic display strings.

## Risks

- Primitive type set may be unspecified.
- Interface/protocol conformance details may be incomplete.

## Estimated Effort

3-5 working days.

## Expected Files Changed

- Type model files.
- Tests.
- Diagnostic display snapshots.

## Completion Checklist

- [x] Type identity is represented.
- [x] Nullable types are represented.
- [ ] Unsupported type forms are blocked.

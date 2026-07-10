# Soundness Report: M0016-022

## Metadata

- Task ID: `M0016-022`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-022-parser-type-name-reference-metadata.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/newlang/src/parser.rs`
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Ordinary test results:
  - `cargo test -p newlang --test parser`: pass
  - `docs/tests/m0016-name-resolution-data-model.sh`: pass
  - `docs/tests/m0016-name-resolution-policy-accepted.sh`: pass

## Safety Invariants Checked

- [x] Type-name metadata does not bind or resolve names.
- [x] Type-name metadata preserves the `NamedType` node id and source span.
- [x] Generic arguments do not reorder type-name metadata away from source encounter order.
- [x] Package declarations, import declarations, and expression identifiers remain excluded from type-name metadata.
- [x] Import, cross-module, package-qualified, member, overload, extension, and type-directed lookup remain inactive.

## Attacks Attempted

```text
Attack: Use nested generic, nullable, grouped, and function types to reorder metadata.
Expected result: References are recorded in source encounter order.
Actual result: Covered by records_named_type_reference_metadata and type_name_reference_metadata_records_grouped_and_function_types_in_order.
Source of truth: M0016-022 Scope.
Outcome: pass

Attack: Smuggle package/import/expression identifiers into type-name metadata.
Expected result: No type-name metadata for those identifiers.
Actual result: Covered by type_name_reference_metadata_excludes_package_import_expression_and_missing_types.
Source of truth: ADR-0026 included and excluded reference nodes.
Outcome: pass

Attack: Add type-name binding while recording parser metadata.
Expected result: No name-resolution table or type binding behavior changes.
Actual result: Change is confined to parser metadata and tests.
Source of truth: M0016-022 Out Of Scope.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/parser.rs`
- Tests run:
  - `cargo test -p newlang --test parser`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Result:
  - pass

## Findings

No blocking, high, medium, or low findings.

## Ambiguities

- None for metadata-only `NamedType` recording.

## Decision

Pass.

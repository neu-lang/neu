# Soundness Report: M0016-019

## Metadata

- Task ID: `M0016-019`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-019-parser-name-reference-metadata.md`
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

- [x] Simple identifier expression references preserve AST node id, text, and source span.
- [x] Member names after `.` are not recorded as independent references.
- [x] Import and package declaration names are not recorded as expression references.
- [x] Metadata collection does not perform lookup.
- [x] Metadata collection does not activate imports, member lookup, overload resolution, or type-directed lookup.

## Attacks Attempted

```text
Attack: Parse multiple simple identifier expressions in a function body.
Expected result: Metadata records only simple expression references in parser encounter order.
Actual result: records_simple_identifier_expression_name_references covers this behavior.
Source of truth: ADR-0026 resolvable simple identifier expression nodes.
Outcome: pass
```

```text
Attack: Parse package, import, and member access names.
Expected result: Package/import names and member names are excluded; object and argument simple identifiers remain recorded.
Actual result: name_reference_metadata_excludes_member_import_and_package_names covers this behavior.
Source of truth: ADR-0026 excluded name-reference nodes.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0016-019-parser-name-reference-metadata.md`
- Result:
  - pass

## Findings

No blocker, high, medium, or low severity findings.

## Ambiguities

- None for simple identifier expression metadata.

## Decision

Pass. Parser metadata now exposes simple identifier expression references without implementing lookup.

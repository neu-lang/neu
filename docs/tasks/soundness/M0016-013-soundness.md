# Soundness Report: M0016-013

## Metadata

- Task ID: `M0016-013`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-013-parser-local-binding-metadata.md`
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

- [x] Valid local `val` and `var` statement names are available for later name-resolution binding.
- [x] Malformed local variable declarations do not create binding metadata.
- [x] Binding metadata preserves AST node id, name text, source span, and `val` or `var` kind.
- [x] Parser metadata does not assign lexical scope ids before scope construction is implemented.
- [x] Parser metadata does not build or query `LocalBindingIndex`.
- [x] Parser metadata does not implement local lookup, duplicate diagnostics, or declaration-order visibility.

## Attacks Attempted

```text
Attack: Parse both val and var declarations and check that their binding kind is preserved.
Expected result: Metadata records Val for val and Var for var in source order.
Actual result: records_local_val_and_var_binding_name_metadata covers this behavior.
Source of truth: ADR-0026 local val/var binding positions.
Outcome: pass
```

```text
Attack: Parse a malformed local declaration followed by a valid one.
Expected result: Only the valid declaration records local binding metadata.
Actual result: local_binding_name_metadata_excludes_malformed_declarations covers this behavior.
Source of truth: M0016-013 scope and parser recovery requirements.
Outcome: pass
```

```text
Attack: Check whether parser metadata introduced scope ids, local binding index construction, or name lookup.
Expected result: No scope ids, `LocalBindingIndex`, or resolver orchestration in parser code.
Actual result: M0016 data-model validator rejects these parser patterns.
Source of truth: M0016-013 out-of-scope section.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/parser.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0016-013-parser-local-binding-metadata.md`
- Result:
  - pass

## Findings

No blocker, high, medium, or low severity findings.

## Ambiguities

- None for this parser-metadata-only slice.

## Decision

Pass. The parser now exposes local binding metadata needed by M0016 without implementing lexical scope construction or lookup.

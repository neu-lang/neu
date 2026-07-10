# Soundness Report: M0016-016

## Metadata

- Task ID: `M0016-016`
- Milestone: `M0016`
- Filed By: `main-task adversarial check`
- Date: `2026-07-10`
- Decision: `pass`

## Inputs Read

- Task file: `docs/tasks/M0016-016-build-local-scope-tree.md`
- Milestone file: `docs/milestones/M0016-name-resolution-pass.md`
- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0026-name-resolution-policy.md`
  - `docs/adr/ADR-0024-expression-statement-pattern-syntax.md`
- Changed files:
  - `crates/newlang/src/name_resolution.rs`
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Ordinary test results:
  - `cargo test -p newlang --test name_resolution`: pass
  - `docs/tests/m0016-name-resolution-data-model.sh`: pass
  - `docs/tests/m0016-name-resolution-policy-accepted.sh`: pass

## Safety Invariants Checked

- [x] Only `Block` and `DeclarationBody` AST nodes create local scopes.
- [x] Containing block scopes are created before child block scopes.
- [x] Nested block scopes inherit from the nearest containing block scope.
- [x] Declaration body scopes remain roots and do not inherit enclosing declaration-body locals.
- [x] Non-scope-owner AST nodes do not create scopes.
- [x] Builder does not assign local bindings to scopes.
- [x] Builder does not implement local lookup or declaration-order visibility.

## Attacks Attempted

```text
Attack: Parse a function block with nested if/else blocks.
Expected result: Three block scopes are created in source order, and both nested blocks parent to the outer function block.
Actual result: builds_local_scope_tree_for_parser_blocks_in_source_order covers this behavior.
Source of truth: ADR-0026 block lexical scope rule.
Outcome: pass
```

```text
Attack: Parse nested declaration bodies.
Expected result: Declaration body scopes exist but have no parent scope, preserving the no-local-inheritance rule for nested declarations.
Actual result: local_scope_tree_builder_keeps_declaration_bodies_as_roots covers this behavior.
Source of truth: ADR-0026 nested declaration-body inheritance rule.
Outcome: pass
```

```text
Attack: Build a scope tree from source, name-expression, and variable-declaration nodes only.
Expected result: No scopes are created.
Actual result: local_scope_tree_builder_ignores_non_scope_owner_nodes covers this behavior.
Source of truth: ADR-0026 scope-owner list.
Outcome: pass
```

## Adversarial Tests

- Tests added:
  - `crates/newlang/tests/name_resolution.rs`
  - `docs/tests/m0016-name-resolution-data-model.sh`
- Tests run:
  - `docs/scripts/adversarial-check.sh docs/tasks/M0016-016-build-local-scope-tree.md`
- Result:
  - pass

## Findings

No blocker, high, medium, or low severity findings.

## Ambiguities

- None for scope tree construction from existing AST scope-owner nodes.

## Decision

Pass. The builder constructs local scope ownership and block parentage without implementing binding assignment or lookup.

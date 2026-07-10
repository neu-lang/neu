# Ambiguity Report: M0022 Ownership Value Categories

## Metadata

- Report ID: `M0022-ownership-value-categories`
- Related Task: `M0022-001`
- Related Milestone: `M0022`
- Filed By: `main task`
- Date: `2026-07-11`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`
- ADRs:
  - `docs/adr/ADR-0001-ownership-model.md`
  - `docs/adr/ADR-0005-copy-move-and-value-categories.md`
- Milestone:
  - `docs/milestones/M0022-ownership-and-move-analysis.md`

## Exact Ambiguous Text Or Missing Rule

```text
Primitive scalar types copy. User-defined types move by default unless
explicitly declared copyable under strict language rules.
```

Missing rules:

- Which primitive scalar types exist in the bootstrap ownership subset.
- Which user-defined value forms are in scope for M0022 move analysis.
- Which contexts perform a move versus a copy.
- Whether assignment to a new local, function argument passing, return, capture,
  and `when` subject evaluation are in the initial move-site set.
- Which diagnostic identifiers, primary locations, and secondary move-origin
  locations are required for use-after-move.
- Whether explicitly copyable user-defined types are in scope now or deferred.

## Competing Interpretations

1. Treat every currently parsed primitive-looking builtin type name as copyable
   and every nominal type as move-only.
2. Limit M0022 to the typed-core primitive types already accepted by M0018 and
   defer all user-defined move-only values until structs/classes are specified.
3. Define a narrow bootstrap subset with `Int`, `Bool`, and no-payload enum
   values copyable or move-only by explicit accepted rule.
4. Block implementation until the language defines a complete value-category
   table and move-site table.

## Why Guessing Is Unsafe

- Copyability controls whether a use-after-move diagnostic is emitted; choosing
  the wrong primitive set changes accepted programs.
- Move sites determine ownership transfer, destruction obligations, and later
  borrow-checker inputs.
- Diagnostics are semantic requirements under ADR-0015; inventing names or
  locations would make future diagnostic compatibility unstable.
- Treating enums or future user-defined types as copyable or move-only without
  authority can break M0021 match behavior and M0023 borrow analysis.

## Affected Work

- Tasks blocked:
  - `M0022-002` and later implementation tasks.
- Milestones affected:
  - `M0022`
  - `M0023`
  - `M0024`
- Tests blocked:
  - Positive primitive-copy fixtures.
  - Negative use-after-move fixtures.
  - Diagnostic snapshots for move origin and invalid use.
- Implementation areas blocked:
  - Ownership analysis pass.
  - Copyability checks.
  - Use-after-move diagnostics.

## Recommended Resolution Path

- [x] main-task language review determines whether existing text resolves it.
- [x] main-task semantic design drafts ADR or spec revision if new semantics are required.
- [x] main-task adversarial check reviews soundness risk.
- [x] main-task diagnostics check reviews diagnostic consequences.
- [x] main-task simplicity check reviews complexity.
- [x] main task approves final resolution.

## Temporary Rule

No implementation may proceed on M0022 ownership or move analysis until the
source of truth defines a bootstrap value-category and move-site subset.

## Resolution

- Decision:
  - ADR-0035 accepted. M0022 has a bootstrap ownership and move-analysis
    subset with copyable `Bool`, `Int`, `Unit`, and `Null`, move-only
    `String`, move-only current-module nominal identities, local-name
    initializer and assignment transfer sites, and `use_after_move`
    diagnostics.
- Source of truth updated:
  - `docs/adr/ADR-0035-bootstrap-ownership-and-move-analysis.md`
  - `docs/SPEC.md`
- Date resolved:
  - `2026-07-11`

# Ambiguity Report: M0028 Static Integer Constant Expressions

## Metadata

- Report ID: `M0028-STATIC-INTEGER-CONSTANT-EXPRESSIONS`
- Related Task: `M0028-005`
- Related Milestone: `M0028`
- Filed By: `main-task language review`
- Date: `2026-07-11`
- Status: `resolved`
- Required Owner: `main-task semantic design`

## Ambiguous Or Missing Authority

- Specification: `docs/SPEC.md`, “ADR-0043: Bootstrap Integer Runtime
  Semantics”.
- ADR: `docs/adr/ADR-0043-bootstrap-integer-runtime-semantics.md`.
- Milestone: `docs/milestones/M0028-executable-expression-frontend-completion.md`.

## Exact Ambiguous Text Or Missing Rule

```text
“If the compiler can prove overflow for a constant expression in the bootstrap
subset, it reports integer_overflow.”

The ADR does not define “constant expression” or which sources of values may
be used to prove negative exponent, invalid shift count, zero divisor, or
overflow.
```

## Competing Interpretations

1. Only literal, grouped, unary, and binary expression trees count as constant.
2. Immutable local `const` bindings with constant initializers also count.
3. Any expression the current compiler can evaluate, including direct calls,
   counts.

## Why Guessing Is Unsafe

- The diagnostic boundary changes accepted versus rejected programs.
- Treating local bindings or calls as constants can introduce unapproved
  evaluation, ordering, recursion, and ownership semantics.
- Restricting to literal trees may omit diagnostics the language intends to
  require.

## Affected Work

- Blocked task: `M0028-005`.
- Blocked behavior: ADR-0043 static integer diagnostics.
- Affected implementation: `crates/compiler/src/type_check.rs`.

## Recommended Resolution Path

- Language Lawyer determines whether existing ADR-0042/0043 text resolves the
  constant-expression boundary.
- Language Designer records the boundary in a new or superseding accepted ADR.
- Diagnostics and adversarial review verify precise error coverage and avoid
  accidental evaluation semantics.

## Temporary Rule

Do not implement ADR-0043 static arithmetic diagnostics until the accepted
source of truth defines the bootstrap constant-expression boundary.

## Resolution

- Decision: ADR-0048 limits bootstrap constants to literal, grouped, unary,
  and accepted binary expression trees.
- Source of truth: `docs/adr/ADR-0048-bootstrap-integer-constant-expressions.md`
  and `docs/SPEC.md`.
- Date resolved: `2026-07-11`.

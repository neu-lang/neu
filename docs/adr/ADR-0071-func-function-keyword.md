# ADR-0071: Use `func` For Function Declarations

- Status: Accepted by the main-task Chief Architect
- Supersedes: the function-keyword spelling portions of ADR-0021 and ADR-0022

## Decision

Neu uses `func` as the only accepted source keyword for function and method
declarations. `func` is reserved in every identifier position. The historical
`fun` spelling remains lexically recognizable only so the parser can issue a
dedicated migration diagnostic with the old keyword's source span; it does not
declare a function and is never a compatibility alias.

The decision changes spelling only. Function signatures, ownership effects,
dispatch, constructors, ABI, symbols, diagnostics other than the migration
diagnostic, and runtime behavior remain governed by their existing accepted
contracts.

Top-level functions, class methods, and interface method declarations all use
the same `func` keyword. Recovery consumes the obsolete keyword through the
existing function declaration parser so the rest of the declaration remains
structurally recoverable while reporting `ObsoleteFunctionKeyword`.

Active examples, fixtures, and source-facing documentation use `func`. Older
ADR text remains historical evidence and is not an accepted source example.

## Consequences

- Existing source using `fun` must be migrated or receives a deterministic
  source-mapped diagnostic.
- No parser, HIR, MIR, ownership, dispatch, or backend semantic contract is
  otherwise changed.
- Later tasks must use `func` in all new source and examples.

## Dependencies

ADR-0021, ADR-0022, ADR-0041, ADR-0065, ADR-0070, and the existing parser,
HIR, MIR, and backend contracts.

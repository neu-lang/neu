# Parser Recovery Architecture

Status: M0010 architecture

Source of truth:

- `docs/diagnostics.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0021-lexical-grammar.md`
- `docs/syntax/grammar-authority-ledger.md`

## Scope

This document defines parser recovery architecture and diagnostic expectations before concrete grammar parsing exists.

No ambiguous syntax is parsed by this milestone. Declaration, type, expression, statement, and pattern grammar remain blocked by the M0008 ambiguity reports.

## Recovery Model

Parser recovery is based on local syntax errors plus synchronization token search.

When a parser milestone has accepted grammar authority, a parser may recover from an unexpected token by:

- emitting one syntax diagnostic with severity `error`
- assigning a primary span to the unexpected token or missing construct location
- skipping tokens until a synchronization token is reached
- resuming at the synchronization token when the relevant grammar permits it
- suppressing duplicate diagnostics for the same skipped region

The initial synchronization token set is architectural only:

- right brace
- semicolon
- end of file

Future syntax milestones may define additional synchronization tokens for specific grammar contexts.

## Diagnostic Shape

Every parser syntax diagnostic must follow `docs/diagnostics.md` and ADR-0015:

- severity
- user-facing message
- primary span
- optional secondary spans
- optional notes
- optional safe suggestions
- source-of-truth citation
- recovery action used

Messages must describe source-level syntax. They must not expose parser implementation internals.

## Synthetic Parser Error

M0010 uses a synthetic parser error to test diagnostic shape without accepting grammar.

The synthetic parser error is not a real language construct. It exists only to validate that future parser diagnostics can record a primary span and recovery action.

## Parser Block Rule

No parser task may accept concrete declaration, type, expression, statement, or pattern syntax until the relevant ambiguity report is resolved by accepted source of truth.

Token-consuming parser infrastructure may proceed only when it does not accept or reject concrete language syntax.

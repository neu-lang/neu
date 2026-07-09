# ADR-0005: Copy, Move, And Value Categories

## Question

Which values copy implicitly, and which move?

## Competing Designs

- Explicit marker for trivially copyable types.
- Everything moves unless explicitly cloned.
- Small primitive values copy, user types move by default.
- Compiler decides copyability structurally.

## Trade-offs

Explicit copy markers make API contracts clear.

Move-by-default is safe but may surprise Kotlin users.

Primitive-copy/user-move balances ergonomics and safety.

Structural copyability is convenient, but can make API changes breaking in subtle ways.

## Recommended Choice

Primitive scalar types copy; user-defined types move by default unless explicitly declared copyable under strict rules.

## Downstream Consequences

- Assignment, parameter passing, and capture rules must distinguish copy from move.
- Diagnostics must identify accidental use-after-move.
- Generic constraints need copyability predicates.

## Dependencies

- ADR-0001
- ADR-0010
- ADR-0016


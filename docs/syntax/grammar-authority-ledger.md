# Grammar Authority Ledger

Status: M0008 authority ledger

Source of truth: `docs/SPEC.md` and accepted ADRs under `docs/adr/`

This ledger records whether planned parser syntax has accepted grammar authority. It does not create syntax and must not be used as a substitute for an accepted syntax ADR.

## Classification Rules

- `specified`: accepted source of truth defines enough syntax for parser fixtures or implementation.
- `ambiguous`: accepted source of truth names a concept but does not define concrete grammar.
- `deferred`: the construct is intentionally outside the current parser milestones.

## Parser Construct Classification

| Construct | Classification | Authority | Owner | Blocking milestone | Notes |
| --- | --- | --- | --- | --- | --- |
| Token spellings | specified | ADR-0021 | Chief Architect | none | Lexer token spellings are accepted, but parser grammar is not. |
| Package declaration | specified | ADR-0022 | Chief Architect | none | Source-file position and qualified-name syntax are specified. |
| Import declaration | specified | ADR-0022 | Chief Architect | none | Import position, qualified-name syntax, and alias syntax are specified; wildcard and grouped imports are deferred. |
| Visibility modifier syntax | specified | ADR-0022 | Chief Architect | none | `public`, `private`, and `internal` placement is specified for declarations covered by ADR-0022. |
| Function declaration | specified | ADR-0022 | Chief Architect | none | Function declaration shell is specified; parameter contents, concrete type syntax, and statement bodies are deferred. |
| Struct declaration | specified | ADR-0022 | Chief Architect | none | Struct declaration shell is specified; constructors, fields, and properties are deferred. |
| Enum or sealed sum declaration | specified | ADR-0022 | Chief Architect | none | Enum declaration shell is specified; variants and sealed modifier spelling are deferred. |
| Interface declaration | specified | ADR-0022 | Chief Architect | none | Interface declaration shell is specified; default method bodies are deferred. |
| Type declaration | specified | ADR-0023 | Chief Architect | none | Named type references and grouped type forms are specified for the bootstrap grammar. |
| Generic parameter syntax | specified | ADR-0023 | Chief Architect | none | Generic parameter lists and optional capability-bound clauses are specified. |
| Generic argument syntax | specified | ADR-0023 | Chief Architect | none | Generic argument lists attach to named type references only. |
| Capability bound syntax | specified | ADR-0023 | Chief Architect | none | Multiple bounds use `&`; comma separates generic parameters. |
| Nullable type syntax | specified | ADR-0023 | Chief Architect | none | Nullable markers are postfix and bind to the immediately preceding primary type. |
| Function type syntax | specified | ADR-0023 | Chief Architect | none | Parenthesized function type parameters followed by `->` and return type are specified. |
| Expression grammar | ambiguous | none | Language Designer | M0013 | Operators have token spellings only; precedence, associativity, and expression forms are missing. |
| Statement grammar | ambiguous | none | Language Designer | M0013 | Control-flow keywords exist lexically, but statement grammar is missing. |
| Pattern grammar | ambiguous | ADR-0012 | Language Designer | M0013 | Exhaustive matching exists semantically, but pattern syntax is missing. |
| Coroutine syntax | ambiguous | ADR-0008, ADR-0009 | Language Designer | M0013 | Structured coroutine semantics exist, but syntax is missing. |
| Unsafe block syntax | ambiguous | ADR-0018 | Language Designer | M0013 | Unsafe boundaries exist, but surface grammar is missing. |
| Macro syntax | deferred | ADR-0019 | Chief Architect | future | Macros are deferred. |
| Compile-time evaluation syntax | deferred | ADR-0019 | Language Designer | future | Bounded compile-time evaluation exists semantically but is outside M0011-M0013 parser scope. |

## Parser Unblock List

Only token-consuming parser infrastructure may proceed before syntax ADRs. This includes parser input streams, cursor mechanics, delimiter balancing helpers, and diagnostic recovery scaffolding that does not accept or reject concrete language constructs.

Concrete parser fixtures may use ADR-0021 token spellings only when the expected behavior is token-stream handling rather than declaration, type, expression, statement, or pattern grammar.

M0011 declaration parser may proceed only for ADR-0022 constructs. Type placeholders, function body placeholders, deferred declaration forms, and all later parser milestones must continue to follow their own authority rows.

M0012 type and generic parser may proceed only for ADR-0023 constructs. Expression, statement, pattern, coroutine, unsafe, and deferred type forms remain blocked until accepted source of truth defines them.

## Parser Block List

- M0011 declaration parser is unblocked only for ADR-0022 declaration syntax.
- M0012 type and generic syntax parser is unblocked only for ADR-0023 type and generic syntax.
- M0013 expression, statement, and pattern parser is blocked on expression, statement, and pattern syntax authority.

## Required Ambiguity Reports

- `docs/ambiguities/M0008-declaration-syntax.md`
- `docs/ambiguities/M0008-type-generic-syntax.md`
- `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`

## Maintenance Rule

When a future accepted ADR or `docs/SPEC.md` update defines grammar, update this ledger in the same task that accepts the grammar. Existing implementation behavior is never authority.

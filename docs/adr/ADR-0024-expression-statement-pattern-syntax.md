# ADR-0024: Expression Statement And Pattern Syntax

Status: Accepted

## Question

What concrete expression grammar, statement grammar, block grammar, pattern grammar, operator precedence, coroutine syntax, unsafe block syntax, parser recovery, and parser diagnostic obligations should the language use for the bootstrap compiler?

## Competing Designs

1. Adopt Kotlin expression, statement, and `when` syntax directly.
2. Define a small Kotlin-like custom body grammar.
3. Define a Rust-like expression-oriented block and pattern grammar with Kotlin-like declarations.
4. Split expression, statement, and pattern syntax into separate ADRs.
5. Continue deferring body syntax until after name resolution and type representation milestones.

## Trade-offs

Adopting Kotlin syntax maximizes surface familiarity, but imports grammar and semantic interactions that are not yet accepted, including expression bodies, receiver forms, labels, lambdas, destructuring, platform-specific smart-cast edge cases, and coroutine modifiers.

A small Kotlin-like custom body grammar preserves the project syntax direction while forcing explicit decisions for precedence, block boundaries, statement termination, pattern forms, ownership scope, unsafe block syntax, coroutine syntax, and diagnostics.

A Rust-like expression-oriented grammar could align with ownership scopes and deterministic destruction, but it would conflict with the Kotlin-like syntax constraint unless heavily adapted.

Splitting expression, statement, and pattern syntax into separate ADRs reduces review scope, but M0013 needs an integrated decision because block grammar, pattern grammar, smart casts, and diagnostics interact.

Continuing to defer avoids premature syntax decisions, but leaves executable bodies, pattern matching, flow typing, and later semantic passes blocked.

## Decision

Define a small Kotlin-like custom body grammar for the bootstrap compiler.

This ADR specifies only the body forms required by the near-term frontend pipeline:

- expression grammar
- operator precedence and associativity
- statement grammar
- block grammar
- variable declaration statement syntax, if included
- assignment statement syntax, if included
- return and error-propagation statement syntax, if included
- `if` syntax
- loop syntax, if included
- `when` or match syntax, if included
- pattern grammar
- unsafe block syntax
- coroutine syntax, if any syntax is included in M0013
- parser recovery boundaries
- parser diagnostic categories

This ADR does not rely on Kotlin, Rust, Go, or existing compiler behavior as implicit authority.

## Concrete Grammar

### Body Grammar Overview

```text
body = block | semicolon-body
semicolon-body = `;`
block = `{` statement* expression? `}`
```

Blocks are syntactic bodies and ownership scopes in the accepted grammar. Whether a block has a value is decided later by type checking; the parser only recognizes an optional trailing expression.

Semicolon bodies remain valid declaration-body placeholders from ADR-0022.

### Block Syntax

```text
block = `{` statement* expression? `}`
```

Statements appear before an optional trailing expression. A trailing expression is an expression not followed by semicolon and immediately before `}`.

This ADR does not make every statement an expression. It only allows an optional final expression in a block.

### Statement Syntax

```text
statement = variable-declaration | assignment-statement | return-statement | expression-statement
variable-declaration = (`val` | `var`) identifier type-annotation? initializer? `;`
type-annotation = `:` type
initializer = `=` expression
assignment-statement = assignment-target `=` expression `;`
assignment-target = postfix-expression
return-statement = `return` expression? `;`
expression-statement = expression `;`
```

Statement separators are explicit semicolons in the bootstrap grammar.

`val` and `var` introduce local bindings syntactically. Binding mutability, ownership, moves, and borrow effects are later semantic checks.

Assignment is a statement, not an expression.

### Expression Grammar

```text
expression = assignment-expression
assignment-expression = logical-or-expression
logical-or-expression = logical-and-expression (`||` logical-and-expression)*
logical-and-expression = equality-expression (`&&` equality-expression)*
equality-expression = comparison-expression ((`==` | `!=`) comparison-expression)*
comparison-expression = additive-expression ((`<` | `>` | `<=` | `>=`) additive-expression)*
additive-expression = multiplicative-expression ((`+` | `-`) multiplicative-expression)*
multiplicative-expression = unary-expression ((`*` | `/` | `%`) unary-expression)*
unary-expression = (`!` | `-`) unary-expression | postfix-expression
postfix-expression = primary-expression postfix-suffix*
postfix-suffix = call-suffix | member-suffix
call-suffix = `(` argument-list? `)`
argument-list = expression (`,` expression)*
member-suffix = `.` identifier
primary-expression = literal-expression | name-expression | grouped-expression | if-expression
literal-expression = integer-literal | string-literal | `true` | `false` | `null`
name-expression = qualified-name
grouped-expression = `(` expression `)`
if-expression = `if` `(` expression `)` block (`else` block)?
```

`assignment-expression` is a naming placeholder for future extension; assignment is not accepted as an expression.

Calls and member access are included because they are needed for basic executable bodies. Indexing is deferred.

`if` is an expression syntactically only when both branches are present. Without `else`, it is parsed as an expression form but later semantic phases decide whether it can be used where a value is required.

### Operator Precedence And Associativity

| Precedence | Operators | Associativity |
| --- | --- | --- |
| 8 | call `()`, member `.` | left |
| 7 | unary `!`, unary `-` | right |
| 6 | `*`, `/`, `%` | left |
| 5 | `+`, `-` | left |
| 4 | `<`, `>`, `<=`, `>=` | left |
| 3 | `==`, `!=` | left |
| 2 | `&&` | left |
| 1 | `||` | left |

There is no assignment expression precedence in the bootstrap grammar because assignment is a statement.

### Pattern Grammar

```text
pattern = wildcard-pattern | literal-pattern | binding-pattern | qualified-case-pattern | grouped-pattern
wildcard-pattern = `_`
literal-pattern = integer-literal | string-literal | `true` | `false` | `null`
binding-pattern = identifier
qualified-case-pattern = qualified-name pattern-arguments?
pattern-arguments = `(` pattern (`,` pattern)* `)`
grouped-pattern = `(` pattern `)`
```

Pattern binding modes are deferred. The parser records binding positions only; move, borrow, copy, and smart-cast behavior are semantic checks.

This ADR defines pattern grammar without accepting match or `when` syntax. Pattern grammar is included so a later accepted match form can reuse it without changing binding syntax.

### Unsafe And Coroutine Syntax

Unsafe block syntax is deferred.

Coroutine syntax is deferred.

The lexer keywords `unsafe`, `break`, `continue`, `for`, `while`, and related tokens do not imply parser acceptance until an accepted ADR defines the corresponding grammar.

### Recovery Boundaries

Expression recovery boundaries are:

- comma
- semicolon
- right parenthesis
- right brace
- declaration-starting keyword
- end of file

Statement recovery boundaries are:

- semicolon
- right brace
- declaration-starting keyword
- end of file

Block recovery boundaries are:

- right brace
- declaration-starting keyword
- end of file

Pattern recovery boundaries are:

- comma
- right parenthesis
- arm delimiter, if a future accepted match grammar defines one
- right brace
- end of file

### Parser Diagnostics

Accepted expression, statement, and pattern syntax must define these diagnostic categories before parser implementation:

| Diagnostic | Primary span | Recovery action | Safe suggestion |
| --- | --- | --- | --- |
| `missing_expression` | expected expression position | skip to expression recovery boundary | none |
| `unexpected_token_in_expression` | unexpected token | skip to expression recovery boundary | none |
| `unsupported_expression_form` | unsupported expression-form token | skip to expression recovery boundary | none |
| `malformed_binary_expression` | operator token or binary-expression range | skip to expression recovery boundary | none |
| `malformed_call_expression` | call argument list range | skip to right parenthesis or expression boundary | none |
| `malformed_member_access` | dot token | skip to expression recovery boundary | none |
| `malformed_block` | block range or opening brace | skip to right brace or block boundary | none |
| `missing_statement` | expected statement position | skip to statement recovery boundary | none |
| `unexpected_token_in_statement` | unexpected token | skip to statement recovery boundary | none |
| `unsupported_statement_form` | unsupported statement-form token | skip to statement recovery boundary | none |
| `malformed_variable_declaration` | declaration range | skip to statement recovery boundary | none |
| `malformed_assignment` | assignment operator or target range | skip to semicolon or statement boundary | none |
| `malformed_return_statement` | return keyword or statement range | skip to statement recovery boundary | none |
| `malformed_conditional` | `if` keyword or conditional range | skip to expression recovery boundary | none |
| `malformed_pattern` | malformed pattern range | skip to pattern recovery boundary | none |
| `unsupported_pattern_form` | unsupported pattern token | skip to pattern recovery boundary | none |
| `missing_pattern_arm_body` | future arm delimiter | skip to future arm or block boundary | none |
| `malformed_unsafe_block` | `unsafe` keyword | skip to statement recovery boundary | none; unsafe block syntax is deferred |
| `malformed_coroutine_construct` | coroutine keyword or construct range | skip to statement recovery boundary | none; coroutine syntax is deferred |

All accepted body syntax diagnostics must cite ADR-0015 and ADR-0024.

Each diagnostic must define a primary span, recovery action, source-of-truth citation, and safe suggestion policy.

### Review Attack Cases

A block that moves a value and then uses it again is parsed as ordinary syntax only. Move validity is deferred to ownership analysis.

A pattern that binds a name records a binding position only. Binding mode and move-versus-borrow behavior are deferred.

A conditional that appears to refine a nullable value is parsed as syntax only. Smart-cast validity is deferred to flow typing.

Coroutine-like syntax remains malformed or unsupported because coroutine syntax is deferred.

Unsafe-like syntax remains malformed or unsupported because unsafe block syntax is deferred.

A future match or `when` arm with a missing body must produce `missing_pattern_arm_body` only after match syntax is accepted.

### Concrete Deferrals

The accepted grammar defers:

- match or `when` syntax
- unsafe block syntax
- coroutine syntax
- loops
- `break` and `continue`
- indexing
- lambdas and closures
- receiver lambdas
- destructuring declarations
- labels
- `try`/`catch` syntax
- error propagation syntax
- `defer` or scope guard syntax
- generator syntax
- async stream syntax
- inline assembly
- compile-time evaluation syntax
- annotations on expressions, statements, or patterns
- advanced pattern guards
- range patterns
- spread operators
- custom infix declarations
- operator overloading syntax

## Downstream Consequences

- M0013 parser fixtures may be created for ADR-0024 constructs.
- M0013 parser implementation may proceed only for ADR-0024 body constructs.
- M0016 name resolution depends on accepted binding positions in statements and patterns.
- M0018 type checking depends on expression precedence and block result rules.
- M0019 flow typing depends on accepted conditional and pattern syntax.
- M0021 exhaustiveness depends on accepted pattern forms.
- M0022 ownership and move analysis depends on accepted block and statement ownership scope.
- M0025 coroutine analysis depends on accepted coroutine syntax or explicit deferral.
- Unsafe and FFI checks depend on accepted unsafe block syntax or explicit deferral.

## Dependencies

- `docs/SPEC.md`
- `docs/adr/ADR-0007-error-handling.md`
- `docs/adr/ADR-0008-structured-concurrency-semantics.md`
- `docs/adr/ADR-0009-async-suspension-and-borrowing.md`
- `docs/adr/ADR-0011-flow-typing-and-smart-casts.md`
- `docs/adr/ADR-0012-pattern-matching-and-algebraic-data.md`
- `docs/adr/ADR-0015-diagnostics-as-semantics.md`
- `docs/adr/ADR-0018-unsafe-ffi-and-trust-boundaries.md`
- `docs/adr/ADR-0021-lexical-grammar.md`
- `docs/ambiguities/M0008-expression-statement-pattern-syntax.md`
- `docs/syntax/grammar-authority-ledger.md`
- Language Lawyer audit
- Adversarial Engineer review
- Diagnostics Engineer review
- Simplicity Guardian review
- Chief Architect approval

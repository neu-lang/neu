# ADR-0078: Optional Semicolons And Newline Termination

## Status

Accepted.

## Decision

Neu preserves line-boundary metadata on each lexer token instead of exposing a
newline token to the parser or to source programs. A line break is recorded
when whitespace or a comment between two tokens contains `\n` or `\r`. Newlines
inside string literals remain lexical errors under the existing string rules;
comments do not become tokens, but a newline in a comment still forms a line
boundary.

An ordinary statement may terminate implicitly at a line boundary when its
last token is a value, identifier, closing delimiter, postfix operator, or
`break`/`continue`, and the next token cannot continue the same expression.
Explicit semicolons remain valid, continue to terminate statements, and may
separate multiple statements on one line. A closing `}` or end of input also
terminates a completed statement without requiring a semicolon.

Newlines do not terminate expressions after binary or unary operators, after a
dot, after a comma, after an opening delimiter, or before a postfix call,
index, or member continuation. Parenthesized, bracketed, and argument/list
contents therefore permit line breaks according to their existing delimiters;
block braces continue to define blocks. A newline before `.` or `[` is a
continuation only when the preceding expression is otherwise complete, and a
newline before `(` continues a call. A newline after a completed expression
before an unrelated statement-starting token terminates the expression.

Declarations, assignments, expression statements, `break`, and `continue`
use these same terminators in function, class, interface, and top-level
contexts. A `return` expression must begin on the same logical line as
`return`; a line break immediately after `return` is a bare return. Bare
returns retain their existing type-checking rules. `else` attaches to the
immediately preceding `if` across line breaks, while a completed block may be
followed by another statement on the next line. Semicolons before or after a
closing brace are accepted as ordinary explicit separators.

Blank lines preserve boundaries only through the line-break metadata and do
not create additional syntax. Recovery synchronizes at semicolons, line
boundaries, closing braces, and declaration starters, preserving source spans
for the existing diagnostics. Type grammar and declaration headers do not use
implicit statement termination inside their existing delimiters.

No indentation semantics, new expression forms, operators, control-flow
forms, runtime behavior, ABI behavior, or backend behavior is introduced.

## Consequences

The AST continues to contain explicit statement nodes regardless of whether a
semicolon, newline, closing brace, or end of input supplied the terminator.
The frontend and all later compiler stages retain their existing contracts.
Malformed continuations remain parser diagnostics rather than being guessed as
separate statements.

## Dependencies

ADR-0024, ADR-0028, ADR-0045, ADR-0053, ADR-0060, ADR-0071, ADR-0077.

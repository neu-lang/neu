# ADR-0021: Lexical Grammar

Status: Accepted

## Question

What concrete lexical grammar should the language use for the bootstrap compiler?

## Competing Designs

1. Adopt Kotlin lexical grammar directly.
2. Define a small Kotlin-like custom lexical grammar.
3. Define an ASCII-only grammar and leave modern language affordances for later.
4. Continue deferring lexical grammar until parser work begins.

## Decision

Define a small Kotlin-like custom lexical grammar for the bootstrap compiler.

The grammar is intentionally smaller than Kotlin. Kotlin remains an ergonomic influence, not imported authority. Every token class accepted by the lexer must be listed here or in a later accepted source-of-truth update.

## Accepted Lexical Grammar

### Source Text

Source files are interpreted as UTF-8 text.

The lexer operates on Unicode scalar values for validity but reports source spans as byte offsets into the original source text. Invalid UTF-8 in file input is a source text error before lexing.

Unicode identifiers are deferred. Source text may contain Unicode in string literal contents, comments, and whitespace recognized by UTF-8 validity, but identifier start and continue rules are ASCII-only for the bootstrap compiler.

### Whitespace

The following characters are whitespace:

- space
- horizontal tab
- carriage return
- line feed
- form feed

Whitespace separates tokens and is not emitted as a token by the bootstrap lexer.

Carriage-return-line-feed sequences are treated as one line break for line tracking. A lone carriage return and a lone line feed are each line breaks.

### Comments

Line comments begin with `//` and continue until, but do not include, the next line break or end of file.

Block comments begin with `/*` and end with the matching `*/`.

Block comments nest. Each `/*` inside a block comment increases nesting depth, and each `*/` decreases nesting depth. A block comment ends only when nesting depth returns to zero.

Comments are not emitted as tokens by the bootstrap lexer.

### Identifiers

Identifier start characters are:

- ASCII letters `A` through `Z`
- ASCII letters `a` through `z`
- underscore `_`

Identifier continue characters are:

- identifier start characters
- ASCII digits `0` through `9`

An identifier that exactly matches an accepted keyword is emitted as that keyword token.

Unicode identifiers are deferred and must be rejected with a lexical diagnostic outside comments and string literal contents.

### Keywords

The accepted keyword set is:

```text
as
break
continue
else
enum
extern
false
for
fun
if
impl
import
in
interface
is
null
package
return
struct
true
type
unsafe
val
var
when
while
```

All keywords are reserved. The bootstrap lexer has no contextual keywords.

Future keywords require an accepted ADR or spec update.

### Integer Literals

The accepted integer literal forms are:

- decimal: one or more ASCII digits
- binary: `0b` or `0B` followed by one or more binary digits
- hexadecimal: `0x` or `0X` followed by one or more hexadecimal digits

Underscores may appear between digits in integer literals. Underscores may not appear at the start, at the end, adjacent to another underscore, or immediately after a base prefix.

Integer literal suffixes are deferred and rejected by the bootstrap lexer.

Integer literal overflow is not a lexer error. If an integer literal is structurally valid, range checking belongs to later semantic analysis.

### String Literals

The accepted string literal form is a double-quoted string:

```text
"..."
```

String contents may contain any Unicode scalar value except an unescaped double quote, an unescaped backslash, or an unescaped line break.

The accepted escape sequences are:

```text
\0
\n
\r
\t
\"
\\
```

Unicode escapes, raw strings, multiline strings, byte strings, character literals, and string interpolation are deferred and rejected by the bootstrap lexer.

### Operators And Delimiters

The accepted operator tokens are:

```text
++
--
==
!=
<=
>=
&&
||
->
=>
..
..<
+
-
*
/
%
=
<
>
!
&
|
.
?
:
```

The accepted delimiter tokens are:

```text
(
)
{
}
[
]
,
;
```

The lexer recognizes the longest accepted operator or delimiter sequence at a given source position.

This ADR defines token spelling only. It does not define parser grammar, precedence, associativity, or semantic meaning.

### Lexical Error Categories

The bootstrap lexer must report these lexical error categories:

- invalid source text encoding before lexing
- unsupported identifier character
- unknown character
- unterminated block comment
- malformed integer literal
- unsupported integer literal suffix
- unterminated string literal
- invalid string escape
- unsupported string literal form

Per ADR-0015, lexical diagnostics are part of the language contract for this lexical grammar.

### Source Span Rules

Token and diagnostic spans are half-open byte ranges in the original source file.

Valid token spans include the full token spelling and exclude surrounding whitespace and comments.

Line comment spans include the leading `//` and all comment body bytes, excluding the terminating line break.

Block comment spans include the opening `/*`, nested comment text, and the matching closing `*/`. An unterminated block comment diagnostic spans from the opening `/*` through end of file.

Malformed integer literal diagnostics span the full malformed literal candidate, including base prefix, digits, underscores, and rejected suffix bytes that are part of the same candidate.

Invalid string escape diagnostics span the backslash and escaped character. Unterminated string literal diagnostics span from the opening quote through the line break or end of file that terminates recovery.

Unsupported identifier character and unknown character diagnostics span the smallest UTF-8 scalar value or byte sequence that can be isolated without consuming a valid neighboring token.

## Trade-offs

This choice gives the compiler enough concrete lexical authority to proceed while avoiding accidental adoption of Kotlin's full grammar. It keeps bootstrap implementation and diagnostics tractable, but it defers Unicode identifiers, raw strings, interpolation, character literals, and numeric suffixes.

The explicit deferrals are intentional. Later accepted ADRs may expand the lexical grammar after parser, type checking, diagnostics, and user ergonomics are better understood.

## Consequences

- The compiler may proceed to concrete lexer fixtures and lexer implementation tasks.
- Lexer tests must use only token forms accepted by this ADR unless they are negative tests for explicitly rejected forms.
- Parser future work may reference token spellings from this ADR, but this ADR does not define parser grammar.
- Unicode identifier support requires a future accepted ADR or spec update.
- String interpolation, raw strings, and numeric suffixes require future accepted ADRs or spec updates.

## Dependencies

- ADR-0015 diagnostics as semantics.
- The compiler source database and byte span model.
- The compiler token model and fixture metadata.

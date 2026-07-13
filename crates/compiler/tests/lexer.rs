use compiler::lexer::{DiagnosticKind, TokenKind, lex};
use compiler::source::SourceFileId;

fn kinds(source: &str) -> Vec<TokenKind> {
    lex(SourceFileId::from_raw(0), source)
        .tokens
        .into_iter()
        .map(|token| token.kind)
        .collect()
}

fn diagnostic_texts(source: &str) -> Vec<(DiagnosticKind, &str)> {
    lex(SourceFileId::from_raw(0), source)
        .diagnostics
        .into_iter()
        .map(|diagnostic| {
            (
                diagnostic.kind,
                &source[diagnostic.span.start()..diagnostic.span.end()],
            )
        })
        .collect()
}

#[test]
fn lexes_keywords_and_identifiers() {
    assert_eq!(
        kinds("func main() { return true }"),
        vec![
            TokenKind::KwFunc,
            TokenKind::Identifier,
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::LeftBrace,
            TokenKind::KwReturn,
            TokenKind::KwTrue,
            TokenKind::RightBrace,
        ]
    );

    assert_eq!(
        kinds("alpha _beta value42 funny"),
        vec![
            TokenKind::Identifier,
            TokenKind::Identifier,
            TokenKind::Identifier,
            TokenKind::Identifier,
        ]
    );
}

#[test]
fn val_and_const_are_reserved_keywords() {
    assert_eq!(
        format!("{:?}", kinds("val const var")),
        "[KwVal, KwConst, KwVar]"
    );
}

#[test]
fn func_is_reserved_and_fun_remains_lexically_distinct_for_migration_diagnostics() {
    assert_eq!(kinds("func fun"), vec![TokenKind::KwFunc, TokenKind::KwFun]);
}

#[test]
fn rejects_unicode_identifiers_with_spans() {
    assert_eq!(
        diagnostic_texts("café"),
        vec![(DiagnosticKind::UnsupportedIdentifierCharacter, "é")]
    );
}

#[test]
fn lexes_literals_without_treating_integer_overflow_as_lexical() {
    assert_eq!(
        kinds("0 42 0b1010 0x2A 1_000 999999999999999999999999999999999999999"),
        vec![
            TokenKind::IntDecimal,
            TokenKind::IntDecimal,
            TokenKind::IntBinary,
            TokenKind::IntHex,
            TokenKind::IntDecimal,
            TokenKind::IntDecimal,
        ]
    );

    assert_eq!(kinds("\"a\\n\\t\\\"\\\\\\0\""), vec![TokenKind::String]);
}

#[test]
fn lexes_decimal_and_exponent_float_literals() {
    let output = lex(SourceFileId::from_raw(91), "1.5 2e3 4.0E-2");

    assert!(output.diagnostics.is_empty());
    assert_eq!(
        output
            .tokens
            .iter()
            .map(|token| token.kind)
            .collect::<Vec<_>>(),
        vec![
            TokenKind::FloatDecimal,
            TokenKind::FloatDecimal,
            TokenKind::FloatDecimal
        ]
    );
}

#[test]
fn rejects_malformed_float_exponents() {
    let output = lex(SourceFileId::from_raw(92), "1e+");

    assert_eq!(output.tokens, vec![]);
    assert_eq!(output.diagnostics.len(), 1);
    assert_eq!(
        output.diagnostics[0].kind,
        DiagnosticKind::MalformedFloatLiteral
    );
}

#[test]
fn skips_comments_and_reports_unterminated_block_comment() {
    assert_eq!(
        kinds("const // comment\nvar"),
        vec![TokenKind::KwConst, TokenKind::KwVar]
    );
    assert_eq!(
        kinds("/* outer /* inner */ done */ const"),
        vec![TokenKind::KwConst]
    );
    assert_eq!(
        diagnostic_texts("const /* open"),
        vec![(DiagnosticKind::UnterminatedBlockComment, "/* open")]
    );
}

#[test]
fn preserves_line_boundaries_without_emitting_newline_tokens() {
    let output = lex(SourceFileId::from_raw(901), "val first\n// gap\nval second");
    assert_eq!(
        output
            .tokens
            .iter()
            .map(|token| (token.kind, token.line_break_before))
            .collect::<Vec<_>>(),
        vec![
            (TokenKind::KwVal, false),
            (TokenKind::Identifier, false),
            (TokenKind::KwVal, true),
            (TokenKind::Identifier, false),
        ]
    );
}

#[test]
fn lexes_operators_and_delimiters_with_longest_match() {
    assert_eq!(
        kinds("++ -- == != <= >= && || -> => .. ..< + - * / % = < > ! & | . ? :"),
        vec![
            TokenKind::PlusPlus,
            TokenKind::MinusMinus,
            TokenKind::EqualEqual,
            TokenKind::BangEqual,
            TokenKind::LessEqual,
            TokenKind::GreaterEqual,
            TokenKind::AmpAmp,
            TokenKind::PipePipe,
            TokenKind::Arrow,
            TokenKind::FatArrow,
            TokenKind::DotDot,
            TokenKind::DotDotLess,
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
            TokenKind::Percent,
            TokenKind::Equal,
            TokenKind::Less,
            TokenKind::Greater,
            TokenKind::Bang,
            TokenKind::Amp,
            TokenKind::Pipe,
            TokenKind::Dot,
            TokenKind::Question,
            TokenKind::Colon,
        ]
    );

    assert_eq!(
        kinds("( ) { } [ ] , ;"),
        vec![
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::LeftBrace,
            TokenKind::RightBrace,
            TokenKind::LeftBracket,
            TokenKind::RightBracket,
            TokenKind::Comma,
            TokenKind::Semicolon,
        ]
    );
}

#[test]
fn lexes_executable_operator_tokens() {
    assert_eq!(
        kinds("** << >> ~ ^ & | + - * / %"),
        vec![
            TokenKind::StarStar,
            TokenKind::LessLess,
            TokenKind::GreaterGreater,
            TokenKind::Tilde,
            TokenKind::Caret,
            TokenKind::Amp,
            TokenKind::Pipe,
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
            TokenKind::Percent,
        ]
    );
}

#[test]
fn reports_lexical_errors_with_precise_spans() {
    assert_eq!(
        diagnostic_texts("\"bad\\q\""),
        vec![(DiagnosticKind::InvalidStringEscape, "\\q")]
    );
    assert_eq!(
        diagnostic_texts("\"bad"),
        vec![(DiagnosticKind::UnterminatedStringLiteral, "\"bad")]
    );
    assert_eq!(
        diagnostic_texts("0x_FF"),
        vec![(DiagnosticKind::MalformedIntegerLiteral, "0x_FF")]
    );
    assert_eq!(
        diagnostic_texts("42u32"),
        vec![(DiagnosticKind::UnsupportedIntegerLiteralSuffix, "42u32")]
    );
    let output = lex(SourceFileId::from_raw(10014), "@Test");
    assert!(output.diagnostics.is_empty());
    assert_eq!(output.tokens[0].kind, TokenKind::At);
}

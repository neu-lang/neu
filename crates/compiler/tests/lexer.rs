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
        kinds("fun main() { return true }"),
        vec![
            TokenKind::KwFun,
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
fn const_is_reserved_and_val_is_an_ordinary_identifier() {
    assert_eq!(
        kinds("const val var"),
        vec![TokenKind::KwConst, TokenKind::Identifier, TokenKind::KwVar]
    );
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
    assert_eq!(
        diagnostic_texts("@"),
        vec![(DiagnosticKind::UnknownCharacter, "@")]
    );
}

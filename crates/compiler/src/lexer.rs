use crate::source::{ByteSpan, SourceFileId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenKind {
    Identifier,
    IntDecimal,
    IntBinary,
    IntHex,
    String,
    KwAs,
    KwBreak,
    KwConst,
    KwContinue,
    KwElse,
    KwEnum,
    KwExtern,
    KwFalse,
    KwFor,
    KwFun,
    KwIf,
    KwImpl,
    KwImport,
    KwIn,
    KwInterface,
    KwIs,
    KwNull,
    KwPackage,
    KwReturn,
    KwStruct,
    KwTrue,
    KwType,
    KwUnsafe,
    KwVar,
    KwWhen,
    KwWhile,
    PlusPlus,
    MinusMinus,
    EqualEqual,
    BangEqual,
    LessEqual,
    GreaterEqual,
    AmpAmp,
    PipePipe,
    StarStar,
    LessLess,
    GreaterGreater,
    Arrow,
    FatArrow,
    DotDotLess,
    DotDot,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    Less,
    Greater,
    Bang,
    Amp,
    Pipe,
    Caret,
    Tilde,
    Dot,
    Question,
    Colon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: ByteSpan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticKind {
    UnsupportedIdentifierCharacter,
    UnknownCharacter,
    UnterminatedBlockComment,
    MalformedIntegerLiteral,
    UnsupportedIntegerLiteralSuffix,
    UnterminatedStringLiteral,
    InvalidStringEscape,
    UnsupportedStringLiteralForm,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub span: ByteSpan,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LexOutput {
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn lex(file: SourceFileId, text: &str) -> LexOutput {
    Lexer::new(file, text).lex()
}

struct Lexer<'source> {
    file: SourceFileId,
    text: &'source str,
    offset: usize,
    tokens: Vec<Token>,
    diagnostics: Vec<Diagnostic>,
}

impl<'source> Lexer<'source> {
    fn new(file: SourceFileId, text: &'source str) -> Self {
        Self {
            file,
            text,
            offset: 0,
            tokens: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    fn lex(mut self) -> LexOutput {
        while !self.is_eof() {
            if self.skip_whitespace() || self.skip_comment() {
                continue;
            }

            let start = self.offset;
            let Some(ch) = self.current_char() else {
                break;
            };

            if is_identifier_start(ch) {
                self.lex_identifier_or_keyword(start);
            } else if ch.is_ascii_digit() {
                self.lex_integer(start);
            } else if ch == '"' {
                self.lex_string(start);
            } else if ch.is_ascii() {
                self.lex_operator_or_delimiter_or_unknown(start);
            } else {
                let end = start + ch.len_utf8();
                self.diagnostic(DiagnosticKind::UnsupportedIdentifierCharacter, start, end);
                self.offset = end;
            }
        }

        LexOutput {
            tokens: self.tokens,
            diagnostics: self.diagnostics,
        }
    }

    fn skip_whitespace(&mut self) -> bool {
        let Some(ch) = self.current_char() else {
            return false;
        };
        if matches!(ch, ' ' | '\t' | '\r' | '\n' | '\u{000C}') {
            self.offset += ch.len_utf8();
            true
        } else {
            false
        }
    }

    fn skip_comment(&mut self) -> bool {
        if self.remaining().starts_with("//") {
            self.offset += 2;
            while let Some(ch) = self.current_char() {
                if matches!(ch, '\n' | '\r') {
                    break;
                }
                self.offset += ch.len_utf8();
            }
            return true;
        }

        if self.remaining().starts_with("/*") {
            let start = self.offset;
            self.offset += 2;
            let mut depth = 1usize;
            while !self.is_eof() {
                if self.remaining().starts_with("/*") {
                    depth += 1;
                    self.offset += 2;
                } else if self.remaining().starts_with("*/") {
                    depth -= 1;
                    self.offset += 2;
                    if depth == 0 {
                        return true;
                    }
                } else {
                    self.bump_char();
                }
            }
            self.diagnostic(
                DiagnosticKind::UnterminatedBlockComment,
                start,
                self.text.len(),
            );
            return true;
        }

        false
    }

    fn lex_identifier_or_keyword(&mut self, start: usize) {
        self.bump_char();
        while let Some(ch) = self.current_char() {
            if is_identifier_continue(ch) {
                self.bump_char();
            } else {
                break;
            }
        }

        let text = &self.text[start..self.offset];
        self.token(
            keyword_kind(text).unwrap_or(TokenKind::Identifier),
            start,
            self.offset,
        );
    }

    fn lex_integer(&mut self, start: usize) {
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.bump_char();
            } else {
                break;
            }
        }

        let text = &self.text[start..self.offset];
        match classify_integer(text) {
            IntegerClass::Token(kind) => self.token(kind, start, self.offset),
            IntegerClass::Malformed => {
                self.diagnostic(DiagnosticKind::MalformedIntegerLiteral, start, self.offset);
            }
            IntegerClass::UnsupportedSuffix => {
                self.diagnostic(
                    DiagnosticKind::UnsupportedIntegerLiteralSuffix,
                    start,
                    self.offset,
                );
            }
        }
    }

    fn lex_string(&mut self, start: usize) {
        self.offset += 1;
        while let Some(ch) = self.current_char() {
            match ch {
                '"' => {
                    self.offset += 1;
                    self.token(TokenKind::String, start, self.offset);
                    return;
                }
                '\n' | '\r' => {
                    let end = self.offset + ch.len_utf8();
                    self.diagnostic(DiagnosticKind::UnterminatedStringLiteral, start, end);
                    self.offset = end;
                    return;
                }
                '\\' => {
                    let escape_start = self.offset;
                    self.offset += 1;
                    let Some(escaped) = self.current_char() else {
                        self.diagnostic(
                            DiagnosticKind::UnterminatedStringLiteral,
                            start,
                            self.text.len(),
                        );
                        return;
                    };
                    let escape_end = self.offset + escaped.len_utf8();
                    if !matches!(escaped, '0' | 'n' | 'r' | 't' | '"' | '\\') {
                        self.diagnostic(
                            DiagnosticKind::InvalidStringEscape,
                            escape_start,
                            escape_end,
                        );
                    }
                    self.offset = escape_end;
                }
                _ => {
                    self.offset += ch.len_utf8();
                }
            }
        }

        self.diagnostic(
            DiagnosticKind::UnterminatedStringLiteral,
            start,
            self.text.len(),
        );
    }

    fn lex_operator_or_delimiter_or_unknown(&mut self, start: usize) {
        for (spelling, kind) in OPERATORS_AND_DELIMITERS {
            if self.remaining().starts_with(spelling) {
                self.offset += spelling.len();
                self.token(*kind, start, self.offset);
                return;
            }
        }

        let Some(ch) = self.current_char() else {
            return;
        };
        let end = start + ch.len_utf8();
        self.diagnostic(DiagnosticKind::UnknownCharacter, start, end);
        self.offset = end;
    }

    fn token(&mut self, kind: TokenKind, start: usize, end: usize) {
        self.tokens.push(Token {
            kind,
            span: self.span(start, end),
        });
    }

    fn diagnostic(&mut self, kind: DiagnosticKind, start: usize, end: usize) {
        self.diagnostics.push(Diagnostic {
            kind,
            span: self.span(start, end),
        });
    }

    fn span(&self, start: usize, end: usize) -> ByteSpan {
        ByteSpan::new(self.file, start, end).expect("lexer creates ordered spans")
    }

    fn is_eof(&self) -> bool {
        self.offset >= self.text.len()
    }

    fn remaining(&self) -> &'source str {
        &self.text[self.offset..]
    }

    fn current_char(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    fn bump_char(&mut self) {
        if let Some(ch) = self.current_char() {
            self.offset += ch.len_utf8();
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum IntegerClass {
    Token(TokenKind),
    Malformed,
    UnsupportedSuffix,
}

fn classify_integer(text: &str) -> IntegerClass {
    if let Some(rest) = text.strip_prefix("0b").or_else(|| text.strip_prefix("0B")) {
        classify_digits(rest, |ch| matches!(ch, '0' | '1'), TokenKind::IntBinary)
    } else if let Some(rest) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        classify_digits(rest, |ch| ch.is_ascii_hexdigit(), TokenKind::IntHex)
    } else {
        classify_digits(text, |ch| ch.is_ascii_digit(), TokenKind::IntDecimal)
    }
}

fn classify_digits(text: &str, is_digit: impl Fn(char) -> bool, kind: TokenKind) -> IntegerClass {
    if text.is_empty() || text.starts_with('_') || text.ends_with('_') || text.contains("__") {
        return IntegerClass::Malformed;
    }

    let mut saw_digit = false;
    for ch in text.chars() {
        if is_digit(ch) {
            saw_digit = true;
        } else if ch == '_' {
        } else if ch.is_ascii_alphabetic() {
            return IntegerClass::UnsupportedSuffix;
        } else {
            return IntegerClass::Malformed;
        }
    }

    if saw_digit {
        IntegerClass::Token(kind)
    } else {
        IntegerClass::Malformed
    }
}

fn is_identifier_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_identifier_continue(ch: char) -> bool {
    is_identifier_start(ch) || ch.is_ascii_digit()
}

fn keyword_kind(text: &str) -> Option<TokenKind> {
    Some(match text {
        "as" => TokenKind::KwAs,
        "break" => TokenKind::KwBreak,
        "const" => TokenKind::KwConst,
        "continue" => TokenKind::KwContinue,
        "else" => TokenKind::KwElse,
        "enum" => TokenKind::KwEnum,
        "extern" => TokenKind::KwExtern,
        "false" => TokenKind::KwFalse,
        "for" => TokenKind::KwFor,
        "fun" => TokenKind::KwFun,
        "if" => TokenKind::KwIf,
        "impl" => TokenKind::KwImpl,
        "import" => TokenKind::KwImport,
        "in" => TokenKind::KwIn,
        "interface" => TokenKind::KwInterface,
        "is" => TokenKind::KwIs,
        "null" => TokenKind::KwNull,
        "package" => TokenKind::KwPackage,
        "return" => TokenKind::KwReturn,
        "struct" => TokenKind::KwStruct,
        "true" => TokenKind::KwTrue,
        "type" => TokenKind::KwType,
        "unsafe" => TokenKind::KwUnsafe,
        "var" => TokenKind::KwVar,
        "when" => TokenKind::KwWhen,
        "while" => TokenKind::KwWhile,
        _ => return None,
    })
}

const OPERATORS_AND_DELIMITERS: &[(&str, TokenKind)] = &[
    ("++", TokenKind::PlusPlus),
    ("--", TokenKind::MinusMinus),
    ("==", TokenKind::EqualEqual),
    ("!=", TokenKind::BangEqual),
    ("<=", TokenKind::LessEqual),
    (">=", TokenKind::GreaterEqual),
    ("&&", TokenKind::AmpAmp),
    ("||", TokenKind::PipePipe),
    ("**", TokenKind::StarStar),
    ("<<", TokenKind::LessLess),
    (">>", TokenKind::GreaterGreater),
    ("->", TokenKind::Arrow),
    ("=>", TokenKind::FatArrow),
    ("..<", TokenKind::DotDotLess),
    ("..", TokenKind::DotDot),
    ("+", TokenKind::Plus),
    ("-", TokenKind::Minus),
    ("*", TokenKind::Star),
    ("/", TokenKind::Slash),
    ("%", TokenKind::Percent),
    ("=", TokenKind::Equal),
    ("<", TokenKind::Less),
    (">", TokenKind::Greater),
    ("!", TokenKind::Bang),
    ("&", TokenKind::Amp),
    ("|", TokenKind::Pipe),
    ("^", TokenKind::Caret),
    ("~", TokenKind::Tilde),
    (".", TokenKind::Dot),
    ("?", TokenKind::Question),
    (":", TokenKind::Colon),
    ("(", TokenKind::LeftParen),
    (")", TokenKind::RightParen),
    ("{", TokenKind::LeftBrace),
    ("}", TokenKind::RightBrace),
    ("[", TokenKind::LeftBracket),
    ("]", TokenKind::RightBracket),
    (",", TokenKind::Comma),
    (";", TokenKind::Semicolon),
];

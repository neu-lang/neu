use crate::ast::{AstArena, AstNodeKind};
use crate::lexer::{self, Token, TokenKind};
use crate::source::{ByteSpan, SourceFileId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticKind {
    MisplacedPackageDeclaration,
    MisplacedImportDeclaration,
    DuplicateVisibilityModifier,
    UnsupportedDeclarationModifier,
    MissingDeclarationName,
    MalformedDeclarationHeader,
    InvalidMemberDeclarationPosition,
    UnexpectedTokenInDeclarationBody,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub span: ByteSpan,
}

#[derive(Debug)]
pub struct ParseOutput {
    pub arena: AstArena,
    pub diagnostics: Vec<Diagnostic>,
    pub lex_diagnostics: Vec<lexer::Diagnostic>,
}

impl ParseOutput {
    pub fn node_kinds(&self) -> Vec<AstNodeKind> {
        self.arena.nodes().iter().map(|node| node.kind).collect()
    }
}

pub fn parse_source(file: SourceFileId, text: &str) -> ParseOutput {
    let lex_output = lexer::lex(file, text);
    let mut parser = Parser::new(file, text, lex_output.tokens);
    parser.parse_source();

    ParseOutput {
        arena: parser.arena,
        diagnostics: parser.diagnostics,
        lex_diagnostics: lex_output.diagnostics,
    }
}

struct Parser<'source> {
    file: SourceFileId,
    text: &'source str,
    tokens: Vec<Token>,
    index: usize,
    arena: AstArena,
    diagnostics: Vec<Diagnostic>,
    saw_package_or_import: bool,
    saw_top_level_declaration: bool,
}

impl<'source> Parser<'source> {
    fn new(file: SourceFileId, text: &'source str, tokens: Vec<Token>) -> Self {
        Self {
            file,
            text,
            tokens,
            index: 0,
            arena: AstArena::new(),
            diagnostics: Vec::new(),
            saw_package_or_import: false,
            saw_top_level_declaration: false,
        }
    }

    fn parse_source(&mut self) {
        self.arena.add_source_file(self.span(0, self.text.len()));
        while !self.is_eof() {
            self.parse_top_level_item();
        }
    }

    fn parse_top_level_item(&mut self) {
        match self.current_kind() {
            Some(TokenKind::KwPackage) => self.parse_package(),
            Some(TokenKind::KwImport) => self.parse_import(),
            Some(TokenKind::RightBrace) => {
                self.diagnostic_current(DiagnosticKind::MalformedDeclarationHeader);
                self.advance();
            }
            _ => self.parse_declaration(false),
        }
    }

    fn parse_package(&mut self) {
        let package = self.current().expect("package token exists").clone();
        if self.saw_package_or_import || self.saw_top_level_declaration {
            self.diagnostic(DiagnosticKind::MisplacedPackageDeclaration, package.span);
        } else {
            self.arena.add_package_declaration(package.span);
        }
        self.saw_package_or_import = true;
        self.advance();
        self.parse_qualified_name();
    }

    fn parse_import(&mut self) {
        let import = self.current().expect("import token exists").clone();
        if self.saw_top_level_declaration {
            self.diagnostic(DiagnosticKind::MisplacedImportDeclaration, import.span);
        } else {
            self.arena.add_import_declaration(import.span);
        }
        self.saw_package_or_import = true;
        self.advance();
        self.parse_qualified_name();
        if self.current_kind() == Some(TokenKind::KwAs) {
            self.advance();
            if self.current_kind() == Some(TokenKind::Identifier) {
                self.advance();
            } else {
                self.diagnostic_at_previous_or_current(DiagnosticKind::MalformedDeclarationHeader);
            }
        }
    }

    fn parse_declaration(&mut self, in_body: bool) {
        if in_body && self.current_kind() == Some(TokenKind::RightBrace) {
            return;
        }

        let mut saw_visibility = false;
        while self.is_visibility() || self.is_unsupported_modifier() {
            let token = self.current().expect("modifier token exists").clone();
            if self.is_visibility() {
                if saw_visibility {
                    self.diagnostic(DiagnosticKind::DuplicateVisibilityModifier, token.span);
                }
                saw_visibility = true;
            } else {
                self.diagnostic(DiagnosticKind::UnsupportedDeclarationModifier, token.span);
            }
            self.advance();
        }

        match self.current_kind() {
            Some(TokenKind::KwFun) => self.parse_function(in_body),
            Some(TokenKind::KwStruct) => {
                self.parse_named_body_declaration(AstNodeKind::StructDeclaration, in_body)
            }
            Some(TokenKind::KwEnum) => {
                self.parse_named_body_declaration(AstNodeKind::EnumDeclaration, in_body)
            }
            Some(TokenKind::KwInterface) => {
                self.parse_named_body_declaration(AstNodeKind::InterfaceDeclaration, in_body);
            }
            Some(TokenKind::KwPackage | TokenKind::KwImport) if in_body => {
                let span = self.current().expect("member token exists").span;
                self.diagnostic(DiagnosticKind::InvalidMemberDeclarationPosition, span);
                self.advance();
                self.skip_to_declaration_boundary(in_body);
            }
            Some(_) if in_body => {
                self.diagnostic_current(DiagnosticKind::UnexpectedTokenInDeclarationBody);
                self.advance();
                self.skip_to_declaration_boundary(in_body);
            }
            Some(_) => {
                self.diagnostic_current(DiagnosticKind::MalformedDeclarationHeader);
                self.advance();
                self.skip_to_declaration_boundary(in_body);
            }
            None => {}
        }
    }

    fn parse_function(&mut self, in_body: bool) {
        let start = self.current().expect("function token exists").span.start();
        self.advance();
        if self.current_kind() != Some(TokenKind::Identifier) {
            self.diagnostic(DiagnosticKind::MissingDeclarationName, self.span_at(start));
            self.skip_to_declaration_boundary(in_body);
            return;
        }
        self.advance();

        if !self.consume_balanced_parentheses() {
            self.diagnostic(
                DiagnosticKind::MalformedDeclarationHeader,
                self.span_at(start),
            );
            self.skip_to_declaration_boundary(in_body);
            return;
        }

        if self.current_kind() == Some(TokenKind::Colon) {
            self.advance();
            self.skip_type_placeholder();
        }

        match self.current_kind() {
            Some(TokenKind::Semicolon) => {
                let end = self.current().expect("semicolon exists").span.end();
                self.arena.add_function_declaration(self.span(start, end));
                self.saw_top_level_declaration |= !in_body;
                self.advance();
            }
            Some(TokenKind::LeftBrace) => {
                let body_start = self.current().expect("body exists").span.start();
                self.arena
                    .add_function_declaration(self.span(start, body_start));
                self.saw_top_level_declaration |= !in_body;
                self.parse_declaration_body();
            }
            _ => {
                self.diagnostic_current_or_span(
                    DiagnosticKind::MalformedDeclarationHeader,
                    self.span_at(start),
                );
                self.skip_to_declaration_boundary(in_body);
            }
        }
    }

    fn parse_named_body_declaration(&mut self, kind: AstNodeKind, in_body: bool) {
        let start = self
            .current()
            .expect("declaration token exists")
            .span
            .start();
        self.advance();
        if self.current_kind() != Some(TokenKind::Identifier) {
            self.diagnostic(DiagnosticKind::MissingDeclarationName, self.span_at(start));
            self.skip_to_declaration_boundary(in_body);
            return;
        }
        self.advance();

        if self.current_kind() != Some(TokenKind::LeftBrace) {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedDeclarationHeader,
                self.span_at(start),
            );
            self.skip_to_declaration_boundary(in_body);
            return;
        }

        let body_start = self.current().expect("body exists").span.start();
        match kind {
            AstNodeKind::StructDeclaration => {
                self.arena
                    .add_struct_declaration(self.span(start, body_start));
            }
            AstNodeKind::EnumDeclaration => {
                self.arena
                    .add_enum_declaration(self.span(start, body_start));
            }
            AstNodeKind::InterfaceDeclaration => {
                self.arena
                    .add_interface_declaration(self.span(start, body_start));
            }
            _ => unreachable!("only named body declarations are parsed here"),
        }
        self.saw_top_level_declaration |= !in_body;
        self.parse_declaration_body();
    }

    fn parse_declaration_body(&mut self) {
        let left_brace = self.current().expect("body starts with left brace").span;
        self.arena.add_declaration_body(left_brace);
        self.advance();

        while !self.is_eof() {
            if self.current_kind() == Some(TokenKind::RightBrace) {
                self.advance();
                return;
            }
            self.parse_declaration(true);
        }
    }

    fn parse_qualified_name(&mut self) {
        if self.current_kind() != Some(TokenKind::Identifier) {
            self.diagnostic_at_previous_or_current(DiagnosticKind::MalformedDeclarationHeader);
            return;
        }
        self.advance();
        while self.current_kind() == Some(TokenKind::Dot) {
            self.advance();
            if self.current_kind() == Some(TokenKind::Identifier) {
                self.advance();
            } else {
                self.diagnostic_at_previous_or_current(DiagnosticKind::MalformedDeclarationHeader);
                return;
            }
        }
    }

    fn consume_balanced_parentheses(&mut self) -> bool {
        if self.current_kind() != Some(TokenKind::LeftParen) {
            return false;
        }
        let mut depth = 0usize;
        while let Some(kind) = self.current_kind() {
            match kind {
                TokenKind::LeftParen => {
                    depth += 1;
                    self.advance();
                }
                TokenKind::RightParen => {
                    depth -= 1;
                    self.advance();
                    if depth == 0 {
                        return true;
                    }
                }
                TokenKind::LeftBrace | TokenKind::RightBrace | TokenKind::Semicolon
                    if depth > 0 =>
                {
                    return false;
                }
                _ => self.advance(),
            }
        }
        false
    }

    fn skip_type_placeholder(&mut self) {
        while let Some(kind) = self.current_kind() {
            if matches!(
                kind,
                TokenKind::Semicolon | TokenKind::LeftBrace | TokenKind::RightBrace
            ) || self.is_declaration_starter()
            {
                return;
            }
            self.advance();
        }
    }

    fn skip_to_declaration_boundary(&mut self, in_body: bool) {
        while let Some(kind) = self.current_kind() {
            if matches!(kind, TokenKind::Semicolon) {
                self.advance();
                return;
            }
            if matches!(kind, TokenKind::RightBrace) {
                return;
            }
            if self.is_declaration_starter()
                || (!in_body && matches!(kind, TokenKind::KwPackage | TokenKind::KwImport))
            {
                return;
            }
            self.advance();
        }
    }

    fn is_declaration_starter(&self) -> bool {
        matches!(
            self.current_kind(),
            Some(
                TokenKind::KwFun | TokenKind::KwStruct | TokenKind::KwEnum | TokenKind::KwInterface
            )
        ) || self.is_visibility()
            || self.is_unsupported_modifier()
    }

    fn is_visibility(&self) -> bool {
        self.current_kind() == Some(TokenKind::Identifier)
            && matches!(self.current_text(), Some("public" | "private" | "internal"))
    }

    fn is_unsupported_modifier(&self) -> bool {
        match self.current_kind() {
            Some(TokenKind::KwExtern) => true,
            Some(TokenKind::Identifier) => {
                matches!(
                    self.current_text(),
                    Some("abstract" | "open" | "override" | "inline" | "suspend")
                )
            }
            _ => false,
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn current_kind(&self) -> Option<TokenKind> {
        self.current().map(|token| token.kind)
    }

    fn current_text(&self) -> Option<&'source str> {
        let span = self.current()?.span;
        self.text.get(span.start()..span.end())
    }

    fn previous_span(&self) -> Option<ByteSpan> {
        self.index
            .checked_sub(1)
            .and_then(|index| self.tokens.get(index))
            .map(|token| token.span)
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn is_eof(&self) -> bool {
        self.index >= self.tokens.len()
    }

    fn diagnostic_current(&mut self, kind: DiagnosticKind) {
        if let Some(token) = self.current() {
            self.diagnostic(kind, token.span);
        }
    }

    fn diagnostic_current_or_span(&mut self, kind: DiagnosticKind, fallback: ByteSpan) {
        let span = self.current().map_or(fallback, |token| token.span);
        self.diagnostic(kind, span);
    }

    fn diagnostic_at_previous_or_current(&mut self, kind: DiagnosticKind) {
        let span = self
            .previous_span()
            .or_else(|| self.current().map(|token| token.span))
            .unwrap_or_else(|| self.span(self.text.len(), self.text.len()));
        self.diagnostic(kind, span);
    }

    fn diagnostic(&mut self, kind: DiagnosticKind, span: ByteSpan) {
        self.diagnostics.push(Diagnostic { kind, span });
    }

    fn span_at(&self, offset: usize) -> ByteSpan {
        self.span(offset, offset)
    }

    fn span(&self, start: usize, end: usize) -> ByteSpan {
        ByteSpan::new(self.file, start, end).expect("parser creates ordered spans")
    }
}

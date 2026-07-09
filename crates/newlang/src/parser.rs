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
    MissingTypeName,
    MalformedNullableType,
    MalformedGenericParameterList,
    MalformedGenericArgumentList,
    MissingGenericBound,
    MalformedCapabilityBound,
    MalformedFunctionType,
    UnsupportedTypeForm,
    UnexpectedTokenInType,
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

        self.parse_generic_parameters();

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
            if self.parse_type().is_none() {
                self.skip_to_declaration_boundary(in_body);
                return;
            }
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

        self.parse_generic_parameters();

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

    fn parse_generic_parameters(&mut self) {
        if self.current_kind() != Some(TokenKind::Less) {
            return;
        }

        let start = self
            .current()
            .expect("generic parameter list exists")
            .span
            .start();
        self.advance();

        if self.current_kind() == Some(TokenKind::Greater) {
            let end = self.current().expect("greater token exists").span.end();
            self.diagnostic(
                DiagnosticKind::MalformedGenericParameterList,
                self.span(start, end),
            );
            self.advance();
            return;
        }

        loop {
            let Some(parameter_start) = self.current().map(|token| token.span.start()) else {
                self.diagnostic(
                    DiagnosticKind::MalformedGenericParameterList,
                    self.span_at(start),
                );
                return;
            };

            if self.current_kind() != Some(TokenKind::Identifier) {
                self.diagnostic_current_or_span(
                    DiagnosticKind::MalformedGenericParameterList,
                    self.span_at(parameter_start),
                );
                self.skip_to_generic_parameter_boundary();
                return;
            }
            self.advance();

            let mut bound_count = 0usize;
            if self.current_kind() == Some(TokenKind::Colon) {
                self.advance();
                if !self.can_start_capability_bound() {
                    self.diagnostic_at_previous_or_current(DiagnosticKind::MissingGenericBound);
                    self.skip_to_generic_parameter_boundary();
                    return;
                }
                bound_count = self.parse_capability_bound_list();
            }

            let parameter_end = self
                .previous_span()
                .map_or(parameter_start, |span| span.end());
            self.arena
                .add_generic_parameter(self.span(parameter_start, parameter_end));

            match self.current_kind() {
                Some(TokenKind::Comma) => {
                    if bound_count == 1 && self.lookahead_kind(1) == Some(TokenKind::Identifier) {
                        self.diagnostic_current(DiagnosticKind::MalformedCapabilityBound);
                    }
                    self.advance();
                    if self.current_kind() == Some(TokenKind::Greater) {
                        self.diagnostic_current(DiagnosticKind::MalformedGenericParameterList);
                        self.advance();
                        return;
                    }
                }
                Some(TokenKind::Greater) => {
                    self.advance();
                    return;
                }
                _ => {
                    self.diagnostic_current_or_span(
                        DiagnosticKind::MalformedGenericParameterList,
                        self.span_at(start),
                    );
                    self.skip_to_generic_parameter_boundary();
                    return;
                }
            }
        }
    }

    fn parse_type(&mut self) -> Option<ByteSpan> {
        if !self.can_start_type() {
            self.diagnostic_current_or_span(
                DiagnosticKind::MissingTypeName,
                self.previous_span()
                    .unwrap_or_else(|| self.span(self.text.len(), self.text.len())),
            );
            return None;
        }

        let mut span = self.parse_primary_type()?;
        if self.current_kind() == Some(TokenKind::Question) {
            let end = self.current().expect("question token exists").span.end();
            self.advance();
            span = self.span(span.start(), end);
            self.arena.add_nullable_type(span);
            if self.current_kind() == Some(TokenKind::Question) {
                self.diagnostic_current(DiagnosticKind::MalformedNullableType);
                self.advance();
            }
        }
        Some(span)
    }

    fn parse_primary_type(&mut self) -> Option<ByteSpan> {
        match self.current_kind() {
            Some(TokenKind::Identifier) => self.parse_named_type(),
            Some(TokenKind::LeftParen) if self.parenthesized_type_is_function_type() => {
                self.parse_function_type()
            }
            Some(TokenKind::LeftParen) => self.parse_grouped_type(),
            Some(_) => {
                self.diagnostic_current(DiagnosticKind::UnsupportedTypeForm);
                self.advance();
                None
            }
            None => {
                self.diagnostic(
                    DiagnosticKind::MissingTypeName,
                    self.span(self.text.len(), self.text.len()),
                );
                None
            }
        }
    }

    fn parse_named_type(&mut self) -> Option<ByteSpan> {
        let start = self.current()?.span.start();
        let mut end = self.parse_qualified_name_for_type()?;
        if self.current_kind() == Some(TokenKind::Less) {
            end = self.parse_generic_arguments().unwrap_or(end);
        }
        let span = self.span(start, end);
        self.arena.add_named_type(span);
        Some(span)
    }

    fn parse_qualified_name_for_type(&mut self) -> Option<usize> {
        if self.current_kind() != Some(TokenKind::Identifier) {
            self.diagnostic_current(DiagnosticKind::MissingTypeName);
            return None;
        }
        let mut end = self.current().expect("identifier exists").span.end();
        self.advance();
        while self.current_kind() == Some(TokenKind::Dot) {
            self.advance();
            if self.current_kind() == Some(TokenKind::Identifier) {
                end = self.current().expect("identifier exists").span.end();
                self.advance();
            } else {
                self.diagnostic_at_previous_or_current(DiagnosticKind::UnexpectedTokenInType);
                return Some(end);
            }
        }
        Some(end)
    }

    fn parse_generic_arguments(&mut self) -> Option<usize> {
        let start = self.current()?.span.start();
        self.advance();

        if self.current_kind() == Some(TokenKind::Greater) {
            let end = self.current().expect("greater token exists").span.end();
            self.diagnostic(
                DiagnosticKind::MalformedGenericArgumentList,
                self.span(start, end),
            );
            self.advance();
            return Some(end);
        }

        loop {
            let argument_start = self.current().map_or(start, |token| token.span.start());
            let Some(argument_span) = self.parse_type() else {
                self.diagnostic_current_or_span(
                    DiagnosticKind::MalformedGenericArgumentList,
                    self.span_at(argument_start),
                );
                self.skip_to_generic_argument_boundary();
                return self.previous_span().map(|span| span.end());
            };
            self.arena.add_generic_argument(argument_span);

            match self.current_kind() {
                Some(TokenKind::Comma) => {
                    self.advance();
                    if self.current_kind() == Some(TokenKind::Greater) {
                        self.diagnostic_current(DiagnosticKind::MalformedGenericArgumentList);
                        self.advance();
                        return self.previous_span().map(|span| span.end());
                    }
                }
                Some(TokenKind::Greater) => {
                    let end = self.current().expect("greater token exists").span.end();
                    self.advance();
                    return Some(end);
                }
                _ => {
                    self.diagnostic_current_or_span(
                        DiagnosticKind::MalformedGenericArgumentList,
                        self.span_at(start),
                    );
                    self.skip_to_generic_argument_boundary();
                    return self.previous_span().map(|span| span.end());
                }
            }
        }
    }

    fn parse_capability_bound_list(&mut self) -> usize {
        let mut count = 0usize;
        loop {
            if self.parse_capability_bound().is_none() {
                return count;
            }
            count += 1;
            if self.current_kind() != Some(TokenKind::Amp) {
                return count;
            }
            self.advance();
            if !self.can_start_capability_bound() {
                self.diagnostic_at_previous_or_current(DiagnosticKind::MalformedCapabilityBound);
                return count;
            }
        }
    }

    fn parse_capability_bound(&mut self) -> Option<ByteSpan> {
        let start = self.current()?.span.start();
        let end = self.parse_qualified_name_for_type()?;
        let span = self.span(start, end);
        self.arena.add_capability_bound(span);
        Some(span)
    }

    fn parse_function_type(&mut self) -> Option<ByteSpan> {
        let start = self.current()?.span.start();
        self.advance();

        if self.current_kind() != Some(TokenKind::RightParen) {
            loop {
                if self.parse_type().is_none() {
                    self.diagnostic_current_or_span(
                        DiagnosticKind::MalformedFunctionType,
                        self.span_at(start),
                    );
                    self.skip_to_type_boundary();
                    return None;
                }
                match self.current_kind() {
                    Some(TokenKind::Comma) => self.advance(),
                    Some(TokenKind::RightParen) => break,
                    _ => {
                        self.diagnostic_current_or_span(
                            DiagnosticKind::MalformedFunctionType,
                            self.span_at(start),
                        );
                        self.skip_to_type_boundary();
                        return None;
                    }
                }
            }
        }

        if self.current_kind() != Some(TokenKind::RightParen) {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedFunctionType,
                self.span_at(start),
            );
            return None;
        }
        self.advance();

        if self.current_kind() != Some(TokenKind::Arrow) {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedFunctionType,
                self.span_at(start),
            );
            return None;
        }
        self.advance();

        let Some(return_type) = self.parse_type() else {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedFunctionType,
                self.span_at(start),
            );
            return None;
        };
        let span = self.span(start, return_type.end());
        self.arena.add_function_type(span);
        Some(span)
    }

    fn parse_grouped_type(&mut self) -> Option<ByteSpan> {
        let start = self.current()?.span.start();
        self.advance();
        let inner = self.parse_type()?;
        if self.current_kind() != Some(TokenKind::RightParen) {
            self.diagnostic_current_or_span(DiagnosticKind::UnexpectedTokenInType, inner);
            self.skip_to_type_boundary();
            return None;
        }
        let end = self.current().expect("right paren exists").span.end();
        self.advance();
        let span = self.span(start, end);
        self.arena.add_grouped_type(span);
        Some(span)
    }

    fn parenthesized_type_is_function_type(&self) -> bool {
        let mut depth = 0usize;
        let mut index = self.index;
        while let Some(token) = self.tokens.get(index) {
            match token.kind {
                TokenKind::LeftParen => depth += 1,
                TokenKind::RightParen => {
                    depth -= 1;
                    if depth == 0 {
                        return self
                            .tokens
                            .get(index + 1)
                            .is_some_and(|next| next.kind == TokenKind::Arrow);
                    }
                }
                _ => {}
            }
            index += 1;
        }
        false
    }

    fn can_start_type(&self) -> bool {
        matches!(
            self.current_kind(),
            Some(TokenKind::Identifier | TokenKind::LeftParen)
        )
    }

    fn can_start_capability_bound(&self) -> bool {
        self.current_kind() == Some(TokenKind::Identifier)
    }

    fn skip_to_type_boundary(&mut self) {
        while let Some(kind) = self.current_kind() {
            if matches!(
                kind,
                TokenKind::Comma
                    | TokenKind::Greater
                    | TokenKind::RightParen
                    | TokenKind::LeftBrace
                    | TokenKind::RightBrace
                    | TokenKind::Semicolon
            ) || self.is_declaration_starter()
            {
                return;
            }
            self.advance();
        }
    }

    fn skip_to_generic_parameter_boundary(&mut self) {
        while let Some(kind) = self.current_kind() {
            if matches!(
                kind,
                TokenKind::Greater | TokenKind::LeftBrace | TokenKind::Semicolon
            ) || self.is_declaration_starter()
            {
                if kind == TokenKind::Greater {
                    self.advance();
                }
                return;
            }
            self.advance();
        }
    }

    fn skip_to_generic_argument_boundary(&mut self) {
        while let Some(kind) = self.current_kind() {
            if matches!(
                kind,
                TokenKind::Greater
                    | TokenKind::Comma
                    | TokenKind::RightParen
                    | TokenKind::LeftBrace
                    | TokenKind::RightBrace
                    | TokenKind::Semicolon
            ) || self.is_declaration_starter()
            {
                if kind == TokenKind::Greater {
                    self.advance();
                }
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

    fn lookahead_kind(&self, distance: usize) -> Option<TokenKind> {
        self.tokens
            .get(self.index + distance)
            .map(|token| token.kind)
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

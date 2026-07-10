use crate::ast::{AstArena, AstNodeKind};
use crate::lexer::{self, Token, TokenKind};
use crate::name_resolution::{DeclarationKind, LocalBindingKind};
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
    MissingExpression,
    UnexpectedTokenInExpression,
    UnsupportedExpressionForm,
    MalformedBinaryExpression,
    MalformedCallExpression,
    MalformedMemberAccess,
    MalformedBlock,
    MissingStatement,
    UnexpectedTokenInStatement,
    UnsupportedStatementForm,
    MalformedVariableDeclaration,
    MalformedAssignment,
    MalformedReturnStatement,
    MalformedConditional,
    MalformedPattern,
    UnsupportedPatternForm,
    MissingPatternArmBody,
    MalformedUnsafeBlock,
    MalformedCoroutineConstruct,
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
    pub declaration_names: Vec<ParsedDeclarationName>,
    pub local_binding_names: Vec<ParsedLocalBindingName>,
    pub name_references: Vec<ParsedNameReference>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedDeclarationName {
    pub declaration: crate::ast::AstNodeId,
    pub kind: DeclarationKind,
    pub name: String,
    pub name_span: ByteSpan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedLocalBindingName {
    pub binding: crate::ast::AstNodeId,
    pub kind: LocalBindingKind,
    pub name: String,
    pub name_span: ByteSpan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedNameReference {
    pub reference: crate::ast::AstNodeId,
    pub name: String,
    pub name_span: ByteSpan,
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
        declaration_names: parser.declaration_names,
        local_binding_names: parser.local_binding_names,
        name_references: parser.name_references,
    }
}

struct Parser<'source> {
    file: SourceFileId,
    text: &'source str,
    tokens: Vec<Token>,
    index: usize,
    arena: AstArena,
    diagnostics: Vec<Diagnostic>,
    declaration_names: Vec<ParsedDeclarationName>,
    local_binding_names: Vec<ParsedLocalBindingName>,
    name_references: Vec<ParsedNameReference>,
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
            declaration_names: Vec::new(),
            local_binding_names: Vec::new(),
            name_references: Vec::new(),
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
        let name = self.current().expect("function name exists").clone();
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
                let declaration = self.arena.add_function_declaration(self.span(start, end));
                self.record_declaration_name(
                    declaration,
                    DeclarationKind::Function,
                    &name,
                    in_body,
                );
                self.saw_top_level_declaration |= !in_body;
                self.advance();
            }
            Some(TokenKind::LeftBrace) => {
                let body_start = self.current().expect("body exists").span.start();
                let declaration = self
                    .arena
                    .add_function_declaration(self.span(start, body_start));
                self.record_declaration_name(
                    declaration,
                    DeclarationKind::Function,
                    &name,
                    in_body,
                );
                self.saw_top_level_declaration |= !in_body;
                self.parse_body_block();
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
        let name = self.current().expect("declaration name exists").clone();
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
        let declaration = match kind {
            AstNodeKind::StructDeclaration => self
                .arena
                .add_struct_declaration(self.span(start, body_start)),
            AstNodeKind::EnumDeclaration => self
                .arena
                .add_enum_declaration(self.span(start, body_start)),
            AstNodeKind::InterfaceDeclaration => self
                .arena
                .add_interface_declaration(self.span(start, body_start)),
            _ => unreachable!("only named body declarations are parsed here"),
        };
        self.record_declaration_name(declaration, DeclarationKind::Type, &name, in_body);
        self.saw_top_level_declaration |= !in_body;
        self.parse_declaration_body();
    }

    fn record_declaration_name(
        &mut self,
        declaration: crate::ast::AstNodeId,
        kind: DeclarationKind,
        name: &Token,
        in_body: bool,
    ) {
        if in_body {
            return;
        }

        self.declaration_names.push(ParsedDeclarationName {
            declaration,
            kind,
            name: self.text[name.span.start()..name.span.end()].to_owned(),
            name_span: name.span,
        });
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

    fn parse_body_block(&mut self) -> Option<ByteSpan> {
        if self.current_kind() != Some(TokenKind::LeftBrace) {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedBlock,
                self.previous_span()
                    .unwrap_or_else(|| self.span(self.text.len(), self.text.len())),
            );
            return None;
        }

        let start = self
            .current()
            .expect("block starts with left brace")
            .span
            .start();
        self.advance();

        while !self.is_eof() {
            match self.current_kind() {
                Some(TokenKind::RightBrace) => {
                    let end = self.current().expect("right brace exists").span.end();
                    self.advance();
                    let span = self.span(start, end);
                    self.arena.add_block(span);
                    return Some(span);
                }
                Some(TokenKind::KwVal | TokenKind::KwVar | TokenKind::KwReturn) => {
                    self.parse_statement();
                }
                Some(kind) if self.can_start_expression_kind(kind) => {
                    let checkpoint = self.index;
                    let expression_start = self.current().map_or(start, |token| token.span.start());
                    let Some(expression_span) = self.parse_expression() else {
                        self.skip_to_statement_boundary();
                        continue;
                    };
                    match self.current_kind() {
                        Some(TokenKind::Semicolon) => {
                            let end = self.current().expect("semicolon exists").span.end();
                            self.advance();
                            if self.tokens.get(checkpoint).is_some_and(|token| {
                                self.expression_start_can_be_assignment_target(token.kind)
                            }) && self
                                .tokens
                                .get(self.index.saturating_sub(2))
                                .is_some_and(|token| token.kind == TokenKind::Equal)
                            {
                                self.arena
                                    .add_assignment_statement(self.span(expression_start, end));
                            } else {
                                self.arena.add_expression_statement(
                                    self.span(expression_span.start(), end),
                                );
                            }
                        }
                        Some(TokenKind::Equal) => {
                            self.advance();
                            if self.parse_expression().is_none() {
                                self.diagnostic_current_or_span(
                                    DiagnosticKind::MalformedAssignment,
                                    expression_span,
                                );
                            }
                            if self.current_kind() == Some(TokenKind::Semicolon) {
                                let end = self.current().expect("semicolon exists").span.end();
                                self.advance();
                                self.arena
                                    .add_assignment_statement(self.span(expression_start, end));
                            } else {
                                self.diagnostic_current_or_span(
                                    DiagnosticKind::MalformedAssignment,
                                    expression_span,
                                );
                                self.skip_to_statement_boundary();
                            }
                        }
                        Some(TokenKind::RightBrace) => {}
                        _ => {
                            self.diagnostic_current_or_span(
                                DiagnosticKind::UnexpectedTokenInStatement,
                                expression_span,
                            );
                            self.skip_to_statement_boundary();
                        }
                    }
                }
                Some(TokenKind::KwUnsafe) => {
                    self.diagnostic_current(DiagnosticKind::MalformedUnsafeBlock);
                    self.advance();
                    self.skip_deferred_construct();
                }
                Some(TokenKind::KwFor | TokenKind::KwWhile | TokenKind::KwWhen) => {
                    self.diagnostic_current(DiagnosticKind::UnsupportedStatementForm);
                    self.advance();
                    self.skip_deferred_construct();
                }
                Some(TokenKind::Identifier) if self.current_text() == Some("async") => {
                    self.diagnostic_current(DiagnosticKind::MalformedCoroutineConstruct);
                    self.advance();
                    self.skip_deferred_construct();
                }
                Some(_) => {
                    self.diagnostic_current(DiagnosticKind::UnexpectedTokenInStatement);
                    self.advance();
                    self.skip_to_statement_boundary();
                }
                None => break,
            }
        }

        self.diagnostic(DiagnosticKind::MalformedBlock, self.span_at(start));
        None
    }

    fn parse_statement(&mut self) {
        match self.current_kind() {
            Some(TokenKind::KwVal | TokenKind::KwVar) => {
                self.parse_variable_declaration_statement()
            }
            Some(TokenKind::KwReturn) => self.parse_return_statement(),
            Some(_) => {
                self.diagnostic_current(DiagnosticKind::MissingStatement);
                self.skip_to_statement_boundary();
            }
            None => {}
        }
    }

    fn parse_variable_declaration_statement(&mut self) {
        let kind = match self
            .current_kind()
            .expect("variable declaration starts with val or var")
        {
            TokenKind::KwVal => LocalBindingKind::Val,
            TokenKind::KwVar => LocalBindingKind::Var,
            _ => unreachable!("variable declaration starts with val or var"),
        };
        let start = self
            .current()
            .expect("variable declaration starts with val or var")
            .span
            .start();
        self.advance();

        if self.current_kind() != Some(TokenKind::Identifier) {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedVariableDeclaration,
                self.span_at(start),
            );
            self.skip_to_statement_boundary();
            return;
        }
        let name = self
            .current()
            .expect("variable declaration name exists")
            .clone();
        self.advance();

        if self.current_kind() == Some(TokenKind::Colon) {
            self.advance();
            if self.parse_type().is_none() {
                self.diagnostic_current_or_span(
                    DiagnosticKind::MalformedVariableDeclaration,
                    self.span_at(start),
                );
                self.skip_to_statement_boundary();
                return;
            }
        }

        if self.current_kind() == Some(TokenKind::Equal) {
            self.advance();
            if self.parse_expression().is_none() {
                self.diagnostic_current_or_span(
                    DiagnosticKind::MalformedVariableDeclaration,
                    self.span_at(start),
                );
                self.skip_to_statement_boundary();
                return;
            }
        }

        if self.current_kind() == Some(TokenKind::Semicolon) {
            let end = self.current().expect("semicolon exists").span.end();
            self.advance();
            let binding = self
                .arena
                .add_variable_declaration_statement(self.span(start, end));
            self.local_binding_names.push(ParsedLocalBindingName {
                binding,
                kind,
                name: self.text[name.span.start()..name.span.end()].to_owned(),
                name_span: name.span,
            });
        } else {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedVariableDeclaration,
                self.span_at(start),
            );
            self.skip_to_statement_boundary();
        }
    }

    fn parse_return_statement(&mut self) {
        let start = self.current().expect("return token exists").span.start();
        self.advance();

        if self.current_kind() != Some(TokenKind::Semicolon) && self.parse_expression().is_none() {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedReturnStatement,
                self.span_at(start),
            );
            self.skip_to_statement_boundary();
            return;
        }

        if self.current_kind() == Some(TokenKind::Semicolon) {
            let end = self.current().expect("semicolon exists").span.end();
            self.advance();
            self.arena.add_return_statement(self.span(start, end));
        } else {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedReturnStatement,
                self.span_at(start),
            );
            self.skip_to_statement_boundary();
        }
    }

    fn parse_expression(&mut self) -> Option<ByteSpan> {
        self.parse_binary_expression(1)
    }

    fn parse_binary_expression(&mut self, min_precedence: u8) -> Option<ByteSpan> {
        let mut left = self.parse_unary_expression()?;
        while let Some((precedence, _)) = self.binary_operator() {
            if precedence < min_precedence {
                break;
            }
            let operator_span = self.current().expect("operator token exists").span;
            self.advance();
            let Some(right) = self.parse_binary_expression(precedence + 1) else {
                self.diagnostic(DiagnosticKind::MalformedBinaryExpression, operator_span);
                return Some(left);
            };
            left = self.span(left.start(), right.end());
            self.arena.add_binary_expression(left);
        }
        Some(left)
    }

    fn parse_unary_expression(&mut self) -> Option<ByteSpan> {
        match self.current_kind() {
            Some(TokenKind::Bang | TokenKind::Minus) => {
                let start = self.current().expect("unary operator exists").span.start();
                self.advance();
                let operand = self.parse_unary_expression()?;
                let span = self.span(start, operand.end());
                self.arena.add_unary_expression(span);
                Some(span)
            }
            _ => self.parse_postfix_expression(),
        }
    }

    fn parse_postfix_expression(&mut self) -> Option<ByteSpan> {
        let mut span = self.parse_primary_expression()?;
        loop {
            match self.current_kind() {
                Some(TokenKind::LeftParen) => {
                    let start = span.start();
                    let Some(end) = self.parse_argument_list() else {
                        self.diagnostic_current_or_span(
                            DiagnosticKind::MalformedCallExpression,
                            span,
                        );
                        return Some(span);
                    };
                    span = self.span(start, end);
                    self.arena.add_call_expression(span);
                }
                Some(TokenKind::Dot) => {
                    let start = span.start();
                    let dot_span = self.current().expect("dot exists").span;
                    self.advance();
                    if self.current_kind() == Some(TokenKind::Identifier) {
                        let end = self.current().expect("member identifier exists").span.end();
                        self.advance();
                        span = self.span(start, end);
                        self.arena.add_member_expression(span);
                    } else {
                        self.diagnostic(DiagnosticKind::MalformedMemberAccess, dot_span);
                        if self.current_kind() == Some(TokenKind::LeftParen) {
                            self.diagnostic_current(DiagnosticKind::MalformedCallExpression);
                        }
                        return Some(span);
                    }
                }
                Some(TokenKind::LeftBracket) => {
                    self.diagnostic_current(DiagnosticKind::UnsupportedExpressionForm);
                    self.advance();
                    self.skip_to_expression_boundary();
                    return Some(span);
                }
                _ => return Some(span),
            }
        }
    }

    fn parse_argument_list(&mut self) -> Option<usize> {
        if self.current_kind() != Some(TokenKind::LeftParen) {
            return None;
        }
        self.advance();
        if self.current_kind() == Some(TokenKind::RightParen) {
            let end = self.current().expect("right paren exists").span.end();
            self.advance();
            return Some(end);
        }
        loop {
            if self.parse_expression().is_none() {
                self.diagnostic_current(DiagnosticKind::MalformedCallExpression);
                self.skip_to_expression_boundary();
                return self.previous_span().map(|span| span.end());
            }
            match self.current_kind() {
                Some(TokenKind::Comma) => {
                    self.advance();
                    if self.current_kind() == Some(TokenKind::RightParen) {
                        self.diagnostic_current(DiagnosticKind::MalformedCallExpression);
                        let end = self.current().expect("right paren exists").span.end();
                        self.advance();
                        return Some(end);
                    }
                }
                Some(TokenKind::RightParen) => {
                    let end = self.current().expect("right paren exists").span.end();
                    self.advance();
                    return Some(end);
                }
                _ => {
                    self.diagnostic_current(DiagnosticKind::MalformedCallExpression);
                    self.skip_to_expression_boundary();
                    return self.previous_span().map(|span| span.end());
                }
            }
        }
    }

    fn parse_primary_expression(&mut self) -> Option<ByteSpan> {
        match self.current_kind() {
            Some(
                TokenKind::IntDecimal
                | TokenKind::IntBinary
                | TokenKind::IntHex
                | TokenKind::String
                | TokenKind::KwTrue
                | TokenKind::KwFalse
                | TokenKind::KwNull,
            ) => {
                let span = self.current().expect("literal token exists").span;
                self.advance();
                self.arena.add_literal_expression(span);
                Some(span)
            }
            Some(TokenKind::Identifier) => self.parse_name_expression(),
            Some(TokenKind::LeftParen) => self.parse_grouped_expression(),
            Some(TokenKind::KwIf) => self.parse_if_expression(),
            Some(TokenKind::Equal) => {
                self.diagnostic_current(DiagnosticKind::UnsupportedExpressionForm);
                None
            }
            Some(_) => {
                self.diagnostic_current(DiagnosticKind::MissingExpression);
                None
            }
            None => {
                self.diagnostic(
                    DiagnosticKind::MissingExpression,
                    self.span(self.text.len(), self.text.len()),
                );
                None
            }
        }
    }

    fn parse_name_expression(&mut self) -> Option<ByteSpan> {
        let name = self.current()?.clone();
        let start = name.span.start();
        let end = name.span.end();
        self.advance();
        let span = self.span(start, end);
        let reference = self.arena.add_name_expression(span);
        self.name_references.push(ParsedNameReference {
            reference,
            name: self.text[name.span.start()..name.span.end()].to_owned(),
            name_span: name.span,
        });
        Some(span)
    }

    fn parse_grouped_expression(&mut self) -> Option<ByteSpan> {
        let start = self.current()?.span.start();
        self.advance();
        let inner = self.parse_expression()?;
        if self.current_kind() != Some(TokenKind::RightParen) {
            self.diagnostic_current_or_span(DiagnosticKind::UnexpectedTokenInExpression, inner);
            self.skip_to_expression_boundary();
            return None;
        }
        let end = self.current().expect("right paren exists").span.end();
        self.advance();
        let span = self.span(start, end);
        self.arena.add_grouped_expression(span);
        Some(span)
    }

    fn parse_if_expression(&mut self) -> Option<ByteSpan> {
        let start = self.current()?.span.start();
        self.advance();
        if self.current_kind() != Some(TokenKind::LeftParen) {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedConditional,
                self.span_at(start),
            );
            self.skip_to_expression_boundary();
            return None;
        }
        self.advance();
        if self.parse_expression().is_none() {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedConditional,
                self.span_at(start),
            );
            self.skip_to_expression_boundary();
            return None;
        }
        if self.current_kind() != Some(TokenKind::RightParen) {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedConditional,
                self.span_at(start),
            );
            self.skip_to_expression_boundary();
            return None;
        }
        self.advance();
        let then_block = self.parse_body_block()?;
        let mut end = then_block.end();
        if self.current_kind() == Some(TokenKind::KwElse) {
            self.advance();
            let Some(else_block) = self.parse_body_block() else {
                self.diagnostic_current_or_span(
                    DiagnosticKind::MalformedConditional,
                    self.span_at(start),
                );
                return Some(self.span(start, end));
            };
            end = else_block.end();
        }
        let span = self.span(start, end);
        self.arena.add_if_expression(span);
        Some(span)
    }

    #[allow(dead_code)]
    fn parse_pattern(&mut self) -> Option<ByteSpan> {
        match self.current_kind() {
            Some(TokenKind::Identifier) if self.current_text() == Some("_") => {
                let span = self.current().expect("wildcard exists").span;
                self.advance();
                self.arena.add_wildcard_pattern(span);
                Some(span)
            }
            Some(
                TokenKind::IntDecimal
                | TokenKind::IntBinary
                | TokenKind::IntHex
                | TokenKind::String
                | TokenKind::KwTrue
                | TokenKind::KwFalse
                | TokenKind::KwNull,
            ) => {
                let span = self.current().expect("literal pattern exists").span;
                self.advance();
                self.arena.add_literal_pattern(span);
                Some(span)
            }
            Some(TokenKind::Identifier) => self.parse_named_pattern(),
            Some(TokenKind::LeftParen) => {
                let start = self.current()?.span.start();
                self.advance();
                let inner = self.parse_pattern()?;
                if self.current_kind() != Some(TokenKind::RightParen) {
                    self.diagnostic_current_or_span(DiagnosticKind::MalformedPattern, inner);
                    self.skip_to_pattern_boundary();
                    return None;
                }
                let end = self.current().expect("right paren exists").span.end();
                self.advance();
                let span = self.span(start, end);
                self.arena.add_grouped_pattern(span);
                Some(span)
            }
            Some(TokenKind::KwWhen) => {
                self.diagnostic_current(DiagnosticKind::UnsupportedPatternForm);
                None
            }
            Some(TokenKind::FatArrow) => {
                self.diagnostic_current(DiagnosticKind::MissingPatternArmBody);
                None
            }
            Some(_) => {
                self.diagnostic_current(DiagnosticKind::UnsupportedPatternForm);
                None
            }
            None => None,
        }
    }

    #[allow(dead_code)]
    fn parse_named_pattern(&mut self) -> Option<ByteSpan> {
        let start = self.current()?.span.start();
        let mut end = self.current()?.span.end();
        self.advance();
        let mut qualified = false;
        while self.current_kind() == Some(TokenKind::Dot)
            && self.lookahead_kind(1) == Some(TokenKind::Identifier)
        {
            qualified = true;
            self.advance();
            end = self
                .current()
                .expect("qualified pattern identifier exists")
                .span
                .end();
            self.advance();
        }
        if self.current_kind() == Some(TokenKind::LeftParen) {
            qualified = true;
            end = self.parse_pattern_arguments().unwrap_or(end);
        }
        let span = self.span(start, end);
        if qualified {
            self.arena.add_qualified_case_pattern(span);
        } else {
            self.arena.add_binding_pattern(span);
        }
        Some(span)
    }

    #[allow(dead_code)]
    fn parse_pattern_arguments(&mut self) -> Option<usize> {
        self.advance();
        if self.current_kind() == Some(TokenKind::RightParen) {
            let end = self.current().expect("right paren exists").span.end();
            self.advance();
            return Some(end);
        }
        loop {
            if self.parse_pattern().is_none() {
                self.diagnostic_current(DiagnosticKind::MalformedPattern);
                self.skip_to_pattern_boundary();
                return self.previous_span().map(|span| span.end());
            }
            match self.current_kind() {
                Some(TokenKind::Comma) => {
                    self.advance();
                    if self.current_kind() == Some(TokenKind::RightParen) {
                        self.diagnostic_current(DiagnosticKind::MalformedPattern);
                        let end = self.current().expect("right paren exists").span.end();
                        self.advance();
                        return Some(end);
                    }
                }
                Some(TokenKind::RightParen) => {
                    let end = self.current().expect("right paren exists").span.end();
                    self.advance();
                    return Some(end);
                }
                _ => {
                    self.diagnostic_current(DiagnosticKind::MalformedPattern);
                    self.skip_to_pattern_boundary();
                    return self.previous_span().map(|span| span.end());
                }
            }
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

    fn can_start_expression_kind(&self, kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::Identifier
                | TokenKind::IntDecimal
                | TokenKind::IntBinary
                | TokenKind::IntHex
                | TokenKind::String
                | TokenKind::KwTrue
                | TokenKind::KwFalse
                | TokenKind::KwNull
                | TokenKind::KwIf
                | TokenKind::LeftParen
                | TokenKind::Bang
                | TokenKind::Minus
        )
    }

    fn expression_start_can_be_assignment_target(&self, kind: TokenKind) -> bool {
        matches!(kind, TokenKind::Identifier | TokenKind::LeftParen)
    }

    fn binary_operator(&self) -> Option<(u8, TokenKind)> {
        let kind = self.current_kind()?;
        let precedence = match kind {
            TokenKind::PipePipe => 1,
            TokenKind::AmpAmp => 2,
            TokenKind::EqualEqual | TokenKind::BangEqual => 3,
            TokenKind::Less
            | TokenKind::Greater
            | TokenKind::LessEqual
            | TokenKind::GreaterEqual => 4,
            TokenKind::Plus | TokenKind::Minus => 5,
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => 6,
            _ => return None,
        };
        Some((precedence, kind))
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

    fn skip_to_expression_boundary(&mut self) {
        while let Some(kind) = self.current_kind() {
            if matches!(
                kind,
                TokenKind::Comma
                    | TokenKind::Semicolon
                    | TokenKind::RightParen
                    | TokenKind::RightBrace
            ) || self.is_declaration_starter()
            {
                return;
            }
            self.advance();
        }
    }

    fn skip_to_statement_boundary(&mut self) {
        while let Some(kind) = self.current_kind() {
            if kind == TokenKind::Semicolon {
                self.advance();
                return;
            }
            if kind == TokenKind::RightBrace || self.is_declaration_starter() {
                return;
            }
            self.advance();
        }
    }

    fn skip_deferred_construct(&mut self) {
        while let Some(kind) = self.current_kind() {
            match kind {
                TokenKind::LeftBrace => {
                    self.skip_balanced_braces();
                    return;
                }
                TokenKind::Semicolon => {
                    self.advance();
                    return;
                }
                TokenKind::RightBrace => return,
                _ => self.advance(),
            }
        }
    }

    fn skip_balanced_braces(&mut self) {
        if self.current_kind() != Some(TokenKind::LeftBrace) {
            return;
        }
        let mut depth = 0usize;
        while let Some(kind) = self.current_kind() {
            match kind {
                TokenKind::LeftBrace => {
                    depth += 1;
                    self.advance();
                }
                TokenKind::RightBrace => {
                    depth -= 1;
                    self.advance();
                    if depth == 0 {
                        return;
                    }
                }
                _ => self.advance(),
            }
        }
    }

    #[allow(dead_code)]
    fn skip_to_pattern_boundary(&mut self) {
        while let Some(kind) = self.current_kind() {
            if matches!(
                kind,
                TokenKind::Comma
                    | TokenKind::RightParen
                    | TokenKind::FatArrow
                    | TokenKind::RightBrace
            ) {
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

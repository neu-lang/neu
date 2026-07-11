use crate::ast::{AstArena, AstNodeId, AstNodeKind};
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
    pub type_name_references: Vec<ParsedTypeNameReference>,
    pub literal_expressions: Vec<ParsedLiteralExpression>,
    pub integer_literals: Vec<ParsedIntegerLiteral>,
    pub float_literals: Vec<ParsedFloatLiteral>,
    pub grouped_expressions: Vec<ParsedGroupedExpression>,
    pub unary_expressions: Vec<ParsedUnaryExpression>,
    pub binary_expressions: Vec<ParsedBinaryExpression>,
    pub if_expressions: Vec<ParsedIfExpression>,
    pub local_declarations: Vec<ParsedLocalDeclaration>,
    pub assignment_statements: Vec<ParsedAssignmentStatement>,
    pub generic_parameters: Vec<ParsedGenericParameter>,
    pub function_parameters: Vec<ParsedFunctionParameter>,
    pub function_declarations: Vec<ParsedFunctionDeclaration>,
    pub return_statements: Vec<ParsedReturnStatement>,
    pub executable_body_statements: Vec<ParsedExecutableBodyStatement>,
    pub call_expressions: Vec<ParsedCallExpression>,
    pub enum_variants: Vec<ParsedEnumVariant>,
    pub when_expressions: Vec<ParsedWhenExpression>,
    pub match_arms: Vec<ParsedMatchArm>,
    pub qualified_case_patterns: Vec<ParsedQualifiedCasePattern>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedCapabilityBound {
    pub bound: AstNodeId,
    pub name: String,
    pub name_span: ByteSpan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedGenericParameter {
    pub parameter: AstNodeId,
    pub name: String,
    pub name_span: ByteSpan,
    pub capability_bounds: Vec<ParsedCapabilityBound>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedFunctionParameter {
    pub function: AstNodeId,
    pub parameter: AstNodeId,
    pub name: String,
    pub name_span: ByteSpan,
    pub annotation: AstNodeId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedFunctionDeclaration {
    pub declaration: AstNodeId,
    pub body: Option<AstNodeId>,
    pub return_annotation: Option<AstNodeId>,
    pub parameters: Vec<AstNodeId>,
    pub top_level: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedReturnStatement {
    pub statement: AstNodeId,
    pub function: AstNodeId,
    pub block: AstNodeId,
    pub value: Option<AstNodeId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedExecutableBodyStatement {
    pub function: AstNodeId,
    pub statement: AstNodeId,
    pub span: ByteSpan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedCallExpression {
    pub expression: AstNodeId,
    pub function: AstNodeId,
    pub callee: AstNodeId,
    pub arguments: Vec<AstNodeId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedEnumVariant {
    pub enum_declaration: AstNodeId,
    pub variant: AstNodeId,
    pub name: String,
    pub name_span: ByteSpan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedWhenExpression {
    pub expression: AstNodeId,
    pub subject: AstNodeId,
    pub arms: Vec<AstNodeId>,
    pub span: ByteSpan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedMatchArm {
    pub arm: AstNodeId,
    pub pattern: AstNodeId,
    pub pattern_kind: AstNodeKind,
    pub body: AstNodeId,
    pub span: ByteSpan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedQualifiedCasePattern {
    pub pattern: AstNodeId,
    pub enum_name: String,
    pub enum_name_span: ByteSpan,
    pub variant_name: String,
    pub variant_name_span: ByteSpan,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedTypeNameReference {
    pub reference: crate::ast::AstNodeId,
    pub name: String,
    pub name_span: ByteSpan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParsedLiteralKind {
    BoolTrue,
    BoolFalse,
    AcceptedInteger,
    AcceptedString,
    Float,
    Unit,
    Null,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedLiteralExpression {
    pub expression: crate::ast::AstNodeId,
    pub kind: ParsedLiteralKind,
    pub span: ByteSpan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedIntegerLiteral {
    pub expression: AstNodeId,
    pub value: Option<u64>,
    pub span: ByteSpan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedFloatLiteral {
    pub expression: AstNodeId,
    pub bits: Option<u64>,
    pub span: ByteSpan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedGroupedExpression {
    pub expression: crate::ast::AstNodeId,
    pub inner: crate::ast::AstNodeId,
    pub span: ByteSpan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParsedUnaryOperator {
    Plus,
    Minus,
    BitwiseNot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedUnaryExpression {
    pub expression: AstNodeId,
    pub operator: ParsedUnaryOperator,
    pub operand: AstNodeId,
    pub span: ByteSpan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParsedBinaryOperator {
    LogicalOr,
    LogicalAnd,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Exponent,
    ShiftLeft,
    ShiftRight,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BinaryAssociativity {
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedBinaryExpression {
    pub expression: AstNodeId,
    pub left: AstNodeId,
    pub operator: ParsedBinaryOperator,
    pub right: AstNodeId,
    pub span: ByteSpan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedIfExpression {
    pub expression: AstNodeId,
    pub condition: AstNodeId,
    pub then_block: AstNodeId,
    pub else_block: Option<AstNodeId>,
    pub span: ByteSpan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedLocalDeclaration {
    pub declaration: AstNodeId,
    pub annotation: Option<AstNodeId>,
    pub initializer: Option<AstNodeId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedAssignmentStatement {
    pub statement: AstNodeId,
    pub target: AstNodeId,
    pub value: AstNodeId,
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
        type_name_references: parser.type_name_references,
        literal_expressions: parser.literal_expressions,
        integer_literals: parser.integer_literals,
        float_literals: parser.float_literals,
        grouped_expressions: parser.grouped_expressions,
        unary_expressions: parser.unary_expressions,
        binary_expressions: parser.binary_expressions,
        if_expressions: parser.if_expressions,
        local_declarations: parser.local_declarations,
        assignment_statements: parser.assignment_statements,
        generic_parameters: parser.generic_parameters,
        function_parameters: parser.function_parameters,
        function_declarations: parser.function_declarations,
        return_statements: parser.return_statements,
        executable_body_statements: parser.executable_body_statements,
        call_expressions: parser.call_expressions,
        enum_variants: parser.enum_variants,
        when_expressions: parser.when_expressions,
        match_arms: parser.match_arms,
        qualified_case_patterns: parser.qualified_case_patterns,
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
    type_name_references: Vec<ParsedTypeNameReference>,
    literal_expressions: Vec<ParsedLiteralExpression>,
    integer_literals: Vec<ParsedIntegerLiteral>,
    float_literals: Vec<ParsedFloatLiteral>,
    grouped_expressions: Vec<ParsedGroupedExpression>,
    unary_expressions: Vec<ParsedUnaryExpression>,
    binary_expressions: Vec<ParsedBinaryExpression>,
    if_expressions: Vec<ParsedIfExpression>,
    local_declarations: Vec<ParsedLocalDeclaration>,
    assignment_statements: Vec<ParsedAssignmentStatement>,
    generic_parameters: Vec<ParsedGenericParameter>,
    function_parameters: Vec<ParsedFunctionParameter>,
    function_declarations: Vec<ParsedFunctionDeclaration>,
    return_statements: Vec<ParsedReturnStatement>,
    executable_body_statements: Vec<ParsedExecutableBodyStatement>,
    call_expressions: Vec<ParsedCallExpression>,
    enum_variants: Vec<ParsedEnumVariant>,
    when_expressions: Vec<ParsedWhenExpression>,
    match_arms: Vec<ParsedMatchArm>,
    qualified_case_patterns: Vec<ParsedQualifiedCasePattern>,
    block_return_indices: Vec<Vec<usize>>,
    saw_package_or_import: bool,
    saw_top_level_declaration: bool,
    current_function: Option<AstNodeId>,
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
            generic_parameters: Vec::new(),
            function_parameters: Vec::new(),
            function_declarations: Vec::new(),
            return_statements: Vec::new(),
            executable_body_statements: Vec::new(),
            call_expressions: Vec::new(),
            enum_variants: Vec::new(),
            when_expressions: Vec::new(),
            match_arms: Vec::new(),
            qualified_case_patterns: Vec::new(),
            block_return_indices: Vec::new(),
            name_references: Vec::new(),
            type_name_references: Vec::new(),
            literal_expressions: Vec::new(),
            integer_literals: Vec::new(),
            float_literals: Vec::new(),
            grouped_expressions: Vec::new(),
            unary_expressions: Vec::new(),
            binary_expressions: Vec::new(),
            if_expressions: Vec::new(),
            local_declarations: Vec::new(),
            assignment_statements: Vec::new(),
            saw_package_or_import: false,
            saw_top_level_declaration: false,
            current_function: None,
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
        let keyword = self.current().expect("function token exists").clone();
        let start = keyword.span.start();
        self.advance();
        if self.current_kind() != Some(TokenKind::Identifier) {
            self.diagnostic(DiagnosticKind::MissingDeclarationName, keyword.span);
            self.skip_to_declaration_boundary(in_body);
            return;
        }
        let name = self.current().expect("function name exists").clone();
        self.advance();

        self.parse_generic_parameters();

        let parameters = if self.function_parameter_list_has_body() {
            let Some(parameters) = self.parse_typed_function_parameters() else {
                self.diagnostic(
                    DiagnosticKind::MalformedDeclarationHeader,
                    self.span_at(start),
                );
                self.skip_to_declaration_boundary(in_body);
                return;
            };
            parameters
        } else {
            if !self.consume_balanced_parentheses() {
                self.diagnostic(
                    DiagnosticKind::MalformedDeclarationHeader,
                    self.span_at(start),
                );
                self.skip_to_declaration_boundary(in_body);
                return;
            }
            Vec::new()
        };

        let mut return_annotation = None;
        if self.current_kind() == Some(TokenKind::Colon) {
            self.advance();
            let Some(return_span) = self.parse_type() else {
                self.skip_to_declaration_boundary(in_body);
                return;
            };
            return_annotation = self.latest_type_node_for_span(return_span);
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
                self.function_declarations.push(ParsedFunctionDeclaration {
                    declaration,
                    body: None,
                    return_annotation,
                    parameters: parameters
                        .into_iter()
                        .map(|parameter| parameter.parameter)
                        .collect(),
                    top_level: !in_body,
                });
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
                let parameter_nodes = parameters
                    .iter()
                    .map(|parameter| parameter.parameter)
                    .collect();
                self.record_function_parameters(declaration, parameters);
                let previous_function = self.current_function.replace(declaration);
                let body = self
                    .parse_body_block()
                    .and_then(|span| self.latest_node_for_span(span, AstNodeKind::Block));
                self.current_function = previous_function;
                self.function_declarations.push(ParsedFunctionDeclaration {
                    declaration,
                    body,
                    return_annotation,
                    parameters: parameter_nodes,
                    top_level: !in_body,
                });
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

    fn parse_typed_function_parameters(&mut self) -> Option<Vec<ParsedFunctionParameter>> {
        if self.current_kind() != Some(TokenKind::LeftParen) {
            return None;
        }
        self.advance();
        let mut parameters = Vec::new();
        if self.current_kind() == Some(TokenKind::RightParen) {
            self.advance();
            return Some(parameters);
        }
        loop {
            let name = self.current()?.clone();
            if name.kind != TokenKind::Identifier {
                self.diagnostic_current_or_span(
                    DiagnosticKind::MalformedDeclarationHeader,
                    name.span,
                );
                return None;
            }
            self.advance();
            if self.current_kind() != Some(TokenKind::Colon) {
                self.diagnostic_current_or_span(
                    DiagnosticKind::MalformedDeclarationHeader,
                    name.span,
                );
                return None;
            }
            self.advance();
            let annotation_span = self.parse_named_type()?;
            let annotation = self
                .type_name_references
                .iter()
                .rev()
                .find(|reference| reference.name_span == annotation_span)
                .map(|reference| reference.reference);
            let Some(annotation) = annotation else {
                self.diagnostic_current_or_span(
                    DiagnosticKind::MalformedDeclarationHeader,
                    name.span,
                );
                return None;
            };
            let parameter = self
                .arena
                .add_function_parameter(self.span(name.span.start(), annotation_span.end()));
            parameters.push(ParsedFunctionParameter {
                function: AstNodeId::from_raw(usize::MAX),
                parameter,
                name: self.text[name.span.start()..name.span.end()].to_owned(),
                name_span: name.span,
                annotation,
            });
            match self.current_kind() {
                Some(TokenKind::Comma) => self.advance(),
                Some(TokenKind::RightParen) => {
                    self.advance();
                    return Some(parameters);
                }
                _ => {
                    self.diagnostic_current_or_span(
                        DiagnosticKind::MalformedDeclarationHeader,
                        name.span,
                    );
                    return None;
                }
            }
        }
    }

    fn record_function_parameters(
        &mut self,
        function: AstNodeId,
        parameters: Vec<ParsedFunctionParameter>,
    ) {
        self.function_parameters
            .extend(parameters.into_iter().map(|mut parameter| {
                parameter.function = function;
                parameter
            }));
    }

    fn function_parameter_list_has_body(&self) -> bool {
        let mut index = self.index;
        let mut depth = 0usize;
        while let Some(token) = self.tokens.get(index) {
            match token.kind {
                TokenKind::LeftParen => depth += 1,
                TokenKind::RightParen => {
                    depth -= 1;
                    if depth == 0 {
                        index += 1;
                        break;
                    }
                }
                _ => {}
            }
            index += 1;
        }
        while let Some(token) = self.tokens.get(index) {
            match token.kind {
                TokenKind::LeftBrace => return true,
                TokenKind::Semicolon => return false,
                _ => index += 1,
            }
        }
        false
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
        if kind == AstNodeKind::EnumDeclaration {
            self.parse_enum_body(declaration);
        } else {
            self.parse_declaration_body();
        }
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

    fn parse_enum_body(&mut self, declaration: AstNodeId) {
        let left_brace = self
            .current()
            .expect("enum body starts with left brace")
            .span;
        self.arena.add_declaration_body(left_brace);
        self.advance();

        while !self.is_eof() {
            match self.current_kind() {
                Some(TokenKind::RightBrace) => {
                    self.advance();
                    return;
                }
                Some(TokenKind::Identifier) => {
                    let name = self.current().expect("enum variant name exists").clone();
                    self.advance();
                    match self.current_kind() {
                        Some(TokenKind::Comma | TokenKind::Semicolon | TokenKind::RightBrace) => {
                            let variant = self.arena.add_enum_variant(name.span);
                            self.enum_variants.push(ParsedEnumVariant {
                                enum_declaration: declaration,
                                variant,
                                name: self.text[name.span.start()..name.span.end()].to_owned(),
                                name_span: name.span,
                            });
                            if self.current_kind() != Some(TokenKind::RightBrace) {
                                self.advance();
                            }
                        }
                        _ => {
                            self.diagnostic_current_or_span(
                                DiagnosticKind::UnexpectedTokenInDeclarationBody,
                                name.span,
                            );
                            self.skip_to_enum_variant_boundary();
                        }
                    }
                }
                Some(TokenKind::Comma | TokenKind::Semicolon) => {
                    self.diagnostic_current(DiagnosticKind::UnexpectedTokenInDeclarationBody);
                    self.advance();
                }
                Some(_) => {
                    self.diagnostic_current(DiagnosticKind::UnexpectedTokenInDeclarationBody);
                    self.skip_to_enum_variant_boundary();
                }
                None => return,
            }
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
        self.block_return_indices.push(Vec::new());

        while !self.is_eof() {
            match self.current_kind() {
                Some(TokenKind::RightBrace) => {
                    let end = self.current().expect("right brace exists").span.end();
                    self.advance();
                    let span = self.span(start, end);
                    let block = self.arena.add_block(span);
                    let return_indices = self
                        .block_return_indices
                        .pop()
                        .expect("each parsed body block has a return frame");
                    for return_index in return_indices {
                        self.return_statements[return_index].block = block;
                    }
                    return Some(span);
                }
                Some(TokenKind::KwConst | TokenKind::KwVar | TokenKind::KwReturn) => {
                    self.parse_statement();
                }
                Some(TokenKind::Identifier) if self.current_text() == Some("async") => {
                    self.diagnostic_current(DiagnosticKind::MalformedCoroutineConstruct);
                    self.advance();
                    self.skip_deferred_construct();
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
                            let target = self.latest_expression_node_for_span(expression_span);
                            self.advance();
                            let Some(value_span) = self.parse_expression() else {
                                self.diagnostic_current_or_span(
                                    DiagnosticKind::MalformedAssignment,
                                    expression_span,
                                );
                                self.skip_to_statement_boundary();
                                continue;
                            };
                            let value = self.latest_expression_node_for_span(value_span);
                            if self.current_kind() == Some(TokenKind::Semicolon) {
                                let end = self.current().expect("semicolon exists").span.end();
                                self.advance();
                                let statement = self
                                    .arena
                                    .add_assignment_statement(self.span(expression_start, end));
                                if let (Some(target), Some(value)) = (target, value) {
                                    self.assignment_statements.push(ParsedAssignmentStatement {
                                        statement,
                                        target,
                                        value,
                                    });
                                    self.record_executable_body_statement(statement);
                                }
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
                            self.diagnostic(
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
                Some(_) => {
                    self.diagnostic_current(DiagnosticKind::UnexpectedTokenInStatement);
                    self.advance();
                    self.skip_to_statement_boundary();
                }
                None => break,
            }
        }

        self.diagnostic(DiagnosticKind::MalformedBlock, self.span_at(start));
        self.block_return_indices
            .pop()
            .expect("each parsed body block has a return frame");
        None
    }

    fn parse_statement(&mut self) {
        match self.current_kind() {
            Some(TokenKind::KwConst | TokenKind::KwVar) => {
                self.parse_variable_declaration_statement()
            }
            Some(TokenKind::KwReturn) => self.parse_return_statement(),
            Some(TokenKind::Identifier) if self.current_text() == Some("async") => {
                self.diagnostic_current(DiagnosticKind::MalformedCoroutineConstruct);
                self.advance();
                self.skip_deferred_construct();
            }
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
            .expect("variable declaration starts with const or var")
        {
            TokenKind::KwConst => LocalBindingKind::Immutable,
            TokenKind::KwVar => LocalBindingKind::Var,
            _ => unreachable!("variable declaration starts with const or var"),
        };
        let start = self
            .current()
            .expect("variable declaration starts with const or var")
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

        let mut annotation = None;
        if self.current_kind() == Some(TokenKind::Colon) {
            self.advance();
            if let Some(span) = self.parse_type() {
                annotation = self.latest_type_node_for_span(span);
            } else {
                self.diagnostic_current_or_span(
                    DiagnosticKind::MalformedVariableDeclaration,
                    self.span_at(start),
                );
                self.skip_to_statement_boundary();
                return;
            }
        }

        let mut initializer = None;
        if self.current_kind() == Some(TokenKind::Equal) {
            self.advance();
            if let Some(span) = self.parse_expression() {
                initializer = self.latest_expression_node_for_span(span);
            } else {
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
            self.local_declarations.push(ParsedLocalDeclaration {
                declaration: binding,
                annotation,
                initializer,
            });
            self.local_binding_names.push(ParsedLocalBindingName {
                binding,
                kind,
                name: self.text[name.span.start()..name.span.end()].to_owned(),
                name_span: name.span,
            });
            self.record_executable_body_statement(binding);
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

        let value = if self.current_kind() == Some(TokenKind::Semicolon) {
            None
        } else if let Some(span) = self.parse_expression() {
            self.latest_expression_node_for_span(span)
        } else {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedReturnStatement,
                self.span_at(start),
            );
            self.skip_to_statement_boundary();
            return;
        };

        if self.current_kind() == Some(TokenKind::Semicolon) {
            let end = self.current().expect("semicolon exists").span.end();
            self.advance();
            let statement = self.arena.add_return_statement(self.span(start, end));
            if let Some(function) = self.current_function {
                let return_index = self.return_statements.len();
                self.return_statements.push(ParsedReturnStatement {
                    statement,
                    function,
                    block: AstNodeId::from_raw(usize::MAX),
                    value,
                });
                self.record_executable_body_statement(statement);
                if let Some(indices) = self.block_return_indices.last_mut() {
                    indices.push(return_index);
                }
            }
        } else {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedReturnStatement,
                self.span_at(start),
            );
            self.skip_to_statement_boundary();
        }
    }

    fn record_executable_body_statement(&mut self, statement: AstNodeId) {
        let Some(function) = self.current_function else {
            return;
        };
        let span = self
            .arena
            .node(statement)
            .expect("parsed executable statement")
            .span;
        self.executable_body_statements
            .push(ParsedExecutableBodyStatement {
                function,
                statement,
                span,
            });
    }

    fn parse_expression(&mut self) -> Option<ByteSpan> {
        self.parse_binary_expression(1)
    }

    fn parse_binary_expression(&mut self, min_precedence: u8) -> Option<ByteSpan> {
        let mut left = self.parse_unary_expression()?;
        while let Some((precedence, associativity, operator)) = self.binary_operator() {
            if precedence < min_precedence {
                break;
            }
            let left_span = left;
            let operator_span = self.current().expect("operator token exists").span;
            self.advance();
            let next_min_precedence = match associativity {
                BinaryAssociativity::Left => precedence + 1,
                BinaryAssociativity::Right => precedence,
            };
            let Some(right) = self.parse_binary_expression(next_min_precedence) else {
                self.diagnostic(DiagnosticKind::MalformedBinaryExpression, operator_span);
                return Some(left);
            };
            left = self.span(left.start(), right.end());
            let expression = self.arena.add_binary_expression(left);
            if let (Some(left_node), Some(right_node), Some(operator)) = (
                self.latest_expression_node_for_span(left_span),
                self.latest_expression_node_for_span(right),
                parsed_binary_operator(operator),
            ) {
                self.record_binary_expression(ParsedBinaryExpression {
                    expression,
                    left: left_node,
                    operator,
                    right: right_node,
                    span: left,
                });
            }
        }
        Some(left)
    }

    fn parse_unary_expression(&mut self) -> Option<ByteSpan> {
        match self.current_kind() {
            Some(TokenKind::Bang | TokenKind::Plus | TokenKind::Minus | TokenKind::Tilde) => {
                let operator = self.current_kind().expect("unary operator exists");
                let start = self.current().expect("unary operator exists").span.start();
                self.advance();
                let operand = self.parse_unary_expression()?;
                let span = self.span(start, operand.end());
                let expression = self.arena.add_unary_expression(span);
                if let (Some(operator), Some(operand)) = (
                    parsed_unary_operator(operator),
                    self.latest_expression_node_for_span(operand),
                ) {
                    self.record_unary_expression(ParsedUnaryExpression {
                        expression,
                        operator,
                        operand,
                        span,
                    });
                }
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
                    let callee = self.latest_expression_node_for_span(span);
                    let Some((end, arguments)) = self.parse_argument_list() else {
                        self.diagnostic_current_or_span(
                            DiagnosticKind::MalformedCallExpression,
                            span,
                        );
                        return Some(span);
                    };
                    span = self.span(start, end);
                    let expression = self.arena.add_call_expression(span);
                    if let (Some(function), Some(callee), Some(arguments)) =
                        (self.current_function, callee, arguments)
                    {
                        self.call_expressions.push(ParsedCallExpression {
                            expression,
                            function,
                            callee,
                            arguments,
                        });
                    }
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

    fn parse_argument_list(&mut self) -> Option<(usize, Option<Vec<AstNodeId>>)> {
        if self.current_kind() != Some(TokenKind::LeftParen) {
            return None;
        }
        self.advance();
        if self.current_kind() == Some(TokenKind::RightParen) {
            let end = self.current().expect("right paren exists").span.end();
            self.advance();
            return Some((end, Some(Vec::new())));
        }
        let mut arguments = Vec::new();
        loop {
            let Some(argument_span) = self.parse_expression() else {
                self.diagnostic_current(DiagnosticKind::MalformedCallExpression);
                self.skip_to_expression_boundary();
                return self.previous_span().map(|span| (span.end(), None));
            };
            let Some(argument) = self.latest_expression_node_for_span(argument_span) else {
                self.diagnostic_current_or_span(
                    DiagnosticKind::MalformedCallExpression,
                    argument_span,
                );
                self.skip_to_expression_boundary();
                return self.previous_span().map(|span| (span.end(), None));
            };
            arguments.push(argument);
            match self.current_kind() {
                Some(TokenKind::Comma) => {
                    self.advance();
                    if self.current_kind() == Some(TokenKind::RightParen) {
                        self.diagnostic_current(DiagnosticKind::MalformedCallExpression);
                        let end = self.current().expect("right paren exists").span.end();
                        self.advance();
                        return Some((end, None));
                    }
                }
                Some(TokenKind::RightParen) => {
                    let end = self.current().expect("right paren exists").span.end();
                    self.advance();
                    return Some((end, Some(arguments)));
                }
                _ => {
                    self.diagnostic_current(DiagnosticKind::MalformedCallExpression);
                    self.skip_to_expression_boundary();
                    return self.previous_span().map(|span| (span.end(), None));
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
                | TokenKind::FloatDecimal
                | TokenKind::String
                | TokenKind::KwTrue
                | TokenKind::KwFalse
                | TokenKind::KwNull,
            ) => {
                let token = self.current().expect("literal token exists").clone();
                let span = token.span;
                self.advance();
                let expression = self.arena.add_literal_expression(span);
                self.literal_expressions.push(ParsedLiteralExpression {
                    expression,
                    kind: parsed_literal_kind(token.kind),
                    span,
                });
                if matches!(
                    token.kind,
                    TokenKind::IntDecimal | TokenKind::IntBinary | TokenKind::IntHex
                ) {
                    self.integer_literals.push(ParsedIntegerLiteral {
                        expression,
                        value: parsed_integer_value(
                            token.kind,
                            &self.text[span.start()..span.end()],
                        ),
                        span,
                    });
                } else if token.kind == TokenKind::FloatDecimal {
                    let bits = self.text[span.start()..span.end()]
                        .parse::<f64>()
                        .ok()
                        .filter(|value| value.is_finite())
                        .map(f64::to_bits);
                    self.float_literals.push(ParsedFloatLiteral {
                        expression,
                        bits,
                        span,
                    });
                }
                Some(span)
            }
            Some(TokenKind::Identifier) => self.parse_name_expression(),
            Some(TokenKind::LeftParen) if self.peek_kind() == Some(TokenKind::RightParen) => {
                let start = self.current().expect("left paren exists").span.start();
                self.advance();
                let end = self.current().expect("right paren exists").span.end();
                self.advance();
                let span = self.span(start, end);
                let expression = self.arena.add_literal_expression(span);
                self.literal_expressions.push(ParsedLiteralExpression {
                    expression,
                    kind: ParsedLiteralKind::Unit,
                    span,
                });
                Some(span)
            }
            Some(TokenKind::LeftParen) => self.parse_grouped_expression(),
            Some(TokenKind::KwIf) => self.parse_if_expression(),
            Some(TokenKind::KwWhen) => self.parse_when_expression(),
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

    fn latest_type_node_for_span(&self, span: ByteSpan) -> Option<AstNodeId> {
        self.arena
            .nodes()
            .iter()
            .rev()
            .find(|node| node.span == span && type_node_kind(node.kind))
            .map(|node| node.id)
    }

    fn latest_expression_node_for_span(&self, span: ByteSpan) -> Option<AstNodeId> {
        self.arena
            .nodes()
            .iter()
            .rev()
            .find(|node| node.span == span && expression_node_kind(node.kind))
            .map(|node| node.id)
    }

    fn latest_node_for_span(&self, span: ByteSpan, kind: AstNodeKind) -> Option<AstNodeId> {
        self.arena
            .nodes()
            .iter()
            .rev()
            .find(|node| node.span == span && node.kind == kind)
            .map(|node| node.id)
    }

    fn latest_pattern_node_for_span(&self, span: ByteSpan) -> Option<AstNodeId> {
        self.arena
            .nodes()
            .iter()
            .rev()
            .find(|node| {
                node.span == span
                    && matches!(
                        node.kind,
                        AstNodeKind::WildcardPattern
                            | AstNodeKind::LiteralPattern
                            | AstNodeKind::BindingPattern
                            | AstNodeKind::QualifiedCasePattern
                            | AstNodeKind::GroupedPattern
                    )
            })
            .map(|node| node.id)
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
        let expression = self.arena.add_grouped_expression(span);
        if let Some(inner) = self.latest_expression_node_for_span(inner) {
            self.record_grouped_expression(ParsedGroupedExpression {
                expression,
                inner,
                span,
            });
        }
        Some(span)
    }

    fn record_grouped_expression(&mut self, grouped: ParsedGroupedExpression) {
        let index = self
            .grouped_expressions
            .partition_point(|existing| existing.span.start() <= grouped.span.start());
        self.grouped_expressions.insert(index, grouped);
    }

    fn record_binary_expression(&mut self, binary: ParsedBinaryExpression) {
        let index = self
            .binary_expressions
            .partition_point(|existing| existing.span.start() <= binary.span.start());
        self.binary_expressions.insert(index, binary);
    }

    fn record_unary_expression(&mut self, unary: ParsedUnaryExpression) {
        let index = self
            .unary_expressions
            .partition_point(|existing| existing.span.start() <= unary.span.start());
        self.unary_expressions.insert(index, unary);
    }

    fn record_if_expression(&mut self, if_expression: ParsedIfExpression) {
        let index = self
            .if_expressions
            .partition_point(|existing| existing.span.start() <= if_expression.span.start());
        self.if_expressions.insert(index, if_expression);
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
        let Some(condition_span) = self.parse_expression() else {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedConditional,
                self.span_at(start),
            );
            self.skip_to_expression_boundary();
            return None;
        };
        let condition = self.latest_expression_node_for_span(condition_span);
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
        let then_block_node = self.latest_node_for_span(then_block, AstNodeKind::Block);
        let mut end = then_block.end();
        let mut else_block_node = None;
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
            else_block_node = self.latest_node_for_span(else_block, AstNodeKind::Block);
        }
        let span = self.span(start, end);
        let expression = self.arena.add_if_expression(span);
        if let (Some(condition), Some(then_block)) = (condition, then_block_node) {
            self.record_if_expression(ParsedIfExpression {
                expression,
                condition,
                then_block,
                else_block: else_block_node,
                span,
            });
        }
        Some(span)
    }

    fn parse_when_expression(&mut self) -> Option<ByteSpan> {
        let start = self.current()?.span.start();
        self.advance();
        if self.current_kind() != Some(TokenKind::LeftParen) {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedConditional,
                self.span_at(start),
            );
            return None;
        }
        self.advance();
        let subject_span = self.parse_expression()?;
        let subject = self.latest_expression_node_for_span(subject_span)?;
        if self.current_kind() != Some(TokenKind::RightParen) {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedConditional,
                self.span_at(start),
            );
            return None;
        }
        self.advance();
        if self.current_kind() != Some(TokenKind::LeftBrace) {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedConditional,
                self.span_at(start),
            );
            return None;
        }
        self.advance();
        let mut arms = Vec::new();
        while !self.is_eof() && self.current_kind() != Some(TokenKind::RightBrace) {
            let arm_start = self.current().map_or(start, |token| token.span.start());
            let Some(pattern_span) = self.parse_pattern() else {
                self.skip_to_pattern_boundary();
                continue;
            };
            let Some(pattern) = self.latest_pattern_node_for_span(pattern_span) else {
                self.skip_to_pattern_boundary();
                continue;
            };
            let pattern_kind = self.arena.node(pattern).expect("pattern exists").kind;
            if !matches!(
                pattern_kind,
                AstNodeKind::QualifiedCasePattern | AstNodeKind::WildcardPattern
            ) || self.current_kind() != Some(TokenKind::Arrow)
            {
                self.diagnostic_current_or_span(DiagnosticKind::MalformedPattern, pattern_span);
                self.skip_to_pattern_boundary();
                continue;
            }
            self.advance();
            let Some(body_span) = self.parse_expression() else {
                self.diagnostic_current_or_span(
                    DiagnosticKind::MissingPatternArmBody,
                    pattern_span,
                );
                self.skip_to_pattern_boundary();
                continue;
            };
            let Some(body) = self.latest_expression_node_for_span(body_span) else {
                continue;
            };
            let end = if self.current_kind() == Some(TokenKind::Semicolon) {
                let end = self.current().expect("semicolon exists").span.end();
                self.advance();
                end
            } else {
                body_span.end()
            };
            let span = self.span(arm_start, end);
            let arm = self.arena.add_match_arm(span);
            self.match_arms.push(ParsedMatchArm {
                arm,
                pattern,
                pattern_kind,
                body,
                span,
            });
            arms.push(arm);
        }
        if self.current_kind() != Some(TokenKind::RightBrace) {
            self.diagnostic_current_or_span(
                DiagnosticKind::MalformedConditional,
                self.span_at(start),
            );
            return None;
        }
        let end = self.current().expect("right brace exists").span.end();
        self.advance();
        let span = self.span(start, end);
        let expression = self.arena.add_when_expression(span);
        self.when_expressions.push(ParsedWhenExpression {
            expression,
            subject,
            arms,
            span,
        });
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
        let first = self.current()?.clone();
        let start = first.span.start();
        let mut end = first.span.end();
        let mut names = vec![first];
        self.advance();
        let mut qualified = false;
        while self.current_kind() == Some(TokenKind::Dot)
            && self.lookahead_kind(1) == Some(TokenKind::Identifier)
        {
            qualified = true;
            self.advance();
            let name = self
                .current()
                .expect("qualified pattern identifier exists")
                .clone();
            end = name.span.end();
            names.push(name);
            self.advance();
        }
        let has_arguments = self.current_kind() == Some(TokenKind::LeftParen);
        if self.current_kind() == Some(TokenKind::LeftParen) {
            qualified = true;
            end = self.parse_pattern_arguments().unwrap_or(end);
        }
        let span = self.span(start, end);
        if qualified {
            let pattern = self.arena.add_qualified_case_pattern(span);
            if names.len() == 2 && !has_arguments {
                self.qualified_case_patterns
                    .push(ParsedQualifiedCasePattern {
                        pattern,
                        enum_name: self.text[names[0].span.start()..names[0].span.end()].to_owned(),
                        enum_name_span: names[0].span,
                        variant_name: self.text[names[1].span.start()..names[1].span.end()]
                            .to_owned(),
                        variant_name_span: names[1].span,
                    });
            }
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
            let name = self
                .current()
                .expect("generic parameter name exists")
                .clone();
            self.advance();

            let mut capability_bounds = Vec::new();
            if self.current_kind() == Some(TokenKind::Colon) {
                self.advance();
                if !self.can_start_capability_bound() {
                    self.diagnostic_at_previous_or_current(DiagnosticKind::MissingGenericBound);
                    self.skip_to_generic_parameter_boundary();
                    return;
                }
                capability_bounds = self.parse_capability_bound_list();
            }

            let parameter_end = self
                .previous_span()
                .map_or(parameter_start, |span| span.end());
            let parameter = self
                .arena
                .add_generic_parameter(self.span(parameter_start, parameter_end));
            let bound_count = capability_bounds.len();
            self.generic_parameters.push(ParsedGenericParameter {
                parameter,
                name: self.text[name.span.start()..name.span.end()].to_owned(),
                name_span: name.span,
                capability_bounds,
            });

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
                Some(TokenKind::Greater | TokenKind::GreaterGreater) => {
                    self.consume_generic_greater();
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
        let name_end = self.parse_qualified_name_for_type()?;
        let mut end = name_end;
        let reference_index = self.type_name_references.len();
        if self.current_kind() == Some(TokenKind::Less) {
            end = self.parse_generic_arguments().unwrap_or(end);
        }
        let span = self.span(start, end);
        let reference = self.arena.add_named_type(span);
        self.type_name_references.insert(
            reference_index,
            ParsedTypeNameReference {
                reference,
                name: self.text[start..name_end].to_owned(),
                name_span: self.span(start, name_end),
            },
        );
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
                Some(TokenKind::Greater | TokenKind::GreaterGreater) => {
                    let end = self
                        .consume_generic_greater()
                        .expect("generic greater token exists");
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

    fn parse_capability_bound_list(&mut self) -> Vec<ParsedCapabilityBound> {
        let mut bounds = Vec::new();
        loop {
            let Some(bound) = self.parse_capability_bound() else {
                return bounds;
            };
            bounds.push(bound);
            if self.current_kind() != Some(TokenKind::Amp) {
                return bounds;
            }
            self.advance();
            if !self.can_start_capability_bound() {
                self.diagnostic_at_previous_or_current(DiagnosticKind::MalformedCapabilityBound);
                return bounds;
            }
        }
    }

    fn parse_capability_bound(&mut self) -> Option<ParsedCapabilityBound> {
        let start = self.current()?.span.start();
        let end = self.parse_qualified_name_for_type()?;
        let span = self.span(start, end);
        let bound = self.arena.add_capability_bound(span);
        Some(ParsedCapabilityBound {
            bound,
            name: self.text[start..end].to_owned(),
            name_span: span,
        })
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
                | TokenKind::FloatDecimal
                | TokenKind::String
                | TokenKind::KwTrue
                | TokenKind::KwFalse
                | TokenKind::KwNull
                | TokenKind::KwIf
                | TokenKind::KwWhen
                | TokenKind::LeftParen
                | TokenKind::Bang
                | TokenKind::Minus
        )
    }

    fn expression_start_can_be_assignment_target(&self, kind: TokenKind) -> bool {
        matches!(kind, TokenKind::Identifier | TokenKind::LeftParen)
    }

    fn binary_operator(&self) -> Option<(u8, BinaryAssociativity, TokenKind)> {
        let kind = self.current_kind()?;
        let (precedence, associativity) = match kind {
            TokenKind::PipePipe => (1, BinaryAssociativity::Left),
            TokenKind::AmpAmp => (2, BinaryAssociativity::Left),
            TokenKind::Pipe => (3, BinaryAssociativity::Left),
            TokenKind::Caret => (4, BinaryAssociativity::Left),
            TokenKind::Amp => (5, BinaryAssociativity::Left),
            TokenKind::EqualEqual | TokenKind::BangEqual => (6, BinaryAssociativity::Left),
            TokenKind::Less
            | TokenKind::Greater
            | TokenKind::LessEqual
            | TokenKind::GreaterEqual => (7, BinaryAssociativity::Left),
            TokenKind::LessLess | TokenKind::GreaterGreater => (8, BinaryAssociativity::Left),
            TokenKind::Plus | TokenKind::Minus => (9, BinaryAssociativity::Left),
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => {
                (10, BinaryAssociativity::Left)
            }
            TokenKind::StarStar => (11, BinaryAssociativity::Right),
            _ => return None,
        };
        Some((precedence, associativity, kind))
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

    fn skip_to_enum_variant_boundary(&mut self) {
        while let Some(kind) = self.current_kind() {
            if matches!(
                kind,
                TokenKind::Comma | TokenKind::Semicolon | TokenKind::RightBrace
            ) {
                if kind != TokenKind::RightBrace {
                    self.advance();
                }
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

    fn peek_kind(&self) -> Option<TokenKind> {
        self.tokens.get(self.index + 1).map(|token| token.kind)
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

    fn consume_generic_greater(&mut self) -> Option<usize> {
        let token = self.tokens.get_mut(self.index)?;
        match token.kind {
            TokenKind::Greater => {
                let end = token.span.end();
                self.index += 1;
                Some(end)
            }
            TokenKind::GreaterGreater => {
                let start = token.span.start();
                let first_end = start + 1;
                let original_end = token.span.end();
                token.kind = TokenKind::Greater;
                token.span =
                    ByteSpan::new(self.file, first_end, original_end).expect("split >> span");
                Some(first_end)
            }
            _ => None,
        }
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

fn parsed_literal_kind(kind: TokenKind) -> ParsedLiteralKind {
    match kind {
        TokenKind::KwTrue => ParsedLiteralKind::BoolTrue,
        TokenKind::KwFalse => ParsedLiteralKind::BoolFalse,
        TokenKind::IntDecimal | TokenKind::IntBinary | TokenKind::IntHex => {
            ParsedLiteralKind::AcceptedInteger
        }
        TokenKind::String => ParsedLiteralKind::AcceptedString,
        TokenKind::FloatDecimal => ParsedLiteralKind::Float,
        TokenKind::KwNull => ParsedLiteralKind::Null,
        _ => unreachable!("parser literal metadata is only built for literal tokens"),
    }
}

fn parsed_integer_value(kind: TokenKind, text: &str) -> Option<u64> {
    let (radix, digits) = match kind {
        TokenKind::IntDecimal => (10, text),
        TokenKind::IntBinary => (
            2,
            text.strip_prefix("0b")
                .or_else(|| text.strip_prefix("0B"))?,
        ),
        TokenKind::IntHex => (
            16,
            text.strip_prefix("0x")
                .or_else(|| text.strip_prefix("0X"))?,
        ),
        _ => return None,
    };
    u64::from_str_radix(&digits.replace('_', ""), radix).ok()
}

fn parsed_binary_operator(kind: TokenKind) -> Option<ParsedBinaryOperator> {
    let operator = match kind {
        TokenKind::PipePipe => ParsedBinaryOperator::LogicalOr,
        TokenKind::AmpAmp => ParsedBinaryOperator::LogicalAnd,
        TokenKind::EqualEqual => ParsedBinaryOperator::Equal,
        TokenKind::BangEqual => ParsedBinaryOperator::NotEqual,
        TokenKind::Less => ParsedBinaryOperator::Less,
        TokenKind::Greater => ParsedBinaryOperator::Greater,
        TokenKind::LessEqual => ParsedBinaryOperator::LessEqual,
        TokenKind::GreaterEqual => ParsedBinaryOperator::GreaterEqual,
        TokenKind::Plus => ParsedBinaryOperator::Plus,
        TokenKind::Minus => ParsedBinaryOperator::Minus,
        TokenKind::Star => ParsedBinaryOperator::Star,
        TokenKind::Slash => ParsedBinaryOperator::Slash,
        TokenKind::Percent => ParsedBinaryOperator::Percent,
        TokenKind::StarStar => ParsedBinaryOperator::Exponent,
        TokenKind::LessLess => ParsedBinaryOperator::ShiftLeft,
        TokenKind::GreaterGreater => ParsedBinaryOperator::ShiftRight,
        TokenKind::Amp => ParsedBinaryOperator::BitwiseAnd,
        TokenKind::Caret => ParsedBinaryOperator::BitwiseXor,
        TokenKind::Pipe => ParsedBinaryOperator::BitwiseOr,
        _ => return None,
    };
    Some(operator)
}

fn parsed_unary_operator(kind: TokenKind) -> Option<ParsedUnaryOperator> {
    match kind {
        TokenKind::Plus => Some(ParsedUnaryOperator::Plus),
        TokenKind::Minus => Some(ParsedUnaryOperator::Minus),
        TokenKind::Tilde => Some(ParsedUnaryOperator::BitwiseNot),
        _ => None,
    }
}

fn type_node_kind(kind: AstNodeKind) -> bool {
    matches!(
        kind,
        AstNodeKind::NamedType
            | AstNodeKind::NullableType
            | AstNodeKind::FunctionType
            | AstNodeKind::GroupedType
    )
}

fn expression_node_kind(kind: AstNodeKind) -> bool {
    matches!(
        kind,
        AstNodeKind::LiteralExpression
            | AstNodeKind::NameExpression
            | AstNodeKind::GroupedExpression
            | AstNodeKind::IfExpression
            | AstNodeKind::WhenExpression
            | AstNodeKind::BinaryExpression
            | AstNodeKind::UnaryExpression
            | AstNodeKind::CallExpression
            | AstNodeKind::MemberExpression
    )
}

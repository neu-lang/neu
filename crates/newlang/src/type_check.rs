use crate::{
    ast::{AstArena, AstNodeId, AstNodeKind},
    name_resolution::{LocalBinding, ResolutionTable},
    parser::{
        ParsedAssignmentStatement, ParsedGroupedExpression, ParsedLiteralExpression,
        ParsedLiteralKind, ParsedLocalDeclaration, ParsedTypeNameReference,
    },
    symbol::SymbolId,
    types::{PrimitiveType, TypeArena, TypeId, TypeKind, TypeRecord},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AmbiguousTypeRule {
    LiteralTyping,
    PrimitiveScalarCatalog,
    AssignmentCompatibility,
    CallResolution,
    FunctionTypeApplication,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeCheckDiagnosticKind {
    AmbiguousTypeRule,
    UnresolvedTypeRule,
    UnsupportedTypeRule,
    TypeMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeRuleDiagnostic {
    Ambiguous(AmbiguousTypeRule),
    MissingAnnotationType,
    MissingResolvedNameType,
    DirectCallDeferred,
    FunctionTypeApplicationDeferred,
    MemberExpressionDeferred,
    BinaryExpressionDeferred,
    UnaryExpressionDeferred,
    IfValueDeferred,
}

impl PartialEq<AmbiguousTypeRule> for TypeRuleDiagnostic {
    fn eq(&self, other: &AmbiguousTypeRule) -> bool {
        matches!(self, Self::Ambiguous(rule) if rule == other)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeCheckDiagnostic {
    kind: TypeCheckDiagnosticKind,
    rule: TypeRuleDiagnostic,
    node: AstNodeId,
    expected_type: Option<TypeId>,
    actual_type: Option<TypeId>,
}

impl TypeCheckDiagnostic {
    pub fn ambiguous_type_rule(rule: AmbiguousTypeRule, node: AstNodeId) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::AmbiguousTypeRule,
            rule: TypeRuleDiagnostic::Ambiguous(rule),
            node,
            expected_type: None,
            actual_type: None,
        }
    }

    pub fn unresolved_type_rule(rule: TypeRuleDiagnostic, node: AstNodeId) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::UnresolvedTypeRule,
            rule,
            node,
            expected_type: None,
            actual_type: None,
        }
    }

    pub fn unsupported_type_rule(rule: TypeRuleDiagnostic, node: AstNodeId) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::UnsupportedTypeRule,
            rule,
            node,
            expected_type: None,
            actual_type: None,
        }
    }

    pub fn type_mismatch(node: AstNodeId, expected_type: TypeId, actual_type: TypeId) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::TypeMismatch,
            rule: TypeRuleDiagnostic::Ambiguous(AmbiguousTypeRule::AssignmentCompatibility),
            node,
            expected_type: Some(expected_type),
            actual_type: Some(actual_type),
        }
    }

    pub fn kind(&self) -> TypeCheckDiagnosticKind {
        self.kind
    }

    pub fn rule(&self) -> TypeRuleDiagnostic {
        self.rule
    }

    pub fn node(&self) -> AstNodeId {
        self.node
    }

    pub fn expected_type(&self) -> Option<TypeId> {
        self.expected_type
    }

    pub fn actual_type(&self) -> Option<TypeId> {
        self.actual_type
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExpressionType {
    expression: AstNodeId,
    ty: TypeId,
}

impl ExpressionType {
    pub fn new(expression: AstNodeId, ty: TypeId) -> Self {
        Self { expression, ty }
    }

    pub fn expression(self) -> AstNodeId {
        self.expression
    }

    pub fn ty(self) -> TypeId {
        self.ty
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSignature {
    declaration: AstNodeId,
    ty: TypeId,
}

impl DeclarationSignature {
    pub fn new(declaration: AstNodeId, ty: TypeId) -> Self {
        Self { declaration, ty }
    }

    pub fn declaration(self) -> AstNodeId {
        self.declaration
    }

    pub fn ty(self) -> TypeId {
        self.ty
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssignmentCheck {
    statement: AstNodeId,
    target: TypeId,
    value: TypeId,
}

impl AssignmentCheck {
    pub fn new(statement: AstNodeId, target: TypeId, value: TypeId) -> Self {
        Self {
            statement,
            target,
            value,
        }
    }

    pub fn statement(self) -> AstNodeId {
        self.statement
    }

    pub fn target(self) -> TypeId {
        self.target
    }

    pub fn value(self) -> TypeId {
        self.value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiteralKind {
    BoolTrue,
    BoolFalse,
    AcceptedInteger,
    AcceptedString,
    Null,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LiteralExpressionInput {
    expression: AstNodeId,
    kind: LiteralKind,
}

impl LiteralExpressionInput {
    pub fn new(expression: AstNodeId, kind: LiteralKind) -> Self {
        Self { expression, kind }
    }

    pub fn expression(self) -> AstNodeId {
        self.expression
    }

    pub fn kind(self) -> LiteralKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KnownSymbolType {
    symbol: SymbolId,
    ty: TypeId,
}

impl KnownSymbolType {
    pub fn new(symbol: SymbolId, ty: TypeId) -> Self {
        Self { symbol, ty }
    }

    pub fn symbol(self) -> SymbolId {
        self.symbol
    }

    pub fn ty(self) -> TypeId {
        self.ty
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeCheckReport {
    expression_types: Vec<ExpressionType>,
    declaration_signatures: Vec<DeclarationSignature>,
    assignment_checks: Vec<AssignmentCheck>,
    diagnostics: Vec<TypeCheckDiagnostic>,
}

impl TypeCheckReport {
    pub fn new() -> Self {
        Self {
            expression_types: Vec::new(),
            declaration_signatures: Vec::new(),
            assignment_checks: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn blocked(diagnostics: Vec<TypeCheckDiagnostic>) -> Self {
        Self {
            diagnostics,
            ..Self::new()
        }
    }

    pub fn is_blocked(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    pub fn record_expression_type(&mut self, expression_type: ExpressionType) {
        self.expression_types.push(expression_type);
    }

    pub fn expression_types(&self) -> &[ExpressionType] {
        &self.expression_types
    }

    pub fn expression_type(&self, expression: AstNodeId) -> Option<TypeId> {
        self.expression_types
            .iter()
            .find(|entry| entry.expression() == expression)
            .map(|entry| entry.ty())
    }

    pub fn record_declaration_signature(&mut self, signature: DeclarationSignature) {
        self.declaration_signatures.push(signature);
    }

    pub fn declaration_signatures(&self) -> &[DeclarationSignature] {
        &self.declaration_signatures
    }

    pub fn declaration_signature(&self, declaration: AstNodeId) -> Option<TypeId> {
        self.declaration_signatures
            .iter()
            .find(|entry| entry.declaration() == declaration)
            .map(|entry| entry.ty())
    }

    pub fn record_assignment_check(&mut self, assignment_check: AssignmentCheck) {
        self.assignment_checks.push(assignment_check);
    }

    pub fn record_diagnostic(&mut self, diagnostic: TypeCheckDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn assignment_checks(&self) -> &[AssignmentCheck] {
        &self.assignment_checks
    }

    pub fn assignment_check(&self, statement: AstNodeId) -> Option<AssignmentCheck> {
        self.assignment_checks
            .iter()
            .find(|entry| entry.statement() == statement)
            .copied()
    }

    pub fn diagnostics(&self) -> &[TypeCheckDiagnostic] {
        &self.diagnostics
    }
}

impl Default for TypeCheckReport {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PrimitiveTypeIds {
    bool_id: TypeId,
    int_id: TypeId,
    string_id: TypeId,
    unit_id: TypeId,
    null_id: TypeId,
}

impl PrimitiveTypeIds {
    fn insert_into(arena: &mut TypeArena) -> Self {
        Self {
            bool_id: arena.insert(TypeRecord::primitive(PrimitiveType::Bool)),
            int_id: arena.insert(TypeRecord::primitive(PrimitiveType::Int)),
            string_id: arena.insert(TypeRecord::primitive(PrimitiveType::String)),
            unit_id: arena.insert(TypeRecord::primitive(PrimitiveType::Unit)),
            null_id: arena.insert(TypeRecord::primitive(PrimitiveType::Null)),
        }
    }

    fn type_for_literal(self, kind: LiteralKind) -> TypeId {
        match kind {
            LiteralKind::BoolTrue | LiteralKind::BoolFalse => self.bool_id,
            LiteralKind::AcceptedInteger => self.int_id,
            LiteralKind::AcceptedString => self.string_id,
            LiteralKind::Null => self.null_id,
        }
    }

    fn type_for_primitive_name(self, name: &str) -> Option<TypeId> {
        match name {
            "Bool" => Some(self.bool_id),
            "Int" => Some(self.int_id),
            "String" => Some(self.string_id),
            "Unit" => Some(self.unit_id),
            "Null" => Some(self.null_id),
            _ => None,
        }
    }
}

pub fn type_literal_expressions(inputs: &[LiteralExpressionInput]) -> (TypeArena, TypeCheckReport) {
    let mut arena = TypeArena::new();
    let primitives = PrimitiveTypeIds::insert_into(&mut arena);
    let mut report = TypeCheckReport::new();

    for input in inputs {
        report.record_expression_type(ExpressionType::new(
            input.expression(),
            primitives.type_for_literal(input.kind()),
        ));
    }

    (arena, report)
}

pub fn type_parser_literals(literals: &[ParsedLiteralExpression]) -> (TypeArena, TypeCheckReport) {
    let inputs: Vec<_> = literals
        .iter()
        .map(|literal| {
            LiteralExpressionInput::new(literal.expression, literal_kind_from_parser(literal.kind))
        })
        .collect();
    type_literal_expressions(&inputs)
}

fn literal_kind_from_parser(kind: ParsedLiteralKind) -> LiteralKind {
    match kind {
        ParsedLiteralKind::BoolTrue => LiteralKind::BoolTrue,
        ParsedLiteralKind::BoolFalse => LiteralKind::BoolFalse,
        ParsedLiteralKind::AcceptedInteger => LiteralKind::AcceptedInteger,
        ParsedLiteralKind::AcceptedString => LiteralKind::AcceptedString,
        ParsedLiteralKind::Null => LiteralKind::Null,
    }
}

pub fn type_primitive_local_declarations(
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
) -> (TypeArena, TypeCheckReport) {
    let mut arena = TypeArena::new();
    let primitives = PrimitiveTypeIds::insert_into(&mut arena);
    let mut report = TypeCheckReport::new();

    for declaration in declarations {
        let Some(annotation_type) =
            primitive_annotation_type(declaration, type_name_references, primitives)
        else {
            report.record_diagnostic(TypeCheckDiagnostic::unresolved_type_rule(
                TypeRuleDiagnostic::MissingAnnotationType,
                declaration.declaration,
            ));
            continue;
        };
        report.record_declaration_signature(DeclarationSignature::new(
            declaration.declaration,
            annotation_type,
        ));
    }

    (arena, report)
}

pub fn type_primitive_local_initializer_declarations(
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
) -> (TypeArena, TypeCheckReport) {
    let mut arena = TypeArena::new();
    let primitives = PrimitiveTypeIds::insert_into(&mut arena);
    let mut report = TypeCheckReport::new();

    for literal in literals {
        let kind = literal_kind_from_parser(literal.kind);
        report.record_expression_type(ExpressionType::new(
            literal.expression,
            primitives.type_for_literal(kind),
        ));
    }

    for declaration in declarations {
        let Some(annotation_type) =
            primitive_annotation_type(declaration, type_name_references, primitives)
        else {
            report.record_diagnostic(TypeCheckDiagnostic::unresolved_type_rule(
                TypeRuleDiagnostic::MissingAnnotationType,
                declaration.declaration,
            ));
            continue;
        };
        report.record_declaration_signature(DeclarationSignature::new(
            declaration.declaration,
            annotation_type,
        ));

        let Some(initializer) = declaration.initializer else {
            continue;
        };
        let Some(initializer_type) = report.expression_type(initializer) else {
            continue;
        };

        if initializer_type == annotation_type {
            report.record_assignment_check(AssignmentCheck::new(
                declaration.declaration,
                annotation_type,
                initializer_type,
            ));
        } else {
            report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                initializer,
                annotation_type,
                initializer_type,
            ));
        }
    }

    (arena, report)
}

fn primitive_annotation_type(
    declaration: &ParsedLocalDeclaration,
    type_name_references: &[ParsedTypeNameReference],
    primitives: PrimitiveTypeIds,
) -> Option<TypeId> {
    let annotation = declaration.annotation?;
    let type_name = type_name_references
        .iter()
        .find(|reference| reference.reference == annotation)?;
    primitives.type_for_primitive_name(type_name.name.as_str())
}

pub fn type_resolved_name_expressions(
    resolutions: &ResolutionTable,
    known_symbols: &[KnownSymbolType],
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for resolved in resolutions.resolved_names() {
        let Some(known) = known_symbols
            .iter()
            .find(|known| known.symbol() == resolved.symbol())
        else {
            report.record_diagnostic(TypeCheckDiagnostic::unresolved_type_rule(
                TypeRuleDiagnostic::MissingResolvedNameType,
                resolved.reference(),
            ));
            continue;
        };
        report.record_expression_type(ExpressionType::new(resolved.reference(), known.ty()));
    }

    report
}

pub fn known_local_symbol_types(
    bindings: &[LocalBinding],
    signatures: &[DeclarationSignature],
) -> Vec<KnownSymbolType> {
    let mut known = Vec::new();

    for binding in bindings {
        let Some(signature) = signatures
            .iter()
            .find(|signature| signature.declaration() == binding.binding())
        else {
            continue;
        };
        known.push(KnownSymbolType::new(binding.key().name(), signature.ty()));
    }

    known
}

pub fn type_grouped_expressions(
    grouped_expressions: &[ParsedGroupedExpression],
    known_expression_types: &[ExpressionType],
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for grouped in grouped_expressions {
        let Some(inner) = known_expression_types
            .iter()
            .find(|entry| entry.expression() == grouped.inner)
        else {
            continue;
        };
        report.record_expression_type(ExpressionType::new(grouped.expression, inner.ty()));
    }

    report
}

pub fn type_assignment_statements(
    assignments: &[ParsedAssignmentStatement],
    known_expression_types: &[ExpressionType],
    arena: &TypeArena,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for assignment in assignments {
        let Some(target_type) = expression_type_in(known_expression_types, assignment.target)
        else {
            continue;
        };
        let Some(value_type) = expression_type_in(known_expression_types, assignment.value) else {
            continue;
        };

        if assignment_compatible(target_type, value_type, arena) {
            report.record_assignment_check(AssignmentCheck::new(
                assignment.statement,
                target_type,
                value_type,
            ));
        } else {
            report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                assignment.value,
                target_type,
                value_type,
            ));
        }
    }

    report
}

fn expression_type_in(
    expression_types: &[ExpressionType],
    expression: AstNodeId,
) -> Option<TypeId> {
    expression_types
        .iter()
        .find(|entry| entry.expression() == expression)
        .map(|entry| entry.ty())
}

fn assignment_compatible(target: TypeId, value: TypeId, arena: &TypeArena) -> bool {
    if target == value {
        return true;
    }

    let Some(target_record) = arena.get(target) else {
        return false;
    };
    let Some(value_record) = arena.get(value) else {
        return false;
    };

    match (target_record.kind(), value_record.kind()) {
        (TypeKind::Nullable(_), TypeKind::Primitive(PrimitiveType::Null)) => true,
        (TypeKind::Nullable(nullable), _) => nullable.base() == value,
        _ => false,
    }
}

pub fn type_unsupported_m0018_expressions(arena: &AstArena) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for node in arena.nodes() {
        let Some(rule) = unsupported_expression_rule(node.kind) else {
            continue;
        };
        report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(rule, node.id));
    }

    report
}

fn unsupported_expression_rule(kind: AstNodeKind) -> Option<TypeRuleDiagnostic> {
    match kind {
        AstNodeKind::CallExpression => Some(TypeRuleDiagnostic::DirectCallDeferred),
        AstNodeKind::MemberExpression => Some(TypeRuleDiagnostic::MemberExpressionDeferred),
        AstNodeKind::BinaryExpression => Some(TypeRuleDiagnostic::BinaryExpressionDeferred),
        AstNodeKind::UnaryExpression => Some(TypeRuleDiagnostic::UnaryExpressionDeferred),
        AstNodeKind::IfExpression => Some(TypeRuleDiagnostic::IfValueDeferred),
        _ => None,
    }
}

pub fn type_m0018_accepted_expressions(
    literals: &[ParsedLiteralExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    resolutions: &ResolutionTable,
    known_symbols: &[KnownSymbolType],
) -> (TypeArena, TypeCheckReport) {
    let (arena, _primitives, report) = type_m0018_accepted_expressions_with_primitives(
        literals,
        grouped_expressions,
        resolutions,
        known_symbols,
    );
    (arena, report)
}

fn type_m0018_accepted_expressions_with_primitives(
    literals: &[ParsedLiteralExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    resolutions: &ResolutionTable,
    known_symbols: &[KnownSymbolType],
) -> (TypeArena, PrimitiveTypeIds, TypeCheckReport) {
    let mut arena = TypeArena::new();
    let primitives = PrimitiveTypeIds::insert_into(&mut arena);
    let mut report = TypeCheckReport::new();

    for literal in literals {
        report.record_expression_type(ExpressionType::new(
            literal.expression,
            primitives.type_for_literal(literal_kind_from_parser(literal.kind)),
        ));
    }

    for resolved in resolutions.resolved_names() {
        let Some(known) = known_symbols
            .iter()
            .find(|known| known.symbol() == resolved.symbol())
        else {
            report.record_diagnostic(TypeCheckDiagnostic::unresolved_type_rule(
                TypeRuleDiagnostic::MissingResolvedNameType,
                resolved.reference(),
            ));
            continue;
        };
        report.record_expression_type(ExpressionType::new(resolved.reference(), known.ty()));
    }

    loop {
        let mut added_group = false;
        for grouped in grouped_expressions {
            if report.expression_type(grouped.expression).is_some() {
                continue;
            }
            let Some(inner) = report.expression_type(grouped.inner) else {
                continue;
            };
            report.record_expression_type(ExpressionType::new(grouped.expression, inner));
            added_group = true;
        }
        if !added_group {
            break;
        }
    }

    (arena, primitives, report)
}

pub fn type_m0018_local_declaration_initializers(
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    resolutions: &ResolutionTable,
    known_symbols: &[KnownSymbolType],
) -> (TypeArena, TypeCheckReport) {
    let (arena, primitives, expression_report) = type_m0018_accepted_expressions_with_primitives(
        literals,
        grouped_expressions,
        resolutions,
        known_symbols,
    );
    let mut report = TypeCheckReport::new();

    for expression_type in expression_report.expression_types() {
        report.record_expression_type(*expression_type);
    }

    for declaration in declarations {
        let Some(annotation_type) =
            primitive_annotation_type(declaration, type_name_references, primitives)
        else {
            report.record_diagnostic(TypeCheckDiagnostic::unresolved_type_rule(
                TypeRuleDiagnostic::MissingAnnotationType,
                declaration.declaration,
            ));
            continue;
        };
        report.record_declaration_signature(DeclarationSignature::new(
            declaration.declaration,
            annotation_type,
        ));

        let Some(initializer) = declaration.initializer else {
            continue;
        };
        let Some(initializer_type) = report.expression_type(initializer) else {
            continue;
        };

        if assignment_compatible(annotation_type, initializer_type, &arena) {
            report.record_assignment_check(AssignmentCheck::new(
                declaration.declaration,
                annotation_type,
                initializer_type,
            ));
        } else {
            report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                initializer,
                annotation_type,
                initializer_type,
            ));
        }
    }

    (arena, report)
}

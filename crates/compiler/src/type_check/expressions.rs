//! Literals, names, declarations, and the core expression typing entry points.

use super::*;

pub(crate) fn primitive_annotation_type_for_node(
    annotation: AstNodeId,
    references: &[ParsedTypeNameReference],
    primitives: PrimitiveTypeIds,
) -> Option<TypeId> {
    let reference = references
        .iter()
        .find(|reference| reference.reference == annotation)?;
    primitives.type_for_primitive_name(&reference.name)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExpressionType {
    pub(crate) expression: AstNodeId,
    pub(crate) ty: TypeId,
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
    pub(crate) declaration: AstNodeId,
    pub(crate) ty: TypeId,
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
    pub(crate) statement: AstNodeId,
    pub(crate) target: TypeId,
    pub(crate) value: TypeId,
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
    Float,
    Null,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LiteralExpressionInput {
    pub(crate) expression: AstNodeId,
    pub(crate) kind: LiteralKind,
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
    pub(crate) symbol: SymbolId,
    pub(crate) ty: TypeId,
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
    pub(crate) expression_types: Vec<ExpressionType>,
    pub(crate) declaration_signatures: Vec<DeclarationSignature>,
    pub(crate) assignment_checks: Vec<AssignmentCheck>,
    pub(crate) refinements: Vec<RefinementRecord>,
    pub(crate) refined_expression_types: Vec<RefinedExpressionType>,
    pub(crate) diagnostics: Vec<TypeCheckDiagnostic>,
    pub(crate) compile_time_constants: Vec<CompileTimeConstant>,
}

impl TypeCheckReport {
    pub fn new() -> Self {
        Self {
            expression_types: Vec::new(),
            declaration_signatures: Vec::new(),
            assignment_checks: Vec::new(),
            refinements: Vec::new(),
            refined_expression_types: Vec::new(),
            diagnostics: Vec::new(),
            compile_time_constants: Vec::new(),
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

    pub fn replace_expression_type(&mut self, expression_type: ExpressionType) {
        self.expression_types
            .retain(|entry| entry.expression() != expression_type.expression());
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

    pub fn replace_declaration_signature(&mut self, signature: DeclarationSignature) {
        self.declaration_signatures
            .retain(|entry| entry.declaration() != signature.declaration());
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

    pub fn record_refinement(&mut self, refinement: RefinementRecord) {
        self.refinements.push(refinement);
    }

    pub fn refinements(&self) -> &[RefinementRecord] {
        &self.refinements
    }

    pub fn refinement(&self, region: AstNodeId) -> Option<&RefinementRecord> {
        self.refinements
            .iter()
            .find(|entry| entry.region() == region)
    }

    pub fn record_refined_expression_type(&mut self, refined_type: RefinedExpressionType) {
        self.refined_expression_types.push(refined_type);
    }

    pub fn refined_expression_types(&self) -> &[RefinedExpressionType] {
        &self.refined_expression_types
    }

    pub fn refined_expression_type(&self, expression: AstNodeId) -> Option<TypeId> {
        self.refined_expression_types
            .iter()
            .find(|entry| entry.expression() == expression)
            .map(|entry| entry.refined_non_null_type())
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
            .cloned()
    }

    pub fn diagnostics(&self) -> &[TypeCheckDiagnostic] {
        &self.diagnostics
    }

    pub fn retain_diagnostics(&mut self, keep: impl Fn(&TypeCheckDiagnostic) -> bool) {
        self.diagnostics.retain(keep);
    }

    pub fn record_compile_time_constant(&mut self, constant: CompileTimeConstant) {
        self.compile_time_constants.push(constant);
    }

    pub fn compile_time_constants(&self) -> &[CompileTimeConstant] {
        &self.compile_time_constants
    }
}

impl Default for TypeCheckReport {
    fn default() -> Self {
        Self::new()
    }
}

pub fn type_function_types(
    parsed: &ParseOutput,
    types: &mut TypeArena,
) -> Vec<(AstNodeId, TypeId)> {
    let primitives = PrimitiveTypeIds::module_owned(types);
    let mut resolved = Vec::new();
    for function in &parsed.function_types {
        let Some(parameters) = function
            .parameters
            .iter()
            .map(|parameter| {
                resolve_annotation_type(
                    *parameter,
                    &parsed.type_name_references,
                    &parsed.array_types,
                    &primitives,
                    &[],
                    types,
                )
            })
            .collect::<Option<Vec<_>>>()
        else {
            continue;
        };
        let return_type = function
            .return_type
            .map(|return_type| {
                resolve_annotation_type(
                    return_type,
                    &parsed.type_name_references,
                    &parsed.array_types,
                    &primitives,
                    &[],
                    types,
                )
            })
            .unwrap_or(Some(primitives.void_id));
        let Some(return_type) = return_type else {
            continue;
        };
        resolved.push((
            function.function_type,
            types.function(FunctionType::new(parameters, return_type)),
        ));
    }
    resolved
}

pub fn type_annotation_type(
    parsed: &ParseOutput,
    annotation: AstNodeId,
    types: &mut TypeArena,
    classes: &[ClassTypeRecord],
) -> Option<TypeId> {
    if let Some(function_type) = parsed
        .function_types
        .iter()
        .find(|function_type| function_type.function_type == annotation)
    {
        let parameters = function_type
            .parameters
            .iter()
            .map(|parameter| {
                resolve_annotation_type(
                    *parameter,
                    &parsed.type_name_references,
                    &parsed.array_types,
                    &PrimitiveTypeIds::module_owned(types),
                    classes,
                    types,
                )
            })
            .collect::<Option<Vec<_>>>()?;
        let return_type = function_type
            .return_type
            .map(|return_type| {
                resolve_annotation_type(
                    return_type,
                    &parsed.type_name_references,
                    &parsed.array_types,
                    &PrimitiveTypeIds::module_owned(types),
                    classes,
                    types,
                )
            })
            .unwrap_or_else(|| Some(PrimitiveTypeIds::module_owned(types).void_id))?;
        return Some(types.function(FunctionType::new(parameters, return_type)));
    }
    // Generic class annotations such as `Box<Int>` keep their type arguments.
    crate::type_check::resolve_annotation_type_with_generics(
        annotation,
        &parsed.type_name_references,
        &parsed.array_types,
        &PrimitiveTypeIds::module_owned(types),
        classes,
        types,
        &[],
    )
}

pub(crate) fn resolve_annotation_type(
    annotation: AstNodeId,
    type_name_references: &[ParsedTypeNameReference],
    array_types: &[ParsedArrayType],
    primitives: &PrimitiveTypeIds,
    classes: &[ClassTypeRecord],
    arena: &mut TypeArena,
) -> Option<TypeId> {
    if let Some(ty) = super::generics::resolve_type_reference(
        type_name_references,
        array_types,
        annotation,
        primitives,
        classes,
        &[],
        arena,
    ) {
        return Some(ty);
    }
    if let Some(reference) = type_name_references
        .iter()
        .find(|reference| reference.reference == annotation)
    {
        if reference.name == "Channel" && reference.generic_argument_names.len() == 1 {
            let element_name = &reference.generic_argument_names[0];
            let element = primitives
                .type_for_primitive_name(element_name)
                .or_else(|| {
                    classes
                        .iter()
                        .find(|class| class.name == *element_name)
                        .map(|class| class.type_id)
                })?;
            return Some(arena.channel(element));
        }
        if reference.name == "Task" && reference.generic_argument_names.len() == 1 {
            let result_name = &reference.generic_argument_names[0];
            let result = primitives
                .type_for_primitive_name(result_name)
                .or_else(|| {
                    classes
                        .iter()
                        .find(|class| class.name == *result_name)
                        .map(|class| class.type_id)
                })?;
            return Some(arena.task(result));
        }
        if !reference.generic_arguments.is_empty() {
            let class = classes.iter().find(|class| class.name == reference.name)?;
            let arguments = reference
                .generic_argument_names
                .iter()
                .map(|name| {
                    primitives.type_for_primitive_name(name).or_else(|| {
                        classes
                            .iter()
                            .find(|class| class.name == *name)
                            .map(|class| class.type_id)
                    })
                })
                .collect::<Option<Vec<_>>>()?;
            let identity = match arena.get(class.type_id)?.kind() {
                TypeKind::Nominal(identity) => identity.clone(),
                _ => return None,
            };
            return Some(arena.generic_instance(GenericTypeIdentity::new(identity, arguments)));
        }
        return primitives
            .type_for_primitive_name(&reference.name)
            .or_else(|| {
                classes
                    .iter()
                    .find(|class| class.name == reference.name)
                    .map(|class| class.type_id)
            });
    }
    let array = array_types.iter().find(|array| array.array == annotation)?;
    let element = resolve_annotation_type(
        array.element_type,
        type_name_references,
        array_types,
        primitives,
        classes,
        arena,
    )?;
    if array.dynamic {
        return Some(arena.dynamic_array(element));
    }
    Some(arena.array(element, array.length?))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveTypeIds {
    pub(crate) bool_id: TypeId,
    pub(crate) int_id: TypeId,
    pub(crate) string_id: TypeId,
    pub(crate) void_id: TypeId,
    pub(crate) null_id: TypeId,
    pub(crate) float_id: TypeId,
    pub(crate) byte_id: TypeId,
}

impl PrimitiveTypeIds {
    pub(crate) fn insert_into(arena: &mut TypeArena) -> Self {
        Self {
            bool_id: arena.insert(TypeRecord::primitive(PrimitiveType::Bool)),
            int_id: arena.insert(TypeRecord::primitive(PrimitiveType::Int)),
            string_id: arena.insert(TypeRecord::primitive(PrimitiveType::String)),
            void_id: arena.insert(TypeRecord::primitive(PrimitiveType::Void)),
            null_id: arena.insert(TypeRecord::primitive(PrimitiveType::Null)),
            float_id: arena.insert(TypeRecord::primitive(PrimitiveType::Float)),
            byte_id: arena.insert(TypeRecord::primitive(PrimitiveType::Byte)),
        }
    }

    pub(crate) fn module_owned(arena: &mut TypeArena) -> Self {
        if arena.records().is_empty() {
            return Self::insert_into(arena);
        }

        let primitive = |expected| {
            arena
                .records()
                .iter()
                .position(|record| record.kind() == &TypeKind::Primitive(expected))
                .map(TypeId::from_raw)
                .expect("module TypeArena contains every bootstrap primitive")
        };
        Self {
            bool_id: primitive(PrimitiveType::Bool),
            int_id: primitive(PrimitiveType::Int),
            string_id: primitive(PrimitiveType::String),
            void_id: primitive(PrimitiveType::Void),
            null_id: primitive(PrimitiveType::Null),
            float_id: primitive(PrimitiveType::Float),
            byte_id: primitive(PrimitiveType::Byte),
        }
    }

    pub(crate) fn type_for_literal(self, kind: LiteralKind) -> TypeId {
        match kind {
            LiteralKind::BoolTrue | LiteralKind::BoolFalse => self.bool_id,
            LiteralKind::AcceptedInteger => self.int_id,
            LiteralKind::AcceptedString => self.string_id,
            LiteralKind::Float => self.float_id,
            LiteralKind::Null => self.null_id,
        }
    }

    pub(crate) fn type_for_primitive_name(self, name: &str) -> Option<TypeId> {
        match name {
            "Bool" => Some(self.bool_id),
            "Int" => Some(self.int_id),
            "String" => Some(self.string_id),
            "Null" => Some(self.null_id),
            "Float" => Some(self.float_id),
            "Byte" => Some(self.byte_id),
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

pub(crate) fn literal_kind_from_parser(kind: ParsedLiteralKind) -> LiteralKind {
    match kind {
        ParsedLiteralKind::BoolTrue => LiteralKind::BoolTrue,
        ParsedLiteralKind::BoolFalse => LiteralKind::BoolFalse,
        ParsedLiteralKind::AcceptedInteger => LiteralKind::AcceptedInteger,
        ParsedLiteralKind::AcceptedString => LiteralKind::AcceptedString,
        ParsedLiteralKind::Float => LiteralKind::Float,
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
    integer_literals: &[ParsedIntegerLiteral],
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

        if annotation_type == primitives.byte_id
            && let Some(integer) = integer_literals
                .iter()
                .find(|literal| literal.expression == initializer)
        {
            match integer.value {
                Some(value) if value <= u64::from(u8::MAX) => {
                    report
                        .replace_expression_type(ExpressionType::new(initializer, annotation_type));
                    report.record_assignment_check(AssignmentCheck::new(
                        declaration.declaration,
                        annotation_type,
                        annotation_type,
                    ));
                    continue;
                }
                Some(_) | None => {
                    report.record_diagnostic(TypeCheckDiagnostic::static_integer(
                        TypeRuleDiagnostic::ByteLiteralOutOfRange,
                        initializer,
                    ));
                    continue;
                }
            }
        }

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

pub(crate) fn primitive_annotation_type(
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

#[allow(clippy::too_many_arguments)]
pub fn type_core(
    arena: &AstArena,
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    assignments: &[ParsedAssignmentStatement],
    resolutions: &ResolutionTable,
    local_bindings: &[LocalBinding],
) -> (TypeArena, TypeCheckReport) {
    type_core_with_options(
        arena,
        declarations,
        type_name_references,
        literals,
        None,
        grouped_expressions,
        assignments,
        resolutions,
        local_bindings,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn type_executable_core(
    arena: &AstArena,
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    integer_literals: &[ParsedIntegerLiteral],
    grouped_expressions: &[ParsedGroupedExpression],
    unary_expressions: &[ParsedUnaryExpression],
    binary_expressions: &[ParsedBinaryExpression],
    assignments: &[ParsedAssignmentStatement],
    resolutions: &ResolutionTable,
    local_bindings: &[LocalBinding],
) -> (TypeArena, TypeCheckReport) {
    let mut type_arena = TypeArena::new();
    let report = type_executable_core_in(
        &mut type_arena,
        arena,
        declarations,
        type_name_references,
        literals,
        integer_literals,
        grouped_expressions,
        unary_expressions,
        binary_expressions,
        assignments,
        resolutions,
        local_bindings,
    );
    (type_arena, report)
}

#[allow(clippy::too_many_arguments)]
pub fn type_executable_core_in(
    type_arena: &mut TypeArena,
    arena: &AstArena,
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    integer_literals: &[ParsedIntegerLiteral],
    grouped_expressions: &[ParsedGroupedExpression],
    unary_expressions: &[ParsedUnaryExpression],
    binary_expressions: &[ParsedBinaryExpression],
    assignments: &[ParsedAssignmentStatement],
    resolutions: &ResolutionTable,
    local_bindings: &[LocalBinding],
) -> TypeCheckReport {
    let mut report = type_core_with_arena(
        type_arena,
        arena,
        declarations,
        type_name_references,
        literals,
        Some(integer_literals),
        grouped_expressions,
        assignments,
        resolutions,
        local_bindings,
        Some((unary_expressions, binary_expressions)),
    );
    for diagnostic in type_static_integer_diagnostics(
        integer_literals,
        grouped_expressions,
        unary_expressions,
        binary_expressions,
    ) {
        report.record_diagnostic(diagnostic);
    }
    report
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn type_core_with_options(
    arena: &AstArena,
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    integer_literals: Option<&[ParsedIntegerLiteral]>,
    grouped_expressions: &[ParsedGroupedExpression],
    assignments: &[ParsedAssignmentStatement],
    resolutions: &ResolutionTable,
    local_bindings: &[LocalBinding],
    executable_operators: Option<(&[ParsedUnaryExpression], &[ParsedBinaryExpression])>,
) -> (TypeArena, TypeCheckReport) {
    let mut type_arena = TypeArena::new();
    let report = type_core_with_arena(
        &mut type_arena,
        arena,
        declarations,
        type_name_references,
        literals,
        integer_literals,
        grouped_expressions,
        assignments,
        resolutions,
        local_bindings,
        executable_operators,
    );
    (type_arena, report)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn type_core_with_arena(
    type_arena: &mut TypeArena,
    arena: &AstArena,
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    integer_literals: Option<&[ParsedIntegerLiteral]>,
    grouped_expressions: &[ParsedGroupedExpression],
    assignments: &[ParsedAssignmentStatement],
    resolutions: &ResolutionTable,
    local_bindings: &[LocalBinding],
    executable_operators: Option<(&[ParsedUnaryExpression], &[ParsedBinaryExpression])>,
) -> TypeCheckReport {
    let primitives = PrimitiveTypeIds::module_owned(type_arena);
    let mut report = TypeCheckReport::new();

    for declaration in declarations {
        if declaration.annotation.is_none() {
            continue;
        }
        if declaration.annotation.is_some_and(|annotation| {
            arena
                .node(annotation)
                .is_some_and(|node| node.kind == AstNodeKind::FunctionType)
        }) {
            continue;
        }
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

    let known_symbols = known_local_symbol_types(local_bindings, report.declaration_signatures());
    record_accepted_expression_types(
        literals,
        grouped_expressions,
        resolutions,
        &known_symbols,
        primitives,
        &mut report,
    );

    // A Byte annotation gives a directly written integer literal its exact
    // contextual type before operator propagation and assignment checking.
    for declaration in declarations {
        if report.declaration_signature(declaration.declaration) != Some(primitives.byte_id) {
            continue;
        }
        let Some(initializer) = declaration.initializer else {
            continue;
        };
        let Some(integer) = integer_literals.and_then(|literals| {
            literals
                .iter()
                .find(|literal| literal.expression == initializer)
        }) else {
            continue;
        };
        if integer
            .value
            .is_some_and(|value| value <= u64::from(u8::MAX))
        {
            report.replace_expression_type(ExpressionType::new(initializer, primitives.byte_id));
        }
    }

    if let Some((unary_expressions, binary_expressions)) = executable_operators {
        let int_unary_expressions: Vec<_> = unary_expressions
            .iter()
            .filter(|expression| {
                !matches!(
                    report.expression_type(expression.operand),
                    Some(ty) if ty == primitives.float_id || ty == primitives.byte_id
                )
            })
            .cloned()
            .collect();
        let int_binary_expressions: Vec<_> = binary_expressions
            .iter()
            .filter(|expression| {
                !matches!(
                    (
                        report.expression_type(expression.left),
                        report.expression_type(expression.right),
                    ),
                    (Some(left), Some(right))
                        if left == primitives.float_id
                            || left == primitives.byte_id
                            || right == primitives.float_id
                            || right == primitives.byte_id
                )
            })
            .cloned()
            .collect();
        let operator_report = type_executable_int_operators(
            &int_unary_expressions,
            &int_binary_expressions,
            grouped_expressions,
            report.expression_types(),
            primitives.int_id,
        );
        merge_type_check_report(&mut report, operator_report);
        let primitive_report = type_primitive_operators(
            unary_expressions,
            binary_expressions,
            report.expression_types(),
            primitives.bool_id,
            primitives.float_id,
            primitives.byte_id,
            primitives.void_id,
        );
        merge_type_check_report(&mut report, primitive_report);
    }

    for declaration in declarations {
        let Some(annotation_type) = report.declaration_signature(declaration.declaration) else {
            continue;
        };
        let Some(initializer) = declaration.initializer else {
            continue;
        };
        let Some(initializer_type) = report.expression_type(initializer) else {
            continue;
        };

        if annotation_type == primitives.byte_id
            && let Some(integer_literals) = integer_literals
            && let Some(integer) = integer_literals
                .iter()
                .find(|literal| literal.expression == initializer)
        {
            match integer.value {
                Some(value) if value <= u64::from(u8::MAX) => {
                    report.record_assignment_check(AssignmentCheck::new(
                        declaration.declaration,
                        annotation_type,
                        annotation_type,
                    ));
                }
                Some(_) | None => report.record_diagnostic(TypeCheckDiagnostic::static_integer(
                    TypeRuleDiagnostic::ByteLiteralOutOfRange,
                    initializer,
                )),
            }
            continue;
        }

        if assignment_compatible(annotation_type, initializer_type, type_arena) {
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

    let assignment_report =
        type_assignment_statements(assignments, report.expression_types(), type_arena);
    merge_type_check_report(&mut report, assignment_report);

    let unsupported_report =
        type_unsupported_expressions_with_operators(arena, executable_operators);
    merge_type_check_report(&mut report, unsupported_report);

    report
}

pub(crate) fn merge_type_check_report(target: &mut TypeCheckReport, source: TypeCheckReport) {
    for expression_type in source.expression_types() {
        target.record_expression_type(*expression_type);
    }
    for signature in source.declaration_signatures() {
        target.record_declaration_signature(*signature);
    }
    for assignment_check in source.assignment_checks() {
        target.record_assignment_check(*assignment_check);
    }
    for diagnostic in source.diagnostics() {
        target.record_diagnostic(diagnostic.clone());
    }
}

pub fn type_accepted_expressions(
    literals: &[ParsedLiteralExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    resolutions: &ResolutionTable,
    known_symbols: &[KnownSymbolType],
) -> (TypeArena, TypeCheckReport) {
    let (arena, _primitives, report) = type_accepted_expressions_with_primitives(
        literals,
        grouped_expressions,
        resolutions,
        known_symbols,
    );
    (arena, report)
}

pub(crate) fn type_accepted_expressions_with_primitives(
    literals: &[ParsedLiteralExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    resolutions: &ResolutionTable,
    known_symbols: &[KnownSymbolType],
) -> (TypeArena, PrimitiveTypeIds, TypeCheckReport) {
    let mut arena = TypeArena::new();
    let primitives = PrimitiveTypeIds::insert_into(&mut arena);
    let mut report = TypeCheckReport::new();

    record_accepted_expression_types(
        literals,
        grouped_expressions,
        resolutions,
        known_symbols,
        primitives,
        &mut report,
    );

    (arena, primitives, report)
}

pub(crate) fn record_accepted_expression_types(
    literals: &[ParsedLiteralExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    resolutions: &ResolutionTable,
    known_symbols: &[KnownSymbolType],
    primitives: PrimitiveTypeIds,
    report: &mut TypeCheckReport,
) {
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
}

pub fn type_primitive_local_declaration_initializers(
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    resolutions: &ResolutionTable,
    known_symbols: &[KnownSymbolType],
) -> (TypeArena, TypeCheckReport) {
    let (arena, primitives, expression_report) = type_accepted_expressions_with_primitives(
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

/// Reports a local whose annotation and initializer are different primitive types.
/// ADR-0059 allows no implicit conversion between `Byte` and `Int`. This runs once
/// the types have settled, since a name initializer can be typed after the first
/// declaration pass, which would otherwise skip the check.
pub(crate) fn diagnose_local_initializer_mismatches(
    parsed: &ParseOutput,
    types: &TypeArena,
    classes: &ClassTypeReport,
    report: &mut TypeCheckReport,
) {
    for declaration in &parsed.local_declarations {
        let Some(initializer) = declaration.initializer else {
            continue;
        };
        let Some(annotation) = report.declaration_signature(declaration.declaration) else {
            continue;
        };
        let Some(value) = report.expression_type(initializer) else {
            continue;
        };
        // A nullable local is checked by the flow analysis, which knows refinements.
        let nullable_or_null = |ty: TypeId| {
            matches!(
                types.get(ty).map(|record| record.kind()),
                Some(TypeKind::Nullable(_) | TypeKind::Primitive(PrimitiveType::Null))
            )
        };
        // A generic enum's constructor is typed as the plain enum, so its
        // instance cannot be compared here.
        let plain_of_same_generic = matches!(
            (
                types.get(annotation).map(|record| record.kind()),
                types.get(value).map(|record| record.kind()),
            ),
            (Some(TypeKind::GenericInstance(instance)), Some(TypeKind::Nominal(identity)))
                if instance.declaration().declaration() == identity.declaration()
        );
        if annotation == value
            || plain_of_same_generic
            || nullable_or_null(annotation)
            || nullable_or_null(value)
            || crate::type_check::flow::assignment_compatible(annotation, value, types)
            || crate::type_check::calls::class_value_assignable(classes, value, annotation)
            || report
                .assignment_checks()
                .iter()
                .any(|check| check.statement == declaration.declaration)
            || report
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.node() == initializer)
        {
            continue;
        }
        report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
            initializer,
            annotation,
            value,
        ));
    }
}

use crate::{
    ast::{AstArena, AstNodeId, AstNodeKind},
    module::PackageNamespace,
    name_resolution::{LocalBinding, LocalBindingKind, ResolutionTable, ResolvedLocalBinding},
    parser::{
        ParseOutput, ParsedAssignmentStatement, ParsedBinaryExpression, ParsedBinaryOperator,
        ParsedFunctionDeclaration, ParsedFunctionParameter, ParsedGenericParameter,
        ParsedGroupedExpression, ParsedIfExpression, ParsedIntegerLiteral, ParsedLiteralExpression,
        ParsedLiteralKind, ParsedLocalDeclaration, ParsedTypeNameReference, ParsedUnaryExpression,
    },
    source::ByteSpan,
    symbol::{SymbolId, SymbolInterner},
    types::{GenericParameterType, PrimitiveType, TypeArena, TypeId, TypeKind, TypeRecord},
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
    InvalidNullableUse,
    InvalidatedRefinement,
    UnsupportedFlowRule,
    AmbiguousFlowRule,
    StaticIntegerDiagnostic,
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
    NullableValueWithoutRefinement,
    NullableAssignmentWithoutRefinement,
    AssignmentInvalidatedRefinement,
    RegionExitInvalidatedRefinement,
    MutableLocalRefinementDeferred,
    BooleanCombinationRefinementDeferred,
    MemberRefinementDeferred,
    CallResultRefinementDeferred,
    ExclusiveBorrowRefinementDeferred,
    AmbiguousLocalBindingFlow,
    AmbiguousNullTestRegion,
    IntegerLiteralOutOfRange,
    ByteLiteralOutOfRange,
    IntegerOverflow,
    DivisionByZero,
    NegativeExponent,
    InvalidShiftCount,
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
    pub fn static_integer(rule: TypeRuleDiagnostic, node: AstNodeId) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::StaticIntegerDiagnostic,
            rule,
            node,
            expected_type: None,
            actual_type: None,
        }
    }
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

    pub fn invalid_nullable_use(
        rule: TypeRuleDiagnostic,
        node: AstNodeId,
        expected_type: TypeId,
        actual_type: TypeId,
    ) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::InvalidNullableUse,
            rule,
            node,
            expected_type: Some(expected_type),
            actual_type: Some(actual_type),
        }
    }

    pub fn invalidated_refinement(
        rule: TypeRuleDiagnostic,
        node: AstNodeId,
        expected_type: TypeId,
        actual_type: TypeId,
    ) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::InvalidatedRefinement,
            rule,
            node,
            expected_type: Some(expected_type),
            actual_type: Some(actual_type),
        }
    }

    pub fn unsupported_flow_rule(rule: TypeRuleDiagnostic, node: AstNodeId) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::UnsupportedFlowRule,
            rule,
            node,
            expected_type: None,
            actual_type: None,
        }
    }

    pub fn ambiguous_flow_rule(rule: TypeRuleDiagnostic, node: AstNodeId) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::AmbiguousFlowRule,
            rule,
            node,
            expected_type: None,
            actual_type: None,
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
pub enum EntryPointDiagnosticKind {
    MissingEntryPoint,
    DuplicateEntryPoint,
    InvalidEntryPointSignature,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EntryPointDiagnosticLocation {
    Source(ByteSpan),
    EntryPackageInput(PackageNamespace),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryPointDiagnostic {
    kind: EntryPointDiagnosticKind,
    location: EntryPointDiagnosticLocation,
}

impl EntryPointDiagnostic {
    fn source(kind: EntryPointDiagnosticKind, span: ByteSpan) -> Self {
        Self {
            kind,
            location: EntryPointDiagnosticLocation::Source(span),
        }
    }

    fn entry_package_input(package: &PackageNamespace) -> Self {
        Self {
            kind: EntryPointDiagnosticKind::MissingEntryPoint,
            location: EntryPointDiagnosticLocation::EntryPackageInput(package.clone()),
        }
    }

    pub fn kind(&self) -> EntryPointDiagnosticKind {
        self.kind
    }

    pub fn source_span(&self) -> Option<ByteSpan> {
        match &self.location {
            EntryPointDiagnosticLocation::Source(span) => Some(*span),
            EntryPointDiagnosticLocation::EntryPackageInput(_) => None,
        }
    }

    pub fn location(&self) -> &EntryPointDiagnosticLocation {
        &self.location
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EntryPoint {
    declaration: AstNodeId,
    source_span: ByteSpan,
}

impl EntryPoint {
    pub fn declaration(self) -> AstNodeId {
        self.declaration
    }

    pub fn source_span(self) -> ByteSpan {
        self.source_span
    }
}

pub struct EntryPointFile<'a> {
    package: &'a PackageNamespace,
    parsed: &'a ParseOutput,
}

impl<'a> EntryPointFile<'a> {
    pub fn new(package: &'a PackageNamespace, parsed: &'a ParseOutput) -> Self {
        Self { package, parsed }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EntryPointReport {
    entry_point: Option<EntryPoint>,
    diagnostics: Vec<EntryPointDiagnostic>,
}

impl EntryPointReport {
    pub fn entry_point(&self) -> Option<EntryPoint> {
        self.entry_point
    }

    pub fn diagnostics(&self) -> &[EntryPointDiagnostic] {
        &self.diagnostics
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReturnPathDiagnosticKind {
    MissingReturn,
    UnreachableReturn,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReturnPathDiagnostic {
    kind: ReturnPathDiagnosticKind,
    node: AstNodeId,
}

impl ReturnPathDiagnostic {
    fn new(kind: ReturnPathDiagnosticKind, node: AstNodeId) -> Self {
        Self { kind, node }
    }

    pub fn kind(self) -> ReturnPathDiagnosticKind {
        self.kind
    }

    pub fn node(self) -> AstNodeId {
        self.node
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReturnPathReport {
    diagnostics: Vec<ReturnPathDiagnostic>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReturnTypeDiagnosticKind {
    ReturnTypeMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReturnTypeDiagnostic {
    kind: ReturnTypeDiagnosticKind,
    span: ByteSpan,
}

impl ReturnTypeDiagnostic {
    fn new(kind: ReturnTypeDiagnosticKind, span: ByteSpan) -> Self {
        Self { kind, span }
    }

    pub fn kind(self) -> ReturnTypeDiagnosticKind {
        self.kind
    }

    pub fn span(self) -> ByteSpan {
        self.span
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReturnTypeReport {
    diagnostics: Vec<ReturnTypeDiagnostic>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsupportedExecutableFormDiagnostic {
    span: ByteSpan,
}

impl UnsupportedExecutableFormDiagnostic {
    fn new(span: ByteSpan) -> Self {
        Self { span }
    }

    pub fn span(self) -> ByteSpan {
        self.span
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UnsupportedExecutableFormReport {
    diagnostics: Vec<UnsupportedExecutableFormDiagnostic>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionSignature {
    declaration: AstNodeId,
    parameter_types: Vec<TypeId>,
    return_type: TypeId,
}

pub struct ExecutableSourceTypes<'a> {
    package: &'a PackageNamespace,
    parsed: &'a ParseOutput,
    signatures: &'a [FunctionSignature],
    expression_types: &'a [ExpressionType],
}

impl<'a> ExecutableSourceTypes<'a> {
    pub fn new(
        package: &'a PackageNamespace,
        parsed: &'a ParseOutput,
        signatures: &'a [FunctionSignature],
        expression_types: &'a [ExpressionType],
    ) -> Self {
        Self {
            package,
            parsed,
            signatures,
            expression_types,
        }
    }

    pub fn package(&self) -> &PackageNamespace {
        self.package
    }

    pub fn parsed(&self) -> &ParseOutput {
        self.parsed
    }

    pub fn signatures(&self) -> &[FunctionSignature] {
        self.signatures
    }

    pub fn expression_types(&self) -> &[ExpressionType] {
        self.expression_types
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectCallDiagnosticKind {
    InvalidCallTarget,
    ArgumentCountMismatch,
    ArgumentTypeMismatch,
    RecursiveCallUnsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirectCallDiagnostic {
    kind: DirectCallDiagnosticKind,
    span: ByteSpan,
}

impl DirectCallDiagnostic {
    pub fn new(kind: DirectCallDiagnosticKind, span: ByteSpan) -> Self {
        Self { kind, span }
    }

    pub fn kind(self) -> DirectCallDiagnosticKind {
        self.kind
    }

    pub fn span(self) -> ByteSpan {
        self.span
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirectCallExpressionType {
    expression_span: ByteSpan,
    ty: TypeId,
}

impl DirectCallExpressionType {
    pub fn expression_span(self) -> ByteSpan {
        self.expression_span
    }

    pub fn ty(self) -> TypeId {
        self.ty
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DirectCallReport {
    expression_types: Vec<ExpressionType>,
    source_expression_types: Vec<DirectCallExpressionType>,
    diagnostics: Vec<DirectCallDiagnostic>,
}

impl DirectCallReport {
    pub fn expression_types(&self) -> &[ExpressionType] {
        &self.expression_types
    }

    pub fn source_expression_types(&self) -> &[DirectCallExpressionType] {
        &self.source_expression_types
    }

    pub fn diagnostics(&self) -> &[DirectCallDiagnostic] {
        &self.diagnostics
    }
}

pub fn check_m0028_direct_calls(sources: &[ExecutableSourceTypes<'_>]) -> DirectCallReport {
    let mut report = DirectCallReport::default();
    let mut resolved_calls = Vec::new();
    let mut call_edges: Vec<(ByteSpan, ByteSpan)> = Vec::new();

    for (source_index, source) in sources.iter().enumerate() {
        for (call_index, call) in source.parsed.call_expressions.iter().enumerate() {
            let Some(callee_name) = source
                .parsed
                .name_references
                .iter()
                .find(|reference| reference.reference == call.callee)
            else {
                let span = source
                    .parsed
                    .arena
                    .node(call.callee)
                    .expect("parsed call callee")
                    .span;
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::InvalidCallTarget,
                    span,
                ));
                continue;
            };
            let targets: Vec<_> = sources
                .iter()
                .enumerate()
                .filter(|(_, candidate)| candidate.package == source.package)
                .flat_map(|(candidate_index, candidate)| {
                    candidate
                        .parsed
                        .declaration_names
                        .iter()
                        .filter(move |name| name.name == callee_name.name)
                        .filter_map(move |name| {
                            candidate
                                .signatures
                                .iter()
                                .position(|signature| signature.declaration == name.declaration)
                                .map(|signature_index| (candidate_index, signature_index))
                        })
                })
                .collect();
            if targets.len() != 1 {
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::InvalidCallTarget,
                    callee_name.name_span,
                ));
                continue;
            }
            let (target_source_index, target_signature_index) = targets[0];
            let target_source = &sources[target_source_index];
            let target = &target_source.signatures[target_signature_index];
            let has_body = target_source
                .parsed
                .function_declarations
                .iter()
                .any(|function| {
                    function.declaration == target.declaration && function.body.is_some()
                });
            if !has_body {
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::InvalidCallTarget,
                    callee_name.name_span,
                ));
                continue;
            }
            let source_function_span = source
                .parsed
                .arena
                .node(call.function)
                .expect("parsed call owner function")
                .span;
            let target_span = target_source
                .parsed
                .arena
                .node(target.declaration)
                .expect("parsed call target function")
                .span;
            call_edges.push((source_function_span, target_span));
            resolved_calls.push((
                source_index,
                call_index,
                source_function_span,
                target_span,
                target_source_index,
                target_signature_index,
            ));
        }
    }

    for (
        source_index,
        call_index,
        source_function_span,
        target_span,
        target_source_index,
        target_signature_index,
    ) in resolved_calls
    {
        let source = &sources[source_index];
        let call = &source.parsed.call_expressions[call_index];
        let target = &sources[target_source_index].signatures[target_signature_index];
        if m0028_call_path_exists(&call_edges, target_span, source_function_span) {
            let span = source
                .parsed
                .arena
                .node(call.expression)
                .expect("parsed call expression")
                .span;
            report.diagnostics.push(DirectCallDiagnostic::new(
                DirectCallDiagnosticKind::RecursiveCallUnsupported,
                span,
            ));
            continue;
        }
        if call.arguments.len() != target.parameter_types.len() {
            let span = source
                .parsed
                .arena
                .node(call.expression)
                .expect("parsed call expression")
                .span;
            report.diagnostics.push(DirectCallDiagnostic::new(
                DirectCallDiagnosticKind::ArgumentCountMismatch,
                span,
            ));
            continue;
        }
        let mut argument_error = false;
        for (argument, parameter) in call.arguments.iter().zip(&target.parameter_types) {
            if source
                .expression_types
                .iter()
                .find(|typed| typed.expression == *argument)
                .map(|typed| typed.ty)
                != Some(*parameter)
            {
                let span = source
                    .parsed
                    .arena
                    .node(*argument)
                    .expect("parsed argument")
                    .span;
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::ArgumentTypeMismatch,
                    span,
                ));
                argument_error = true;
            }
        }
        if !argument_error {
            let call_span = source
                .parsed
                .arena
                .node(call.expression)
                .expect("parsed call expression")
                .span;
            report
                .expression_types
                .push(ExpressionType::new(call.expression, target.return_type));
            report
                .source_expression_types
                .push(DirectCallExpressionType {
                    expression_span: call_span,
                    ty: target.return_type,
                });
        }
    }
    report
}

pub fn apply_m0028_direct_call_results(
    report: &mut TypeCheckReport,
    parsed: &ParseOutput,
    direct_calls: &DirectCallReport,
) {
    for call in &parsed.call_expressions {
        let span = parsed
            .arena
            .node(call.expression)
            .expect("parsed call expression")
            .span;
        let Some(ty) = direct_calls
            .source_expression_types
            .iter()
            .find(|typed| typed.expression_span == span)
            .map(|typed| typed.ty)
        else {
            continue;
        };
        report.record_expression_type(ExpressionType::new(call.expression, ty));
        report.diagnostics.retain(|diagnostic| {
            diagnostic.node() != call.expression
                || diagnostic.rule() != TypeRuleDiagnostic::DirectCallDeferred
        });
    }
}

fn m0028_call_path_exists(
    call_edges: &[(ByteSpan, ByteSpan)],
    start: ByteSpan,
    goal: ByteSpan,
) -> bool {
    let mut pending = vec![start];
    let mut visited = Vec::new();

    while let Some(current) = pending.pop() {
        if current == goal {
            return true;
        }
        if visited.contains(&current) {
            continue;
        }
        visited.push(current);
        pending.extend(
            call_edges
                .iter()
                .filter_map(|(owner, target)| (*owner == current).then_some(*target)),
        );
    }

    false
}

impl FunctionSignature {
    pub fn declaration(&self) -> AstNodeId {
        self.declaration
    }

    pub fn parameter_types(&self) -> &[TypeId] {
        &self.parameter_types
    }

    pub fn return_type(&self) -> TypeId {
        self.return_type
    }
}

impl ReturnPathReport {
    pub fn diagnostics(&self) -> &[ReturnPathDiagnostic] {
        &self.diagnostics
    }
}

impl ReturnTypeReport {
    pub fn diagnostics(&self) -> &[ReturnTypeDiagnostic] {
        &self.diagnostics
    }
}

impl UnsupportedExecutableFormReport {
    pub fn diagnostics(&self) -> &[UnsupportedExecutableFormDiagnostic] {
        &self.diagnostics
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefinementRecord {
    region: AstNodeId,
    binding_use: AstNodeId,
    originating_null_test: AstNodeId,
    binding: LocalBinding,
    original_nullable_type: TypeId,
    refined_non_null_type: TypeId,
}

impl RefinementRecord {
    pub fn new(
        region: AstNodeId,
        binding_use: AstNodeId,
        originating_null_test: AstNodeId,
        binding: LocalBinding,
        original_nullable_type: TypeId,
        refined_non_null_type: TypeId,
    ) -> Self {
        Self {
            region,
            binding_use,
            originating_null_test,
            binding,
            original_nullable_type,
            refined_non_null_type,
        }
    }

    pub fn region(&self) -> AstNodeId {
        self.region
    }

    pub fn binding_use(&self) -> AstNodeId {
        self.binding_use
    }

    pub fn originating_null_test(&self) -> AstNodeId {
        self.originating_null_test
    }

    pub fn binding(&self) -> &LocalBinding {
        &self.binding
    }

    pub fn original_nullable_type(&self) -> TypeId {
        self.original_nullable_type
    }

    pub fn refined_non_null_type(&self) -> TypeId {
        self.refined_non_null_type
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RefinedExpressionType {
    expression: AstNodeId,
    refinement: AstNodeId,
    original_nullable_type: TypeId,
    refined_non_null_type: TypeId,
}

impl RefinedExpressionType {
    pub fn new(
        expression: AstNodeId,
        refinement: AstNodeId,
        original_nullable_type: TypeId,
        refined_non_null_type: TypeId,
    ) -> Self {
        Self {
            expression,
            refinement,
            original_nullable_type,
            refined_non_null_type,
        }
    }

    pub fn expression(self) -> AstNodeId {
        self.expression
    }

    pub fn refinement(self) -> AstNodeId {
        self.refinement
    }

    pub fn original_nullable_type(self) -> TypeId {
        self.original_nullable_type
    }

    pub fn refined_non_null_type(self) -> TypeId {
        self.refined_non_null_type
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NullTestRefinedBranch {
    Then,
    Else,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecognizedNullTest {
    expression: AstNodeId,
    name_expression: AstNodeId,
    null_literal: AstNodeId,
    operator: ParsedBinaryOperator,
    refined_branch: NullTestRefinedBranch,
}

impl RecognizedNullTest {
    pub fn new(
        expression: AstNodeId,
        name_expression: AstNodeId,
        null_literal: AstNodeId,
        operator: ParsedBinaryOperator,
        refined_branch: NullTestRefinedBranch,
    ) -> Self {
        Self {
            expression,
            name_expression,
            null_literal,
            operator,
            refined_branch,
        }
    }

    pub fn expression(self) -> AstNodeId {
        self.expression
    }

    pub fn name_expression(self) -> AstNodeId {
        self.name_expression
    }

    pub fn null_literal(self) -> AstNodeId {
        self.null_literal
    }

    pub fn operator(self) -> ParsedBinaryOperator {
        self.operator
    }

    pub fn refined_branch(self) -> NullTestRefinedBranch {
        self.refined_branch
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EligibleNullTestRefinement {
    null_test: RecognizedNullTest,
    binding: LocalBinding,
    original_nullable_type: TypeId,
    refined_non_null_type: TypeId,
}

impl EligibleNullTestRefinement {
    pub fn new(
        null_test: RecognizedNullTest,
        binding: LocalBinding,
        original_nullable_type: TypeId,
        refined_non_null_type: TypeId,
    ) -> Self {
        Self {
            null_test,
            binding,
            original_nullable_type,
            refined_non_null_type,
        }
    }

    pub fn null_test(&self) -> RecognizedNullTest {
        self.null_test
    }

    pub fn binding(&self) -> &LocalBinding {
        &self.binding
    }

    pub fn original_nullable_type(&self) -> TypeId {
        self.original_nullable_type
    }

    pub fn refined_non_null_type(&self) -> TypeId {
        self.refined_non_null_type
    }
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
    Unit,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GenericParameterTypeRecord {
    parameter: AstNodeId,
    ty: TypeId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapabilityBoundRecord {
    parameter: AstNodeId,
    ty: TypeId,
    bound: AstNodeId,
    symbol: SymbolId,
}

impl CapabilityBoundRecord {
    pub fn new(parameter: AstNodeId, ty: TypeId, bound: AstNodeId, symbol: SymbolId) -> Self {
        Self {
            parameter,
            ty,
            bound,
            symbol,
        }
    }

    pub fn parameter(self) -> AstNodeId {
        self.parameter
    }

    pub fn ty(self) -> TypeId {
        self.ty
    }

    pub fn bound(self) -> AstNodeId {
        self.bound
    }

    pub fn symbol(self) -> SymbolId {
        self.symbol
    }
}

impl GenericParameterTypeRecord {
    pub fn new(parameter: AstNodeId, ty: TypeId) -> Self {
        Self { parameter, ty }
    }

    pub fn parameter(self) -> AstNodeId {
        self.parameter
    }

    pub fn ty(self) -> TypeId {
        self.ty
    }
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

pub fn build_m0020_generic_parameter_types(
    parameters: &[ParsedGenericParameter],
    symbols: &mut SymbolInterner,
    type_arena: &mut TypeArena,
) -> Vec<GenericParameterTypeRecord> {
    parameters
        .iter()
        .map(|parameter| {
            let symbol = symbols.intern(&parameter.name);
            let ty = type_arena.insert(TypeRecord::generic_parameter(GenericParameterType::new(
                parameter.parameter,
                symbol,
            )));
            GenericParameterTypeRecord::new(parameter.parameter, ty)
        })
        .collect()
}

pub fn build_m0020_capability_bound_records(
    parameters: &[ParsedGenericParameter],
    parameter_types: &[GenericParameterTypeRecord],
    symbols: &mut SymbolInterner,
) -> Vec<CapabilityBoundRecord> {
    let mut records = Vec::new();

    for parameter in parameters {
        let Some(parameter_type) = parameter_types
            .iter()
            .find(|record| record.parameter() == parameter.parameter)
        else {
            continue;
        };

        for bound in &parameter.capability_bounds {
            records.push(CapabilityBoundRecord::new(
                parameter.parameter,
                parameter_type.ty(),
                bound.bound,
                symbols.intern(&bound.name),
            ));
        }
    }

    records
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeCheckReport {
    expression_types: Vec<ExpressionType>,
    declaration_signatures: Vec<DeclarationSignature>,
    assignment_checks: Vec<AssignmentCheck>,
    refinements: Vec<RefinementRecord>,
    refined_expression_types: Vec<RefinedExpressionType>,
    diagnostics: Vec<TypeCheckDiagnostic>,
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

pub fn check_m0028_entry_point(
    entry_package: &PackageNamespace,
    files: &[EntryPointFile<'_>],
) -> EntryPointReport {
    let candidates: Vec<_> = files
        .iter()
        .filter(|file| file.package == entry_package)
        .flat_map(|file| {
            file.parsed
                .function_declarations
                .iter()
                .filter(move |function| {
                    function.top_level
                        && file.parsed.declaration_names.iter().any(|name| {
                            name.declaration == function.declaration && name.name == "main"
                        })
                })
                .map(move |function| (file.parsed, function))
        })
        .collect();

    if candidates.is_empty() {
        return EntryPointReport {
            entry_point: None,
            diagnostics: vec![EntryPointDiagnostic::entry_package_input(entry_package)],
        };
    }

    if candidates.len() > 1 {
        return EntryPointReport {
            entry_point: None,
            diagnostics: candidates
                .iter()
                .map(|(parsed, function)| {
                    EntryPointDiagnostic::source(
                        EntryPointDiagnosticKind::DuplicateEntryPoint,
                        parsed
                            .arena
                            .node(function.declaration)
                            .expect("parsed function declaration is in its arena")
                            .span,
                    )
                })
                .collect(),
        };
    }

    let (parsed, candidate) = candidates[0];
    let has_int_return = candidate.return_annotation.is_some_and(|annotation| {
        parsed
            .type_name_references
            .iter()
            .any(|reference| reference.reference == annotation && reference.name == "Int")
    });
    let span = parsed
        .arena
        .node(candidate.declaration)
        .expect("parsed function declaration is in its arena")
        .span;
    if candidate.body.is_none() || !candidate.parameters.is_empty() || !has_int_return {
        return EntryPointReport {
            entry_point: None,
            diagnostics: vec![EntryPointDiagnostic::source(
                EntryPointDiagnosticKind::InvalidEntryPointSignature,
                span,
            )],
        };
    }

    EntryPointReport {
        entry_point: Some(EntryPoint {
            declaration: candidate.declaration,
            source_span: span,
        }),
        diagnostics: Vec::new(),
    }
}

pub fn check_m0028_straight_line_returns(parsed: &ParseOutput) -> ReturnPathReport {
    let mut diagnostics = Vec::new();
    for function in &parsed.function_declarations {
        let is_int_function = function.return_annotation.is_some_and(|annotation| {
            parsed
                .type_name_references
                .iter()
                .any(|reference| reference.reference == annotation && reference.name == "Int")
        });
        let Some(body) = function.body else {
            continue;
        };
        if !is_int_function {
            continue;
        }

        let direct_returns: Vec<_> = parsed
            .return_statements
            .iter()
            .filter(|returned| returned.function == function.declaration && returned.block == body)
            .collect();
        let Some((first, later)) = direct_returns.split_first() else {
            diagnostics.push(ReturnPathDiagnostic::new(
                ReturnPathDiagnosticKind::MissingReturn,
                function.declaration,
            ));
            continue;
        };
        let _ = first;
        diagnostics.extend(later.iter().map(|returned| {
            ReturnPathDiagnostic::new(
                ReturnPathDiagnosticKind::UnreachableReturn,
                returned.statement,
            )
        }));
    }
    ReturnPathReport { diagnostics }
}

pub fn check_m0028_return_expression_types(
    parsed: &ParseOutput,
    signatures: &[FunctionSignature],
    expression_types: &[ExpressionType],
) -> ReturnTypeReport {
    let mut diagnostics = Vec::new();

    for returned in &parsed.return_statements {
        let Some(value) = returned.value else {
            continue;
        };
        let Some(expected) = signatures
            .iter()
            .find(|signature| signature.declaration == returned.function)
            .map(|signature| signature.return_type)
        else {
            continue;
        };
        let Some(actual) = expression_types
            .iter()
            .find(|typed| typed.expression == value)
            .map(|typed| typed.ty)
        else {
            continue;
        };
        if actual != expected {
            let span = parsed.arena.node(value).expect("parsed return value").span;
            diagnostics.push(ReturnTypeDiagnostic::new(
                ReturnTypeDiagnosticKind::ReturnTypeMismatch,
                span,
            ));
        }
    }

    ReturnTypeReport { diagnostics }
}

pub fn check_m0028_unsupported_executable_forms(
    parsed: &ParseOutput,
) -> UnsupportedExecutableFormReport {
    let specific_form_spans: Vec<_> = parsed
        .arena
        .nodes()
        .iter()
        .filter(|node| {
            matches!(
                node.kind,
                AstNodeKind::IfExpression
                    | AstNodeKind::BinaryExpression
                    | AstNodeKind::UnaryExpression
                    | AstNodeKind::CallExpression
                    | AstNodeKind::MemberExpression
            )
        })
        .map(|node| node.span)
        .collect();
    let mut candidates: Vec<_> = parsed
        .arena
        .nodes()
        .iter()
        .filter(|node| {
            matches!(
                node.kind,
                AstNodeKind::ImportDeclaration
                    | AstNodeKind::StructDeclaration
                    | AstNodeKind::EnumDeclaration
                    | AstNodeKind::InterfaceDeclaration
                    | AstNodeKind::NullableType
                    | AstNodeKind::GenericParameter
                    | AstNodeKind::GenericArgument
                    | AstNodeKind::CapabilityBound
                    | AstNodeKind::FunctionType
                    | AstNodeKind::GroupedType
                    | AstNodeKind::WhenExpression
            )
        })
        .map(|node| node.span)
        .collect();

    candidates.extend(
        parsed
            .literal_expressions
            .iter()
            .filter(|literal| literal.kind != ParsedLiteralKind::AcceptedInteger)
            .map(|literal| literal.span),
    );
    candidates.extend(
        parsed
            .type_name_references
            .iter()
            .filter(|reference| reference.name != "Int")
            .map(|reference| reference.name_span),
    );
    candidates.extend(
        parsed
            .function_declarations
            .iter()
            .filter(|function| function.body.is_none())
            .filter_map(|function| {
                parsed
                    .arena
                    .node(function.declaration)
                    .map(|node| node.span)
            }),
    );
    candidates.extend(
        parsed
            .local_declarations
            .iter()
            .filter(|declaration| declaration.initializer.is_none())
            .filter_map(|declaration| {
                parsed
                    .arena
                    .node(declaration.declaration)
                    .map(|node| node.span)
            }),
    );
    candidates.extend(
        parsed
            .return_statements
            .iter()
            .filter(|returned| returned.value.is_none())
            .filter_map(|returned| parsed.arena.node(returned.statement).map(|node| node.span)),
    );

    candidates.sort_by(|left, right| {
        left.file()
            .index()
            .cmp(&right.file().index())
            .then(left.start().cmp(&right.start()))
            .then(right.end().cmp(&left.end()))
    });
    candidates.dedup();

    let mut diagnostics = Vec::new();
    for span in candidates {
        if specific_form_spans
            .iter()
            .any(|specific| m0028_span_contains(*specific, span))
        {
            continue;
        }
        if diagnostics
            .iter()
            .any(|diagnostic: &UnsupportedExecutableFormDiagnostic| {
                m0028_span_contains(diagnostic.span, span)
            })
        {
            continue;
        }
        diagnostics.push(UnsupportedExecutableFormDiagnostic::new(span));
    }

    UnsupportedExecutableFormReport { diagnostics }
}

fn m0028_span_contains(outer: ByteSpan, inner: ByteSpan) -> bool {
    outer.file() == inner.file()
        && outer.start() <= inner.start()
        && inner.end() <= outer.end()
        && outer != inner
}

pub fn type_m0028_function_signatures(
    functions: &[ParsedFunctionDeclaration],
    parameters: &[ParsedFunctionParameter],
    type_name_references: &[ParsedTypeNameReference],
) -> (TypeArena, Vec<FunctionSignature>) {
    let mut arena = TypeArena::new();
    let signatures =
        type_m0028_function_signatures_in(&mut arena, functions, parameters, type_name_references);
    (arena, signatures)
}

pub fn type_m0028_function_signatures_in(
    arena: &mut TypeArena,
    functions: &[ParsedFunctionDeclaration],
    parameters: &[ParsedFunctionParameter],
    type_name_references: &[ParsedTypeNameReference],
) -> Vec<FunctionSignature> {
    let primitives = PrimitiveTypeIds::module_owned(arena);
    let is_int_annotation = |annotation| {
        type_name_references
            .iter()
            .any(|reference| reference.reference == annotation && reference.name == "Int")
    };
    let mut signatures = Vec::new();

    for function in functions {
        let Some(return_annotation) = function.return_annotation else {
            continue;
        };
        if !is_int_annotation(return_annotation) {
            continue;
        }
        let function_parameters: Vec<_> = parameters
            .iter()
            .filter(|parameter| parameter.function == function.declaration)
            .collect();
        if function_parameters
            .iter()
            .any(|parameter| !is_int_annotation(parameter.annotation))
        {
            continue;
        }
        signatures.push(FunctionSignature {
            declaration: function.declaration,
            parameter_types: vec![primitives.int_id; function_parameters.len()],
            return_type: primitives.int_id,
        });
    }

    signatures
}

pub fn recognize_m0019_null_tests(
    binary_expressions: &[ParsedBinaryExpression],
    literal_expressions: &[ParsedLiteralExpression],
    arena: &AstArena,
) -> Vec<RecognizedNullTest> {
    let mut recognized = Vec::new();

    for binary in binary_expressions {
        let refined_branch = match binary.operator {
            ParsedBinaryOperator::NotEqual => NullTestRefinedBranch::Then,
            ParsedBinaryOperator::Equal => NullTestRefinedBranch::Else,
            _ => continue,
        };

        let left_is_null = is_null_literal(binary.left, literal_expressions);
        let right_is_null = is_null_literal(binary.right, literal_expressions);
        let (name_expression, null_literal) = match (left_is_null, right_is_null) {
            (true, false) if is_name_expression(binary.right, arena) => (binary.right, binary.left),
            (false, true) if is_name_expression(binary.left, arena) => (binary.left, binary.right),
            _ => continue,
        };

        recognized.push(RecognizedNullTest::new(
            binary.expression,
            name_expression,
            null_literal,
            binary.operator,
            refined_branch,
        ));
    }

    recognized
}

fn is_null_literal(expression: AstNodeId, literal_expressions: &[ParsedLiteralExpression]) -> bool {
    literal_expressions
        .iter()
        .any(|literal| literal.expression == expression && literal.kind == ParsedLiteralKind::Null)
}

fn is_name_expression(expression: AstNodeId, arena: &AstArena) -> bool {
    arena
        .node(expression)
        .is_some_and(|node| node.kind == AstNodeKind::NameExpression)
}

pub fn select_m0019_eligible_null_tests(
    null_tests: &[RecognizedNullTest],
    resolutions: &ResolutionTable,
    local_bindings: &[LocalBinding],
    declaration_signatures: &[DeclarationSignature],
    arena: &TypeArena,
) -> (Vec<EligibleNullTestRefinement>, TypeCheckReport) {
    let mut eligible = Vec::new();
    let mut report = TypeCheckReport::new();

    for null_test in null_tests {
        let Some(resolved) = resolutions.get(null_test.name_expression()) else {
            continue;
        };
        let matching_bindings: Vec<_> = local_bindings
            .iter()
            .filter(|binding| binding.key().name() == resolved.symbol())
            .collect();

        let [binding] = matching_bindings.as_slice() else {
            if matching_bindings.len() > 1 {
                report.record_diagnostic(TypeCheckDiagnostic::ambiguous_flow_rule(
                    TypeRuleDiagnostic::AmbiguousLocalBindingFlow,
                    null_test.name_expression(),
                ));
            }
            continue;
        };

        match binding.kind() {
            LocalBindingKind::Immutable => {}
            LocalBindingKind::Var => {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_flow_rule(
                    TypeRuleDiagnostic::MutableLocalRefinementDeferred,
                    null_test.name_expression(),
                ));
                continue;
            }
        }

        let Some(signature) = declaration_signatures
            .iter()
            .find(|signature| signature.declaration() == binding.binding())
        else {
            continue;
        };
        let Some(record) = arena.get(signature.ty()) else {
            continue;
        };
        let TypeKind::Nullable(nullable) = record.kind() else {
            continue;
        };

        eligible.push(EligibleNullTestRefinement::new(
            *null_test,
            (*binding).clone(),
            signature.ty(),
            nullable.base(),
        ));
    }

    (eligible, report)
}

pub fn record_m0019_branch_refinements(
    eligible_refinements: &[EligibleNullTestRefinement],
    if_expressions: &[ParsedIfExpression],
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for eligible in eligible_refinements {
        let Some(if_expression) = if_expressions
            .iter()
            .find(|if_expression| if_expression.condition == eligible.null_test().expression())
        else {
            continue;
        };

        let Some(region) = m0019_refined_branch_region(eligible.null_test(), if_expression) else {
            continue;
        };

        report.record_refinement(RefinementRecord::new(
            region,
            eligible.null_test().name_expression(),
            eligible.null_test().expression(),
            eligible.binding().clone(),
            eligible.original_nullable_type(),
            eligible.refined_non_null_type(),
        ));
    }

    report
}

fn m0019_refined_branch_region(
    null_test: RecognizedNullTest,
    if_expression: &ParsedIfExpression,
) -> Option<AstNodeId> {
    match null_test.refined_branch() {
        NullTestRefinedBranch::Then => Some(if_expression.then_block),
        NullTestRefinedBranch::Else => if_expression.else_block,
    }
}

pub fn record_m0019_refined_expression_types(
    report: &mut TypeCheckReport,
    arena: &AstArena,
    resolved_local_bindings: &[ResolvedLocalBinding],
) {
    let mut refined_types = Vec::new();
    let mut diagnostics = Vec::new();

    for resolved in resolved_local_bindings {
        let Some(expression) = arena.node(resolved.reference()) else {
            continue;
        };
        if expression.kind != AstNodeKind::NameExpression {
            continue;
        }

        let mut matching_refinement = None;
        let mut ambiguous = false;
        for refinement in report.refinements() {
            if refinement.binding() != resolved.binding()
                || !m0019_expression_is_inside_refinement_region(
                    arena,
                    resolved.reference(),
                    refinement.region(),
                )
            {
                continue;
            }

            if matching_refinement.is_some() {
                ambiguous = true;
                break;
            }
            matching_refinement = Some(refinement);
        }

        if ambiguous {
            diagnostics.push(TypeCheckDiagnostic::ambiguous_flow_rule(
                TypeRuleDiagnostic::AmbiguousNullTestRegion,
                resolved.reference(),
            ));
        } else if let Some(refinement) = matching_refinement {
            refined_types.push(RefinedExpressionType::new(
                resolved.reference(),
                refinement.region(),
                refinement.original_nullable_type(),
                refinement.refined_non_null_type(),
            ));
        }
    }

    for refined_type in refined_types {
        report.record_refined_expression_type(refined_type);
    }
    for diagnostic in diagnostics {
        report.record_diagnostic(diagnostic);
    }
}

fn m0019_expression_is_inside_refinement_region(
    arena: &AstArena,
    expression: AstNodeId,
    region: AstNodeId,
) -> bool {
    let Some(expression) = arena.node(expression) else {
        return false;
    };
    let Some(region) = arena.node(region) else {
        return false;
    };

    expression.kind == AstNodeKind::NameExpression
        && region.kind == AstNodeKind::Block
        && expression.span.file() == region.span.file()
        && region.span.start() <= expression.span.start()
        && expression.span.end() <= region.span.end()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PrimitiveTypeIds {
    bool_id: TypeId,
    int_id: TypeId,
    string_id: TypeId,
    unit_id: TypeId,
    null_id: TypeId,
    float_id: TypeId,
    byte_id: TypeId,
}

impl PrimitiveTypeIds {
    fn insert_into(arena: &mut TypeArena) -> Self {
        Self {
            bool_id: arena.insert(TypeRecord::primitive(PrimitiveType::Bool)),
            int_id: arena.insert(TypeRecord::primitive(PrimitiveType::Int)),
            string_id: arena.insert(TypeRecord::primitive(PrimitiveType::String)),
            unit_id: arena.insert(TypeRecord::primitive(PrimitiveType::Unit)),
            null_id: arena.insert(TypeRecord::primitive(PrimitiveType::Null)),
            float_id: arena.insert(TypeRecord::primitive(PrimitiveType::Float)),
            byte_id: arena.insert(TypeRecord::primitive(PrimitiveType::Byte)),
        }
    }

    fn module_owned(arena: &mut TypeArena) -> Self {
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
            unit_id: primitive(PrimitiveType::Unit),
            null_id: primitive(PrimitiveType::Null),
            float_id: primitive(PrimitiveType::Float),
            byte_id: primitive(PrimitiveType::Byte),
        }
    }

    fn type_for_literal(self, kind: LiteralKind) -> TypeId {
        match kind {
            LiteralKind::BoolTrue | LiteralKind::BoolFalse => self.bool_id,
            LiteralKind::AcceptedInteger => self.int_id,
            LiteralKind::AcceptedString => self.string_id,
            LiteralKind::Float => self.float_id,
            LiteralKind::Unit => self.unit_id,
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

fn literal_kind_from_parser(kind: ParsedLiteralKind) -> LiteralKind {
    match kind {
        ParsedLiteralKind::BoolTrue => LiteralKind::BoolTrue,
        ParsedLiteralKind::BoolFalse => LiteralKind::BoolFalse,
        ParsedLiteralKind::AcceptedInteger => LiteralKind::AcceptedInteger,
        ParsedLiteralKind::AcceptedString => LiteralKind::AcceptedString,
        ParsedLiteralKind::Float => LiteralKind::Float,
        ParsedLiteralKind::Unit => LiteralKind::Unit,
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

pub fn type_m0028_executable_int_operators(
    unary_expressions: &[ParsedUnaryExpression],
    binary_expressions: &[ParsedBinaryExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    known_expression_types: &[ExpressionType],
    int_type: TypeId,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();
    for expression_type in known_expression_types {
        report.record_expression_type(*expression_type);
    }

    let mut completed = Vec::new();
    loop {
        let mut changed = false;

        for unary in unary_expressions {
            if completed.contains(&unary.expression) {
                continue;
            }
            let Some(operand_type) = report.expression_type(unary.operand) else {
                continue;
            };

            completed.push(unary.expression);
            changed = true;
            if operand_type == int_type {
                report.record_expression_type(ExpressionType::new(unary.expression, int_type));
            } else {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    unary.operand,
                    int_type,
                    operand_type,
                ));
            }
        }

        for binary in binary_expressions {
            if completed.contains(&binary.expression)
                || !is_m0028_executable_int_operator(binary.operator)
            {
                continue;
            }
            let (Some(left_type), Some(right_type)) = (
                report.expression_type(binary.left),
                report.expression_type(binary.right),
            ) else {
                continue;
            };

            completed.push(binary.expression);
            changed = true;
            if left_type != int_type {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    binary.left,
                    int_type,
                    left_type,
                ));
            } else if right_type != int_type {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    binary.right,
                    int_type,
                    right_type,
                ));
            } else {
                report.record_expression_type(ExpressionType::new(binary.expression, int_type));
            }
        }

        for grouped in grouped_expressions {
            if completed.contains(&grouped.expression) {
                continue;
            }
            let Some(inner_type) = report.expression_type(grouped.inner) else {
                continue;
            };

            completed.push(grouped.expression);
            changed = true;
            report.record_expression_type(ExpressionType::new(grouped.expression, inner_type));
        }

        if !changed {
            break;
        }
    }

    report
}

fn is_m0028_executable_int_operator(operator: ParsedBinaryOperator) -> bool {
    matches!(
        operator,
        ParsedBinaryOperator::Plus
            | ParsedBinaryOperator::Minus
            | ParsedBinaryOperator::Star
            | ParsedBinaryOperator::Slash
            | ParsedBinaryOperator::Percent
            | ParsedBinaryOperator::Exponent
            | ParsedBinaryOperator::BitwiseAnd
            | ParsedBinaryOperator::BitwiseOr
            | ParsedBinaryOperator::BitwiseXor
            | ParsedBinaryOperator::ShiftLeft
            | ParsedBinaryOperator::ShiftRight
    )
}

pub fn type_m0028_static_integer_diagnostics(
    literals: &[ParsedIntegerLiteral],
    grouped: &[ParsedGroupedExpression],
    unary: &[ParsedUnaryExpression],
    binaries: &[ParsedBinaryExpression],
) -> Vec<TypeCheckDiagnostic> {
    fn is_constant_expression(
        node: AstNodeId,
        literals: &[ParsedIntegerLiteral],
        grouped: &[ParsedGroupedExpression],
        unary: &[ParsedUnaryExpression],
        binaries: &[ParsedBinaryExpression],
    ) -> bool {
        if literals.iter().any(|literal| literal.expression == node) {
            return true;
        }
        if let Some(group) = grouped.iter().find(|group| group.expression == node) {
            return is_constant_expression(group.inner, literals, grouped, unary, binaries);
        }
        if let Some(expression) = unary
            .iter()
            .find(|expression| expression.expression == node)
        {
            return matches!(
                expression.operator,
                crate::parser::ParsedUnaryOperator::Plus
                    | crate::parser::ParsedUnaryOperator::Minus
                    | crate::parser::ParsedUnaryOperator::BitwiseNot
            ) && is_constant_expression(
                expression.operand,
                literals,
                grouped,
                unary,
                binaries,
            );
        }
        let Some(expression) = binaries
            .iter()
            .find(|expression| expression.expression == node)
        else {
            return false;
        };
        is_m0028_executable_int_operator(expression.operator)
            && is_constant_expression(expression.left, literals, grouped, unary, binaries)
            && is_constant_expression(expression.right, literals, grouped, unary, binaries)
    }

    fn is_min_int_magnitude(
        node: AstNodeId,
        literals: &[ParsedIntegerLiteral],
        grouped: &[ParsedGroupedExpression],
    ) -> bool {
        if let Some(literal) = literals.iter().find(|literal| literal.expression == node) {
            return literal.value == Some((i64::MAX as u64) + 1);
        }
        grouped
            .iter()
            .find(|group| group.expression == node)
            .is_some_and(|group| is_min_int_magnitude(group.inner, literals, grouped))
    }

    fn checked_power(base: i64, mut exponent: i64) -> Option<i64> {
        let mut result = 1_i64;
        let mut factor = base;
        while exponent > 0 {
            if exponent % 2 != 0 {
                result = result.checked_mul(factor)?;
            }
            exponent /= 2;
            if exponent > 0 {
                factor = factor.checked_mul(factor)?;
            }
        }
        Some(result)
    }

    fn evaluate(
        node: AstNodeId,
        literals: &[ParsedIntegerLiteral],
        grouped: &[ParsedGroupedExpression],
        unary: &[ParsedUnaryExpression],
        binaries: &[ParsedBinaryExpression],
    ) -> Result<Option<i64>, TypeRuleDiagnostic> {
        if let Some(literal) = literals.iter().find(|literal| literal.expression == node) {
            return literal
                .value
                .and_then(|value| i64::try_from(value).ok())
                .map(Some)
                .ok_or(TypeRuleDiagnostic::IntegerLiteralOutOfRange);
        }
        if let Some(group) = grouped.iter().find(|group| group.expression == node) {
            return evaluate(group.inner, literals, grouped, unary, binaries);
        }
        if let Some(expression) = unary
            .iter()
            .find(|expression| expression.expression == node)
        {
            if expression.operator == crate::parser::ParsedUnaryOperator::Minus
                && is_min_int_magnitude(expression.operand, literals, grouped)
            {
                return Ok(Some(i64::MIN));
            }
            let value = evaluate(expression.operand, literals, grouped, unary, binaries)?;
            return match (expression.operator, value) {
                (_, None) => Ok(None),
                (crate::parser::ParsedUnaryOperator::Plus, Some(value)) => Ok(Some(value)),
                (crate::parser::ParsedUnaryOperator::Minus, Some(value)) => value
                    .checked_neg()
                    .map(Some)
                    .ok_or(TypeRuleDiagnostic::IntegerOverflow),
                (crate::parser::ParsedUnaryOperator::BitwiseNot, Some(value)) => Ok(Some(!value)),
            };
        }
        let Some(expression) = binaries
            .iter()
            .find(|expression| expression.expression == node)
        else {
            return Ok(None);
        };
        let (Some(left), Some(right)) = (
            evaluate(expression.left, literals, grouped, unary, binaries)?,
            evaluate(expression.right, literals, grouped, unary, binaries)?,
        ) else {
            return Ok(None);
        };
        let value = match expression.operator {
            ParsedBinaryOperator::Plus => left.checked_add(right),
            ParsedBinaryOperator::Minus => left.checked_sub(right),
            ParsedBinaryOperator::Star => left.checked_mul(right),
            ParsedBinaryOperator::Slash => {
                return if right == 0 {
                    Err(TypeRuleDiagnostic::DivisionByZero)
                } else {
                    left.checked_div(right)
                        .map(Some)
                        .ok_or(TypeRuleDiagnostic::IntegerOverflow)
                };
            }
            ParsedBinaryOperator::Percent => {
                return if right == 0 {
                    Err(TypeRuleDiagnostic::DivisionByZero)
                } else {
                    left.checked_rem(right)
                        .map(Some)
                        .ok_or(TypeRuleDiagnostic::IntegerOverflow)
                };
            }
            ParsedBinaryOperator::Exponent => {
                if right < 0 {
                    return Err(TypeRuleDiagnostic::NegativeExponent);
                }
                checked_power(left, right)
            }
            ParsedBinaryOperator::ShiftLeft | ParsedBinaryOperator::ShiftRight
                if !(0..64).contains(&right) =>
            {
                return Err(TypeRuleDiagnostic::InvalidShiftCount);
            }
            ParsedBinaryOperator::ShiftLeft => left.checked_shl(right as u32),
            ParsedBinaryOperator::ShiftRight => Some(left >> right),
            ParsedBinaryOperator::BitwiseAnd => Some(left & right),
            ParsedBinaryOperator::BitwiseOr => Some(left | right),
            ParsedBinaryOperator::BitwiseXor => Some(left ^ right),
            _ => return Ok(None),
        };
        value.map(Some).ok_or(TypeRuleDiagnostic::IntegerOverflow)
    }

    let expressions: Vec<_> = literals
        .iter()
        .map(|literal| literal.expression)
        .chain(grouped.iter().map(|expression| expression.expression))
        .chain(unary.iter().map(|expression| expression.expression))
        .chain(binaries.iter().map(|expression| expression.expression))
        .filter(|expression| {
            is_constant_expression(*expression, literals, grouped, unary, binaries)
        })
        .collect();
    let children: Vec<_> = grouped
        .iter()
        .filter(|expression| {
            is_constant_expression(expression.expression, literals, grouped, unary, binaries)
        })
        .map(|expression| expression.inner)
        .chain(
            unary
                .iter()
                .filter(|expression| {
                    is_constant_expression(
                        expression.expression,
                        literals,
                        grouped,
                        unary,
                        binaries,
                    )
                })
                .map(|expression| expression.operand),
        )
        .chain(
            binaries
                .iter()
                .filter(|expression| {
                    is_constant_expression(
                        expression.expression,
                        literals,
                        grouped,
                        unary,
                        binaries,
                    )
                })
                .flat_map(|expression| [expression.left, expression.right]),
        )
        .collect();

    let mut diagnostics = Vec::new();
    for expression in expressions
        .into_iter()
        .filter(|expression| !children.contains(expression))
    {
        if let Err(rule) = evaluate(expression, literals, grouped, unary, binaries) {
            diagnostics.push(TypeCheckDiagnostic::static_integer(rule, expression));
        }
    }
    diagnostics
}

pub fn type_assignment_statements(
    assignments: &[ParsedAssignmentStatement],
    known_expression_types: &[ExpressionType],
    arena: &TypeArena,
) -> TypeCheckReport {
    type_assignment_statements_with_refinements(
        assignments,
        known_expression_types,
        None,
        arena,
        false,
    )
}

pub fn type_m0019_assignment_statements(
    assignments: &[ParsedAssignmentStatement],
    known_expression_types: &[ExpressionType],
    flow_report: &TypeCheckReport,
    resolved_local_bindings: &[ResolvedLocalBinding],
    ast_arena: &AstArena,
    type_arena: &TypeArena,
) -> TypeCheckReport {
    type_assignment_statements_with_refinements(
        assignments,
        known_expression_types,
        Some((flow_report, resolved_local_bindings, ast_arena)),
        type_arena,
        true,
    )
}

pub fn type_m0019_local_declaration_initializers(
    declarations: &[ParsedLocalDeclaration],
    declaration_signatures: &[DeclarationSignature],
    known_expression_types: &[ExpressionType],
    flow_report: &TypeCheckReport,
    resolved_local_bindings: &[ResolvedLocalBinding],
    ast_arena: &AstArena,
    type_arena: &TypeArena,
) -> TypeCheckReport {
    type_m0019_local_declaration_initializers_with_region_exit_invalidations(
        declarations,
        declaration_signatures,
        known_expression_types,
        flow_report,
        resolved_local_bindings,
        None,
        ast_arena,
        type_arena,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn type_m0019_region_exit_refinement_invalidations(
    declarations: &[ParsedLocalDeclaration],
    declaration_signatures: &[DeclarationSignature],
    known_expression_types: &[ExpressionType],
    flow_report: &TypeCheckReport,
    resolved_local_bindings: &[ResolvedLocalBinding],
    if_expressions: &[ParsedIfExpression],
    ast_arena: &AstArena,
    type_arena: &TypeArena,
) -> TypeCheckReport {
    type_m0019_local_declaration_initializers_with_region_exit_invalidations(
        declarations,
        declaration_signatures,
        known_expression_types,
        flow_report,
        resolved_local_bindings,
        Some(if_expressions),
        ast_arena,
        type_arena,
    )
}

#[allow(clippy::too_many_arguments)]
fn type_m0019_local_declaration_initializers_with_region_exit_invalidations(
    declarations: &[ParsedLocalDeclaration],
    declaration_signatures: &[DeclarationSignature],
    known_expression_types: &[ExpressionType],
    flow_report: &TypeCheckReport,
    resolved_local_bindings: &[ResolvedLocalBinding],
    if_expressions: Option<&[ParsedIfExpression]>,
    ast_arena: &AstArena,
    type_arena: &TypeArena,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for signature in declaration_signatures {
        report.record_declaration_signature(*signature);
    }
    for expression_type in known_expression_types {
        report.record_expression_type(*expression_type);
    }

    for declaration in declarations {
        if declaration.annotation.is_none() {
            continue;
        }
        let Some(target_type) = declaration_signatures
            .iter()
            .find(|signature| signature.declaration() == declaration.declaration)
            .map(|signature| signature.ty())
        else {
            continue;
        };
        let Some(initializer) = declaration.initializer else {
            continue;
        };
        let Some(original_initializer_type) =
            expression_type_in(known_expression_types, initializer)
        else {
            continue;
        };
        let refined_initializer_type = valid_m0019_refined_value_type(
            initializer,
            original_initializer_type,
            Some((flow_report, resolved_local_bindings, ast_arena)),
            type_arena,
        );
        let effective_initializer_type =
            refined_initializer_type.unwrap_or(original_initializer_type);

        if assignment_compatible(target_type, effective_initializer_type, type_arena) {
            report.record_assignment_check(AssignmentCheck::new(
                declaration.declaration,
                target_type,
                effective_initializer_type,
            ));
        } else if refined_initializer_type.is_none()
            && exact_m0019_nullable_name_initializer(
                initializer,
                original_initializer_type,
                target_type,
                resolved_local_bindings,
                ast_arena,
                type_arena,
            )
        {
            let diagnostic = if if_expressions.is_some_and(|if_expressions| {
                exact_m0019_region_exit_nullable_name_initializer(
                    initializer,
                    original_initializer_type,
                    target_type,
                    flow_report,
                    resolved_local_bindings,
                    if_expressions,
                    ast_arena,
                    type_arena,
                )
            }) {
                TypeCheckDiagnostic::invalidated_refinement(
                    TypeRuleDiagnostic::RegionExitInvalidatedRefinement,
                    initializer,
                    target_type,
                    original_initializer_type,
                )
            } else {
                TypeCheckDiagnostic::invalid_nullable_use(
                    TypeRuleDiagnostic::NullableAssignmentWithoutRefinement,
                    initializer,
                    target_type,
                    original_initializer_type,
                )
            };
            report.record_diagnostic(diagnostic);
        } else {
            report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                initializer,
                target_type,
                effective_initializer_type,
            ));
        }
    }

    report
}

fn type_assignment_statements_with_refinements(
    assignments: &[ParsedAssignmentStatement],
    known_expression_types: &[ExpressionType],
    flow_context: Option<(&TypeCheckReport, &[ResolvedLocalBinding], &AstArena)>,
    arena: &TypeArena,
    diagnose_nullable_use: bool,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for assignment in assignments {
        let Some(target_type) = expression_type_in(known_expression_types, assignment.target)
        else {
            continue;
        };
        let Some(original_value_type) =
            expression_type_in(known_expression_types, assignment.value)
        else {
            continue;
        };
        let refined_value_type = valid_m0019_refined_value_type(
            assignment.value,
            original_value_type,
            flow_context,
            arena,
        );
        let effective_value_type = refined_value_type.unwrap_or(original_value_type);

        if assignment_compatible(target_type, effective_value_type, arena) {
            report.record_assignment_check(AssignmentCheck::new(
                assignment.statement,
                target_type,
                effective_value_type,
            ));
        } else if diagnose_nullable_use
            && refined_value_type.is_none()
            && nullable_base_type(original_value_type, arena) == Some(target_type)
        {
            report.record_diagnostic(TypeCheckDiagnostic::invalid_nullable_use(
                TypeRuleDiagnostic::NullableAssignmentWithoutRefinement,
                assignment.value,
                target_type,
                original_value_type,
            ));
        } else {
            report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                assignment.value,
                target_type,
                effective_value_type,
            ));
        }
    }

    report
}

fn valid_m0019_refined_value_type(
    expression: AstNodeId,
    original_type: TypeId,
    flow_context: Option<(&TypeCheckReport, &[ResolvedLocalBinding], &AstArena)>,
    type_arena: &TypeArena,
) -> Option<TypeId> {
    let (flow_report, resolved_local_bindings, ast_arena) = flow_context?;
    let mut matching = flow_report
        .refined_expression_types()
        .iter()
        .filter(|entry| entry.expression() == expression);
    let refined = matching.next()?;
    if matching.next().is_some() || refined.original_nullable_type() != original_type {
        return None;
    }

    let mut matching_refinements = flow_report
        .refinements()
        .iter()
        .filter(|entry| entry.region() == refined.refinement());
    let provenance = matching_refinements.next()?;
    if matching_refinements.next().is_some()
        || provenance.original_nullable_type() != refined.original_nullable_type()
        || provenance.refined_non_null_type() != refined.refined_non_null_type()
        || provenance.binding().kind() != LocalBindingKind::Immutable
        || !m0019_expression_is_inside_refinement_region(ast_arena, expression, provenance.region())
    {
        return None;
    }

    let mut matching_resolutions = resolved_local_bindings
        .iter()
        .filter(|resolved| resolved.reference() == expression);
    let resolved = matching_resolutions.next()?;
    if matching_resolutions.next().is_some() || resolved.binding() != provenance.binding() {
        return None;
    }

    let base = nullable_base_type(original_type, type_arena)?;
    (refined.refined_non_null_type() == base).then_some(base)
}

fn exact_m0019_nullable_name_initializer(
    initializer: AstNodeId,
    original_type: TypeId,
    target_type: TypeId,
    resolved_local_bindings: &[ResolvedLocalBinding],
    ast_arena: &AstArena,
    type_arena: &TypeArena,
) -> bool {
    if ast_arena.node(initializer).map(|node| node.kind) != Some(AstNodeKind::NameExpression)
        || nullable_base_type(original_type, type_arena) != Some(target_type)
    {
        return false;
    }

    let mut matching = resolved_local_bindings
        .iter()
        .filter(|resolved| resolved.reference() == initializer);
    matching.next().is_some() && matching.next().is_none()
}

#[allow(clippy::too_many_arguments)]
fn exact_m0019_region_exit_nullable_name_initializer(
    initializer: AstNodeId,
    original_type: TypeId,
    target_type: TypeId,
    flow_report: &TypeCheckReport,
    resolved_local_bindings: &[ResolvedLocalBinding],
    if_expressions: &[ParsedIfExpression],
    ast_arena: &AstArena,
    type_arena: &TypeArena,
) -> bool {
    if ast_arena.node(initializer).map(|node| node.kind) != Some(AstNodeKind::NameExpression)
        || nullable_base_type(original_type, type_arena) != Some(target_type)
    {
        return false;
    }
    let Some(initializer_span) = ast_arena.node(initializer).map(|node| node.span) else {
        return false;
    };

    let mut matching_resolutions = resolved_local_bindings
        .iter()
        .filter(|resolved| resolved.reference() == initializer);
    let Some(resolved) = matching_resolutions.next() else {
        return false;
    };
    if matching_resolutions.next().is_some()
        || resolved.binding().kind() != LocalBindingKind::Immutable
    {
        return false;
    }

    flow_report.refinements().iter().any(|refinement| {
        if refinement.binding() != resolved.binding()
            || refinement.binding().kind() != LocalBindingKind::Immutable
            || refinement.original_nullable_type() != original_type
            || refinement.refined_non_null_type() != target_type
        {
            return false;
        }

        let Some(if_expression) = if_expressions
            .iter()
            .find(|if_expression| if_expression.condition == refinement.originating_null_test())
        else {
            return false;
        };
        let is_refined_branch = refinement.region() == if_expression.then_block
            || if_expression.else_block == Some(refinement.region());

        is_refined_branch
            && if_expression.span.file() == initializer_span.file()
            && if_expression.span.end() <= initializer_span.start()
            && m0019_immediate_containing_block(ast_arena, if_expression.span)
                == m0019_immediate_containing_block(ast_arena, initializer_span)
    })
}

fn m0019_immediate_containing_block(
    arena: &AstArena,
    span: crate::source::ByteSpan,
) -> Option<AstNodeId> {
    arena
        .nodes()
        .iter()
        .filter(|node| {
            node.kind == AstNodeKind::Block
                && node.span.file() == span.file()
                && node.span.start() <= span.start()
                && span.end() <= node.span.end()
        })
        .min_by_key(|node| node.span.end() - node.span.start())
        .map(|node| node.id)
}

fn nullable_base_type(ty: TypeId, arena: &TypeArena) -> Option<TypeId> {
    let record = arena.get(ty)?;
    let TypeKind::Nullable(nullable) = record.kind() else {
        return None;
    };
    Some(nullable.base())
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
    type_unsupported_expressions(arena, None)
}

fn type_unsupported_expressions(
    arena: &AstArena,
    executable_operators: Option<(&[ParsedUnaryExpression], &[ParsedBinaryExpression])>,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for node in arena.nodes() {
        if executable_operators.is_some_and(|(unary, binary)| {
            unary
                .iter()
                .any(|expression| expression.expression == node.id)
                || binary.iter().any(|expression| {
                    expression.expression == node.id
                        && is_m0028_executable_int_operator(expression.operator)
                })
        }) {
            continue;
        }
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

#[allow(clippy::too_many_arguments)]
pub fn type_m0018_core(
    arena: &AstArena,
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    assignments: &[ParsedAssignmentStatement],
    resolutions: &ResolutionTable,
    local_bindings: &[LocalBinding],
) -> (TypeArena, TypeCheckReport) {
    type_core(
        arena,
        declarations,
        type_name_references,
        literals,
        grouped_expressions,
        assignments,
        resolutions,
        local_bindings,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn type_m0028_executable_core(
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
    let report = type_m0028_executable_core_in(
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
pub fn type_m0028_executable_core_in(
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
        grouped_expressions,
        assignments,
        resolutions,
        local_bindings,
        Some((unary_expressions, binary_expressions)),
    );
    for diagnostic in type_m0028_static_integer_diagnostics(
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
fn type_core(
    arena: &AstArena,
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
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
        grouped_expressions,
        assignments,
        resolutions,
        local_bindings,
        executable_operators,
    );
    (type_arena, report)
}

#[allow(clippy::too_many_arguments)]
fn type_core_with_arena(
    type_arena: &mut TypeArena,
    arena: &AstArena,
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    assignments: &[ParsedAssignmentStatement],
    resolutions: &ResolutionTable,
    local_bindings: &[LocalBinding],
    executable_operators: Option<(&[ParsedUnaryExpression], &[ParsedBinaryExpression])>,
) -> TypeCheckReport {
    let primitives = PrimitiveTypeIds::module_owned(type_arena);
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

    let known_symbols = known_local_symbol_types(local_bindings, report.declaration_signatures());
    record_m0018_accepted_expression_types(
        literals,
        grouped_expressions,
        resolutions,
        &known_symbols,
        primitives,
        &mut report,
    );

    if let Some((unary_expressions, binary_expressions)) = executable_operators {
        let operator_report = type_m0028_executable_int_operators(
            unary_expressions,
            binary_expressions,
            grouped_expressions,
            report.expression_types(),
            primitives.int_id,
        );
        merge_type_check_report(&mut report, operator_report);
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

    let unsupported_report = type_unsupported_expressions(arena, executable_operators);
    merge_type_check_report(&mut report, unsupported_report);

    report
}

fn merge_type_check_report(target: &mut TypeCheckReport, source: TypeCheckReport) {
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

    record_m0018_accepted_expression_types(
        literals,
        grouped_expressions,
        resolutions,
        known_symbols,
        primitives,
        &mut report,
    );

    (arena, primitives, report)
}

fn record_m0018_accepted_expression_types(
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

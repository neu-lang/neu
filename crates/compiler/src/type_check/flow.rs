//! Local inference, flow refinements for null tests, control flow, enum `when`, and assignments.

use super::*;
use crate::name_resolution::DeclarationKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefinementRecord {
    pub(crate) region: AstNodeId,
    pub(crate) binding_use: AstNodeId,
    pub(crate) originating_null_test: AstNodeId,
    pub(crate) binding: LocalBinding,
    pub(crate) original_nullable_type: TypeId,
    pub(crate) refined_non_null_type: TypeId,
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
    pub(crate) expression: AstNodeId,
    pub(crate) refinement: AstNodeId,
    pub(crate) original_nullable_type: TypeId,
    pub(crate) refined_non_null_type: TypeId,
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
    pub(crate) expression: AstNodeId,
    pub(crate) name_expression: AstNodeId,
    pub(crate) null_literal: AstNodeId,
    pub(crate) operator: ParsedBinaryOperator,
    pub(crate) refined_branch: NullTestRefinedBranch,
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
    pub(crate) null_test: RecognizedNullTest,
    pub(crate) binding: LocalBinding,
    pub(crate) original_nullable_type: TypeId,
    pub(crate) refined_non_null_type: TypeId,
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

/// The function type a lambda argument must have, from the annotated
/// parameter of the callee (ADR-0125). An overloaded callee has no single
/// signature to read, so it provides no context.
fn argument_context_type(
    parsed: &ParseOutput,
    lambda: AstNodeId,
    types: &mut TypeArena,
) -> Option<TypeId> {
    let call = parsed
        .call_expressions
        .iter()
        .find(|call| call.arguments.contains(&lambda))?;
    let index = call
        .arguments
        .iter()
        .position(|argument| *argument == lambda)?;
    let callee = parsed
        .name_references
        .iter()
        .find(|reference| reference.reference == call.callee)?
        .name
        .as_str();
    let mut functions = parsed.declaration_names.iter().filter(|declaration| {
        declaration.kind == DeclarationKind::Function && declaration.name == callee
    });
    let function = functions.next()?;
    if functions.next().is_some() {
        return None;
    }
    let annotation = parsed
        .function_parameters
        .iter()
        .filter(|parameter| parameter.function == function.declaration)
        .nth(index)?
        .annotation;
    type_annotation_type(parsed, annotation, types, &[])
}

pub fn type_lambda_expressions(
    parsed: &ParseOutput,
    types: &mut TypeArena,
    report: &mut TypeCheckReport,
) {
    for lambda in &parsed.lambda_expressions {
        let expected_type = parsed
            .local_declarations
            .iter()
            .find(|declaration| declaration.initializer == Some(lambda.expression))
            .and_then(|declaration| declaration.annotation)
            .and_then(|annotation| type_annotation_type(parsed, annotation, types, &[]))
            .or_else(|| argument_context_type(parsed, lambda.expression, types));
        let expected_parameters = expected_type.and_then(|expected| {
            types.get(expected).and_then(|record| match record.kind() {
                TypeKind::Function(function) => Some(function.parameters().to_vec()),
                _ => None,
            })
        });
        let spawned = parsed.call_expressions.iter().any(|call| {
            call.arguments.contains(&lambda.expression)
                && parsed.name_references.iter().any(|reference| {
                    reference.reference == call.callee && reference.name == "spawn"
                })
        });
        for reference in parsed.name_references.iter().filter(|reference| {
            parsed
                .arena
                .node(lambda.body)
                .is_some_and(|body| refinement_span_contains(body.span, reference.name_span))
                && !lambda
                    .parameters
                    .iter()
                    .any(|parameter| parameter.name == reference.name)
        }) {
            let binding = parsed
                .local_binding_names
                .iter()
                .find(|binding| binding.name == reference.name);
            let capture_is_literal = binding
                .and_then(|binding| {
                    parsed
                        .local_declarations
                        .iter()
                        .find(|declaration| declaration.declaration == binding.binding)
                })
                .and_then(|declaration| declaration.initializer)
                .is_some_and(|initializer| {
                    parsed
                        .literal_expressions
                        .iter()
                        .any(|literal| literal.expression == initializer)
                });
            if report.expression_type(reference.reference).is_none() {
                continue;
            }
            let copyable = report
                .expression_type(reference.reference)
                .and_then(|typed| classify_ownership_category(types, typed))
                == Some(OwnershipCategory::Copyable);
            let move_only = report
                .expression_type(reference.reference)
                .and_then(|typed| classify_ownership_category(types, typed))
                == Some(OwnershipCategory::MoveOnly);
            let immutable =
                binding.is_some_and(|binding| binding.kind == LocalBindingKind::Immutable);
            let spawned_capture = spawned
                && report
                    .expression_type(reference.reference)
                    .is_some_and(|ty| {
                        satisfies_thread_capability(types, ty, ThreadCapability::Send)
                    });
            if !(capture_is_literal && (copyable || move_only && immutable)) && !spawned_capture {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::LambdaCaptureUnsupported,
                    reference.reference,
                ));
            }
        }
        let Some(parameter_types) = lambda
            .parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                parameter
                    .annotation
                    .and_then(|annotation| type_annotation_type(parsed, annotation, types, &[]))
                    .or_else(|| {
                        expected_parameters
                            .as_ref()
                            .and_then(|types| types.get(index).copied())
                    })
            })
            .collect::<Option<Vec<_>>>()
        else {
            // A parameter with no annotation and no context has no type
            // (ADR-0125). Report it at the lambda and stop typing it.
            if lambda
                .parameters
                .iter()
                .any(|parameter| parameter.annotation.is_none())
            {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::LambdaParameterTypeRequired,
                    lambda.expression,
                ));
            }
            continue;
        };
        for (parameter, parameter_type) in lambda.parameters.iter().zip(&parameter_types) {
            for reference in parsed
                .name_references
                .iter()
                .filter(|reference| reference.name == parameter.name)
            {
                if parsed.arena.node(lambda.body).is_some_and(|body| {
                    body.span.file() == reference.name_span.file()
                        && body.span.start() <= reference.name_span.start()
                        && reference.name_span.end() <= body.span.end()
                }) {
                    report.replace_expression_type(ExpressionType::new(
                        reference.reference,
                        *parameter_type,
                    ));
                }
            }
        }
        let int_type = PrimitiveTypeIds::module_owned(types).int_id;
        let primitive_report = type_executable_int_operators(
            &parsed.unary_expressions,
            &parsed.binary_expressions,
            &parsed.grouped_expressions,
            report.expression_types(),
            int_type,
        );
        merge_type_check_report(report, primitive_report);
        let body_type = report.expression_type(lambda.body).or_else(|| {
            parsed
                .literal_expressions
                .iter()
                .find(|literal| literal.expression == lambda.body)
                .and_then(|literal| report.expression_type(literal.expression))
        });
        let Some(body_type) = body_type else {
            continue;
        };
        let function_type = types.function(FunctionType::new(parameter_types, body_type));
        report.record_expression_type(ExpressionType::new(lambda.expression, function_type));
    }
}

pub fn type_bind_function_values(
    parsed: &ParseOutput,
    signatures: &[FunctionSignature],
    types: &mut TypeArena,
    report: &mut TypeCheckReport,
) {
    for function in &parsed.function_declarations {
        let Some(signature) = signatures
            .iter()
            .find(|signature| signature.declaration == function.declaration)
        else {
            continue;
        };
        let body_span = function
            .body
            .and_then(|body| parsed.arena.node(body))
            .map(|node| node.span);
        for (index, parameter) in parsed
            .function_parameters
            .iter()
            .filter(|parameter| parameter.function == function.declaration)
            .enumerate()
        {
            let receiver_offset = usize::from(function.owner.is_some() && !function.is_static);
            let Some(parameter_type) = signature
                .parameter_types
                .get(index + receiver_offset)
                .copied()
            else {
                continue;
            };
            for reference in parsed
                .name_references
                .iter()
                .filter(|reference| reference.name == parameter.name)
            {
                if body_span.is_some_and(|span| refinement_span_contains(span, reference.name_span))
                {
                    report.replace_expression_type(ExpressionType::new(
                        reference.reference,
                        parameter_type,
                    ));
                }
            }
        }
    }

    for binding in &parsed.local_binding_names {
        let Some(declaration) = parsed
            .local_declarations
            .iter()
            .find(|declaration| declaration.declaration == binding.binding)
        else {
            continue;
        };
        let Some(local_type) = report
            .declaration_signatures()
            .iter()
            .find(|signature| signature.declaration() == declaration.declaration)
            .map(|signature| signature.ty())
        else {
            continue;
        };
        let declaration_span = parsed.arena.node(binding.binding).map(|node| node.span);
        for reference in parsed
            .name_references
            .iter()
            .filter(|reference| reference.name == binding.name)
        {
            if declaration_span.is_some_and(|span| span.end() <= reference.name_span.start()) {
                report
                    .replace_expression_type(ExpressionType::new(reference.reference, local_type));
            }
        }
    }

    for function in parsed
        .function_declarations
        .iter()
        .filter(|function| function.owner.is_none())
    {
        let Some(signature) = signatures
            .iter()
            .find(|signature| signature.declaration == function.declaration)
        else {
            continue;
        };
        let function_type = types.function(FunctionType::new(
            signature.parameter_types.clone(),
            signature.return_type,
        ));
        for reference in parsed
            .name_references
            .iter()
            .filter(|reference| reference.name == function.name)
        {
            report.replace_expression_type(ExpressionType::new(reference.reference, function_type));
        }
    }
}

pub fn infer_local_types(parsed: &ParseOutput, report: &mut TypeCheckReport) {
    for declaration in &parsed.local_declarations {
        if declaration.annotation.is_some()
            || report
                .declaration_signatures()
                .iter()
                .any(|signature| signature.declaration() == declaration.declaration)
        {
            continue;
        }
        let Some(initializer) = declaration.initializer else {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::InferredInitializerRequired,
                declaration.declaration,
            ));
            continue;
        };
        let Some(ty) = report.expression_type(initializer) else {
            continue;
        };
        if report.expression_type(initializer).is_some_and(|_| {
            parsed.literal_expressions.iter().any(|literal| {
                literal.expression == initializer && literal.kind == ParsedLiteralKind::Null
            })
        }) {
            continue;
        }
        report.record_declaration_signature(DeclarationSignature::new(declaration.declaration, ty));
        report.record_assignment_check(AssignmentCheck::new(declaration.declaration, ty, ty));
        if let Some(binding) = parsed
            .local_binding_names
            .iter()
            .find(|binding| binding.binding == declaration.declaration)
        {
            let declaration_span = parsed.arena.node(binding.binding).map(|node| node.span);
            for reference in parsed
                .name_references
                .iter()
                .filter(|reference| reference.name == binding.name)
            {
                if declaration_span.is_some_and(|span| span.end() <= reference.name_span.start()) {
                    report.replace_expression_type(ExpressionType::new(reference.reference, ty));
                }
            }
        }
    }
}

pub fn diagnose_unresolved_types(parsed: &ParseOutput, report: &mut TypeCheckReport) {
    for declaration in &parsed.local_declarations {
        if declaration.annotation.is_some()
            || report
                .declaration_signatures()
                .iter()
                .any(|signature| signature.declaration() == declaration.declaration)
        {
            continue;
        }
        if let Some(initializer) = declaration.initializer {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::InferredTypeUnavailable,
                initializer,
            ));
        }
    }
}

pub fn validate_inferred_assignments(
    parsed: &ParseOutput,
    report: &mut TypeCheckReport,
    types: &TypeArena,
) {
    for assignment in &parsed.assignment_statements {
        let Some(target_name) = parsed
            .name_references
            .iter()
            .find(|reference| reference.reference == assignment.target)
        else {
            continue;
        };
        let Some(binding) = parsed
            .local_binding_names
            .iter()
            .find(|binding| binding.name == target_name.name)
        else {
            continue;
        };
        let Some(declaration) = parsed
            .local_declarations
            .iter()
            .find(|declaration| declaration.declaration == binding.binding)
        else {
            continue;
        };
        if declaration.annotation.is_some() {
            continue;
        }
        let Some(target_type) = report.expression_type(assignment.target) else {
            continue;
        };
        let Some(value_type) = report.expression_type(assignment.value) else {
            continue;
        };
        if !assignment_compatible(target_type, value_type, types) {
            report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                assignment.value,
                target_type,
                value_type,
            ));
        }
    }
}

pub(crate) fn refinement_span_contains(outer: ByteSpan, inner: ByteSpan) -> bool {
    outer.file() == inner.file() && outer.start() <= inner.start() && inner.end() <= outer.end()
}

pub fn recognize_null_tests(
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

pub(crate) fn is_null_literal(
    expression: AstNodeId,
    literal_expressions: &[ParsedLiteralExpression],
) -> bool {
    literal_expressions
        .iter()
        .any(|literal| literal.expression == expression && literal.kind == ParsedLiteralKind::Null)
}

pub(crate) fn is_name_expression(expression: AstNodeId, arena: &AstArena) -> bool {
    arena
        .node(expression)
        .is_some_and(|node| node.kind == AstNodeKind::NameExpression)
}

pub fn select_eligible_null_tests(
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

pub fn record_branch_refinements(
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

        let Some(region) = refined_branch_region(eligible.null_test(), if_expression) else {
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

pub(crate) fn refined_branch_region(
    null_test: RecognizedNullTest,
    if_expression: &ParsedIfExpression,
) -> Option<AstNodeId> {
    match null_test.refined_branch() {
        NullTestRefinedBranch::Then => Some(if_expression.then_block),
        NullTestRefinedBranch::Else => if_expression.else_block,
    }
}

pub fn record_refined_expression_types(
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
                || !expression_is_inside_refinement_region(
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

pub(crate) fn expression_is_inside_refinement_region(
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

pub fn type_control_flow(
    parsed: &ParseOutput,
    known_expression_types: &[ExpressionType],
    int_type: TypeId,
    bool_type: TypeId,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();
    for expression_type in known_expression_types {
        report.record_expression_type(*expression_type);
    }
    let statement_conditionals: Vec<_> = parsed
        .if_statements
        .iter()
        .map(|statement| statement.expression)
        .collect();
    report.diagnostics.retain(|diagnostic| {
        !(diagnostic.rule() == TypeRuleDiagnostic::IfValueDeferred
            && statement_conditionals.contains(&diagnostic.node()))
    });

    for loop_statement in &parsed.for_statements {
        let Some(binding_span) = parsed.arena.node(loop_statement.body).map(|node| node.span)
        else {
            continue;
        };
        for reference in parsed.name_references.iter().filter(|reference| {
            reference.name == loop_statement.binding_name
                && binding_span.file() == reference.name_span.file()
                && binding_span.start() <= reference.name_span.start()
                && reference.name_span.end() <= binding_span.end()
        }) {
            report.record_expression_type(ExpressionType::new(reference.reference, int_type));
        }
        if loop_statement.array.is_none() {
            for expression in [loop_statement.start, loop_statement.end] {
                if let Some(actual) = report.expression_type(expression)
                    && actual != int_type
                {
                    report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                        expression, int_type, actual,
                    ));
                }
            }
        }
    }

    for if_statement in &parsed.if_statements {
        let Some(if_expression) = parsed
            .if_expressions
            .iter()
            .find(|expression| expression.expression == if_statement.expression)
        else {
            continue;
        };
        if let Some(actual) = report.expression_type(if_expression.condition)
            && actual != bool_type
        {
            report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                if_expression.condition,
                bool_type,
                actual,
            ));
        }
    }

    let int_unary = parsed
        .unary_expressions
        .iter()
        .filter(|expression| report.expression_type(expression.operand) == Some(int_type))
        .cloned()
        .collect::<Vec<_>>();
    let int_binary = parsed
        .binary_expressions
        .iter()
        .filter(|expression| {
            report.expression_type(expression.left) == Some(int_type)
                || report.expression_type(expression.right) == Some(int_type)
        })
        .cloned()
        .collect::<Vec<_>>();
    let int_report = type_executable_int_operators(
        &int_unary,
        &int_binary,
        &parsed.grouped_expressions,
        report.expression_types(),
        int_type,
    );
    merge_type_check_report(&mut report, int_report);
    for binary in &parsed.binary_expressions {
        let Some(left) = report.expression_type(binary.left) else {
            continue;
        };
        let Some(right) = report.expression_type(binary.right) else {
            continue;
        };
        if left == int_type
            && right == int_type
            && matches!(
                binary.operator,
                ParsedBinaryOperator::Equal
                    | ParsedBinaryOperator::NotEqual
                    | ParsedBinaryOperator::Less
                    | ParsedBinaryOperator::Greater
                    | ParsedBinaryOperator::LessEqual
                    | ParsedBinaryOperator::GreaterEqual
            )
        {
            report.record_expression_type(ExpressionType::new(binary.expression, bool_type));
        }
    }
    report
}

pub fn type_array_iterations(
    parsed: &ParseOutput,
    known_expression_types: &[ExpressionType],
    types: &TypeArena,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();
    for expression_type in known_expression_types {
        report.record_expression_type(*expression_type);
    }
    for loop_statement in parsed
        .for_statements
        .iter()
        .filter(|loop_| loop_.array.is_some())
    {
        let Some(array_expression) = loop_statement.array else {
            continue;
        };
        let Some(array_type) = report.expression_type(array_expression) else {
            continue;
        };
        let element_type = match types.get(array_type).map(|record| record.kind()) {
            Some(TypeKind::Array(array)) => Some(array.element()),
            Some(TypeKind::DynamicArray(array)) => Some(array.element()),
            _ => None,
        };
        let Some(element_type) = element_type else {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ArrayIterationTargetInvalid,
                array_expression,
            ));
            continue;
        };
        let Some(body_span) = parsed.arena.node(loop_statement.body).map(|node| node.span) else {
            continue;
        };
        if let Some(member) = parsed.member_expressions.iter().find(|member| {
            body_span.start() <= member.span.start()
                && member.span.end() <= body_span.end()
                && matches!(member.name.as_str(), "add" | "remove")
        }) && parsed.call_expressions.iter().any(|call| {
            call.callee == member.expression
                && parsed.arena.node(call.expression).is_some_and(|node| {
                    body_span.start() <= node.span.start() && node.span.end() <= body_span.end()
                })
        }) {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ArrayIterationStructuralMutation,
                member.expression,
            ));
        }
        for reference in parsed.name_references.iter().filter(|reference| {
            reference.name == loop_statement.binding_name
                && body_span.file() == reference.name_span.file()
                && body_span.start() <= reference.name_span.start()
                && reference.name_span.end() <= body_span.end()
        }) {
            report.replace_expression_type(ExpressionType::new(reference.reference, element_type));
        }
    }
    report
}

pub fn type_value_conditionals(
    parsed: &ParseOutput,
    known_expression_types: &[ExpressionType],
    type_arena: &TypeArena,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();
    for expression_type in known_expression_types {
        report.record_expression_type(*expression_type);
    }
    let statement_conditionals = parsed
        .if_statements
        .iter()
        .map(|statement| statement.expression)
        .collect::<Vec<_>>();
    let bool_type = type_arena
        .records()
        .iter()
        .find(|record| record.kind() == &TypeKind::Primitive(PrimitiveType::Bool))
        .map(|record| record.id());
    for conditional in &parsed.if_expressions {
        if statement_conditionals.contains(&conditional.expression) {
            continue;
        }
        let Some(bool_type) = bool_type else {
            continue;
        };
        if report.expression_type(conditional.condition) != Some(bool_type) {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ConditionalConditionNotBool,
                conditional.condition,
            ));
        }
        let Some(else_block) = conditional.else_block else {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ConditionalElseRequired,
                conditional.expression,
            ));
            continue;
        };
        let then_value = conditional_branch_value(parsed, conditional.then_block);
        let else_value = conditional_branch_value(parsed, else_block);
        let (Some(then_value), Some(else_value)) = (then_value, else_value) else {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ConditionalMissingBranchValue,
                conditional.expression,
            ));
            continue;
        };
        let (Some(then_type), Some(else_type)) = (
            report.expression_type(then_value),
            report.expression_type(else_value),
        ) else {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ConditionalMissingBranchValue,
                conditional.expression,
            ));
            continue;
        };
        if then_type != else_type {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ConditionalBranchTypeMismatch,
                conditional.expression,
            ));
            continue;
        }
        report.record_expression_type(ExpressionType::new(conditional.expression, then_type));
    }
    report
}

pub fn type_enum_whens(
    parsed: &ParseOutput,
    known_expression_types: &[ExpressionType],
    classes: &ClassTypeReport,
    type_arena: &mut TypeArena,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();
    for expression_type in known_expression_types {
        report.record_expression_type(*expression_type);
    }
    for when in &parsed.when_expressions {
        let primitives = PrimitiveTypeIds::module_owned(type_arena);
        let Some(subject_type) = report.expression_type(when.subject) else {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::WhenSubjectInvalid,
                when.subject,
            ));
            continue;
        };
        if let Some(element_type) =
            type_arena
                .get(subject_type)
                .and_then(|record| match record.kind() {
                    TypeKind::ChannelResult(result) => Some(result.element()),
                    _ => None,
                })
        {
            let mut covered = std::collections::HashSet::new();
            let mut arm_type = None;
            for arm_id in &when.arms {
                let Some(arm) = parsed.match_arms.iter().find(|arm| arm.arm == *arm_id) else {
                    continue;
                };
                let Some(pattern) = parsed
                    .qualified_case_patterns
                    .iter()
                    .find(|pattern| pattern.pattern == arm.pattern)
                else {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::WhenPatternInvalid,
                        arm.pattern,
                    ));
                    continue;
                };
                let tag = match (pattern.enum_name.as_str(), pattern.variant_name.as_str()) {
                    ("ChannelResult", "Message") if pattern.payloads.len() == 1 => Some(0_i64),
                    ("ChannelResult", "Closed") if pattern.payloads.is_empty() => Some(1_i64),
                    _ => None,
                };
                let Some(tag) = tag else {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::WhenPatternInvalid,
                        arm.pattern,
                    ));
                    continue;
                };
                if !covered.insert(tag) {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::WhenDuplicatePattern,
                        arm.pattern,
                    ));
                }
                if tag == 0 {
                    let binding = parsed
                        .pattern_bindings
                        .iter()
                        .find(|binding| pattern.payloads.contains(&binding.pattern));
                    if let Some(binding) = binding {
                        for reference in parsed
                            .name_references
                            .iter()
                            .filter(|reference| reference.name == binding.name)
                        {
                            report.replace_expression_type(ExpressionType::new(
                                reference.reference,
                                element_type,
                            ));
                        }
                    }
                }
                type_string_operations(parsed, &mut report, type_arena, &parsed.array_types);
                let Some(body_type) = report.expression_type(arm.body) else {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::WhenMissingArmValue,
                        arm.body,
                    ));
                    continue;
                };
                if let Some(expected) = arm_type {
                    if expected != body_type {
                        report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                            TypeRuleDiagnostic::WhenArmTypeMismatch,
                            arm.body,
                        ));
                    }
                } else {
                    arm_type = Some(body_type);
                }
            }
            if covered.len() != 2 {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::WhenNonExhaustive,
                    when.expression,
                ));
            }
            if let Some(arm_type) = arm_type {
                report.record_expression_type(ExpressionType::new(when.expression, arm_type));
            }
            continue;
        }
        let enum_declaration = match type_arena.get(subject_type).map(|record| record.kind()) {
            Some(TypeKind::Nominal(identity)) => Some(identity.declaration()),
            Some(TypeKind::GenericInstance(identity)) => Some(identity.declaration().declaration()),
            _ => None,
        };
        let Some(enum_class) = classes.classes().iter().find(|class| {
            Some(class.declaration()) == enum_declaration
                && parsed
                    .enum_variants
                    .iter()
                    .any(|variant| variant.enum_declaration == class.declaration())
        }) else {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::WhenSubjectInvalid,
                when.subject,
            ));
            continue;
        };
        let variants = parsed
            .enum_variants
            .iter()
            .filter(|variant| variant.enum_declaration == enum_class.declaration())
            .collect::<Vec<_>>();
        let generic_bindings = generic_enum_bindings(parsed, subject_type, enum_class, type_arena);
        let mut covered = std::collections::HashSet::new();
        let mut wildcard = false;
        let mut arm_type = None;
        for arm_id in &when.arms {
            let Some(arm) = parsed.match_arms.iter().find(|arm| arm.arm == *arm_id) else {
                continue;
            };
            let pattern_variant = parsed
                .qualified_case_patterns
                .iter()
                .find(|pattern| pattern.pattern == arm.pattern)
                .and_then(|pattern| {
                    variants.iter().find(|variant| {
                        variant.name == pattern.variant_name
                            && enum_class.name() == pattern.enum_name
                    })
                });
            if parsed
                .arena
                .node(arm.pattern)
                .is_some_and(|node| node.kind == crate::ast::AstNodeKind::WildcardPattern)
            {
                wildcard = true;
            } else if let Some(variant) = pattern_variant {
                if !covered.insert(variant.variant) {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::WhenDuplicatePattern,
                        arm.pattern,
                    ));
                }
                if let Some(pattern) = parsed
                    .qualified_case_patterns
                    .iter()
                    .find(|pattern| pattern.pattern == arm.pattern)
                {
                    if variant.arguments.len() != pattern.payloads.len() {
                        report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                            TypeRuleDiagnostic::WhenPatternInvalid,
                            arm.pattern,
                        ));
                    }
                    let fields = parsed
                        .class_declarations
                        .iter()
                        .find(|declaration| declaration.declaration == enum_class.declaration())
                        .map(|declaration| {
                            declaration
                                .constructor_parameters
                                .iter()
                                .filter(|parameter| parameter.field)
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    for (index, binding) in pattern.payloads.iter().enumerate() {
                        let Some(binding_name) = parsed
                            .pattern_bindings
                            .iter()
                            .find(|candidate| candidate.pattern == *binding)
                        else {
                            continue;
                        };
                        let field_type = variant
                            .arguments
                            .get(index)
                            .and_then(|argument| {
                                resolve_enum_payload_type(
                                    *argument,
                                    parsed,
                                    &primitives,
                                    classes.classes(),
                                    type_arena,
                                    &generic_bindings,
                                )
                            })
                            .or_else(|| {
                                fields.get(index).and_then(|field| {
                                    resolve_annotation_type_with_generics(
                                        field.annotation,
                                        &parsed.type_name_references,
                                        &parsed.array_types,
                                        &primitives,
                                        classes.classes(),
                                        type_arena,
                                        &generic_bindings,
                                    )
                                })
                            });
                        let Some(field_type) = field_type else {
                            continue;
                        };
                        for reference in parsed
                            .name_references
                            .iter()
                            .filter(|reference| reference.name == binding_name.name)
                        {
                            report.replace_expression_type(ExpressionType::new(
                                reference.reference,
                                field_type,
                            ));
                        }
                    }
                }
            } else {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::WhenPatternInvalid,
                    arm.pattern,
                ));
            }
            let Some(body_type) = report.expression_type(arm.body) else {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::WhenMissingArmValue,
                    arm.body,
                ));
                continue;
            };
            if let Some(expected) = arm_type {
                if expected != body_type {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::WhenArmTypeMismatch,
                        arm.body,
                    ));
                }
            } else {
                arm_type = Some(body_type);
            }
        }
        if !wildcard && covered.len() != variants.len() {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::WhenNonExhaustive,
                when.expression,
            ));
        }
        if let Some(arm_type) = arm_type {
            report.record_expression_type(ExpressionType::new(when.expression, arm_type));
        }
    }
    report
}

pub fn apply_enum_constructor_facts(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    report: &mut TypeCheckReport,
    types: &mut TypeArena,
) {
    let primitives = PrimitiveTypeIds::module_owned(types);
    for call in &parsed.call_expressions {
        let Some(member) = parsed
            .member_expressions
            .iter()
            .find(|member| member.expression == call.callee)
        else {
            continue;
        };
        let Some(receiver_name) = parsed
            .name_references
            .iter()
            .find(|reference| reference.reference == member.receiver)
            .map(|reference| reference.name.as_str())
        else {
            continue;
        };
        let Some(class) = classes.classes().iter().find(|class| {
            class.name() == receiver_name
                && parsed.enum_variants.iter().any(|variant| {
                    variant.enum_declaration == class.declaration() && variant.name == member.name
                })
        }) else {
            continue;
        };
        let Some(variant) = parsed.enum_variants.iter().find(|variant| {
            variant.enum_declaration == class.declaration() && variant.name == member.name
        }) else {
            continue;
        };
        if variant.arguments.len() == call.arguments.len() {
            let generic_parameters = parsed
                .generic_parameters
                .iter()
                .filter(|parameter| parameter.owner == Some(class.declaration()))
                .collect::<Vec<_>>();
            let mut generic_arguments = vec![None; generic_parameters.len()];
            for (annotation, argument) in variant.arguments.iter().zip(&call.arguments) {
                let Some(actual) = report.expression_type(*argument) else {
                    continue;
                };
                if let Some(reference) = parsed
                    .type_name_references
                    .iter()
                    .find(|reference| reference.reference == *annotation)
                    && let Some(index) = generic_parameters
                        .iter()
                        .position(|parameter| parameter.name == reference.name)
                {
                    generic_arguments[index] = Some(actual);
                } else if let Some(reference) = parsed
                    .name_references
                    .iter()
                    .find(|reference| reference.reference == *annotation)
                    && let Some(index) = generic_parameters
                        .iter()
                        .position(|parameter| parameter.name == reference.name)
                {
                    generic_arguments[index] = Some(actual);
                } else if let Some(array) = parsed
                    .array_types
                    .iter()
                    .find(|array| array.array == *annotation && array.dynamic)
                    && let Some(element) = parsed
                        .type_name_references
                        .iter()
                        .find(|element| element.reference == array.element_type)
                    && let Some(index) = generic_parameters
                        .iter()
                        .position(|parameter| parameter.name == element.name)
                    && let Some(TypeKind::DynamicArray(array)) =
                        types.get(actual).map(|record| record.kind())
                {
                    generic_arguments[index] = Some(array.element());
                }
            }
            if generic_arguments.iter().any(Option::is_none)
                && let Some(expected_annotation) = parsed
                    .local_declarations
                    .iter()
                    .find(|local| local.initializer == Some(call.expression))
                    .and_then(|local| local.annotation)
                && let Some(expected_type) = resolve_annotation_type(
                    expected_annotation,
                    &parsed.type_name_references,
                    &parsed.array_types,
                    &primitives,
                    classes.classes(),
                    types,
                )
                && let Some(TypeKind::GenericInstance(identity)) =
                    types.get(expected_type).map(|record| record.kind())
            {
                for (slot, argument) in generic_arguments.iter_mut().zip(identity.arguments()) {
                    if slot.is_none() {
                        *slot = Some(*argument);
                    }
                }
            }
            if !generic_parameters.is_empty() && generic_arguments.iter().all(Option::is_some) {
                let identity = match types.get(class.type_id).map(|record| record.kind()) {
                    Some(TypeKind::Nominal(identity)) => identity.clone(),
                    _ => continue,
                };
                let arguments = generic_arguments.into_iter().flatten().collect::<Vec<_>>();
                let result_type = if arguments.is_empty() {
                    class.type_id
                } else {
                    types.generic_instance(GenericTypeIdentity::new(identity, arguments))
                };
                report.replace_expression_type(ExpressionType::new(call.expression, result_type));
                report.replace_expression_type(ExpressionType::new(member.expression, result_type));
                report.retain_diagnostics(|diagnostic| {
                    (diagnostic.node() != call.expression && diagnostic.node() != member.expression)
                        || !matches!(
                            diagnostic.rule(),
                            TypeRuleDiagnostic::DirectCallDeferred
                                | TypeRuleDiagnostic::MemberExpressionDeferred
                        )
                });
            }
        }
        let Some(declaration) = parsed
            .class_declarations
            .iter()
            .find(|declaration| declaration.declaration == class.declaration())
        else {
            continue;
        };
        let fields = declaration
            .constructor_parameters
            .iter()
            .filter(|parameter| parameter.field)
            .collect::<Vec<_>>();
        if fields.len() != call.arguments.len() {
            continue;
        }
        let valid = fields.iter().zip(&call.arguments).all(|(field, argument)| {
            let Some(expected) = resolve_annotation_type(
                field.annotation,
                &parsed.type_name_references,
                &parsed.array_types,
                &primitives,
                classes.classes(),
                types,
            ) else {
                return false;
            };
            report.expression_type(*argument) == Some(expected)
        });
        if valid {
            report.replace_expression_type(ExpressionType::new(call.expression, class.type_id));
            report.retain_diagnostics(|diagnostic| {
                (diagnostic.node() != call.expression && diagnostic.node() != member.expression)
                    || !matches!(
                        diagnostic.rule(),
                        TypeRuleDiagnostic::DirectCallDeferred
                            | TypeRuleDiagnostic::MemberExpressionDeferred
                    )
            });
        }
    }
}

pub fn apply_enum_function_facts(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    report: &mut TypeCheckReport,
    types: &mut TypeArena,
) {
    let primitives = PrimitiveTypeIds::module_owned(types);
    for call in &parsed.call_expressions {
        let Some(member) = parsed
            .member_expressions
            .iter()
            .find(|member| member.expression == call.callee)
        else {
            continue;
        };
        let Some(receiver_name) = parsed
            .name_references
            .iter()
            .find(|reference| reference.reference == member.receiver)
            .map(|reference| reference.name.as_str())
        else {
            continue;
        };
        let Some(class) = classes.classes().iter().find(|class| {
            class.name() == receiver_name
                && parsed
                    .enum_variants
                    .iter()
                    .any(|variant| variant.enum_declaration == class.declaration())
        }) else {
            continue;
        };
        let Some(function) = parsed.function_declarations.iter().find(|function| {
            function.owner == Some(class.declaration())
                && function.name == member.name
                && function.is_static
        }) else {
            continue;
        };
        let parameter_types = parsed
            .function_parameters
            .iter()
            .filter(|parameter| parameter.function == function.declaration)
            .map(|parameter| {
                resolve_annotation_type(
                    parameter.annotation,
                    &parsed.type_name_references,
                    &parsed.array_types,
                    &primitives,
                    classes.classes(),
                    types,
                )
            })
            .collect::<Option<Vec<_>>>();
        let Some(parameter_types) = parameter_types else {
            continue;
        };
        if parameter_types.len() != call.arguments.len()
            || !parameter_types
                .iter()
                .zip(&call.arguments)
                .all(|(expected, argument)| report.expression_type(*argument) == Some(*expected))
        {
            continue;
        }
        let Some(return_annotation) = function.return_annotation else {
            continue;
        };
        let Some(return_type) = resolve_annotation_type(
            return_annotation,
            &parsed.type_name_references,
            &parsed.array_types,
            &primitives,
            classes.classes(),
            types,
        ) else {
            continue;
        };
        report.replace_expression_type(ExpressionType::new(call.expression, return_type));
        report.retain_diagnostics(|diagnostic| {
            (diagnostic.node() != call.expression && diagnostic.node() != member.expression)
                || !matches!(
                    diagnostic.rule(),
                    TypeRuleDiagnostic::DirectCallDeferred
                        | TypeRuleDiagnostic::MemberExpressionDeferred
                )
        });
    }
}

pub fn apply_value_conditional_results(target: &mut TypeCheckReport, source: &TypeCheckReport) {
    merge_type_check_report(target, source.clone());
}

pub(crate) fn conditional_branch_value(
    parsed: &ParseOutput,
    block: AstNodeId,
) -> Option<AstNodeId> {
    let block_span = parsed.arena.node(block)?.span;
    parsed
        .executable_body_statements
        .iter()
        .filter(|statement| {
            block_span.file() == statement.span.file()
                && block_span.start() <= statement.span.start()
                && statement.span.end() <= block_span.end()
                && parsed
                    .arena
                    .node(statement.statement)
                    .is_some_and(|node| node.kind == AstNodeKind::ExpressionStatement)
        })
        .filter_map(|statement| {
            parsed
                .arena
                .nodes()
                .iter()
                .filter(|node| {
                    node.span.file() == statement.span.file()
                        && node.span.start() == statement.span.start()
                        && node.span.end() <= statement.span.end()
                        && !matches!(
                            node.kind,
                            AstNodeKind::ExpressionStatement | AstNodeKind::Block
                        )
                })
                .max_by_key(|node| node.span.end())
                .map(|node| node.id)
        })
        .max_by_key(|expression| {
            parsed
                .arena
                .node(*expression)
                .map_or(0, |node| node.span.end())
        })
}

pub fn apply_control_flow_results(
    target: &mut TypeCheckReport,
    parsed: &ParseOutput,
    source: &TypeCheckReport,
) {
    let statement_conditionals: Vec<_> = parsed
        .if_statements
        .iter()
        .map(|statement| statement.expression)
        .collect();
    target.diagnostics.retain(|diagnostic| {
        !(diagnostic.rule() == TypeRuleDiagnostic::IfValueDeferred
            && statement_conditionals.contains(&diagnostic.node()))
    });
    merge_type_check_report(target, source.clone());
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

pub fn type_assignment_statements_with_flow(
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

pub fn type_local_declaration_initializers(
    declarations: &[ParsedLocalDeclaration],
    declaration_signatures: &[DeclarationSignature],
    known_expression_types: &[ExpressionType],
    flow_report: &TypeCheckReport,
    resolved_local_bindings: &[ResolvedLocalBinding],
    ast_arena: &AstArena,
    type_arena: &TypeArena,
) -> TypeCheckReport {
    type_local_declaration_initializers_with_region_exit_invalidations(
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
pub fn type_region_exit_refinement_invalidations(
    declarations: &[ParsedLocalDeclaration],
    declaration_signatures: &[DeclarationSignature],
    known_expression_types: &[ExpressionType],
    flow_report: &TypeCheckReport,
    resolved_local_bindings: &[ResolvedLocalBinding],
    if_expressions: &[ParsedIfExpression],
    ast_arena: &AstArena,
    type_arena: &TypeArena,
) -> TypeCheckReport {
    type_local_declaration_initializers_with_region_exit_invalidations(
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
pub(crate) fn type_local_declaration_initializers_with_region_exit_invalidations(
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
        let refined_initializer_type = valid_refined_value_type(
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
            && exact_nullable_name_initializer(
                initializer,
                original_initializer_type,
                target_type,
                resolved_local_bindings,
                ast_arena,
                type_arena,
            )
        {
            let diagnostic = if if_expressions.is_some_and(|if_expressions| {
                exact_region_exit_nullable_name_initializer(
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

pub(crate) fn type_assignment_statements_with_refinements(
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
        let refined_value_type =
            valid_refined_value_type(assignment.value, original_value_type, flow_context, arena);
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

pub(crate) fn valid_refined_value_type(
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
        || !expression_is_inside_refinement_region(ast_arena, expression, provenance.region())
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

pub(crate) fn exact_nullable_name_initializer(
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
pub(crate) fn exact_region_exit_nullable_name_initializer(
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
            && immediate_containing_block(ast_arena, if_expression.span)
                == immediate_containing_block(ast_arena, initializer_span)
    })
}

pub(crate) fn immediate_containing_block(
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

pub(crate) fn nullable_base_type(ty: TypeId, arena: &TypeArena) -> Option<TypeId> {
    let record = arena.get(ty)?;
    let TypeKind::Nullable(nullable) = record.kind() else {
        return None;
    };
    Some(nullable.base())
}

pub(crate) fn expression_type_in(
    expression_types: &[ExpressionType],
    expression: AstNodeId,
) -> Option<TypeId> {
    expression_types
        .iter()
        .find(|entry| entry.expression() == expression)
        .map(|entry| entry.ty())
}

pub(crate) fn assignment_compatible(target: TypeId, value: TypeId, arena: &TypeArena) -> bool {
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
        (TypeKind::DynamicArray(target), TypeKind::DynamicArray(value)) => {
            target.element() == value.element()
                || matches!(
                    arena.get(target.element()).map(|record| record.kind()),
                    Some(TypeKind::GenericParameter(_))
                )
        }
        (_, TypeKind::GenericParameter(_)) => true,
        _ => false,
    }
}

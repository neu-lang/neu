use crate::{
    ast::AstNodeKind,
    module::{FunctionSymbolIdentity, ModuleName, PackageNamespace},
    name_resolution::LocalBindingKind,
    ownership_effects::OwnershipEffectContract,
    parser::{ParseOutput, ParsedBinaryOperator, ParsedLiteralKind},
    source::ByteSpan,
    type_check::{ClassTypeReport, ExpressionType, FunctionSignature, ResolvedCallDeclaration},
    types::TypeId,
};

macro_rules! hir_id {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(usize);

        impl $name {
            pub fn from_raw(raw: usize) -> Self {
                Self(raw)
            }
            pub fn index(self) -> usize {
                self.0
            }
        }
    };
}

hir_id!(HirFunctionId);
hir_id!(HirParameterId);
hir_id!(HirLocalId);
hir_id!(HirExpressionId);

pub struct CheckedHirSource<'a> {
    module: ModuleName,
    package: PackageNamespace,
    parsed: &'a ParseOutput,
    signatures: &'a [FunctionSignature],
    expression_types: &'a [ExpressionType],
    clean: bool,
    byte_type: Option<TypeId>,
    effect_contracts: Option<&'a [OwnershipEffectContract]>,
    class_types: Option<&'a ClassTypeReport>,
    call_targets: Option<&'a [ResolvedCallDeclaration]>,
}
impl<'a> CheckedHirSource<'a> {
    pub fn new(
        module: ModuleName,
        package: PackageNamespace,
        parsed: &'a ParseOutput,
        signatures: &'a [FunctionSignature],
        expression_types: &'a [ExpressionType],
        clean: bool,
    ) -> Self {
        Self {
            module,
            package,
            parsed,
            signatures,
            expression_types,
            clean,
            byte_type: None,
            effect_contracts: None,
            class_types: None,
            call_targets: None,
        }
    }

    pub fn with_byte_type(mut self, byte_type: TypeId) -> Self {
        self.byte_type = Some(byte_type);
        self
    }

    pub fn with_effect_contracts(
        mut self,
        effect_contracts: &'a [OwnershipEffectContract],
    ) -> Self {
        self.effect_contracts = Some(effect_contracts);
        self
    }

    pub fn with_class_types(mut self, class_types: &'a ClassTypeReport) -> Self {
        self.class_types = Some(class_types);
        self
    }

    pub fn with_call_targets(mut self, call_targets: &'a [ResolvedCallDeclaration]) -> Self {
        self.call_targets = Some(call_targets);
        self
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HirLoweringError {
    FrontendNotClean,
    MissingType,
    UnsupportedExpression,
}

pub fn lower_checked_hir_source(
    source: CheckedHirSource<'_>,
) -> Result<HirModule, HirLoweringError> {
    if !source.clean {
        return Err(HirLoweringError::FrontendNotClean);
    }
    let mut functions = Vec::new();
    for function in &source.parsed.function_declarations {
        if function.body.is_none() {
            continue;
        }
        let Some(signature) = source
            .signatures
            .iter()
            .find(|signature| signature.declaration() == function.declaration)
        else {
            continue;
        };
        let span = source
            .parsed
            .arena
            .node(function.declaration)
            .ok_or(HirLoweringError::UnsupportedExpression)?
            .span;
        let symbol_name = function
            .owner
            .and_then(|owner| {
                source
                    .parsed
                    .class_declarations
                    .iter()
                    .find(|class| class.declaration == owner)
                    .map(|class| format!("{}::{}", class.name, function.name))
            })
            .unwrap_or_else(|| function.name.clone());
        let symbol_identity = source
            .parsed
            .declaration_names
            .iter()
            .find(|name| name.declaration == function.declaration)
            .map(|name| {
                FunctionSymbolIdentity::new(
                    source.module.clone(),
                    source.package.clone(),
                    if function.owner.is_some() {
                        symbol_name.clone()
                    } else {
                        name.name.clone()
                    },
                )
            })
            .unwrap_or_else(|| {
                FunctionSymbolIdentity::new(
                    source.module.clone(),
                    source.package.clone(),
                    symbol_name.clone(),
                )
            });
        let id = HirFunctionId::from_raw(functions.len());
        let mut expressions = Vec::new();
        let mut returns = Vec::new();
        let mut locals = Vec::new();
        let mut local_bindings = Vec::new();
        for local in source.parsed.local_declarations.iter().filter(|local| {
            source
                .parsed
                .executable_body_statements
                .iter()
                .any(|statement| {
                    statement.function == function.declaration
                        && statement.statement == local.declaration
                })
        }) {
            let Some(initializer) = local.initializer else {
                continue;
            };
            let ty = source
                .expression_types
                .iter()
                .find(|typed| typed.expression() == initializer)
                .map(|typed| typed.ty())
                .ok_or(HirLoweringError::MissingType)?;
            let span = source
                .parsed
                .arena
                .node(local.declaration)
                .ok_or(HirLoweringError::UnsupportedExpression)?
                .span;
            let mutable = source
                .parsed
                .local_binding_names
                .iter()
                .find(|binding| binding.binding == local.declaration)
                .is_some_and(|binding| binding.kind == LocalBindingKind::Var);
            let local_id = HirLocalId::from_raw(locals.len());
            locals.push(HirLocal::new(local_id, span, ty, mutable));
            local_bindings.push((local.declaration, local_id));
        }
        for loop_ in source
            .parsed
            .for_statements
            .iter()
            .filter(|loop_| loop_.function == function.declaration)
        {
            let ty = source
                .expression_types
                .iter()
                .find(|typed| typed.expression() == loop_.start)
                .map(|typed| typed.ty())
                .ok_or(HirLoweringError::MissingType)?;
            let span = source
                .parsed
                .for_statements
                .iter()
                .find(|candidate| candidate.statement == loop_.statement)
                .map(|candidate| candidate.span)
                .unwrap_or(loop_.span);
            let local_id = HirLocalId::from_raw(locals.len());
            locals.push(HirLocal::new(local_id, span, ty, false));
            local_bindings.push((loop_.binding, local_id));
        }
        let mut parameters = Vec::new();
        if function.owner.is_some() && !function.is_static {
            let parameter_span = source
                .parsed
                .arena
                .node(function.declaration)
                .ok_or(HirLoweringError::UnsupportedExpression)?
                .span;
            let ty = *signature
                .parameter_types()
                .first()
                .ok_or(HirLoweringError::UnsupportedExpression)?;
            parameters.push(HirParameter::new(
                HirParameterId::from_raw(0),
                parameter_span,
                ty,
            ));
        }
        parameters.extend(
            source
                .parsed
                .function_parameters
                .iter()
                .filter(|parameter| parameter.function == function.declaration)
                .enumerate()
                .map(|(index, parameter)| {
                    let parameter_span = source
                        .parsed
                        .arena
                        .node(parameter.parameter)
                        .ok_or(HirLoweringError::UnsupportedExpression)?
                        .span;
                    let ty = *signature
                        .parameter_types()
                        .get(index + usize::from(function.owner.is_some() && !function.is_static))
                        .ok_or(HirLoweringError::UnsupportedExpression)?;
                    Ok(HirParameter::new(
                        HirParameterId::from_raw(
                            index + usize::from(function.owner.is_some() && !function.is_static),
                        ),
                        parameter_span,
                        ty,
                    ))
                })
                .collect::<Result<Vec<_>, HirLoweringError>>()?,
        );
        let mut assignments = Vec::new();
        for statement in source
            .parsed
            .executable_body_statements
            .iter()
            .filter(|statement| statement.function == function.declaration)
        {
            if let Some(local) = source
                .parsed
                .local_declarations
                .iter()
                .find(|local| local.declaration == statement.statement)
            {
                let value = local
                    .initializer
                    .ok_or(HirLoweringError::UnsupportedExpression)?;
                let expression = lower_expression(
                    &source,
                    function.declaration,
                    value,
                    &local_bindings,
                    &mut expressions,
                )?;
                let local_id = local_bindings
                    .iter()
                    .find(|(binding, _)| *binding == local.declaration)
                    .map(|(_, local_id)| *local_id)
                    .ok_or(HirLoweringError::UnsupportedExpression)?;
                locals
                    .iter_mut()
                    .find(|local| local.id() == local_id)
                    .ok_or(HirLoweringError::UnsupportedExpression)?
                    .set_initializer(expression);
                continue;
            }
            if let Some(assignment) = source
                .parsed
                .assignment_statements
                .iter()
                .find(|assignment| assignment.statement == statement.statement)
            {
                if let Some(member) = source
                    .parsed
                    .member_expressions
                    .iter()
                    .find(|member| member.expression == assignment.target)
                {
                    let receiver = lower_expression(
                        &source,
                        function.declaration,
                        member.receiver,
                        &local_bindings,
                        &mut expressions,
                    )?;
                    let value = lower_expression(
                        &source,
                        function.declaration,
                        assignment.value,
                        &local_bindings,
                        &mut expressions,
                    )?;
                    assignments.push(HirAssignment::field_assignment(
                        statement.span,
                        receiver,
                        field_index_for_receiver(&source, member.receiver, &member.name),
                        value,
                    ));
                    continue;
                }
                let (target, index) = if let Some(index) = source
                    .parsed
                    .index_expressions
                    .iter()
                    .find(|index| index.expression == assignment.target)
                {
                    let target = local_binding_id(
                        &source,
                        function.declaration,
                        index.array,
                        &local_bindings,
                    )
                    .ok_or(HirLoweringError::UnsupportedExpression)?;
                    let index_value = lower_expression(
                        &source,
                        function.declaration,
                        index.index,
                        &local_bindings,
                        &mut expressions,
                    )?;
                    (target, Some(index_value))
                } else {
                    (
                        local_binding_id(
                            &source,
                            function.declaration,
                            assignment.target,
                            &local_bindings,
                        )
                        .ok_or(HirLoweringError::UnsupportedExpression)?,
                        None,
                    )
                };
                let value = lower_expression(
                    &source,
                    function.declaration,
                    assignment.value,
                    &local_bindings,
                    &mut expressions,
                )?;
                assignments.push(match index {
                    Some(index) => HirAssignment::indexed(statement.span, target, index, value),
                    None => HirAssignment::new(statement.span, target, value),
                });
                continue;
            }
            if let Some(returned) = source
                .parsed
                .return_statements
                .iter()
                .find(|returned| returned.statement == statement.statement)
            {
                let value = returned
                    .value
                    .ok_or(HirLoweringError::UnsupportedExpression)?;
                let expression = lower_expression(
                    &source,
                    function.declaration,
                    value,
                    &local_bindings,
                    &mut expressions,
                )?;
                returns.push(HirReturn::new(statement.span, expression));
                continue;
            }
            if source
                .parsed
                .arena
                .node(statement.statement)
                .is_some_and(|node| {
                    matches!(
                        node.kind,
                        AstNodeKind::ExpressionStatement
                            | AstNodeKind::CallExpression
                            | AstNodeKind::MemberExpression
                    )
                })
            {
                let Some(expression) = source
                    .parsed
                    .call_expressions
                    .iter()
                    .find(|call| {
                        source
                            .parsed
                            .arena
                            .node(call.expression)
                            .is_some_and(|node| node.span.start() == statement.span.start())
                    })
                    .map(|call| call.expression)
                    .or_else(|| {
                        source
                            .parsed
                            .member_expressions
                            .iter()
                            .find(|member| member.span.start() == statement.span.start())
                            .map(|member| member.expression)
                    })
                    .or_else(|| {
                        source
                            .parsed
                            .when_expressions
                            .iter()
                            .find(|when| when.span.start() == statement.span.start())
                            .map(|when| when.expression)
                    })
                else {
                    continue;
                };
                lower_expression(
                    &source,
                    function.declaration,
                    expression,
                    &local_bindings,
                    &mut expressions,
                )?;
            }
        }
        let has_control_flow = source
            .parsed
            .if_statements
            .iter()
            .any(|statement| statement.function == function.declaration)
            || source
                .parsed
                .for_statements
                .iter()
                .any(|statement| statement.function == function.declaration)
            || source
                .parsed
                .loop_control_statements
                .iter()
                .any(|statement| statement.function == function.declaration)
            || source.parsed.when_expressions.iter().any(|when| {
                function
                    .body
                    .and_then(|body| source.parsed.arena.node(body))
                    .is_some_and(|body| {
                        body.span.start() <= when.span.start() && when.span.end() <= body.span.end()
                    })
            });
        let control_flow = if has_control_flow {
            if let Some(body) = function.body {
                lower_control_flow_block(
                    &source,
                    function.declaration,
                    body,
                    &local_bindings,
                    &mut expressions,
                )?
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        let declaration = function.declaration;
        let hir_function = HirFunction::new(
            id,
            source.module.clone(),
            source.package.clone(),
            span,
            declaration_is_main(source.parsed, function.declaration),
            signature.return_type(),
            parameters,
            locals,
            expressions,
            returns,
            HirSafetyFacts::executable_subset_checked(),
            vec![],
        )
        .with_assignments(assignments)
        .with_control_flow(control_flow)
        .with_symbol_identity(symbol_identity);
        let function = match source.effect_contracts.and_then(|contracts| {
            contracts
                .iter()
                .find(|contract| contract.function() == declaration)
        }) {
            Some(contract) => hir_function.with_effect_contract(contract.clone()),
            None => hir_function,
        };
        functions.push(function);
    }
    Ok(HirModule::new(source.module, functions))
}

fn lower_expression(
    source: &CheckedHirSource<'_>,
    function_declaration: crate::ast::AstNodeId,
    expression: crate::ast::AstNodeId,
    local_bindings: &[(crate::ast::AstNodeId, HirLocalId)],
    output: &mut Vec<HirExpression>,
) -> Result<HirExpressionId, HirLoweringError> {
    let Some(ty) = source
        .expression_types
        .iter()
        .find(|typed| typed.expression() == expression)
        .map(|typed| typed.ty())
    else {
        return Err(HirLoweringError::MissingType);
    };
    let span = source
        .parsed
        .arena
        .node(expression)
        .ok_or(HirLoweringError::UnsupportedExpression)?
        .span;
    if let Some(existing) = output.iter().find(|candidate| candidate.span() == span) {
        return Ok(existing.id());
    }
    let id = HirExpressionId::from_raw(output.len());
    if let Some(if_expression) = source
        .parsed
        .if_expressions
        .iter()
        .find(|if_expression| if_expression.expression == expression)
        && let Some(else_block) = if_expression.else_block
        && !source
            .parsed
            .if_statements
            .iter()
            .any(|statement| statement.expression == expression)
    {
        let condition = lower_expression(
            source,
            function_declaration,
            if_expression.condition,
            local_bindings,
            output,
        )?;
        let then_expression =
            conditional_branch_expression(source.parsed, if_expression.then_block)
                .ok_or(HirLoweringError::UnsupportedExpression)?;
        let then_value = lower_expression(
            source,
            function_declaration,
            then_expression,
            local_bindings,
            output,
        )?;
        let else_expression = conditional_branch_expression(source.parsed, else_block)
            .ok_or(HirLoweringError::UnsupportedExpression)?;
        let else_value = lower_expression(
            source,
            function_declaration,
            else_expression,
            local_bindings,
            output,
        )?;
        let conditional_id = HirExpressionId::from_raw(output.len());
        output.push(HirExpression::conditional(
            conditional_id,
            span,
            ty,
            condition,
            then_value,
            else_value,
        ));
        return Ok(conditional_id);
    }
    if let Some(when) = source
        .parsed
        .when_expressions
        .iter()
        .find(|when| when.expression == expression)
    {
        let subject = lower_expression(
            source,
            function_declaration,
            when.subject,
            local_bindings,
            output,
        )?;
        let mut arms = Vec::new();
        for arm_id in &when.arms {
            let arm = source
                .parsed
                .match_arms
                .iter()
                .find(|arm| arm.arm == *arm_id)
                .ok_or(HirLoweringError::UnsupportedExpression)?;
            let tag = source
                .parsed
                .qualified_case_patterns
                .iter()
                .find(|pattern| pattern.pattern == arm.pattern)
                .and_then(|pattern| {
                    source
                        .parsed
                        .enum_variants
                        .iter()
                        .find(|variant| variant.name == pattern.variant_name)
                        .and_then(|variant| {
                            source
                                .parsed
                                .enum_variants
                                .iter()
                                .filter(|candidate| {
                                    candidate.enum_declaration == variant.enum_declaration
                                })
                                .position(|candidate| candidate.variant == variant.variant)
                        })
                        .and_then(|tag| i64::try_from(tag).ok())
                });
            let value = if let Some(reference) = source
                .parsed
                .name_references
                .iter()
                .find(|reference| reference.reference == arm.body)
            {
                let binding = source.parsed.pattern_bindings.iter().find(|binding| {
                    binding.name == reference.name
                        && source
                            .parsed
                            .qualified_case_patterns
                            .iter()
                            .find(|pattern| pattern.pattern == arm.pattern)
                            .is_some_and(|pattern| pattern.payloads.contains(&binding.pattern))
                });
                if let Some(binding) = binding {
                    let index = source
                        .parsed
                        .qualified_case_patterns
                        .iter()
                        .find(|pattern| pattern.pattern == arm.pattern)
                        .and_then(|pattern| {
                            pattern
                                .payloads
                                .iter()
                                .position(|id| *id == binding.pattern)
                        })
                        .ok_or(HirLoweringError::UnsupportedExpression)?;
                    let value_id = HirExpressionId::from_raw(output.len());
                    let value_ty = source
                        .expression_types
                        .iter()
                        .find(|typed| typed.expression() == arm.body)
                        .map(|typed| typed.ty())
                        .ok_or(HirLoweringError::MissingType)?;
                    output.push(HirExpression::enum_payload(
                        value_id,
                        source
                            .parsed
                            .arena
                            .node(arm.body)
                            .ok_or(HirLoweringError::UnsupportedExpression)?
                            .span,
                        value_ty,
                        subject,
                        index,
                    ));
                    value_id
                } else {
                    lower_expression(
                        source,
                        function_declaration,
                        arm.body,
                        local_bindings,
                        output,
                    )?
                }
            } else {
                lower_expression(
                    source,
                    function_declaration,
                    arm.body,
                    local_bindings,
                    output,
                )?
            };
            arms.push((tag, value));
        }
        let when_id = HirExpressionId::from_raw(output.len());
        output.push(HirExpression::when(when_id, span, ty, subject, arms));
        return Ok(when_id);
    }
    if let Some(literal) = source
        .parsed
        .literal_expressions
        .iter()
        .find(|literal| literal.expression == expression)
    {
        match literal.kind {
            ParsedLiteralKind::BoolTrue | ParsedLiteralKind::BoolFalse => {
                output.push(HirExpression::bool_literal(
                    id,
                    span,
                    ty,
                    literal.kind == ParsedLiteralKind::BoolTrue,
                ));
            }
            ParsedLiteralKind::Unit => output.push(HirExpression::unit_literal(id, span, ty)),
            ParsedLiteralKind::Float => {
                let bits = source
                    .parsed
                    .float_literals
                    .iter()
                    .find(|float| float.expression == expression)
                    .and_then(|float| float.bits)
                    .ok_or(HirLoweringError::UnsupportedExpression)?;
                output.push(HirExpression::float_literal(
                    id,
                    span,
                    ty,
                    f64::from_bits(bits),
                ));
            }
            ParsedLiteralKind::AcceptedInteger => {
                let value = source
                    .parsed
                    .integer_literals
                    .iter()
                    .find(|integer| integer.expression == expression)
                    .and_then(|integer| integer.value)
                    .and_then(|value| i64::try_from(value).ok())
                    .ok_or(HirLoweringError::UnsupportedExpression)?;
                if source.byte_type == Some(ty) {
                    let value =
                        u8::try_from(value).map_err(|_| HirLoweringError::UnsupportedExpression)?;
                    output.push(HirExpression::byte_literal(id, span, ty, value));
                } else {
                    output.push(HirExpression::int_literal(id, span, ty, value));
                }
            }
            ParsedLiteralKind::AcceptedString | ParsedLiteralKind::Null => {
                let string = source
                    .parsed
                    .string_literals
                    .iter()
                    .find(|string| string.expression == expression)
                    .ok_or(HirLoweringError::UnsupportedExpression)?;
                output.push(HirExpression::string_literal(
                    id,
                    span,
                    ty,
                    string.bytes.clone(),
                ));
            }
        }
        return Ok(id);
    }
    if let Some(binary) = source
        .parsed
        .binary_expressions
        .iter()
        .find(|binary| binary.expression == expression)
    {
        let left = lower_expression(
            source,
            function_declaration,
            binary.left,
            local_bindings,
            output,
        )?;
        let right = lower_expression(
            source,
            function_declaration,
            binary.right,
            local_bindings,
            output,
        )?;
        let id = HirExpressionId::from_raw(output.len());
        output.push(HirExpression::binary(
            id,
            span,
            ty,
            lower_binary_operator(binary.operator)?,
            left,
            right,
        ));
        return Ok(id);
    }
    if let Some(member) = source
        .parsed
        .member_expressions
        .iter()
        .find(|member| member.expression == expression)
    {
        if let Some(tag) = enum_variant_tag(source, member) {
            output.push(HirExpression::enum_variant(id, span, ty, tag));
            return Ok(id);
        }
        let receiver = lower_expression(
            source,
            function_declaration,
            member.receiver,
            local_bindings,
            output,
        )?;
        let id = HirExpressionId::from_raw(output.len());
        if let Some(index) = enum_field_index(source, member.receiver, &member.name) {
            output.push(HirExpression::enum_payload(id, span, ty, receiver, index));
            return Ok(id);
        }
        if member.name == "length" {
            output.push(HirExpression::string_length(id, span, ty, receiver));
        } else {
            output.push(HirExpression::field_access(
                id,
                span,
                ty,
                receiver,
                member.name.clone(),
                field_index_for_receiver(source, member.receiver, &member.name),
            ));
        }
        return Ok(id);
    }
    if let Some(new_expression) = source
        .parsed
        .new_expressions
        .iter()
        .find(|new_expression| new_expression.expression == expression)
    {
        if new_expression.dynamic_array {
            let id = HirExpressionId::from_raw(output.len());
            output.push(HirExpression::dynamic_array_new(id, span, ty));
            return Ok(id);
        }
        let mut argument_nodes = Vec::new();
        append_superclass_argument_nodes(
            source.parsed,
            &new_expression.type_name,
            &mut argument_nodes,
        );
        argument_nodes.extend(new_expression.arguments.iter().copied());
        let arguments = argument_nodes
            .iter()
            .map(|argument| {
                lower_expression(
                    source,
                    function_declaration,
                    *argument,
                    local_bindings,
                    output,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let id = HirExpressionId::from_raw(output.len());
        output.push(HirExpression::new_object(
            id,
            span,
            ty,
            new_expression.type_name.clone(),
            arguments,
        ));
        return Ok(id);
    }
    if let Some(array) = source
        .parsed
        .array_literals
        .iter()
        .find(|array| array.expression == expression)
    {
        let elements = array
            .elements
            .iter()
            .map(|element| {
                lower_expression(
                    source,
                    function_declaration,
                    *element,
                    local_bindings,
                    output,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let id = HirExpressionId::from_raw(output.len());
        output.push(HirExpression::array_literal(id, span, ty, elements));
        return Ok(id);
    }
    if let Some(index) = source
        .parsed
        .index_expressions
        .iter()
        .find(|index| index.expression == expression)
    {
        let array = lower_expression(
            source,
            function_declaration,
            index.array,
            local_bindings,
            output,
        )?;
        let index_value = lower_expression(
            source,
            function_declaration,
            index.index,
            local_bindings,
            output,
        )?;
        let id = HirExpressionId::from_raw(output.len());
        output.push(HirExpression::index(id, span, ty, array, index_value));
        return Ok(id);
    }
    if let Some(unary) = source
        .parsed
        .unary_expressions
        .iter()
        .find(|unary| unary.expression == expression)
    {
        let operand = lower_expression(
            source,
            function_declaration,
            unary.operand,
            local_bindings,
            output,
        )?;
        let id = HirExpressionId::from_raw(output.len());
        output.push(HirExpression::unary(
            id,
            span,
            ty,
            lower_unary_operator(unary.operator)?,
            operand,
        ));
        return Ok(id);
    }
    if source
        .parsed
        .function_declarations
        .iter()
        .find(|function| function.declaration == function_declaration)
        .is_some_and(|function| function.owner.is_some())
        && source.parsed.name_references.iter().any(|name| {
            name.reference == expression && matches!(name.name.as_str(), "this" | "super")
        })
    {
        output.push(HirExpression::parameter_read(
            id,
            span,
            ty,
            HirParameterId::from_raw(0),
        ));
        return Ok(id);
    }
    if let Some(name) = source
        .parsed
        .name_references
        .iter()
        .find(|name| name.reference == expression)
        && let Some(parameter_index) = source
            .parsed
            .function_parameters
            .iter()
            .filter(|parameter| parameter.function == function_declaration)
            .position(|parameter| parameter.name == name.name)
    {
        output.push(HirExpression::parameter_read(
            id,
            span,
            ty,
            HirParameterId::from_raw(
                parameter_index
                    + usize::from(
                        source
                            .parsed
                            .function_declarations
                            .iter()
                            .find(|function| function.declaration == function_declaration)
                            .is_some_and(|function| function.owner.is_some()),
                    ),
            ),
        ));
        return Ok(id);
    }
    if let Some(name) = source
        .parsed
        .name_references
        .iter()
        .find(|name| name.reference == expression)
        && let Some((receiver_type, field_index)) =
            bare_field_for_method(source, function_declaration, &name.name)
    {
        let receiver_id = HirExpressionId::from_raw(output.len());
        output.push(HirExpression::parameter_read(
            receiver_id,
            span,
            receiver_type,
            HirParameterId::from_raw(0),
        ));
        let field_id = HirExpressionId::from_raw(output.len());
        output.push(HirExpression::field_access(
            field_id,
            span,
            ty,
            receiver_id,
            name.name.clone(),
            field_index,
        ));
        return Ok(field_id);
    }
    if let Some(local) = local_binding_id(source, function_declaration, expression, local_bindings)
    {
        output.push(HirExpression::local_read(id, span, ty, local));
        return Ok(id);
    }
    if let Some(call) = source
        .parsed
        .call_expressions
        .iter()
        .find(|call| call.expression == expression)
    {
        if let Some(member) = source
            .parsed
            .member_expressions
            .iter()
            .find(|member| member.expression == call.callee)
            && let Some(tag) = enum_variant_tag(source, member)
        {
            let payloads = call
                .arguments
                .iter()
                .map(|argument| {
                    lower_expression(
                        source,
                        function_declaration,
                        *argument,
                        local_bindings,
                        output,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            output.push(HirExpression::enum_construct(id, span, ty, tag, payloads));
            return Ok(id);
        }
        let is_clone = source
            .parsed
            .name_references
            .iter()
            .find(|name| name.reference == call.callee)
            .is_some_and(|name| name.name == "clone");
        if is_clone {
            let argument = *call
                .arguments
                .first()
                .ok_or(HirLoweringError::UnsupportedExpression)?;
            let argument = lower_expression(
                source,
                function_declaration,
                argument,
                local_bindings,
                output,
            )?;
            let id = HirExpressionId::from_raw(output.len());
            output.push(HirExpression::string_clone(id, span, ty, argument));
            return Ok(id);
        }
        if let Some(member) = source
            .parsed
            .member_expressions
            .iter()
            .find(|member| member.expression == call.callee)
            && matches!(member.name.as_str(), "size" | "add" | "remove")
        {
            let array = lower_expression(
                source,
                function_declaration,
                member.receiver,
                local_bindings,
                output,
            )?;
            let arguments = call
                .arguments
                .iter()
                .map(|argument| {
                    lower_expression(
                        source,
                        function_declaration,
                        *argument,
                        local_bindings,
                        output,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            let id = HirExpressionId::from_raw(output.len());
            match member.name.as_str() {
                "size" => output.push(HirExpression {
                    id,
                    span,
                    ty,
                    kind: HirExpressionKind::DynamicArraySize(array),
                }),
                "add" => output.push(HirExpression {
                    id,
                    span,
                    ty,
                    kind: HirExpressionKind::DynamicArrayAdd {
                        array,
                        value: arguments[0],
                        index: arguments.get(1).copied(),
                    },
                }),
                "remove" => output.push(HirExpression {
                    id,
                    span,
                    ty,
                    kind: HirExpressionKind::DynamicArrayRemove {
                        array,
                        index: arguments[0],
                    },
                }),
                _ => unreachable!(),
            }
            return Ok(id);
        }
        let declaration = if let Some(name) = source
            .parsed
            .name_references
            .iter()
            .find(|name| name.reference == call.callee)
        {
            let selected = source
                .call_targets
                .and_then(|targets| {
                    targets
                        .iter()
                        .find(|target| target.call() == call.expression)
                })
                .map(|target| target.declaration());
            hir_function_index(
                source.parsed,
                source
                    .parsed
                    .function_declarations
                    .iter()
                    .position(|function| {
                        selected.is_some_and(|declaration| function.declaration == declaration)
                            || (selected.is_none() && function.name == name.name)
                    })
                    .ok_or(HirLoweringError::UnsupportedExpression)?,
            )
            .ok_or(HirLoweringError::UnsupportedExpression)?
        } else {
            method_declaration_index(source, call.callee, call.function)
                .ok_or(HirLoweringError::UnsupportedExpression)?
        };
        let mut arguments = Vec::new();
        if let Some(member) = source
            .parsed
            .member_expressions
            .iter()
            .find(|member| member.expression == call.callee)
        {
            let is_static = source
                .parsed
                .function_declarations
                .iter()
                .find(|function| {
                    hir_function_index(
                        source.parsed,
                        source
                            .parsed
                            .function_declarations
                            .iter()
                            .position(|candidate| candidate.declaration == function.declaration)
                            .unwrap_or(usize::MAX),
                    )
                    .is_some_and(|index| index == declaration)
                })
                .is_some_and(|function| function.is_static);
            if !is_static {
                arguments.push(lower_expression(
                    source,
                    function_declaration,
                    member.receiver,
                    local_bindings,
                    output,
                )?);
            }
        }
        arguments.extend(
            call.arguments
                .iter()
                .map(|argument| {
                    lower_expression(
                        source,
                        function_declaration,
                        *argument,
                        local_bindings,
                        output,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?,
        );
        let id = HirExpressionId::from_raw(output.len());
        let (dispatch, targets) =
            method_dispatch_facts(source, call.callee, function_declaration, declaration);
        output.push(HirExpression::direct_call(
            id,
            span,
            ty,
            HirDirectCall::new(HirFunctionId::from_raw(declaration), arguments)
                .with_dispatch(dispatch, targets),
        ));
        return Ok(id);
    }
    Err(HirLoweringError::UnsupportedExpression)
}

fn enum_variant_tag(
    source: &CheckedHirSource<'_>,
    member: &crate::parser::ParsedMemberExpression,
) -> Option<i64> {
    let receiver_name = source
        .parsed
        .name_references
        .iter()
        .find(|name| name.reference == member.receiver)
        .map(|name| name.name.as_str())?;
    let declaration = source
        .class_types?
        .classes()
        .iter()
        .find(|class| {
            class.name() == receiver_name
                && source.parsed.enum_variants.iter().any(|variant| {
                    variant.enum_declaration == class.declaration() && variant.name == member.name
                })
        })
        .map(|class| class.declaration())?;
    source
        .parsed
        .enum_variants
        .iter()
        .filter(|variant| variant.enum_declaration == declaration)
        .position(|variant| variant.name == member.name)
        .and_then(|tag| i64::try_from(tag).ok())
}

fn enum_field_index(
    source: &CheckedHirSource<'_>,
    receiver: crate::ast::AstNodeId,
    field_name: &str,
) -> Option<usize> {
    let receiver_type = source
        .expression_types
        .iter()
        .find(|typed| typed.expression() == receiver)
        .map(|typed| typed.ty())?;
    let class = source
        .class_types?
        .classes()
        .iter()
        .find(|class| class.type_id() == receiver_type)?;
    if !source
        .parsed
        .enum_variants
        .iter()
        .any(|variant| variant.enum_declaration == class.declaration())
    {
        return None;
    }
    source
        .parsed
        .class_declarations
        .iter()
        .find(|declaration| declaration.declaration == class.declaration())?
        .constructor_parameters
        .iter()
        .filter(|parameter| parameter.field)
        .position(|parameter| parameter.name == field_name)
}

fn conditional_branch_expression(
    parsed: &ParseOutput,
    block: crate::ast::AstNodeId,
) -> Option<crate::ast::AstNodeId> {
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

fn append_superclass_argument_nodes(
    parsed: &ParseOutput,
    class_name: &str,
    output: &mut Vec<crate::ast::AstNodeId>,
) {
    let Some(class) = parsed
        .class_declarations
        .iter()
        .find(|class| class.name == class_name)
    else {
        return;
    };
    if let Some(superclass) = class.superclass.as_deref() {
        append_superclass_argument_nodes(parsed, superclass, output);
    }
    output.extend(class.superclass_arguments.iter().copied());
}

fn field_index_for_receiver(
    source: &CheckedHirSource<'_>,
    receiver: crate::ast::AstNodeId,
    field_name: &str,
) -> usize {
    let class_name = source
        .parsed
        .new_expressions
        .iter()
        .find(|expression| expression.expression == receiver)
        .map(|expression| expression.type_name.clone())
        .or_else(|| {
            let name = source
                .parsed
                .name_references
                .iter()
                .find(|name| name.reference == receiver)?;
            if matches!(name.name.as_str(), "this" | "super") {
                let receiver_span = source.parsed.arena.node(receiver)?.span;
                let function = source
                    .parsed
                    .function_declarations
                    .iter()
                    .find(|function| {
                        function
                            .body
                            .and_then(|body| source.parsed.arena.node(body))
                            .is_some_and(|body| {
                                body.span.file() == receiver_span.file()
                                    && body.span.start() <= receiver_span.start()
                                    && receiver_span.end() <= body.span.end()
                            })
                    })?;
                let owner = function.owner?;
                let class = source
                    .parsed
                    .class_declarations
                    .iter()
                    .find(|class| class.declaration == owner)?;
                return if name.name == "super" {
                    class.superclass.clone()
                } else {
                    Some(class.name.clone())
                };
            }
            let binding = source
                .parsed
                .local_binding_names
                .iter()
                .find(|binding| binding.name == name.name)?;
            let declaration = source
                .parsed
                .local_declarations
                .iter()
                .find(|declaration| declaration.declaration == binding.binding)?;
            let annotation = declaration.annotation?;
            source
                .parsed
                .type_name_references
                .iter()
                .find(|reference| reference.reference == annotation)
                .map(|reference| reference.name.clone())
        });
    let Some(class_name) = class_name else {
        return 0;
    };
    let Some(class) = source
        .parsed
        .class_declarations
        .iter()
        .find(|class| class.name == class_name)
    else {
        return 0;
    };
    field_index_for_class(source.parsed, class, field_name)
}

fn field_index_for_class(
    parsed: &ParseOutput,
    class: &crate::parser::ParsedClassDeclaration,
    field_name: &str,
) -> usize {
    let mut index = 0;
    if let Some(superclass) = class.superclass.as_deref()
        && let Some(parent) = parsed
            .class_declarations
            .iter()
            .find(|candidate| candidate.name == superclass)
    {
        let parent_field_count = field_count(parsed, parent);
        if let Some(found) = field_index_for_class_if_present(parsed, parent, field_name) {
            return found;
        }
        index += parent_field_count;
    }
    if let Some(found) = field_index_for_class_if_present(parsed, class, field_name) {
        return index + found;
    }
    0
}

fn field_count(parsed: &ParseOutput, class: &crate::parser::ParsedClassDeclaration) -> usize {
    class
        .constructor_parameters
        .iter()
        .filter(|parameter| parameter.field)
        .count()
        + class.fields.len()
        + class.superclass.as_deref().map_or(0, |superclass| {
            parsed
                .class_declarations
                .iter()
                .find(|candidate| candidate.name == superclass)
                .map_or(0, |parent| field_count(parsed, parent))
        })
}

fn field_index_for_class_if_present(
    parsed: &ParseOutput,
    class: &crate::parser::ParsedClassDeclaration,
    field_name: &str,
) -> Option<usize> {
    let mut index = 0;
    for parameter in class
        .constructor_parameters
        .iter()
        .filter(|parameter| parameter.field)
    {
        if parameter.name == field_name {
            return Some(index);
        }
        index += 1;
    }
    for field in &class.fields {
        let field = parsed
            .field_declarations
            .iter()
            .find(|candidate| candidate.declaration == *field)?;
        if field.name == field_name {
            return Some(index);
        }
        index += 1;
    }
    None
}

fn bare_field_for_method(
    source: &CheckedHirSource<'_>,
    function_declaration: crate::ast::AstNodeId,
    field_name: &str,
) -> Option<(TypeId, usize)> {
    if source.parsed.local_binding_names.iter().any(|binding| {
        binding.name == field_name
            && source
                .parsed
                .executable_body_statements
                .iter()
                .any(|statement| {
                    statement.function == function_declaration
                        && statement.statement == binding.binding
                })
    }) {
        return None;
    }
    let function = source
        .parsed
        .function_declarations
        .iter()
        .find(|function| function.declaration == function_declaration)?;
    let owner = function.owner?;
    let class = source
        .parsed
        .class_declarations
        .iter()
        .find(|class| class.declaration == owner)?;
    let signature = source
        .signatures
        .iter()
        .find(|signature| signature.declaration() == function_declaration)?;
    let receiver_type = *signature.parameter_types().first()?;
    let field_index = field_index_for_class(source.parsed, class, field_name);
    let owns_field = class
        .constructor_parameters
        .iter()
        .filter(|parameter| parameter.field)
        .any(|parameter| parameter.name == field_name)
        || class.fields.iter().any(|field| {
            source
                .parsed
                .field_declarations
                .iter()
                .find(|candidate| candidate.declaration == *field)
                .is_some_and(|field| field.name == field_name)
        })
        || class.superclass.as_deref().is_some_and(|superclass| {
            source
                .parsed
                .class_declarations
                .iter()
                .find(|candidate| candidate.name == superclass)
                .is_some_and(|parent| bare_field_exists(source.parsed, parent, field_name))
        });
    owns_field.then_some((receiver_type, field_index))
}

fn bare_field_exists(
    parsed: &ParseOutput,
    class: &crate::parser::ParsedClassDeclaration,
    field_name: &str,
) -> bool {
    class
        .constructor_parameters
        .iter()
        .filter(|parameter| parameter.field)
        .any(|parameter| parameter.name == field_name)
        || class.fields.iter().any(|field| {
            parsed
                .field_declarations
                .iter()
                .find(|candidate| candidate.declaration == *field)
                .is_some_and(|field| field.name == field_name)
        })
        || class.superclass.as_deref().is_some_and(|superclass| {
            parsed
                .class_declarations
                .iter()
                .find(|candidate| candidate.name == superclass)
                .is_some_and(|parent| bare_field_exists(parsed, parent, field_name))
        })
}

fn method_declaration_index(
    source: &CheckedHirSource<'_>,
    callee: crate::ast::AstNodeId,
    function_declaration: crate::ast::AstNodeId,
) -> Option<usize> {
    let member = source
        .parsed
        .member_expressions
        .iter()
        .find(|member| member.expression == callee)?;
    if let Some(call) = source
        .parsed
        .call_expressions
        .iter()
        .find(|call| call.callee == callee)
        && let Some(declaration) = source
            .call_targets
            .and_then(|targets| {
                targets
                    .iter()
                    .find(|target| target.call() == call.expression)
            })
            .map(|target| target.declaration())
        && source
            .parsed
            .function_declarations
            .iter()
            .find(|function| function.declaration == declaration)
            .is_some_and(|function| function.body.is_some())
    {
        let index = source
            .parsed
            .function_declarations
            .iter()
            .position(|function| function.declaration == declaration)?;
        return hir_function_index(source.parsed, index);
    }
    if let Some(index) = source
        .parsed
        .index_expressions
        .iter()
        .find(|index| index.expression == member.receiver)
    {
        let name = source
            .parsed
            .name_references
            .iter()
            .find(|name| name.reference == index.array)?;
        let binding = source
            .parsed
            .local_binding_names
            .iter()
            .find(|binding| binding.name == name.name)?;
        let declaration = source
            .parsed
            .local_declarations
            .iter()
            .find(|declaration| declaration.declaration == binding.binding)?;
        let annotation = declaration.annotation?;
        let array = source
            .parsed
            .array_types
            .iter()
            .find(|array| array.array == annotation)?;
        let element_reference = source
            .parsed
            .type_name_references
            .iter()
            .find(|reference| reference.reference == array.element_type)?;
        let class = source
            .parsed
            .class_declarations
            .iter()
            .find(|class| class.name == element_reference.name)?;
        let declaration = if class.interface {
            source
                .class_types?
                .classes()
                .iter()
                .find(|candidate| {
                    !candidate.is_interface()
                        && candidate
                            .interfaces()
                            .iter()
                            .any(|name| name == &class.name)
                })
                .and_then(|candidate| {
                    method_owner_declaration(source, candidate.name(), &member.name)
                })?
        } else {
            source
                .parsed
                .function_declarations
                .iter()
                .find(|function| {
                    function.owner == Some(class.declaration) && function.name == member.name
                })?
                .declaration
        };
        let index = source
            .parsed
            .function_declarations
            .iter()
            .position(|function| function.declaration == declaration)?;
        return hir_function_index(source.parsed, index);
    }
    let receiver = source
        .parsed
        .name_references
        .iter()
        .find(|name| name.reference == member.receiver)?;
    if let Some(class) = source.class_types.and_then(|classes| {
        classes.classes().iter().find(|class| {
            class.name() == receiver.name
                && source
                    .parsed
                    .enum_variants
                    .iter()
                    .any(|variant| variant.enum_declaration == class.declaration())
        })
    }) {
        let declaration = source
            .parsed
            .function_declarations
            .iter()
            .find(|function| {
                function.owner == Some(class.declaration())
                    && function.name == member.name
                    && function.is_static
            })?
            .declaration;
        let index = source
            .parsed
            .function_declarations
            .iter()
            .position(|function| function.declaration == declaration)?;
        return hir_function_index(source.parsed, index);
    }
    if matches!(receiver.name.as_str(), "this" | "super") {
        let owner = source
            .parsed
            .function_declarations
            .iter()
            .find(|function| function.declaration == function_declaration)?
            .owner?;
        let class = source
            .parsed
            .class_declarations
            .iter()
            .find(|class| class.declaration == owner)?;
        let class_name = if receiver.name == "super" {
            class.superclass.as_deref()?
        } else {
            class.name.as_str()
        };
        let class = source
            .parsed
            .class_declarations
            .iter()
            .find(|class| class.name == class_name)?;
        return source
            .parsed
            .function_declarations
            .iter()
            .position(|function| {
                function.owner == Some(class.declaration) && function.name == member.name
            })
            .and_then(|index| hir_function_index(source.parsed, index));
    }
    if let Some(parameter) = source.parsed.function_parameters.iter().find(|parameter| {
        parameter.function == function_declaration && parameter.name == receiver.name
    }) {
        let class_name = source
            .parsed
            .type_name_references
            .iter()
            .find(|reference| reference.reference == parameter.annotation)?
            .name
            .clone();
        let class = source
            .parsed
            .class_declarations
            .iter()
            .find(|class| class.name == class_name)?;
        let declaration = if class.interface {
            source
                .class_types?
                .classes()
                .iter()
                .find(|candidate| {
                    !candidate.is_interface()
                        && candidate
                            .interfaces()
                            .iter()
                            .any(|name| name == &class.name)
                })
                .and_then(|candidate| {
                    method_owner_declaration(source, candidate.name(), &member.name)
                })?
        } else {
            source
                .parsed
                .function_declarations
                .iter()
                .find(|function| {
                    function.owner == Some(class.declaration) && function.name == member.name
                })?
                .declaration
        };
        return source
            .parsed
            .function_declarations
            .iter()
            .position(|function| function.declaration == declaration)
            .and_then(|index| hir_function_index(source.parsed, index));
    }
    let Some(binding) = source
        .parsed
        .local_binding_names
        .iter()
        .find(|binding| binding.name == receiver.name)
    else {
        let receiver_type = source
            .expression_types
            .iter()
            .find(|typed| typed.expression() == member.receiver)
            .map(|typed| typed.ty())?;
        let class = source
            .class_types?
            .classes()
            .iter()
            .find(|class| class.type_id() == receiver_type)?;
        return source
            .parsed
            .function_declarations
            .iter()
            .position(|function| {
                function.owner == Some(class.declaration()) && function.name == member.name
            })
            .and_then(|index| hir_function_index(source.parsed, index));
    };
    let declaration = source
        .parsed
        .local_declarations
        .iter()
        .find(|declaration| declaration.declaration == binding.binding)?;
    let annotation = declaration.annotation?;
    let class_name = source
        .parsed
        .type_name_references
        .iter()
        .find(|reference| reference.reference == annotation)?
        .name
        .clone();
    let class = source
        .parsed
        .class_declarations
        .iter()
        .find(|class| class.name == class_name)?;
    let direct = source
        .parsed
        .function_declarations
        .iter()
        .position(|function| {
            function.owner == Some(class.declaration) && function.name == member.name
        })
        .and_then(|index| hir_function_index(source.parsed, index));
    direct.or_else(|| {
        source
            .class_types?
            .classes()
            .iter()
            .find(|candidate| {
                !candidate.is_interface()
                    && candidate
                        .interfaces()
                        .iter()
                        .any(|name| name == &class_name)
            })
            .and_then(|candidate| method_owner_declaration(source, candidate.name(), &member.name))
            .and_then(|declaration| {
                let index = source
                    .parsed
                    .function_declarations
                    .iter()
                    .position(|function| function.declaration == declaration)?;
                hir_function_index(source.parsed, index)
            })
    })
}

fn hir_function_index(parsed: &ParseOutput, parsed_index: usize) -> Option<usize> {
    parsed
        .function_declarations
        .iter()
        .take(parsed_index + 1)
        .filter(|function| function.body.is_some())
        .count()
        .checked_sub(1)
}

fn method_dispatch_facts(
    source: &CheckedHirSource<'_>,
    callee: crate::ast::AstNodeId,
    function_declaration: crate::ast::AstNodeId,
    static_callee: usize,
) -> (HirDispatchKind, Vec<HirDispatchTarget>) {
    let Some(member) = source
        .parsed
        .member_expressions
        .iter()
        .find(|member| member.expression == callee)
    else {
        return (HirDispatchKind::Direct, Vec::new());
    };
    let Some(classes) = source.class_types else {
        return (HirDispatchKind::Direct, Vec::new());
    };
    let Some(receiver_class) = receiver_class_name(source, member.receiver, function_declaration)
    else {
        return (HirDispatchKind::Direct, Vec::new());
    };
    let Some(receiver_record) = classes
        .classes()
        .iter()
        .find(|class| class.name() == receiver_class)
    else {
        return (HirDispatchKind::Direct, Vec::new());
    };
    if source
        .parsed
        .enum_variants
        .iter()
        .any(|variant| variant.enum_declaration == receiver_record.declaration())
    {
        return (HirDispatchKind::Direct, Vec::new());
    }
    let Some(static_function) = parsed_function_for_hir(source.parsed, static_callee) else {
        return (HirDispatchKind::Direct, Vec::new());
    };
    let Some(static_signature) = source
        .signatures
        .iter()
        .find(|signature| signature.declaration() == static_function.declaration)
    else {
        return (HirDispatchKind::Direct, Vec::new());
    };
    let receiver_name = source
        .parsed
        .name_references
        .iter()
        .find(|name| name.reference == member.receiver)
        .map(|name| name.name.as_str());
    if receiver_name == Some("super") {
        return (HirDispatchKind::StaticSuper, Vec::new());
    }
    if receiver_record.is_interface() {
        return (
            HirDispatchKind::Interface,
            dispatch_targets_for_classes(
                source,
                classes,
                receiver_record.name(),
                &member.name,
                static_signature.parameter_types(),
            ),
        );
    }
    if static_function.is_final || static_function.visibility == "private" {
        return (HirDispatchKind::Direct, Vec::new());
    }
    let targets = dispatch_targets_for_classes(
        source,
        classes,
        receiver_record.name(),
        &member.name,
        static_signature.parameter_types(),
    );
    if targets.is_empty() {
        (HirDispatchKind::Direct, Vec::new())
    } else {
        (HirDispatchKind::VirtualClass, targets)
    }
}

fn receiver_class_name<'a>(
    source: &'a CheckedHirSource<'_>,
    receiver: crate::ast::AstNodeId,
    function_declaration: crate::ast::AstNodeId,
) -> Option<&'a str> {
    if let Some(name) = source
        .parsed
        .name_references
        .iter()
        .find(|name| name.reference == receiver)
    {
        if name.name == "this" {
            let owner = source
                .parsed
                .function_declarations
                .iter()
                .find(|function| function.declaration == function_declaration)?
                .owner?;
            return source
                .parsed
                .class_declarations
                .iter()
                .find(|class| class.declaration == owner)
                .map(|class| class.name.as_str());
        }
        if name.name == "super" {
            let owner = source
                .parsed
                .function_declarations
                .iter()
                .find(|function| function.declaration == function_declaration)?
                .owner?;
            let class = source
                .parsed
                .class_declarations
                .iter()
                .find(|class| class.declaration == owner)?;
            return class.superclass.as_deref();
        }
        if let Some(parameter) = source.parsed.function_parameters.iter().find(|parameter| {
            parameter.function == function_declaration && parameter.name == name.name
        }) {
            return source
                .parsed
                .type_name_references
                .iter()
                .find(|reference| reference.reference == parameter.annotation)
                .map(|reference| reference.name.as_str());
        }
        let binding = source
            .parsed
            .local_binding_names
            .iter()
            .find(|binding| binding.name == name.name)?;
        let declaration = source
            .parsed
            .local_declarations
            .iter()
            .find(|declaration| declaration.declaration == binding.binding)?;
        let annotation = declaration.annotation?;
        return source
            .parsed
            .type_name_references
            .iter()
            .find(|reference| reference.reference == annotation)
            .map(|reference| reference.name.as_str());
    }
    let inferred = source
        .expression_types
        .iter()
        .find(|typed| typed.expression() == receiver)
        .map(|typed| typed.ty())
        .and_then(|receiver_type| {
            source
                .class_types?
                .classes()
                .iter()
                .find(|class| class.type_id() == receiver_type)
        })
        .map(|class| class.name());
    inferred.or_else(|| {
        source
            .parsed
            .new_expressions
            .iter()
            .find(|new_expression| new_expression.expression == receiver)
            .map(|new_expression| new_expression.type_name.as_str())
            .or_else(|| {
                let index = source
                    .parsed
                    .index_expressions
                    .iter()
                    .find(|index| index.expression == receiver)?;
                let name = source
                    .parsed
                    .name_references
                    .iter()
                    .find(|name| name.reference == index.array)?;
                let binding = source
                    .parsed
                    .local_binding_names
                    .iter()
                    .find(|binding| binding.name == name.name)?;
                let declaration = source
                    .parsed
                    .local_declarations
                    .iter()
                    .find(|declaration| declaration.declaration == binding.binding)?;
                let annotation = declaration.annotation?;
                let array = source
                    .parsed
                    .array_types
                    .iter()
                    .find(|array| array.array == annotation)?;
                source
                    .parsed
                    .type_name_references
                    .iter()
                    .find(|reference| reference.reference == array.element_type)
                    .map(|reference| reference.name.as_str())
            })
    })
}

fn dispatch_targets_for_classes(
    source: &CheckedHirSource<'_>,
    classes: &ClassTypeReport,
    receiver_name: &str,
    method_name: &str,
    parameter_types: &[TypeId],
) -> Vec<HirDispatchTarget> {
    classes
        .classes()
        .iter()
        .filter(|candidate| {
            (!candidate.is_interface())
                && (candidate.name() == receiver_name
                    || candidate.superclass() == Some(receiver_name)
                    || is_subclass_of(classes, candidate.name(), receiver_name)
                    || (classes
                        .classes()
                        .iter()
                        .find(|base| base.name() == receiver_name)
                        .is_some_and(|base| base.is_interface())
                        && implements_interface(classes, candidate.name(), receiver_name)))
        })
        .filter_map(|candidate| {
            let declaration = method_owner_declaration_for_signature(
                source,
                candidate.name(),
                method_name,
                parameter_types,
            )?;
            let parsed_index = source
                .parsed
                .function_declarations
                .iter()
                .position(|function| function.declaration == declaration)?;
            let hir_index = hir_function_index(source.parsed, parsed_index)?;
            Some(HirDispatchTarget::new(
                candidate.type_id(),
                HirFunctionId::from_raw(hir_index),
            ))
        })
        .collect()
}

fn method_owner_declaration_for_signature(
    source: &CheckedHirSource<'_>,
    class_name: &str,
    method_name: &str,
    parameter_types: &[TypeId],
) -> Option<crate::ast::AstNodeId> {
    let class = source
        .parsed
        .class_declarations
        .iter()
        .find(|class| class.name == class_name)?;
    if let Some(function) = source.parsed.function_declarations.iter().find(|function| {
        function.owner == Some(class.declaration)
            && function.name == method_name
            && source
                .signatures
                .iter()
                .find(|signature| signature.declaration() == function.declaration)
                .is_some_and(|signature| {
                    signature.parameter_types().get(1..) == parameter_types.get(1..)
                })
    }) {
        return Some(function.declaration);
    }
    class.superclass.as_deref().and_then(|parent| {
        method_owner_declaration_for_signature(source, parent, method_name, parameter_types)
    })
}

fn method_owner_declaration(
    source: &CheckedHirSource<'_>,
    class_name: &str,
    method_name: &str,
) -> Option<crate::ast::AstNodeId> {
    let class = source
        .parsed
        .class_declarations
        .iter()
        .find(|class| class.name == class_name)?;
    if let Some(function) = source.parsed.function_declarations.iter().find(|function| {
        function.owner == Some(class.declaration)
            && function.name == method_name
            && function.body.is_some()
    }) {
        return Some(function.declaration);
    }
    class
        .superclass
        .as_deref()
        .and_then(|parent| method_owner_declaration(source, parent, method_name))
}

fn parsed_function_for_hir(
    parsed: &ParseOutput,
    hir_index: usize,
) -> Option<&crate::parser::ParsedFunctionDeclaration> {
    parsed
        .function_declarations
        .iter()
        .filter(|function| function.body.is_some())
        .nth(hir_index)
}

fn is_subclass_of(classes: &ClassTypeReport, candidate: &str, base: &str) -> bool {
    let mut current = classes
        .classes()
        .iter()
        .find(|class| class.name() == candidate);
    while let Some(class) = current {
        if class.superclass() == Some(base) {
            return true;
        }
        current = class.superclass().and_then(|parent| {
            classes
                .classes()
                .iter()
                .find(|class| class.name() == parent)
        });
    }
    false
}

fn implements_interface(classes: &ClassTypeReport, candidate: &str, interface: &str) -> bool {
    let Some(class) = classes
        .classes()
        .iter()
        .find(|class| class.name() == candidate)
    else {
        return false;
    };
    class.interfaces().iter().any(|name| name == interface)
        || class
            .superclass()
            .is_some_and(|parent| implements_interface(classes, parent, interface))
}

fn lower_control_flow_block(
    source: &CheckedHirSource<'_>,
    function_declaration: crate::ast::AstNodeId,
    block: crate::ast::AstNodeId,
    local_bindings: &[(crate::ast::AstNodeId, HirLocalId)],
    expressions: &mut Vec<HirExpression>,
) -> Result<Vec<HirControlFlow>, HirLoweringError> {
    let Some(block_span) = source.parsed.arena.node(block).map(|node| node.span) else {
        return Err(HirLoweringError::UnsupportedExpression);
    };
    let nested_bodies: Vec<_> = source
        .parsed
        .if_statements
        .iter()
        .filter(|statement| statement.function == function_declaration)
        .flat_map(|statement| [Some(statement.then_block), statement.else_block])
        .chain(
            source
                .parsed
                .for_statements
                .iter()
                .filter(|statement| statement.function == function_declaration)
                .map(|statement| Some(statement.body)),
        )
        .flatten()
        .filter_map(|body| source.parsed.arena.node(body).map(|node| node.span))
        .collect();

    let direct_statements: Vec<_> = source
        .parsed
        .executable_body_statements
        .iter()
        .filter(|statement| {
            statement.function == function_declaration
                && span_contains(block_span, statement.span)
                && !nested_bodies
                    .iter()
                    .any(|body| *body != block_span && span_contains(*body, statement.span))
        })
        .collect();
    let mut output = Vec::new();
    for statement in direct_statements {
        if let Some(if_statement) = source
            .parsed
            .if_statements
            .iter()
            .find(|candidate| candidate.statement == statement.statement)
        {
            let condition = lower_expression(
                source,
                function_declaration,
                source
                    .parsed
                    .if_expressions
                    .iter()
                    .find(|expression| expression.expression == if_statement.expression)
                    .ok_or(HirLoweringError::UnsupportedExpression)?
                    .condition,
                local_bindings,
                expressions,
            )?;
            let then_body = lower_control_flow_block(
                source,
                function_declaration,
                if_statement.then_block,
                local_bindings,
                expressions,
            )?;
            let else_body = if let Some(else_block) = if_statement.else_block {
                lower_control_flow_block(
                    source,
                    function_declaration,
                    else_block,
                    local_bindings,
                    expressions,
                )?
            } else {
                Vec::new()
            };
            output.push(HirControlFlow::If {
                condition,
                then_body,
                else_body,
                span: if_statement.span,
            });
            continue;
        }
        if let Some(for_statement) = source
            .parsed
            .for_statements
            .iter()
            .find(|candidate| candidate.statement == statement.statement)
        {
            let start = lower_expression(
                source,
                function_declaration,
                for_statement.start,
                local_bindings,
                expressions,
            )?;
            let end = lower_expression(
                source,
                function_declaration,
                for_statement.end,
                local_bindings,
                expressions,
            )?;
            let binding = local_binding_id(
                source,
                function_declaration,
                for_statement.start,
                local_bindings,
            )
            .or_else(|| {
                local_bindings
                    .iter()
                    .find(|(candidate, _)| *candidate == for_statement.binding)
                    .map(|(_, local)| *local)
            })
            .ok_or(HirLoweringError::UnsupportedExpression)?;
            let body = lower_control_flow_block(
                source,
                function_declaration,
                for_statement.body,
                local_bindings,
                expressions,
            )?;
            output.push(HirControlFlow::For {
                binding,
                start,
                end,
                body,
                span: for_statement.span,
            });
            continue;
        }
        if let Some(local) = source
            .parsed
            .local_declarations
            .iter()
            .find(|local| local.declaration == statement.statement)
        {
            let value = lower_expression(
                source,
                function_declaration,
                local
                    .initializer
                    .ok_or(HirLoweringError::UnsupportedExpression)?,
                local_bindings,
                expressions,
            )?;
            let local_id = local_bindings
                .iter()
                .find(|(binding, _)| *binding == local.declaration)
                .map(|(_, local)| *local)
                .ok_or(HirLoweringError::UnsupportedExpression)?;
            output.push(HirControlFlow::LocalInitializer {
                local: local_id,
                value,
                span: statement.span,
            });
            continue;
        }
        if let Some(assignment) = source
            .parsed
            .assignment_statements
            .iter()
            .find(|assignment| assignment.statement == statement.statement)
        {
            let (target, index) = if let Some(index) = source
                .parsed
                .index_expressions
                .iter()
                .find(|index| index.expression == assignment.target)
            {
                let target =
                    local_binding_id(source, function_declaration, index.array, local_bindings)
                        .ok_or(HirLoweringError::UnsupportedExpression)?;
                let index_value = lower_expression(
                    source,
                    function_declaration,
                    index.index,
                    local_bindings,
                    expressions,
                )?;
                (target, Some(index_value))
            } else {
                (
                    local_binding_id(
                        source,
                        function_declaration,
                        assignment.target,
                        local_bindings,
                    )
                    .ok_or(HirLoweringError::UnsupportedExpression)?,
                    None,
                )
            };
            let value = lower_expression(
                source,
                function_declaration,
                assignment.value,
                local_bindings,
                expressions,
            )?;
            output.push(HirControlFlow::Assignment(match index {
                Some(index) => HirAssignment::indexed(statement.span, target, index, value),
                None => HirAssignment::new(statement.span, target, value),
            }));
            continue;
        }
        if let Some(returned) = source
            .parsed
            .return_statements
            .iter()
            .find(|returned| returned.statement == statement.statement)
        {
            let value = lower_expression(
                source,
                function_declaration,
                returned
                    .value
                    .ok_or(HirLoweringError::UnsupportedExpression)?,
                local_bindings,
                expressions,
            )?;
            output.push(HirControlFlow::Return(HirReturn::new(
                statement.span,
                value,
            )));
            continue;
        }
        if let Some(control) = source
            .parsed
            .loop_control_statements
            .iter()
            .find(|control| control.statement == statement.statement)
        {
            output.push(match control.kind {
                crate::parser::ParsedLoopControlKind::Break => {
                    HirControlFlow::Break { span: control.span }
                }
                crate::parser::ParsedLoopControlKind::Continue => {
                    HirControlFlow::Continue { span: control.span }
                }
            });
        }
    }
    Ok(output)
}

fn span_contains(outer: ByteSpan, inner: ByteSpan) -> bool {
    outer.file() == inner.file() && outer.start() <= inner.start() && inner.end() <= outer.end()
}

fn local_binding_id(
    source: &CheckedHirSource<'_>,
    function_declaration: crate::ast::AstNodeId,
    expression: crate::ast::AstNodeId,
    local_bindings: &[(crate::ast::AstNodeId, HirLocalId)],
) -> Option<HirLocalId> {
    let name = source
        .parsed
        .name_references
        .iter()
        .find(|name| name.reference == expression)?;
    let expression_span = source.parsed.arena.node(expression)?.span;
    local_bindings
        .iter()
        .filter_map(|(binding, local_id)| {
            let binding_name = source
                .parsed
                .local_binding_names
                .iter()
                .find(|candidate| candidate.binding == *binding)?;
            if binding_name.name != name.name {
                return None;
            }
            let belongs_to_function =
                source
                    .parsed
                    .executable_body_statements
                    .iter()
                    .any(|statement| {
                        statement.function == function_declaration
                            && statement.statement == *binding
                    });
            if !belongs_to_function {
                return None;
            }
            let binding_span = source
                .parsed
                .for_statements
                .iter()
                .find(|loop_| loop_.binding == *binding)
                .map(|loop_| loop_.binding_name_span)
                .or_else(|| source.parsed.arena.node(*binding).map(|node| node.span))?;
            (binding_span.end() <= expression_span.start())
                .then_some((*local_id, binding_span.end()))
        })
        .max_by_key(|(_, end)| *end)
        .map(|(local_id, _)| local_id)
}

fn declaration_is_main(parsed: &ParseOutput, declaration: crate::ast::AstNodeId) -> bool {
    parsed
        .declaration_names
        .iter()
        .any(|name| name.declaration == declaration && name.name == "main")
}
fn lower_binary_operator(
    operator: ParsedBinaryOperator,
) -> Result<HirBinaryOperator, HirLoweringError> {
    Ok(match operator {
        ParsedBinaryOperator::LogicalOr => HirBinaryOperator::LogicalOr,
        ParsedBinaryOperator::LogicalAnd => HirBinaryOperator::LogicalAnd,
        ParsedBinaryOperator::Equal => HirBinaryOperator::Equal,
        ParsedBinaryOperator::NotEqual => HirBinaryOperator::NotEqual,
        ParsedBinaryOperator::Less => HirBinaryOperator::Less,
        ParsedBinaryOperator::Greater => HirBinaryOperator::Greater,
        ParsedBinaryOperator::LessEqual => HirBinaryOperator::LessEqual,
        ParsedBinaryOperator::GreaterEqual => HirBinaryOperator::GreaterEqual,
        ParsedBinaryOperator::Plus => HirBinaryOperator::Plus,
        ParsedBinaryOperator::Minus => HirBinaryOperator::Minus,
        ParsedBinaryOperator::Star => HirBinaryOperator::Multiply,
        ParsedBinaryOperator::Slash => HirBinaryOperator::Divide,
        ParsedBinaryOperator::Percent => HirBinaryOperator::Remainder,
        ParsedBinaryOperator::Exponent => HirBinaryOperator::Exponent,
        ParsedBinaryOperator::BitwiseAnd => HirBinaryOperator::BitwiseAnd,
        ParsedBinaryOperator::BitwiseOr => HirBinaryOperator::BitwiseOr,
        ParsedBinaryOperator::BitwiseXor => HirBinaryOperator::BitwiseXor,
        ParsedBinaryOperator::ShiftLeft => HirBinaryOperator::ShiftLeft,
        ParsedBinaryOperator::ShiftRight => HirBinaryOperator::ShiftRight,
    })
}

fn lower_unary_operator(
    operator: crate::parser::ParsedUnaryOperator,
) -> Result<HirUnaryOperator, HirLoweringError> {
    Ok(match operator {
        crate::parser::ParsedUnaryOperator::Not => HirUnaryOperator::Not,
        crate::parser::ParsedUnaryOperator::Plus => HirUnaryOperator::Plus,
        crate::parser::ParsedUnaryOperator::Minus => HirUnaryOperator::Minus,
        crate::parser::ParsedUnaryOperator::BitwiseNot => HirUnaryOperator::BitwiseNot,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HirModule {
    name: ModuleName,
    functions: Vec<HirFunction>,
}

impl HirModule {
    pub fn new(name: ModuleName, functions: Vec<HirFunction>) -> Self {
        Self { name, functions }
    }
    pub fn name(&self) -> &ModuleName {
        &self.name
    }
    pub fn functions(&self) -> &[HirFunction] {
        &self.functions
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HirParameter {
    id: HirParameterId,
    span: ByteSpan,
    ty: TypeId,
}
impl HirParameter {
    pub fn new(id: HirParameterId, span: ByteSpan, ty: TypeId) -> Self {
        Self { id, span, ty }
    }
    pub fn id(&self) -> HirParameterId {
        self.id
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
    pub fn ty(&self) -> TypeId {
        self.ty
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HirLocal {
    id: HirLocalId,
    span: ByteSpan,
    ty: TypeId,
    mutable: bool,
    initializer: Option<HirExpressionId>,
}
impl HirLocal {
    pub fn new(id: HirLocalId, span: ByteSpan, ty: TypeId, mutable: bool) -> Self {
        Self {
            id,
            span,
            ty,
            mutable,
            initializer: None,
        }
    }
    pub fn id(&self) -> HirLocalId {
        self.id
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
    pub fn ty(&self) -> TypeId {
        self.ty
    }
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }
    pub fn initializer(&self) -> Option<HirExpressionId> {
        self.initializer
    }
    fn set_initializer(&mut self, initializer: HirExpressionId) {
        self.initializer = Some(initializer);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HirDirectCall {
    callee: HirFunctionId,
    arguments: Vec<HirExpressionId>,
    dispatch: HirDispatchKind,
    targets: Vec<HirDispatchTarget>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HirDispatchKind {
    Direct,
    VirtualClass,
    Interface,
    StaticSuper,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HirDispatchTarget {
    receiver_type: TypeId,
    callee: HirFunctionId,
}

impl HirDispatchTarget {
    pub fn new(receiver_type: TypeId, callee: HirFunctionId) -> Self {
        Self {
            receiver_type,
            callee,
        }
    }
    pub fn receiver_type(self) -> TypeId {
        self.receiver_type
    }
    pub fn callee(self) -> HirFunctionId {
        self.callee
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HirUnaryOperator {
    Not,
    Plus,
    Minus,
    BitwiseNot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HirBinaryOperator {
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
    Multiply,
    Divide,
    Remainder,
    Exponent,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ShiftLeft,
    ShiftRight,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HirUnary {
    operator: HirUnaryOperator,
    operand: HirExpressionId,
}
impl HirUnary {
    pub fn new(operator: HirUnaryOperator, operand: HirExpressionId) -> Self {
        Self { operator, operand }
    }
    pub fn operator(&self) -> HirUnaryOperator {
        self.operator
    }
    pub fn operand(&self) -> HirExpressionId {
        self.operand
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HirBinary {
    operator: HirBinaryOperator,
    left: HirExpressionId,
    right: HirExpressionId,
}
impl HirBinary {
    pub fn new(operator: HirBinaryOperator, left: HirExpressionId, right: HirExpressionId) -> Self {
        Self {
            operator,
            left,
            right,
        }
    }
    pub fn operator(&self) -> HirBinaryOperator {
        self.operator
    }
    pub fn left(&self) -> HirExpressionId {
        self.left
    }
    pub fn right(&self) -> HirExpressionId {
        self.right
    }
}
impl HirDirectCall {
    pub fn new(callee: HirFunctionId, arguments: Vec<HirExpressionId>) -> Self {
        Self {
            callee,
            arguments,
            dispatch: HirDispatchKind::Direct,
            targets: Vec::new(),
        }
    }
    pub fn with_dispatch(
        mut self,
        dispatch: HirDispatchKind,
        targets: Vec<HirDispatchTarget>,
    ) -> Self {
        self.dispatch = dispatch;
        self.targets = targets;
        self
    }
    pub fn callee(&self) -> HirFunctionId {
        self.callee
    }
    pub fn arguments(&self) -> &[HirExpressionId] {
        &self.arguments
    }
    pub fn dispatch(&self) -> HirDispatchKind {
        self.dispatch
    }
    pub fn targets(&self) -> &[HirDispatchTarget] {
        &self.targets
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HirExpressionKind {
    IntLiteral(i64),
    EnumVariant(i64),
    BoolLiteral(bool),
    UnitLiteral,
    FloatLiteral(u64),
    ByteLiteral(u8),
    ParameterRead(HirParameterId),
    LocalRead(HirLocalId),
    Unary(HirUnary),
    Binary(HirBinary),
    Conditional {
        condition: HirExpressionId,
        then_value: HirExpressionId,
        else_value: HirExpressionId,
    },
    When {
        subject: HirExpressionId,
        arms: Vec<(Option<i64>, HirExpressionId)>,
    },
    EnumConstruct {
        tag: i64,
        payloads: Vec<HirExpressionId>,
    },
    EnumPayload {
        subject: HirExpressionId,
        index: usize,
    },
    DirectCall(HirDirectCall),
    ArrayLiteral(Vec<HirExpressionId>),
    Index {
        array: HirExpressionId,
        index: HirExpressionId,
    },
    StringLiteral(Vec<u8>),
    StringLength(HirExpressionId),
    StringClone(HirExpressionId),
    FieldAccess {
        receiver: HirExpressionId,
        name: String,
        index: usize,
    },
    NewObject {
        type_name: String,
        arguments: Vec<HirExpressionId>,
    },
    DynamicArrayNew,
    DynamicArraySize(HirExpressionId),
    DynamicArrayAdd {
        array: HirExpressionId,
        value: HirExpressionId,
        index: Option<HirExpressionId>,
    },
    DynamicArrayRemove {
        array: HirExpressionId,
        index: HirExpressionId,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HirExpression {
    id: HirExpressionId,
    span: ByteSpan,
    ty: TypeId,
    kind: HirExpressionKind,
}
impl HirExpression {
    pub fn int_literal(id: HirExpressionId, span: ByteSpan, ty: TypeId, value: i64) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::IntLiteral(value),
        }
    }

    pub fn enum_variant(id: HirExpressionId, span: ByteSpan, ty: TypeId, tag: i64) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::EnumVariant(tag),
        }
    }

    pub fn bool_literal(id: HirExpressionId, span: ByteSpan, ty: TypeId, value: bool) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::BoolLiteral(value),
        }
    }

    pub fn unit_literal(id: HirExpressionId, span: ByteSpan, ty: TypeId) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::UnitLiteral,
        }
    }

    pub fn float_literal(id: HirExpressionId, span: ByteSpan, ty: TypeId, value: f64) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::FloatLiteral(value.to_bits()),
        }
    }

    pub fn byte_literal(id: HirExpressionId, span: ByteSpan, ty: TypeId, value: u8) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::ByteLiteral(value),
        }
    }
    pub fn local_read(id: HirExpressionId, span: ByteSpan, ty: TypeId, local: HirLocalId) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::LocalRead(local),
        }
    }
    pub fn parameter_read(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        parameter: HirParameterId,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::ParameterRead(parameter),
        }
    }
    pub fn unary(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        operator: HirUnaryOperator,
        operand: HirExpressionId,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::Unary(HirUnary::new(operator, operand)),
        }
    }
    pub fn binary(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        operator: HirBinaryOperator,
        left: HirExpressionId,
        right: HirExpressionId,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::Binary(HirBinary::new(operator, left, right)),
        }
    }
    pub fn direct_call(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        call: HirDirectCall,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::DirectCall(call),
        }
    }

    pub fn conditional(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        condition: HirExpressionId,
        then_value: HirExpressionId,
        else_value: HirExpressionId,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::Conditional {
                condition,
                then_value,
                else_value,
            },
        }
    }

    pub fn when(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        subject: HirExpressionId,
        arms: Vec<(Option<i64>, HirExpressionId)>,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::When { subject, arms },
        }
    }

    pub fn enum_construct(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        tag: i64,
        payloads: Vec<HirExpressionId>,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::EnumConstruct { tag, payloads },
        }
    }

    pub fn enum_payload(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        subject: HirExpressionId,
        index: usize,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::EnumPayload { subject, index },
        }
    }

    pub fn array_literal(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        elements: Vec<HirExpressionId>,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::ArrayLiteral(elements),
        }
    }

    pub fn index(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        array: HirExpressionId,
        index: HirExpressionId,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::Index { array, index },
        }
    }

    pub fn string_literal(id: HirExpressionId, span: ByteSpan, ty: TypeId, bytes: Vec<u8>) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::StringLiteral(bytes),
        }
    }

    pub fn string_length(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        receiver: HirExpressionId,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::StringLength(receiver),
        }
    }

    pub fn field_access(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        receiver: HirExpressionId,
        name: String,
        index: usize,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::FieldAccess {
                receiver,
                name,
                index,
            },
        }
    }

    pub fn new_object(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        type_name: String,
        arguments: Vec<HirExpressionId>,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::NewObject {
                type_name,
                arguments,
            },
        }
    }

    pub fn dynamic_array_new(id: HirExpressionId, span: ByteSpan, ty: TypeId) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::DynamicArrayNew,
        }
    }

    pub fn string_clone(
        id: HirExpressionId,
        span: ByteSpan,
        ty: TypeId,
        value: HirExpressionId,
    ) -> Self {
        Self {
            id,
            span,
            ty,
            kind: HirExpressionKind::StringClone(value),
        }
    }
    pub fn id(&self) -> HirExpressionId {
        self.id
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
    pub fn ty(&self) -> TypeId {
        self.ty
    }
    pub fn kind(&self) -> &HirExpressionKind {
        &self.kind
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HirReturn {
    span: ByteSpan,
    expression: HirExpressionId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HirAssignment {
    span: ByteSpan,
    target: HirLocalId,
    value: HirExpressionId,
    index: Option<HirExpressionId>,
    field: Option<(HirExpressionId, usize)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HirControlFlow {
    LocalInitializer {
        local: HirLocalId,
        value: HirExpressionId,
        span: ByteSpan,
    },
    Assignment(HirAssignment),
    Return(HirReturn),
    If {
        condition: HirExpressionId,
        then_body: Vec<HirControlFlow>,
        else_body: Vec<HirControlFlow>,
        span: ByteSpan,
    },
    For {
        binding: HirLocalId,
        start: HirExpressionId,
        end: HirExpressionId,
        body: Vec<HirControlFlow>,
        span: ByteSpan,
    },
    Break {
        span: ByteSpan,
    },
    Continue {
        span: ByteSpan,
    },
}
impl HirAssignment {
    pub fn new(span: ByteSpan, target: HirLocalId, value: HirExpressionId) -> Self {
        Self {
            span,
            target,
            value,
            index: None,
            field: None,
        }
    }
    pub fn indexed(
        span: ByteSpan,
        target: HirLocalId,
        index: HirExpressionId,
        value: HirExpressionId,
    ) -> Self {
        Self {
            span,
            target,
            value,
            index: Some(index),
            field: None,
        }
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
    pub fn target(&self) -> HirLocalId {
        self.target
    }
    pub fn value(&self) -> HirExpressionId {
        self.value
    }
    pub fn index(&self) -> Option<HirExpressionId> {
        self.index
    }
    pub fn field(&self) -> Option<(HirExpressionId, usize)> {
        self.field
    }
    pub fn field_assignment(
        span: ByteSpan,
        receiver: HirExpressionId,
        index: usize,
        value: HirExpressionId,
    ) -> Self {
        Self {
            span,
            target: HirLocalId::from_raw(usize::MAX),
            value,
            index: None,
            field: Some((receiver, index)),
        }
    }
}
impl HirReturn {
    pub fn new(span: ByteSpan, expression: HirExpressionId) -> Self {
        Self { span, expression }
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
    pub fn expression(&self) -> HirExpressionId {
        self.expression
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HirSafetyFacts {
    executable_subset_checked: bool,
}
impl HirSafetyFacts {
    pub fn executable_subset_checked() -> Self {
        Self {
            executable_subset_checked: true,
        }
    }
    pub fn is_executable_subset_checked(&self) -> bool {
        self.executable_subset_checked
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HirUnsupportedForm {
    span: ByteSpan,
}
impl HirUnsupportedForm {
    pub fn new(span: ByteSpan) -> Self {
        Self { span }
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HirFunction {
    id: HirFunctionId,
    module: ModuleName,
    package: PackageNamespace,
    span: ByteSpan,
    entry: bool,
    return_type: TypeId,
    parameters: Vec<HirParameter>,
    locals: Vec<HirLocal>,
    expressions: Vec<HirExpression>,
    returns: Vec<HirReturn>,
    assignments: Vec<HirAssignment>,
    control_flow: Vec<HirControlFlow>,
    safety_facts: HirSafetyFacts,
    unsupported_forms: Vec<HirUnsupportedForm>,
    symbol_identity: Option<FunctionSymbolIdentity>,
    effect_contract: Option<OwnershipEffectContract>,
}
impl HirFunction {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: HirFunctionId,
        module: ModuleName,
        package: PackageNamespace,
        span: ByteSpan,
        entry: bool,
        return_type: TypeId,
        parameters: Vec<HirParameter>,
        locals: Vec<HirLocal>,
        expressions: Vec<HirExpression>,
        returns: Vec<HirReturn>,
        safety_facts: HirSafetyFacts,
        unsupported_forms: Vec<HirUnsupportedForm>,
    ) -> Self {
        Self {
            id,
            module,
            package,
            span,
            entry,
            return_type,
            parameters,
            locals,
            expressions,
            returns,
            assignments: Vec::new(),
            control_flow: Vec::new(),
            safety_facts,
            unsupported_forms,
            symbol_identity: None,
            effect_contract: None,
        }
    }
    pub fn id(&self) -> HirFunctionId {
        self.id
    }
    pub fn module(&self) -> &ModuleName {
        &self.module
    }
    pub fn package(&self) -> &PackageNamespace {
        &self.package
    }
    pub fn span(&self) -> ByteSpan {
        self.span
    }
    pub fn is_entry(&self) -> bool {
        self.entry
    }
    pub fn return_type(&self) -> TypeId {
        self.return_type
    }
    pub fn parameters(&self) -> &[HirParameter] {
        &self.parameters
    }
    pub fn locals(&self) -> &[HirLocal] {
        &self.locals
    }
    pub fn expressions(&self) -> &[HirExpression] {
        &self.expressions
    }
    pub fn returns(&self) -> &[HirReturn] {
        &self.returns
    }
    pub fn with_assignments(mut self, assignments: Vec<HirAssignment>) -> Self {
        self.assignments = assignments;
        self
    }
    pub fn assignments(&self) -> &[HirAssignment] {
        &self.assignments
    }
    pub fn with_control_flow(mut self, control_flow: Vec<HirControlFlow>) -> Self {
        self.control_flow = control_flow;
        self
    }
    pub fn control_flow(&self) -> &[HirControlFlow] {
        &self.control_flow
    }
    pub fn safety_facts(&self) -> &HirSafetyFacts {
        &self.safety_facts
    }
    pub fn unsupported_forms(&self) -> &[HirUnsupportedForm] {
        &self.unsupported_forms
    }
    pub fn with_symbol_identity(mut self, identity: FunctionSymbolIdentity) -> Self {
        self.symbol_identity = Some(identity);
        self
    }
    pub fn with_effect_contract(mut self, contract: OwnershipEffectContract) -> Self {
        self.effect_contract = Some(contract);
        self
    }
    pub fn effect_contract(&self) -> Option<&OwnershipEffectContract> {
        self.effect_contract.as_ref()
    }
    pub fn symbol_identity(&self) -> Option<&FunctionSymbolIdentity> {
        self.symbol_identity.as_ref()
    }
    pub fn direct_call(&self, id: HirExpressionId) -> Option<&HirDirectCall> {
        self.expressions
            .iter()
            .find(|expression| expression.id == id)
            .and_then(|expression| match &expression.kind {
                HirExpressionKind::DirectCall(call) => Some(call),
                _ => None,
            })
    }
    pub fn local_read(&self, id: HirExpressionId) -> Option<HirLocalId> {
        self.expressions
            .iter()
            .find(|expression| expression.id == id)
            .and_then(|expression| match expression.kind {
                HirExpressionKind::LocalRead(local) => Some(local),
                _ => None,
            })
    }
    pub fn parameter_read(&self, id: HirExpressionId) -> Option<HirParameterId> {
        self.expressions
            .iter()
            .find(|expression| expression.id == id)
            .and_then(|expression| match expression.kind {
                HirExpressionKind::ParameterRead(parameter) => Some(parameter),
                _ => None,
            })
    }
    pub fn unary(&self, id: HirExpressionId) -> Option<&HirUnary> {
        self.expressions
            .iter()
            .find(|expression| expression.id == id)
            .and_then(|expression| match &expression.kind {
                HirExpressionKind::Unary(unary) => Some(unary),
                _ => None,
            })
    }
    pub fn binary(&self, id: HirExpressionId) -> Option<&HirBinary> {
        self.expressions
            .iter()
            .find(|expression| expression.id == id)
            .and_then(|expression| match &expression.kind {
                HirExpressionKind::Binary(binary) => Some(binary),
                _ => None,
            })
    }
}

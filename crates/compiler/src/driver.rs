use std::{
    fs,
    path::{Path, PathBuf},
};

use target_lexicon::Triple;

use crate::{
    backend::{CraneliftLoweringError, emit_mir_module_to_object_for_target},
    hir::{CheckedHirSource, HirLoweringError, lower_checked_hir_source},
    linker::{LinkInvocation, LinkInvocationError},
    mir::{MirLoweringError, lower_hir_to_mir},
    module::{ModuleName, PackageNamespace},
    name_resolution::{
        bind_local_name_references, build_local_scope_tree, build_scoped_local_binding_index,
    },
    ownership::{analyze_ownership_with_extra_transfers, collect_ownership_call_transfers},
    ownership_effects::infer_source_parameter_effects,
    parser,
    source::SourceFileId,
    target_pack::{TargetPackRegistry, TargetPackRegistryError},
    type_check::{
        ConstructorDiagnostic, DeclarationSignature, DirectCallDiagnostic, EntryPointDiagnostic,
        ExecutableSourceTypes, ReturnPathDiagnostic, ReturnTypeDiagnostic, TypeCheckDiagnostic,
        TypeRuleDiagnostic, UnsupportedExecutableFormDiagnostic, apply_m0028_direct_call_results,
        apply_m0060_control_flow_results, apply_m0068_class_type_facts,
        apply_m0068_field_access_facts, apply_m0070_method_call_facts,
        apply_m0070_receiver_name_facts, apply_m0070_receiver_signatures,
        apply_m0077_value_conditional_results, apply_m0081_enum_constructor_facts,
        apply_m0081_enum_function_facts, check_m0028_direct_calls, check_m0028_entry_point,
        check_m0028_return_expression_types, check_m0028_straight_line_returns,
        check_m0028_unsupported_executable_forms, check_m0069_constructor_calls,
        check_m0087_indirect_calls, diagnose_m0090_unresolved_types, infer_m0090_local_types,
        merge_type_check_report, type_m0028_executable_core_in,
        type_m0028_executable_int_operators, type_m0060_control_flow,
        type_m0063_array_expressions_with_classes, type_m0063_function_signatures_in_with_classes,
        type_m0064_string_operations, type_m0068_class_types_in,
        type_m0073_dynamic_array_operations, type_m0077_value_conditionals, type_m0080_enum_whens,
        type_m0086_annotation_type, type_m0088_bind_function_values, type_m0088_lambda_expressions,
        validate_m0061_compile_time_constants, validate_m0090_inferred_assignments,
    },
    types::{PrimitiveType, TypeArena, TypeKind},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceDriverOptions {
    source_file: SourceFileId,
    module: ModuleName,
    package: PackageNamespace,
    target: Triple,
    target_packs: PathBuf,
    output: PathBuf,
}

impl SourceDriverOptions {
    pub fn new(
        source_file: SourceFileId,
        module: ModuleName,
        package: PackageNamespace,
        target: Triple,
        target_packs: impl Into<PathBuf>,
        output: impl Into<PathBuf>,
    ) -> Self {
        Self {
            source_file,
            module,
            package,
            target,
            target_packs: target_packs.into(),
            output: output.into(),
        }
    }

    pub fn source_file(&self) -> SourceFileId {
        self.source_file
    }

    pub fn module(&self) -> &ModuleName {
        &self.module
    }

    pub fn package(&self) -> &PackageNamespace {
        &self.package
    }

    pub fn target(&self) -> Triple {
        self.target.clone()
    }

    pub fn target_packs(&self) -> &Path {
        &self.target_packs
    }

    pub fn output(&self) -> &Path {
        &self.output
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DriverError {
    Io {
        operation: &'static str,
        path: PathBuf,
    },
    LexDiagnostics(Vec<crate::lexer::Diagnostic>),
    ParseDiagnostics(Vec<parser::Diagnostic>),
    EntryPointDiagnostics(Vec<EntryPointDiagnostic>),
    ReturnPathDiagnostics(Vec<ReturnPathDiagnostic>),
    ReturnTypeDiagnostics(Vec<ReturnTypeDiagnostic>),
    UnsupportedExecutableForms(Vec<UnsupportedExecutableFormDiagnostic>),
    TypeDiagnostics(Vec<TypeCheckDiagnostic>),
    ConstructorDiagnostics(Vec<ConstructorDiagnostic>),
    DirectCallDiagnostics(Vec<DirectCallDiagnostic>),
    OwnershipDiagnostics(Vec<crate::ownership::OwnershipDiagnostic>),
    Hir(HirLoweringError),
    Mir(MirLoweringError),
    Backend(CraneliftLoweringError),
    TargetPack(TargetPackRegistryError),
    Link(LinkInvocationError),
}

pub fn compile_source_to_executable(
    source: &str,
    options: SourceDriverOptions,
) -> Result<PathBuf, DriverError> {
    let parsed = parser::parse_source(options.source_file(), source);
    if !parsed.lex_diagnostics.is_empty() {
        return Err(DriverError::LexDiagnostics(parsed.lex_diagnostics.clone()));
    }
    if !parsed.diagnostics.is_empty() {
        return Err(DriverError::ParseDiagnostics(parsed.diagnostics.clone()));
    }

    let entry = check_m0028_entry_point(
        options.package(),
        &[crate::type_check::EntryPointFile::new(
            options.package(),
            &parsed,
        )],
    );
    if !entry.diagnostics().is_empty() {
        return Err(DriverError::EntryPointDiagnostics(
            entry.diagnostics().to_vec(),
        ));
    }

    let pack = TargetPackRegistry::new(options.target_packs())
        .resolve(options.target())
        .map_err(DriverError::TargetPack)?;
    let mut types = TypeArena::new();
    let class_types =
        type_m0068_class_types_in(&mut types, &parsed, options.module(), options.package());
    if !class_types.diagnostics().is_empty() {
        return Err(DriverError::TypeDiagnostics(
            class_types.diagnostics().to_vec(),
        ));
    }
    let constructor_diagnostics = check_m0069_constructor_calls(&parsed, &class_types);
    if !constructor_diagnostics.is_empty() {
        return Err(DriverError::ConstructorDiagnostics(constructor_diagnostics));
    }
    let mut signatures = type_m0063_function_signatures_in_with_classes(
        &mut types,
        &parsed.function_declarations,
        &parsed.function_parameters,
        &parsed.type_name_references,
        &parsed.array_types,
        class_types.classes(),
    );
    let missing_function_signatures = parsed
        .function_declarations
        .iter()
        .filter(|function| {
            !signatures
                .iter()
                .any(|signature| signature.declaration() == function.declaration)
        })
        .filter_map(|function| {
            let parameters = parsed
                .function_parameters
                .iter()
                .filter(|parameter| parameter.function == function.declaration)
                .map(|parameter| {
                    type_m0086_annotation_type(
                        &parsed,
                        parameter.annotation,
                        &mut types,
                        class_types.classes(),
                    )
                })
                .collect::<Option<Vec<_>>>()?;
            let return_type = type_m0086_annotation_type(
                &parsed,
                function.return_annotation?,
                &mut types,
                class_types.classes(),
            )?;
            Some(crate::type_check::FunctionSignature::new(
                function.declaration,
                parameters,
                return_type,
            ))
        })
        .collect::<Vec<_>>();
    signatures.extend(missing_function_signatures);
    apply_m0070_receiver_signatures(&parsed, &class_types, &mut signatures);
    let mut report = type_m0028_executable_core_in(
        &mut types,
        &parsed.arena,
        &parsed.local_declarations,
        &parsed.type_name_references,
        &parsed.literal_expressions,
        &parsed.integer_literals,
        &parsed.grouped_expressions,
        &parsed.unary_expressions,
        &parsed.binary_expressions,
        &parsed.assignment_statements,
        &crate::name_resolution::ResolutionTable::new(),
        &[],
    );
    apply_m0068_class_type_facts(&parsed, &class_types, &mut report);
    for local in &parsed.local_declarations {
        let Some(annotation) = local.annotation else {
            continue;
        };
        if let Some(ty) =
            type_m0086_annotation_type(&parsed, annotation, &mut types, class_types.classes())
        {
            report.record_declaration_signature(crate::type_check::DeclarationSignature::new(
                local.declaration,
                ty,
            ));
        }
    }
    type_m0063_array_expressions_with_classes(
        &mut types,
        &parsed,
        &mut report,
        class_types.classes(),
    );
    type_m0073_dynamic_array_operations(&parsed, &mut report, &types);
    type_m0064_string_operations(&parsed, &mut report, &mut types, &parsed.array_types);
    apply_m0070_receiver_name_facts(&parsed, &class_types, &mut report);
    apply_m0068_field_access_facts(&parsed, &class_types, &mut report);
    apply_m0070_method_call_facts(&parsed, &class_types, &mut report);
    apply_m0081_enum_constructor_facts(&parsed, &class_types, &mut report, &mut types);
    apply_m0081_enum_function_facts(&parsed, &class_types, &mut report, &mut types);
    type_m0088_bind_function_values(&parsed, &signatures, &mut types, &mut report);
    type_m0088_lambda_expressions(&parsed, &mut types, &mut report);
    infer_m0090_local_types(&parsed, &mut report);
    type_m0088_bind_function_values(&parsed, &signatures, &mut types, &mut report);
    apply_m0070_receiver_name_facts(&parsed, &class_types, &mut report);
    apply_m0068_field_access_facts(&parsed, &class_types, &mut report);
    apply_m0070_method_call_facts(&parsed, &class_types, &mut report);
    if let Some(int_type) = types
        .records()
        .iter()
        .find(|record| record.kind() == &TypeKind::Primitive(PrimitiveType::Int))
        .map(|record| record.id())
    {
        let int_unary = parsed
            .unary_expressions
            .iter()
            .filter(|expression| {
                report
                    .expression_type(expression.operand)
                    .is_none_or(|ty| ty == int_type)
            })
            .cloned()
            .collect::<Vec<_>>();
        let int_binary = parsed
            .binary_expressions
            .iter()
            .filter(|expression| {
                report
                    .expression_type(expression.left)
                    .is_none_or(|ty| ty == int_type)
                    || report
                        .expression_type(expression.right)
                        .is_none_or(|ty| ty == int_type)
            })
            .cloned()
            .collect::<Vec<_>>();
        let inferred_operators = type_m0028_executable_int_operators(
            &int_unary,
            &int_binary,
            &parsed.grouped_expressions,
            report.expression_types(),
            int_type,
        );
        merge_type_check_report(&mut report, inferred_operators);
    }
    infer_m0090_local_types(&parsed, &mut report);
    type_m0088_bind_function_values(&parsed, &signatures, &mut types, &mut report);
    diagnose_m0090_unresolved_types(&parsed, &mut report);
    validate_m0090_inferred_assignments(&parsed, &mut report, &types);
    let indirect_calls = check_m0087_indirect_calls(&parsed, report.expression_types(), &types);
    merge_type_check_report(&mut report, indirect_calls);
    let function_typed_calls = parsed
        .call_expressions
        .iter()
        .filter(|call| {
            report
                .expression_type(call.callee)
                .and_then(|ty| types.get(ty))
                .is_some_and(|record| matches!(record.kind(), TypeKind::Function(_)))
        })
        .map(|call| call.expression)
        .collect::<Vec<_>>();
    report.retain_diagnostics(|diagnostic| {
        !function_typed_calls.contains(&diagnostic.node())
            || diagnostic.rule() != TypeRuleDiagnostic::DirectCallDeferred
    });
    let calls = check_m0028_direct_calls(&[ExecutableSourceTypes::new(
        options.package(),
        &parsed,
        &signatures,
        report.expression_types(),
    )
    .with_class_types(&class_types)]);
    if !calls.diagnostics().is_empty() {
        return Err(DriverError::DirectCallDiagnostics(
            calls.diagnostics().to_vec(),
        ));
    }
    apply_m0028_direct_call_results(&mut report, &parsed, &calls);
    let conditional_report =
        type_m0077_value_conditionals(&parsed, report.expression_types(), &types);
    apply_m0077_value_conditional_results(&mut report, &conditional_report);
    let when_report =
        type_m0080_enum_whens(&parsed, report.expression_types(), &class_types, &mut types);
    merge_type_check_report(&mut report, when_report);
    let statement_conditionals = parsed
        .if_statements
        .iter()
        .map(|statement| statement.expression)
        .collect::<Vec<_>>();
    report.retain_diagnostics(|diagnostic| {
        diagnostic.rule() != TypeRuleDiagnostic::IfValueDeferred
            || statement_conditionals.contains(&diagnostic.node())
    });
    let expression_types = report.expression_types().to_vec();
    validate_m0061_compile_time_constants(&parsed, &expression_types, &types, &mut report);
    report.retain_diagnostics(|diagnostic| {
        !matches!(
            diagnostic.kind(),
            crate::type_check::TypeCheckDiagnosticKind::UnresolvedTypeRule
        ) || !parsed.local_declarations.iter().any(|declaration| {
            declaration.annotation.is_some_and(|annotation| {
                parsed
                    .array_types
                    .iter()
                    .any(|array| array.array == annotation)
            }) && diagnostic.node() == declaration.declaration
        })
    });
    let int_type = types
        .records()
        .iter()
        .find(|record| record.kind() == &TypeKind::Primitive(PrimitiveType::Int))
        .map(|record| record.id())
        .expect("bootstrap type checker creates Int");
    let bool_type = types
        .records()
        .iter()
        .find(|record| record.kind() == &TypeKind::Primitive(PrimitiveType::Bool))
        .map(|record| record.id())
        .expect("bootstrap type checker creates Bool");
    let control_flow =
        type_m0060_control_flow(&parsed, report.expression_types(), int_type, bool_type);
    apply_m0060_control_flow_results(&mut report, &parsed, &control_flow);

    let return_paths = check_m0028_straight_line_returns(&parsed);
    if !return_paths.diagnostics().is_empty() {
        return Err(DriverError::ReturnPathDiagnostics(
            return_paths.diagnostics().to_vec(),
        ));
    }
    let return_types =
        check_m0028_return_expression_types(&parsed, &signatures, report.expression_types());
    if !return_types.diagnostics().is_empty() {
        return Err(DriverError::ReturnTypeDiagnostics(
            return_types.diagnostics().to_vec(),
        ));
    }
    let unsupported = check_m0028_unsupported_executable_forms(&parsed);
    if !unsupported.diagnostics().is_empty() {
        return Err(DriverError::UnsupportedExecutableForms(
            unsupported.diagnostics().to_vec(),
        ));
    }
    if !report.diagnostics().is_empty() {
        return Err(DriverError::TypeDiagnostics(report.diagnostics().to_vec()));
    }

    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = crate::symbol::SymbolInterner::new();
    let local_index = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );
    let resolved_locals = bind_local_name_references(
        &parsed.arena,
        &parsed.name_references,
        &scopes,
        local_index.index(),
        &mut interner,
    );
    let local_signatures = parsed
        .local_declarations
        .iter()
        .filter_map(|declaration| {
            let annotation = declaration.annotation?;
            let ty =
                type_m0086_annotation_type(&parsed, annotation, &mut types, class_types.classes())?;
            Some(DeclarationSignature::new(declaration.declaration, ty))
        })
        .collect::<Vec<_>>();
    let call_transfers = collect_ownership_call_transfers(
        &parsed,
        resolved_locals.resolved_local_bindings(),
        &signatures,
        &types,
    );
    let ownership = analyze_ownership_with_extra_transfers(
        &parsed.local_declarations,
        &parsed.assignment_statements,
        resolved_locals.resolved_local_bindings(),
        &local_signatures,
        &types,
        &call_transfers,
    );
    if !ownership.diagnostics().is_empty() {
        return Err(DriverError::OwnershipDiagnostics(
            ownership.diagnostics().to_vec(),
        ));
    }

    let byte_type = types
        .records()
        .iter()
        .find(|record| record.kind() == &TypeKind::Primitive(PrimitiveType::Byte))
        .map(|record| record.id())
        .expect("bootstrap type checker creates Byte");
    let effect_contracts = parsed
        .function_declarations
        .iter()
        .map(|function| infer_source_parameter_effects(&parsed, function.declaration))
        .collect::<Vec<_>>();
    let hir = lower_checked_hir_source(
        CheckedHirSource::new(
            options.module().clone(),
            options.package().clone(),
            &parsed,
            &signatures,
            report.expression_types(),
            true,
        )
        .with_byte_type(byte_type)
        .with_type_arena(&types)
        .with_effect_contracts(&effect_contracts)
        .with_class_types(&class_types)
        .with_call_targets(calls.resolved_declarations()),
    )
    .map_err(DriverError::Hir)?;
    let mir = lower_hir_to_mir(&hir, &types).map_err(DriverError::Mir)?;
    let object = emit_mir_module_to_object_for_target(
        &mir,
        &types,
        pack.language_entry_symbol(),
        options.target(),
    )
    .map_err(DriverError::Backend)?;

    let object_path = PathBuf::from(format!("{}.o", options.output().display()));
    fs::write(&object_path, object).map_err(|_| DriverError::Io {
        operation: "write object",
        path: object_path.clone(),
    })?;
    let invocation =
        LinkInvocation::new(&pack, &object_path, options.output()).map_err(DriverError::Link)?;
    invocation.execute().map_err(DriverError::Link)?;
    Ok(options.output().to_owned())
}

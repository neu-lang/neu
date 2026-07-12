use std::{
    fs,
    path::{Path, PathBuf},
};

use target_lexicon::Triple;

use crate::{
    backend::{CraneliftLoweringError, emit_mir_module_to_object_for_target},
    hir::{CheckedHirSource, HirLoweringError, lower_checked_hir_source},
    linker::{LinkInvocationError, SystemLinkInvocation},
    mir::{MirLoweringError, lower_hir_to_mir},
    module::{
        ModuleName, PackageGraphDiagnostic, PackageNamespace, VirtualPackageGraph, VirtualSource,
    },
    name_resolution::{
        bind_local_name_references, build_local_scope_tree, build_scoped_local_binding_index,
    },
    ownership::{
        analyze_ownership_with_extra_transfers, collect_ownership_call_transfers,
        collect_ownership_capture_transfers,
    },
    ownership_effects::infer_source_parameter_effects,
    parser,
    source::SourceFileId,
    target_pack::TargetPackRegistryError,
    type_check::{
        ConstructorDiagnostic, DeclarationSignature, DirectCallDiagnostic, EntryPointDiagnostic,
        ExecutableSourceTypes, ReturnPathDiagnostic, ReturnTypeDiagnostic, TypeCheckDiagnostic,
        TypeRuleDiagnostic, UnsupportedExecutableFormDiagnostic, apply_class_type_facts,
        apply_control_flow_results, apply_direct_call_results, apply_enum_constructor_facts,
        apply_enum_function_facts, apply_field_access_facts, apply_method_call_facts,
        apply_receiver_name_facts, apply_receiver_signatures, apply_value_conditional_results,
        check_constructor_calls, check_direct_calls, check_entry_point, check_indirect_calls,
        check_return_expression_types, check_straight_line_returns,
        check_unsupported_executable_forms, diagnose_unresolved_types, infer_local_types,
        merge_type_check_report, type_annotation_type, type_array_expressions_with_classes,
        type_array_iterations, type_bind_function_values, type_class_types_in, type_control_flow,
        type_dynamic_array_operations, type_enum_whens, type_executable_core_in,
        type_executable_int_operators, type_function_signatures_in_with_classes,
        type_lambda_expressions, type_string_operations, type_value_conditionals,
        validate_compile_time_constants, validate_inferred_assignments,
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
    HostOnlyTarget(Triple),
    Link(LinkInvocationError),
    PackageGraph(Vec<PackageGraphDiagnostic>),
    Manifest(crate::manifest::ManifestDiagnostic),
    Dependency(crate::dependency::DependencyDiagnostic),
}

pub fn validate_virtual_project(
    entry: impl Into<PathBuf>,
    sources: impl IntoIterator<Item = VirtualSource>,
) -> Result<VirtualPackageGraph, DriverError> {
    VirtualPackageGraph::build(entry, sources).map_err(DriverError::PackageGraph)
}

pub fn compile_virtual_project_to_executable(
    project: &VirtualPackageGraph,
    options: SourceDriverOptions,
) -> Result<PathBuf, DriverError> {
    compile_source_to_executable(&project.bootstrap_source(), options)
}

pub fn compile_manifest_to_executable(
    manifest_path: impl AsRef<std::path::Path>,
    output: impl Into<PathBuf>,
) -> Result<PathBuf, DriverError> {
    let manifest_path = manifest_path.as_ref();
    let resolver = crate::dependency::GitDependencyResolver::from_environment()
        .map_err(DriverError::Dependency)?;
    resolver
        .resolve_project(manifest_path)
        .map_err(DriverError::Dependency)?;
    let (manifest, root) =
        crate::manifest::ProjectManifest::load(manifest_path).map_err(DriverError::Manifest)?;
    let sources = manifest
        .load_sources(&root)
        .map_err(DriverError::Manifest)?;
    let entry = manifest.entrypoint().to_path_buf();
    let graph = validate_virtual_project(entry.clone(), sources)?;
    let entry_file = graph
        .files()
        .iter()
        .find(|file| file.path == entry)
        .expect("manifest entrypoint is in the source graph");
    let module =
        ModuleName::parse(manifest.name()).expect("manifest name was validated while loading");
    compile_virtual_project_to_executable(
        &graph,
        SourceDriverOptions::new(
            entry_file.id,
            module,
            PackageNamespace::root(),
            Triple::host(),
            PathBuf::new(),
            output,
        ),
    )
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

    let entry = check_entry_point(
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

    let target = options.target();
    if target != Triple::host() {
        return Err(DriverError::HostOnlyTarget(target));
    }
    let mut types = TypeArena::new();
    let class_types = type_class_types_in(&mut types, &parsed, options.module(), options.package());
    if !class_types.diagnostics().is_empty() {
        return Err(DriverError::TypeDiagnostics(
            class_types.diagnostics().to_vec(),
        ));
    }
    let constructor_diagnostics = check_constructor_calls(&parsed, &class_types);
    if !constructor_diagnostics.is_empty() {
        return Err(DriverError::ConstructorDiagnostics(constructor_diagnostics));
    }
    let mut signatures = type_function_signatures_in_with_classes(
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
                    type_annotation_type(
                        &parsed,
                        parameter.annotation,
                        &mut types,
                        class_types.classes(),
                    )
                })
                .collect::<Option<Vec<_>>>()?;
            let return_type = type_annotation_type(
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
    apply_receiver_signatures(&parsed, &class_types, &mut signatures);
    let mut report = type_executable_core_in(
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
    apply_class_type_facts(&parsed, &class_types, &mut report);
    for local in &parsed.local_declarations {
        let Some(annotation) = local.annotation else {
            continue;
        };
        if let Some(ty) =
            type_annotation_type(&parsed, annotation, &mut types, class_types.classes())
        {
            report.record_declaration_signature(crate::type_check::DeclarationSignature::new(
                local.declaration,
                ty,
            ));
        }
    }
    type_array_expressions_with_classes(&mut types, &parsed, &mut report, class_types.classes());
    type_dynamic_array_operations(&parsed, &mut report, &types);
    type_string_operations(&parsed, &mut report, &mut types, &parsed.array_types);
    apply_receiver_name_facts(&parsed, &class_types, &mut report);
    apply_field_access_facts(&parsed, &class_types, &mut report);
    apply_method_call_facts(&parsed, &class_types, &mut report);
    apply_enum_constructor_facts(&parsed, &class_types, &mut report, &mut types);
    apply_enum_function_facts(&parsed, &class_types, &mut report, &mut types);
    type_bind_function_values(&parsed, &signatures, &mut types, &mut report);
    type_lambda_expressions(&parsed, &mut types, &mut report);
    let array_iterations = type_array_iterations(&parsed, report.expression_types(), &types);
    merge_type_check_report(&mut report, array_iterations);
    infer_local_types(&parsed, &mut report);
    type_bind_function_values(&parsed, &signatures, &mut types, &mut report);
    apply_receiver_name_facts(&parsed, &class_types, &mut report);
    apply_field_access_facts(&parsed, &class_types, &mut report);
    apply_method_call_facts(&parsed, &class_types, &mut report);
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
        let inferred_operators = type_executable_int_operators(
            &int_unary,
            &int_binary,
            &parsed.grouped_expressions,
            report.expression_types(),
            int_type,
        );
        merge_type_check_report(&mut report, inferred_operators);
    }
    infer_local_types(&parsed, &mut report);
    type_bind_function_values(&parsed, &signatures, &mut types, &mut report);
    diagnose_unresolved_types(&parsed, &mut report);
    validate_inferred_assignments(&parsed, &mut report, &types);
    let indirect_calls = check_indirect_calls(&parsed, report.expression_types(), &types);
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
    let calls = check_direct_calls(&[ExecutableSourceTypes::new(
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
    apply_direct_call_results(&mut report, &parsed, &calls);
    let conditional_report = type_value_conditionals(&parsed, report.expression_types(), &types);
    apply_value_conditional_results(&mut report, &conditional_report);
    let when_report = type_enum_whens(&parsed, report.expression_types(), &class_types, &mut types);
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
    validate_compile_time_constants(&parsed, &expression_types, &types, &mut report);
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
    let control_flow = type_control_flow(&parsed, report.expression_types(), int_type, bool_type);
    apply_control_flow_results(&mut report, &parsed, &control_flow);
    let array_iterations = type_array_iterations(&parsed, report.expression_types(), &types);
    merge_type_check_report(&mut report, array_iterations);

    let return_paths = check_straight_line_returns(&parsed);
    if !return_paths.diagnostics().is_empty() {
        return Err(DriverError::ReturnPathDiagnostics(
            return_paths.diagnostics().to_vec(),
        ));
    }
    let return_types =
        check_return_expression_types(&parsed, &signatures, report.expression_types());
    if !return_types.diagnostics().is_empty() {
        return Err(DriverError::ReturnTypeDiagnostics(
            return_types.diagnostics().to_vec(),
        ));
    }
    let unsupported = check_unsupported_executable_forms(&parsed);
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
            let ty = type_annotation_type(&parsed, annotation, &mut types, class_types.classes())?;
            Some(DeclarationSignature::new(declaration.declaration, ty))
        })
        .collect::<Vec<_>>();
    let call_transfers = collect_ownership_call_transfers(
        &parsed,
        resolved_locals.resolved_local_bindings(),
        &signatures,
        &types,
    );
    let capture_transfers = collect_ownership_capture_transfers(
        &parsed,
        resolved_locals.resolved_local_bindings(),
        &local_signatures,
        &types,
    );
    let mut ownership_transfers = call_transfers;
    ownership_transfers.extend(capture_transfers);
    let ownership = analyze_ownership_with_extra_transfers(
        &parsed.local_declarations,
        &parsed.assignment_statements,
        resolved_locals.resolved_local_bindings(),
        &local_signatures,
        &types,
        &ownership_transfers,
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
    let object = emit_mir_module_to_object_for_target(&mir, &types, "main", target)
        .map_err(DriverError::Backend)?;

    let object_path = PathBuf::from(format!("{}.o", options.output().display()));
    fs::write(&object_path, object).map_err(|_| DriverError::Io {
        operation: "write object",
        path: object_path.clone(),
    })?;
    let invocation =
        SystemLinkInvocation::new(&object_path, options.output()).map_err(DriverError::Link)?;
    invocation.execute().map_err(DriverError::Link)?;
    Ok(options.output().to_owned())
}

use std::{
    fs,
    path::{Path, PathBuf},
};

use target_lexicon::Triple;

use crate::{
    backend::{CraneliftLoweringError, emit_mir_module_to_object},
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
    type_check::{
        ConstructorDiagnostic, DirectCallDiagnostic, EntryPointDiagnostic, ExecutableSourceTypes,
        IntrinsicDiagnostic, ReturnPathDiagnostic, ReturnTypeDiagnostic, TestDiagnostic,
        TypeCheckDiagnostic, TypeRuleDiagnostic, UnsupportedExecutableFormDiagnostic,
        apply_class_type_facts, apply_control_flow_results, apply_direct_call_results,
        apply_enum_constructor_facts, apply_enum_function_facts, apply_enum_method_call_facts,
        apply_field_access_facts, apply_intrinsic_call_facts, apply_method_call_facts,
        apply_receiver_name_facts, apply_receiver_signatures, apply_value_conditional_results,
        check_constructor_calls, check_direct_calls, check_entry_point, check_indirect_calls,
        check_return_expression_types, check_straight_line_returns,
        check_unsupported_executable_forms, diagnose_unresolved_types, infer_local_types,
        merge_type_check_report, type_annotation_type, type_array_expressions_with_classes,
        type_array_iterations, type_bind_function_values, type_class_types_in,
        type_concurrency_operations, type_control_flow, type_dynamic_array_operations,
        type_enum_whens, type_executable_core_in, type_executable_int_operators,
        type_function_signatures_in_with_generics, type_lambda_expressions, type_string_operations,
        type_value_conditionals, validate_compile_time_constants, validate_concurrency_structure,
        validate_inferred_assignments, validate_intrinsic_calls,
        validate_task_member_cancellation_structure, validate_test_declarations,
    },
    types::{PrimitiveType, TypeArena, TypeKind},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceDriverOptions {
    source_file: SourceFileId,
    module: ModuleName,
    package: PackageNamespace,
    target: Triple,
    output: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestMetadata {
    symbol: String,
    declaration: crate::ast::AstNodeId,
    span: crate::source::ByteSpan,
    source: String,
}

impl TestMetadata {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn declaration(&self) -> crate::ast::AstNodeId {
        self.declaration
    }

    pub fn span(&self) -> crate::source::ByteSpan {
        self.span
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestProject {
    source: String,
    source_file: SourceFileId,
    module: ModuleName,
    package: PackageNamespace,
    tests: Vec<TestMetadata>,
}

impl TestProject {
    pub fn tests(&self) -> &[TestMetadata] {
        &self.tests
    }

    pub fn source(&self) -> &str {
        &self.source
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
}

pub fn discover_tests(
    parsed: &parser::ParseOutput,
    module: &ModuleName,
    package: &PackageNamespace,
) -> Result<Vec<TestMetadata>, DriverError> {
    let diagnostics = validate_test_declarations(parsed);
    if !diagnostics.is_empty() {
        return Err(DriverError::TestDiagnostics(diagnostics));
    }
    let mut tests = parsed
        .function_declarations
        .iter()
        .filter(|function| function.is_test)
        .filter_map(|function| {
            let span = parsed.arena.node(function.declaration)?.span;
            let package_name = package.as_str();
            let symbol = if package_name.is_empty() {
                format!("{}::{}", module.as_str(), function.name)
            } else {
                format!("{}::{}::{}", module.as_str(), package_name, function.name)
            };
            Some(TestMetadata {
                symbol,
                declaration: function.declaration,
                span,
                source: String::new(),
            })
        })
        .collect::<Vec<_>>();
    tests.sort_by(|left, right| {
        left.symbol
            .cmp(&right.symbol)
            .then_with(|| left.span.file().cmp(&right.span.file()))
            .then_with(|| left.span.start().cmp(&right.span.start()))
    });
    Ok(tests)
}

impl SourceDriverOptions {
    pub fn new(
        source_file: SourceFileId,
        module: ModuleName,
        package: PackageNamespace,
        output: impl Into<PathBuf>,
    ) -> Self {
        Self::for_target(source_file, module, package, Triple::host(), output)
    }

    pub fn for_target(
        source_file: SourceFileId,
        module: ModuleName,
        package: PackageNamespace,
        target: Triple,
        output: impl Into<PathBuf>,
    ) -> Self {
        Self {
            source_file,
            module,
            package,
            target,
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
    TestDiagnostics(Vec<TestDiagnostic>),
    IntrinsicDiagnostics(Vec<IntrinsicDiagnostic>),
    ConstructorDiagnostics(Vec<ConstructorDiagnostic>),
    DirectCallDiagnostics(Vec<DirectCallDiagnostic>),
    OwnershipDiagnostics(Vec<crate::ownership::OwnershipDiagnostic>),
    Hir(HirLoweringError),
    Mir(MirLoweringError),
    Backend(CraneliftLoweringError),
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
    compile_manifest_to_executable_for_target(manifest_path, Triple::host(), output)
}

pub fn compile_manifest_to_executable_for_target(
    manifest_path: impl AsRef<std::path::Path>,
    target: Triple,
    output: impl Into<PathBuf>,
) -> Result<PathBuf, DriverError> {
    let manifest_path = manifest_path.as_ref();
    let resolver = crate::dependency::GitDependencyResolver::from_environment()
        .map_err(DriverError::Dependency)?;
    let dependencies = resolver
        .load_project_dependencies(manifest_path)
        .map_err(DriverError::Dependency)?;
    let (manifest, root) =
        crate::manifest::ProjectManifest::load(manifest_path).map_err(DriverError::Manifest)?;
    let sources = manifest
        .load_sources(&root)
        .map_err(DriverError::Manifest)?;
    let entry = manifest
        .require_entrypoint()
        .map_err(DriverError::Manifest)?
        .to_path_buf();
    let graph = VirtualPackageGraph::build_with_dependencies(entry.clone(), sources, dependencies)
        .map_err(DriverError::PackageGraph)?;
    let entry_file = graph
        .files()
        .iter()
        .find(|file| file.path == entry)
        .expect("manifest entrypoint is in the source graph");
    let module =
        ModuleName::parse(manifest.name()).expect("manifest name was validated while loading");
    compile_virtual_project_to_executable(
        &graph,
        SourceDriverOptions::for_target(
            entry_file.id,
            module,
            PackageNamespace::root(),
            target,
            output,
        ),
    )
}

pub fn discover_manifest_tests(
    manifest_path: impl AsRef<std::path::Path>,
) -> Result<TestProject, DriverError> {
    let manifest_path = manifest_path.as_ref();
    let resolver = crate::dependency::GitDependencyResolver::from_environment()
        .map_err(DriverError::Dependency)?;
    let dependencies = resolver
        .load_project_dependencies(manifest_path)
        .map_err(DriverError::Dependency)?;
    let (manifest, root) =
        crate::manifest::ProjectManifest::load(manifest_path).map_err(DriverError::Manifest)?;
    let sources = manifest
        .load_sources(&root)
        .map_err(DriverError::Manifest)?;
    let entry = manifest.entrypoint().map(ToOwned::to_owned);
    if entry.is_none() {
        let graph = VirtualPackageGraph::build_library(
            sources
                .iter()
                .map(|source| VirtualSource::new(source.path(), source.source())),
        )
        .map_err(DriverError::PackageGraph)?;
        let module = ModuleName::parse(manifest.name()).expect("manifest name was validated");
        let mut tests = Vec::new();
        for source in &sources {
            let source_file = graph
                .files()
                .iter()
                .find(|file| file.path == source.path())
                .map(|file| file.id)
                .expect("library source is in the source graph");
            let parsed = parser::parse_source(source_file, source.source());
            if !parsed.lex_diagnostics.is_empty() {
                return Err(DriverError::LexDiagnostics(parsed.lex_diagnostics));
            }
            if !parsed.diagnostics.is_empty() {
                return Err(DriverError::ParseDiagnostics(parsed.diagnostics));
            }
            let mut discovered = discover_tests(&parsed, &module, &PackageNamespace::root())?;
            for test in &mut discovered {
                test.source = source.source().to_owned();
            }
            tests.extend(discovered);
        }
        tests.sort_by(|left, right| {
            left.symbol
                .cmp(&right.symbol)
                .then_with(|| left.span.start().cmp(&right.span.start()))
        });
        let (source, source_file) = sources
            .first()
            .map(|source| {
                let id = graph
                    .files()
                    .iter()
                    .find(|file| file.path == source.path())
                    .unwrap()
                    .id;
                (source.source().to_owned(), id)
            })
            .ok_or_else(|| {
                DriverError::Manifest(crate::manifest::ManifestDiagnostic::missing_sources())
            })?;
        return Ok(TestProject {
            source,
            source_file,
            module,
            package: PackageNamespace::root(),
            tests,
        });
    }
    let entry = entry.expect("entrypoint exists");
    let graph = VirtualPackageGraph::build_with_dependencies(entry.clone(), sources, dependencies)
        .map_err(DriverError::PackageGraph)?;
    let entry_file = graph
        .files()
        .iter()
        .find(|file| file.path == entry)
        .expect("test project entry source is in the source graph");
    let module = ModuleName::parse(manifest.name()).expect("manifest name was validated");
    let source = graph.bootstrap_source();
    let parsed = parser::parse_source(entry_file.id, &source);
    if !parsed.lex_diagnostics.is_empty() {
        return Err(DriverError::LexDiagnostics(parsed.lex_diagnostics));
    }
    if !parsed.diagnostics.is_empty() {
        return Err(DriverError::ParseDiagnostics(parsed.diagnostics));
    }
    let mut tests = discover_tests(&parsed, &module, &PackageNamespace::root())?;
    for test in &mut tests {
        test.source = source.clone();
    }
    Ok(TestProject {
        source,
        source_file: entry_file.id,
        module,
        package: PackageNamespace::root(),
        tests,
    })
}

pub fn compile_source_to_executable(
    source: &str,
    options: SourceDriverOptions,
) -> Result<PathBuf, DriverError> {
    compile_source_with_entry(source, options, None)
}

pub fn compile_source_to_test_executable(
    source: &str,
    options: SourceDriverOptions,
    test_declaration: crate::ast::AstNodeId,
) -> Result<PathBuf, DriverError> {
    compile_source_with_entry(source, options, Some(test_declaration))
}

fn compile_source_with_entry(
    source: &str,
    options: SourceDriverOptions,
    selected_entry: Option<crate::ast::AstNodeId>,
) -> Result<PathBuf, DriverError> {
    let parsed = parser::parse_source(options.source_file(), source);
    if !parsed.lex_diagnostics.is_empty() {
        return Err(DriverError::LexDiagnostics(parsed.lex_diagnostics.clone()));
    }
    if !parsed.diagnostics.is_empty() {
        return Err(DriverError::ParseDiagnostics(parsed.diagnostics.clone()));
    }

    if selected_entry.is_none() {
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
    } else {
        let test_diagnostics = validate_test_declarations(&parsed);
        if !test_diagnostics.is_empty() {
            return Err(DriverError::TestDiagnostics(test_diagnostics));
        }
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
    let mut signatures = type_function_signatures_in_with_generics(
        &mut types,
        &parsed.function_declarations,
        &parsed.function_parameters,
        &parsed.type_name_references,
        &parsed.array_types,
        class_types.classes(),
        &parsed.generic_parameters,
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
            let return_type = function
                .is_test
                .then(|| {
                    types
                        .records()
                        .iter()
                        .find(|record| record.kind() == &TypeKind::Primitive(PrimitiveType::Unit))
                        .map(|record| record.id())
                })
                .flatten()
                .or_else(|| {
                    type_annotation_type(
                        &parsed,
                        function.return_annotation?,
                        &mut types,
                        class_types.classes(),
                    )
                })?;
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
    apply_enum_method_call_facts(&parsed, &class_types, &mut report, &types);
    apply_enum_function_facts(&parsed, &class_types, &mut report, &mut types);
    type_bind_function_values(&parsed, &signatures, &mut types, &mut report);
    type_lambda_expressions(&parsed, &mut types, &mut report);
    let array_iterations = type_array_iterations(&parsed, report.expression_types(), &types);
    merge_type_check_report(&mut report, array_iterations);
    // These facts form a small dependency cycle. A bounded fixed point keeps
    // the pass order explicit while avoiding repeated whole-program rescans.
    for _ in 0..3 {
        infer_local_types(&parsed, &mut report);
        type_lambda_expressions(&parsed, &mut types, &mut report);
        type_bind_function_values(&parsed, &signatures, &mut types, &mut report);
        type_concurrency_operations(&parsed, &mut types, &mut report);
    }
    validate_concurrency_structure(&parsed, &mut report, &types);
    validate_task_member_cancellation_structure(&parsed, &mut report, &types);
    let mut concurrency_calls = parsed
        .call_expressions
        .iter()
        .filter(|call| {
            parsed
                .name_references
                .iter()
                .find(|reference| reference.reference == call.callee)
                .is_some_and(|reference| {
                    matches!(
                        reference.name.as_str(),
                        "spawn" | "await" | "cancel" | "channel" | "send" | "receive" | "close"
                    )
                })
                || parsed.member_expressions.iter().any(|member| {
                    member.expression == call.callee
                        && member.name == "cancel"
                        && report.expression_type(member.receiver).is_some_and(|ty| {
                            types
                                .get(ty)
                                .is_some_and(|record| matches!(record.kind(), TypeKind::Task(_)))
                        })
                })
        })
        .map(|call| call.expression)
        .collect::<Vec<_>>();
    concurrency_calls.extend(
        parsed
            .call_expressions
            .iter()
            .filter(|call| {
                parsed.member_expressions.iter().any(|member| {
                    member.expression == call.callee
                        && member.name == "cancel"
                        && report.expression_type(member.receiver).is_some_and(|ty| {
                            types
                                .get(ty)
                                .is_some_and(|record| matches!(record.kind(), TypeKind::Task(_)))
                        })
                })
            })
            .map(|call| call.callee),
    );
    report.retain_diagnostics(|diagnostic| {
        !concurrency_calls.contains(&diagnostic.node())
            || !matches!(
                diagnostic.rule(),
                TypeRuleDiagnostic::DirectCallDeferred
                    | TypeRuleDiagnostic::MemberExpressionDeferred
            )
    });
    apply_receiver_name_facts(&parsed, &class_types, &mut report);
    apply_field_access_facts(&parsed, &class_types, &mut report);
    apply_method_call_facts(&parsed, &class_types, &mut report);
    apply_enum_method_call_facts(&parsed, &class_types, &mut report, &types);
    report.retain_diagnostics(|diagnostic| {
        !concurrency_calls.contains(&diagnostic.node())
            || !matches!(
                diagnostic.rule(),
                TypeRuleDiagnostic::DirectCallDeferred
                    | TypeRuleDiagnostic::MemberExpressionDeferred
            )
    });
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
    let intrinsic_diagnostics =
        validate_intrinsic_calls(&parsed, report.expression_types(), &types);
    if !intrinsic_diagnostics.is_empty() {
        return Err(DriverError::IntrinsicDiagnostics(intrinsic_diagnostics));
    }
    apply_intrinsic_call_facts(&parsed, &mut report, &types);
    let conditional_report = type_value_conditionals(&parsed, report.expression_types(), &types);
    apply_value_conditional_results(&mut report, &conditional_report);
    let mut string_after_when = crate::type_check::TypeCheckReport::new();
    for expression_type in report.expression_types() {
        string_after_when.record_expression_type(*expression_type);
    }
    type_string_operations(
        &parsed,
        &mut string_after_when,
        &mut types,
        &parsed.array_types,
    );
    for expression_type in string_after_when.expression_types() {
        report.replace_expression_type(*expression_type);
    }
    let when_report = type_enum_whens(&parsed, report.expression_types(), &class_types, &mut types);
    merge_type_check_report(&mut report, when_report);
    let string_member_nodes = parsed
        .member_expressions
        .iter()
        .filter(|member| {
            report.expression_type(member.receiver).is_some_and(|ty| {
                types.get(ty).is_some_and(|record| {
                    record.kind() == &TypeKind::Primitive(PrimitiveType::String)
                })
            })
        })
        .map(|member| member.expression)
        .collect::<Vec<_>>();
    report.retain_diagnostics(|diagnostic| {
        diagnostic.rule() != TypeRuleDiagnostic::MemberExpressionDeferred
            || !string_member_nodes.contains(&diagnostic.node())
    });
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
    let local_signatures = report.declaration_signatures().to_vec();
    let call_transfers = collect_ownership_call_transfers(
        &parsed,
        resolved_locals.resolved_local_bindings(),
        &signatures,
        &local_signatures,
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
    let hir = lower_checked_hir_source({
        let source = CheckedHirSource::new(
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
        .with_call_targets(calls.resolved_declarations());
        match selected_entry {
            Some(entry) => source.with_entry_declaration(entry),
            None => source,
        }
    })
    .map_err(DriverError::Hir)?;
    let mir = lower_hir_to_mir(&hir, &types).map_err(DriverError::Mir)?;
    let object = emit_mir_module_to_object(&mir, &types, "main").map_err(DriverError::Backend)?;

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

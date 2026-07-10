use newlang::ast::{AstArena, AstNodeId};
use newlang::module::{ModuleMetadata, ModuleName, PackageNamespace};
use newlang::name_resolution::{
    analyze_duplicate_enum_variants, analyze_duplicate_match_arms, analyze_when_subjects,
    bind_accepted_name_references, bind_local_name_references,
    bind_package_qualified_type_references, bind_unqualified_function_references,
    bind_unqualified_type_references, build_declaration_index, build_enum_variant_index,
    build_function_parameter_binding_index, build_local_binding_index, build_local_scope_tree,
    build_scoped_binding_index, build_scoped_local_binding_index, resolve_enum_parameter_types,
    resolve_qualified_variant_arms, DeclarationIndex, DeclarationInsert, DeclarationKey,
    DeclarationKind, DeclaredName, LocalBinding, LocalBindingIndex, LocalBindingInsert,
    LocalBindingKey, LocalBindingKind, LocalNameLookup, LocalNameLookupResult, LocalScopeId,
    LocalScopeTree, ResolutionDiagnostic, ResolutionDiagnosticKind, ResolutionInsert,
    ResolutionTable, ResolvedName, TopLevelLookup, TopLevelLookupResult,
};
use newlang::parser::parse_source;
use newlang::source::{ByteSpan, SourceFileId};
use newlang::symbol::{SymbolId, SymbolInterner};

#[test]
fn m0021_duplicate_match_arm_diagnostics_report_second_variant_and_wildcard() {
    let file = SourceFileId::from_raw(811);
    let parsed = parse_source(
        file,
        "enum Signal { Red } fun first(signal: Signal) { when (signal) { Signal.Red -> 0; Signal.Red -> 1; _ -> 2; _ -> 3 } } fun second(signal: Signal) { when (signal) { Signal.Red -> 0 } }",
    );
    let metadata = ModuleMetadata::new(ModuleName::parse("demo.app").unwrap(), [file]).unwrap();
    let mut interner = SymbolInterner::new();
    let variants = build_enum_variant_index(
        &metadata,
        &parsed.enum_variants,
        &parsed.declaration_names,
        &mut interner,
    );
    let enum_types = resolve_enum_parameter_types(
        &parsed.arena,
        &metadata,
        &parsed.function_parameters,
        &parsed.type_name_references,
        &parsed.declaration_names,
        &mut interner,
    );
    let scopes = build_local_scope_tree(&parsed.arena);
    let bindings = build_scoped_binding_index(
        &parsed.arena,
        &parsed.function_parameters,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );
    let names = bind_local_name_references(
        &parsed.arena,
        &parsed.name_references,
        &scopes,
        bindings.index(),
        &mut interner,
    );
    let subjects = analyze_when_subjects(
        &parsed.when_expressions,
        names.resolved_local_bindings(),
        &enum_types,
    );
    let resolved = resolve_qualified_variant_arms(
        &parsed.when_expressions,
        &parsed.match_arms,
        &parsed.qualified_case_patterns,
        subjects.subjects(),
        &variants,
        &mut interner,
    );
    let diagnostics = analyze_duplicate_match_arms(
        &parsed.when_expressions,
        &parsed.match_arms,
        resolved.arms(),
    );

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].node(), parsed.match_arms[1].pattern);
    assert_eq!(diagnostics[1].node(), parsed.match_arms[3].pattern);
}

#[test]
fn m0021_duplicate_enum_variant_diagnoses_only_same_enum_repeat() {
    let file = SourceFileId::from_raw(810);
    let parsed = parse_source(file, "enum Signal { Red, Red } enum Other { Red }");
    let diagnostics =
        analyze_duplicate_enum_variants(&parsed.enum_variants, &mut SymbolInterner::new());

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].node(), parsed.enum_variants[1].variant);
}

#[test]
fn m0021_when_subject_analysis_accepts_enum_parameter_only() {
    let file = SourceFileId::from_raw(806);
    let parsed = parse_source(
        file,
        "enum Signal { Red } fun code(signal: Signal) { when (signal) { _ -> 0 } }",
    );
    let metadata = ModuleMetadata::new(ModuleName::parse("demo.app").unwrap(), [file]).unwrap();
    let mut interner = SymbolInterner::new();
    let enum_types = resolve_enum_parameter_types(
        &parsed.arena,
        &metadata,
        &parsed.function_parameters,
        &parsed.type_name_references,
        &parsed.declaration_names,
        &mut interner,
    );
    let scopes = build_local_scope_tree(&parsed.arena);
    let bindings = build_scoped_binding_index(
        &parsed.arena,
        &parsed.function_parameters,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );
    let names = bind_local_name_references(
        &parsed.arena,
        &parsed.name_references,
        &scopes,
        bindings.index(),
        &mut interner,
    );
    let report = analyze_when_subjects(
        &parsed.when_expressions,
        names.resolved_local_bindings(),
        &enum_types,
    );

    assert!(report.diagnostics().is_empty());
    assert_eq!(
        report.subjects()[0].enum_declaration(),
        parsed.declaration_names[0].declaration
    );

    let non_enum_file = SourceFileId::from_raw(807);
    let non_enum = parse_source(
        non_enum_file,
        "struct Signal {} fun code(signal: Signal) { when (signal) { _ -> 0 } }",
    );
    let non_enum_metadata =
        ModuleMetadata::new(ModuleName::parse("demo.app").unwrap(), [non_enum_file]).unwrap();
    let enum_types = resolve_enum_parameter_types(
        &non_enum.arena,
        &non_enum_metadata,
        &non_enum.function_parameters,
        &non_enum.type_name_references,
        &non_enum.declaration_names,
        &mut interner,
    );
    let scopes = build_local_scope_tree(&non_enum.arena);
    let bindings = build_scoped_binding_index(
        &non_enum.arena,
        &non_enum.function_parameters,
        &non_enum.local_binding_names,
        &scopes,
        &mut interner,
    );
    let names = bind_local_name_references(
        &non_enum.arena,
        &non_enum.name_references,
        &scopes,
        bindings.index(),
        &mut interner,
    );
    let report = analyze_when_subjects(
        &non_enum.when_expressions,
        names.resolved_local_bindings(),
        &enum_types,
    );
    assert_eq!(report.diagnostics().len(), 1);
    assert_eq!(
        report.diagnostics()[0].node(),
        non_enum.when_expressions[0].subject
    );
}

#[test]
fn m0021_qualified_variant_arm_resolves_subject_enum_variant() {
    let file = SourceFileId::from_raw(808);
    let parsed = parse_source(
        file,
        "enum Signal { Red } fun code(signal: Signal) { when (signal) { Signal.Red -> 0 } }",
    );
    let metadata = ModuleMetadata::new(ModuleName::parse("demo.app").unwrap(), [file]).unwrap();
    let mut interner = SymbolInterner::new();
    let variants = build_enum_variant_index(
        &metadata,
        &parsed.enum_variants,
        &parsed.declaration_names,
        &mut interner,
    );
    let enum_types = resolve_enum_parameter_types(
        &parsed.arena,
        &metadata,
        &parsed.function_parameters,
        &parsed.type_name_references,
        &parsed.declaration_names,
        &mut interner,
    );
    let scopes = build_local_scope_tree(&parsed.arena);
    let bindings = build_scoped_binding_index(
        &parsed.arena,
        &parsed.function_parameters,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );
    let names = bind_local_name_references(
        &parsed.arena,
        &parsed.name_references,
        &scopes,
        bindings.index(),
        &mut interner,
    );
    let subjects = analyze_when_subjects(
        &parsed.when_expressions,
        names.resolved_local_bindings(),
        &enum_types,
    );
    let arms = resolve_qualified_variant_arms(
        &parsed.when_expressions,
        &parsed.match_arms,
        &parsed.qualified_case_patterns,
        subjects.subjects(),
        &variants,
        &mut interner,
    );

    assert!(arms.diagnostics().is_empty());
    assert_eq!(arms.arms().len(), 1);
    assert_eq!(arms.arms()[0].variant(), parsed.enum_variants[0].variant);

    let other = parse_source(
        SourceFileId::from_raw(809),
        "enum Signal { Red } enum Other { Red } fun code(signal: Signal) { when (signal) { Other.Red -> 0 } }",
    );
    let other_file = SourceFileId::from_raw(809);
    let other_metadata =
        ModuleMetadata::new(ModuleName::parse("demo.app").unwrap(), [other_file]).unwrap();
    let variants = build_enum_variant_index(
        &other_metadata,
        &other.enum_variants,
        &other.declaration_names,
        &mut interner,
    );
    let enum_types = resolve_enum_parameter_types(
        &other.arena,
        &other_metadata,
        &other.function_parameters,
        &other.type_name_references,
        &other.declaration_names,
        &mut interner,
    );
    let scopes = build_local_scope_tree(&other.arena);
    let bindings = build_scoped_binding_index(
        &other.arena,
        &other.function_parameters,
        &other.local_binding_names,
        &scopes,
        &mut interner,
    );
    let names = bind_local_name_references(
        &other.arena,
        &other.name_references,
        &scopes,
        bindings.index(),
        &mut interner,
    );
    let subjects = analyze_when_subjects(
        &other.when_expressions,
        names.resolved_local_bindings(),
        &enum_types,
    );
    let arms = resolve_qualified_variant_arms(
        &other.when_expressions,
        &other.match_arms,
        &other.qualified_case_patterns,
        subjects.subjects(),
        &variants,
        &mut interner,
    );
    assert_eq!(arms.diagnostics().len(), 1);
    assert_eq!(
        arms.diagnostics()[0].kind(),
        newlang::name_resolution::MatchDiagnosticKind::UnknownMatchVariant
    );
}

#[test]
fn m0021_function_parameter_binding_uses_owning_body_scope() {
    let file = SourceFileId::from_raw(802);
    let parsed = parse_source(
        file,
        "fun code(signal: Signal) { when (signal) { _ -> 0 } }",
    );
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();

    let built = build_function_parameter_binding_index(
        &parsed.arena,
        &parsed.function_parameters,
        &scopes,
        &mut interner,
    );

    assert!(built.diagnostics().is_empty());
    assert_eq!(built.index().bindings().len(), 1);
    assert_eq!(
        built.index().bindings()[0].binding(),
        parsed.function_parameters[0].parameter
    );
    assert_eq!(
        built.index().bindings()[0].kind(),
        LocalBindingKind::Immutable
    );
    assert_eq!(
        built.index().bindings()[0].key().scope(),
        scopes.scopes()[0].id()
    );
}

#[test]
fn m0021_combined_binding_index_resolves_parameter_use() {
    let file = SourceFileId::from_raw(805);
    let parsed = parse_source(
        file,
        "fun code(signal: Signal) { when (signal) { _ -> 0 } }",
    );
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let locals = build_scoped_binding_index(
        &parsed.arena,
        &parsed.function_parameters,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_local_name_references(
        &parsed.arena,
        &parsed.name_references,
        &scopes,
        locals.index(),
        &mut interner,
    );

    assert!(bound.diagnostics().is_empty());
    assert_eq!(
        bound.table().resolved_names()[0].reference(),
        parsed.when_expressions[0].subject
    );
}

#[test]
fn m0021_enum_parameter_type_identity_records_same_package_enum() {
    let file = SourceFileId::from_raw(803);
    let parsed = parse_source(
        file,
        "enum Signal { Red } fun code(signal: Signal) { when (signal) { _ -> 0 } }",
    );
    let metadata = ModuleMetadata::with_packages(
        ModuleName::parse("demo.app").unwrap(),
        [(file, PackageNamespace::parse("demo.pkg").unwrap())],
    )
    .unwrap();
    let mut interner = SymbolInterner::new();

    let resolved = resolve_enum_parameter_types(
        &parsed.arena,
        &metadata,
        &parsed.function_parameters,
        &parsed.type_name_references,
        &parsed.declaration_names,
        &mut interner,
    );

    assert_eq!(resolved.records().len(), 1);
    assert_eq!(
        resolved.records()[0].parameter(),
        parsed.function_parameters[0].parameter
    );
    assert_eq!(
        resolved.records()[0].enum_declaration(),
        parsed.declaration_names[0].declaration
    );

    let non_enum = parse_source(
        SourceFileId::from_raw(804),
        "struct Signal {} fun code(signal: Signal) { when (signal) { _ -> 0 } }",
    );
    let non_enum_metadata = ModuleMetadata::new(
        ModuleName::parse("demo.app").unwrap(),
        [SourceFileId::from_raw(804)],
    )
    .unwrap();
    let non_enum = resolve_enum_parameter_types(
        &non_enum.arena,
        &non_enum_metadata,
        &non_enum.function_parameters,
        &non_enum.type_name_references,
        &non_enum.declaration_names,
        &mut interner,
    );
    assert!(non_enum.records().is_empty());
}

#[test]
fn m0021_enum_variant_identity_preserves_enum_and_variant_source_order() {
    let file = SourceFileId::from_raw(800);
    let parsed = parse_source(file, "enum Signal { Red, Green } enum Status { Red }");
    let metadata = ModuleMetadata::with_packages(
        ModuleName::parse("demo.app").unwrap(),
        [(file, PackageNamespace::parse("demo.pkg").unwrap())],
    )
    .unwrap();
    let mut interner = SymbolInterner::new();

    let index = build_enum_variant_index(
        &metadata,
        &parsed.enum_variants,
        &parsed.declaration_names,
        &mut interner,
    );

    assert_eq!(index.variants().len(), 3);
    let first = &index.variants()[0];
    let third = &index.variants()[2];
    assert_eq!(
        first.enum_declaration(),
        parsed.enum_variants[0].enum_declaration
    );
    assert_eq!(first.variant(), parsed.enum_variants[0].variant);
    assert_eq!(first.variant_name(), third.variant_name());
    assert_ne!(first.enum_declaration(), third.enum_declaration());
    assert_eq!(first.module(), metadata.name());
    assert_eq!(
        first.package(),
        &PackageNamespace::parse("demo.pkg").unwrap()
    );
}

#[test]
fn m0021_enum_variant_identity_accepts_empty_parser_metadata() {
    let file = SourceFileId::from_raw(801);
    let metadata = ModuleMetadata::new(ModuleName::parse("demo.app").unwrap(), [file]).unwrap();
    let mut interner = SymbolInterner::new();
    let index = build_enum_variant_index(&metadata, &[], &[], &mut interner);

    assert!(index.variants().is_empty());
}

#[test]
fn resolved_names_preserve_reference_symbol_and_insertion_order() {
    let mut table = ResolutionTable::new();
    let first = ResolvedName::new(AstNodeId::from_raw(1), SymbolId::from_raw(10));
    let second = ResolvedName::new(AstNodeId::from_raw(2), SymbolId::from_raw(20));

    assert_eq!(table.insert(first), ResolutionInsert::Inserted(first));
    assert_eq!(table.insert(second), ResolutionInsert::Inserted(second));

    assert_eq!(table.resolved_names(), [first, second]);
}

#[test]
fn resolved_name_lookup_uses_reference_node() {
    let mut table = ResolutionTable::new();
    let reference = AstNodeId::from_raw(7);
    let symbol = SymbolId::from_raw(3);

    table.insert(ResolvedName::new(reference, symbol));

    assert_eq!(table.get(reference).unwrap().symbol(), symbol);
    assert_eq!(table.get(AstNodeId::from_raw(8)), None);
}

#[test]
fn duplicate_resolved_name_insert_preserves_existing_record() {
    let mut table = ResolutionTable::new();
    let reference = AstNodeId::from_raw(1);
    let existing = ResolvedName::new(reference, SymbolId::from_raw(10));
    let attempted = ResolvedName::new(reference, SymbolId::from_raw(20));

    assert_eq!(table.insert(existing), ResolutionInsert::Inserted(existing));
    assert_eq!(
        table.insert(attempted),
        ResolutionInsert::Duplicate {
            existing,
            attempted
        }
    );
    assert_eq!(table.get(reference), Some(&existing));
}

#[test]
fn diagnostics_preserve_kind_and_primary_span() {
    let span = ByteSpan::new(SourceFileId::from_raw(0), 4, 9).unwrap();
    let diagnostic = ResolutionDiagnostic::new(ResolutionDiagnosticKind::UnresolvedName, span);

    assert_eq!(diagnostic.kind(), ResolutionDiagnosticKind::UnresolvedName);
    assert_eq!(diagnostic.primary_span(), span);
}

#[test]
fn diagnostic_kinds_cover_accepted_adr0026_variants() {
    let kinds = [
        ResolutionDiagnosticKind::UnresolvedName,
        ResolutionDiagnosticKind::DuplicateName,
        ResolutionDiagnosticKind::AmbiguousName,
        ResolutionDiagnosticKind::UnsupportedImportResolution,
        ResolutionDiagnosticKind::UnsupportedCrossModuleLookup,
        ResolutionDiagnosticKind::UnsupportedMemberResolution,
    ];

    assert_eq!(kinds.len(), 6);
}

#[test]
fn declaration_key_preserves_adr0026_top_level_tuple() {
    let module = ModuleName::parse("demo.app").unwrap();
    let package = PackageNamespace::parse("demo.pkg").unwrap();
    let symbol = SymbolId::from_raw(4);
    let key = DeclarationKey::new(
        module.clone(),
        package.clone(),
        symbol,
        DeclarationKind::Function,
    );

    assert_eq!(key.module(), &module);
    assert_eq!(key.package(), &package);
    assert_eq!(key.name(), symbol);
    assert_eq!(key.kind(), DeclarationKind::Function);
}

#[test]
fn declaration_index_preserves_insertion_order_and_lookup_by_key() {
    let module = ModuleName::parse("demo").unwrap();
    let package = PackageNamespace::root();
    let first_key = DeclarationKey::new(
        module.clone(),
        package.clone(),
        SymbolId::from_raw(1),
        DeclarationKind::Function,
    );
    let second_key = DeclarationKey::new(
        module,
        package,
        SymbolId::from_raw(2),
        DeclarationKind::Type,
    );
    let first = DeclaredName::new(first_key.clone(), AstNodeId::from_raw(10));
    let second = DeclaredName::new(second_key.clone(), AstNodeId::from_raw(11));
    let mut index = DeclarationIndex::new();

    assert_eq!(
        index.insert(first.clone()),
        DeclarationInsert::Inserted(first.clone())
    );
    assert_eq!(
        index.insert(second.clone()),
        DeclarationInsert::Inserted(second.clone())
    );

    assert_eq!(index.declarations(), [first.clone(), second]);
    assert_eq!(index.get(&first_key), Some(&first));
}

#[test]
fn declaration_index_key_includes_module_package_and_kind() {
    let symbol = SymbolId::from_raw(7);
    let first = DeclaredName::new(
        DeclarationKey::new(
            ModuleName::parse("demo.one").unwrap(),
            PackageNamespace::parse("pkg").unwrap(),
            symbol,
            DeclarationKind::Function,
        ),
        AstNodeId::from_raw(1),
    );
    let second = DeclaredName::new(
        DeclarationKey::new(
            ModuleName::parse("demo.two").unwrap(),
            PackageNamespace::parse("pkg").unwrap(),
            symbol,
            DeclarationKind::Function,
        ),
        AstNodeId::from_raw(2),
    );
    let third = DeclaredName::new(
        DeclarationKey::new(
            ModuleName::parse("demo.one").unwrap(),
            PackageNamespace::parse("other").unwrap(),
            symbol,
            DeclarationKind::Function,
        ),
        AstNodeId::from_raw(3),
    );
    let fourth = DeclaredName::new(
        DeclarationKey::new(
            ModuleName::parse("demo.one").unwrap(),
            PackageNamespace::parse("pkg").unwrap(),
            symbol,
            DeclarationKind::Type,
        ),
        AstNodeId::from_raw(4),
    );
    let mut index = DeclarationIndex::new();

    assert!(matches!(
        index.insert(first),
        DeclarationInsert::Inserted(_)
    ));
    assert!(matches!(
        index.insert(second),
        DeclarationInsert::Inserted(_)
    ));
    assert!(matches!(
        index.insert(third),
        DeclarationInsert::Inserted(_)
    ));
    assert!(matches!(
        index.insert(fourth),
        DeclarationInsert::Inserted(_)
    ));
    assert_eq!(index.declarations().len(), 4);
}

#[test]
fn duplicate_declaration_key_preserves_existing_declaration() {
    let key = DeclarationKey::new(
        ModuleName::parse("demo").unwrap(),
        PackageNamespace::root(),
        SymbolId::from_raw(1),
        DeclarationKind::Function,
    );
    let existing = DeclaredName::new(key.clone(), AstNodeId::from_raw(10));
    let attempted = DeclaredName::new(key.clone(), AstNodeId::from_raw(11));
    let mut index = DeclarationIndex::new();

    assert_eq!(
        index.insert(existing.clone()),
        DeclarationInsert::Inserted(existing.clone())
    );
    assert_eq!(
        index.insert(attempted.clone()),
        DeclarationInsert::Duplicate {
            existing: existing.clone(),
            attempted
        }
    );
    assert_eq!(index.get(&key), Some(&existing));
    assert_eq!(index.declarations().len(), 1);
}

#[test]
fn builds_declaration_index_from_parser_metadata_and_module_package() {
    let file = SourceFileId::from_raw(21);
    let parsed = parse_source(file, "fun main(); struct Box {}");
    assert!(parsed.diagnostics.is_empty());
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [(file, PackageNamespace::parse("demo.pkg").unwrap())],
    )
    .unwrap();
    let mut interner = SymbolInterner::new();

    let built = build_declaration_index(&metadata, &parsed.declaration_names, &mut interner);

    assert_eq!(interner.symbols(), ["main", "Box"]);
    assert_eq!(built.index().declarations().len(), 2);
    assert_eq!(built.inserts().len(), 2);
    assert!(built
        .inserts()
        .iter()
        .all(|insert| matches!(insert, DeclarationInsert::Inserted(_))));
    assert!(built.index().declarations().iter().all(|declaration| {
        declaration.key().module().as_str() == "demo"
            && declaration.key().package().as_str() == "demo.pkg"
    }));
}

#[test]
fn declaration_index_builder_preserves_duplicate_insert_results() {
    let first_file = SourceFileId::from_raw(22);
    let second_file = SourceFileId::from_raw(23);
    let first = parse_source(first_file, "fun dup();");
    let second = parse_source(second_file, "fun dup();");
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [
            (first_file, PackageNamespace::root()),
            (second_file, PackageNamespace::root()),
        ],
    )
    .unwrap();
    let mut declarations = first.declaration_names.clone();
    declarations.extend(second.declaration_names.clone());
    let mut interner = SymbolInterner::new();

    let built = build_declaration_index(&metadata, &declarations, &mut interner);

    assert_eq!(built.index().declarations().len(), 1);
    assert!(matches!(built.inserts()[0], DeclarationInsert::Inserted(_)));
    assert!(matches!(
        built.inserts()[1],
        DeclarationInsert::Duplicate { .. }
    ));
    assert_eq!(built.diagnostics().len(), 1);
    assert_eq!(
        built.diagnostics()[0].kind(),
        ResolutionDiagnosticKind::DuplicateName
    );
    assert_eq!(
        built.diagnostics()[0].primary_span(),
        second.declaration_names[0].name_span
    );
}

#[test]
fn declaration_index_builder_keeps_same_name_in_distinct_packages() {
    let first_file = SourceFileId::from_raw(24);
    let second_file = SourceFileId::from_raw(25);
    let first = parse_source(first_file, "fun shared();");
    let second = parse_source(second_file, "fun shared();");
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [
            (first_file, PackageNamespace::parse("one").unwrap()),
            (second_file, PackageNamespace::parse("two").unwrap()),
        ],
    )
    .unwrap();
    let mut declarations = first.declaration_names.clone();
    declarations.extend(second.declaration_names.clone());
    let mut interner = SymbolInterner::new();

    let built = build_declaration_index(&metadata, &declarations, &mut interner);

    assert_eq!(built.index().declarations().len(), 2);
    assert!(built.diagnostics().is_empty());
    assert!(built
        .inserts()
        .iter()
        .all(|insert| matches!(insert, DeclarationInsert::Inserted(_))));
}

#[test]
fn duplicate_declaration_diagnostics_do_not_replace_existing_declaration() {
    let first_file = SourceFileId::from_raw(26);
    let second_file = SourceFileId::from_raw(27);
    let first = parse_source(first_file, "struct Thing {}");
    let second = parse_source(second_file, "struct Thing {}");
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [
            (first_file, PackageNamespace::root()),
            (second_file, PackageNamespace::root()),
        ],
    )
    .unwrap();
    let mut declarations = first.declaration_names.clone();
    declarations.extend(second.declaration_names.clone());
    let mut interner = SymbolInterner::new();

    let built = build_declaration_index(&metadata, &declarations, &mut interner);

    assert_eq!(built.index().declarations().len(), 1);
    assert_eq!(
        built.index().declarations()[0].declaration(),
        first.declaration_names[0].declaration
    );
    assert_eq!(built.diagnostics().len(), 1);
    assert_eq!(
        built.diagnostics()[0].primary_span(),
        second.declaration_names[0].name_span
    );
}

#[test]
fn top_level_lookup_finds_exact_declaration_key() {
    let module = ModuleName::parse("demo").unwrap();
    let package = PackageNamespace::parse("pkg").unwrap();
    let name = SymbolId::from_raw(1);
    let declaration = DeclaredName::new(
        DeclarationKey::new(
            module.clone(),
            package.clone(),
            name,
            DeclarationKind::Function,
        ),
        AstNodeId::from_raw(9),
    );
    let mut index = DeclarationIndex::new();
    index.insert(declaration.clone());
    let span = ByteSpan::new(SourceFileId::from_raw(30), 4, 8).unwrap();

    let result = index.lookup_top_level(TopLevelLookup::new(
        module,
        package,
        name,
        DeclarationKind::Function,
        span,
    ));

    assert_eq!(result, TopLevelLookupResult::Found(declaration));
}

#[test]
fn top_level_lookup_requires_exact_package_and_kind() {
    let module = ModuleName::parse("demo").unwrap();
    let name = SymbolId::from_raw(1);
    let declaration = DeclaredName::new(
        DeclarationKey::new(
            module.clone(),
            PackageNamespace::parse("one").unwrap(),
            name,
            DeclarationKind::Function,
        ),
        AstNodeId::from_raw(9),
    );
    let mut index = DeclarationIndex::new();
    index.insert(declaration);
    let span = ByteSpan::new(SourceFileId::from_raw(31), 0, 4).unwrap();

    let wrong_package = index.lookup_top_level(TopLevelLookup::new(
        module.clone(),
        PackageNamespace::parse("two").unwrap(),
        name,
        DeclarationKind::Function,
        span,
    ));
    let wrong_kind = index.lookup_top_level(TopLevelLookup::new(
        module,
        PackageNamespace::parse("one").unwrap(),
        name,
        DeclarationKind::Type,
        span,
    ));

    assert!(matches!(wrong_package, TopLevelLookupResult::Unresolved(_)));
    assert!(matches!(wrong_kind, TopLevelLookupResult::Unresolved(_)));
}

#[test]
fn missing_top_level_lookup_returns_unresolved_name_diagnostic() {
    let index = DeclarationIndex::new();
    let span = ByteSpan::new(SourceFileId::from_raw(32), 12, 18).unwrap();

    let result = index.lookup_top_level(TopLevelLookup::new(
        ModuleName::parse("demo").unwrap(),
        PackageNamespace::root(),
        SymbolId::from_raw(99),
        DeclarationKind::Function,
        span,
    ));

    match result {
        TopLevelLookupResult::Unresolved(diagnostic) => {
            assert_eq!(diagnostic.kind(), ResolutionDiagnosticKind::UnresolvedName);
            assert_eq!(diagnostic.primary_span(), span);
        }
        TopLevelLookupResult::Found(_) => panic!("missing top-level lookup should be unresolved"),
    }
}

#[test]
fn local_binding_key_preserves_scope_and_symbol() {
    let scope = LocalScopeId::from_raw(2);
    let symbol = SymbolId::from_raw(7);
    let key = LocalBindingKey::new(scope, symbol);

    assert_eq!(scope.index(), 2);
    assert_eq!(key.scope(), scope);
    assert_eq!(key.name(), symbol);
}

#[test]
fn local_binding_index_preserves_insertion_order_and_lookup_by_key() {
    let scope = LocalScopeId::from_raw(1);
    let first_key = LocalBindingKey::new(scope, SymbolId::from_raw(10));
    let second_key = LocalBindingKey::new(scope, SymbolId::from_raw(11));
    let first = LocalBinding::new(
        first_key,
        AstNodeId::from_raw(40),
        LocalBindingKind::Immutable,
    );
    let second = LocalBinding::new(second_key, AstNodeId::from_raw(41), LocalBindingKind::Var);
    let mut index = LocalBindingIndex::new();

    assert_eq!(
        index.insert(first.clone()),
        LocalBindingInsert::Inserted(first.clone())
    );
    assert_eq!(
        index.insert(second.clone()),
        LocalBindingInsert::Inserted(second.clone())
    );

    assert_eq!(index.bindings(), [first.clone(), second]);
    assert_eq!(index.get(&first_key), Some(&first));
    assert_eq!(first.kind(), LocalBindingKind::Immutable);
}

#[test]
fn local_binding_index_allows_same_name_in_distinct_scopes() {
    let name = SymbolId::from_raw(12);
    let outer = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(1), name),
        AstNodeId::from_raw(50),
        LocalBindingKind::Immutable,
    );
    let inner = LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(2), name),
        AstNodeId::from_raw(51),
        LocalBindingKind::Var,
    );
    let mut index = LocalBindingIndex::new();

    assert!(matches!(
        index.insert(outer),
        LocalBindingInsert::Inserted(_)
    ));
    assert!(matches!(
        index.insert(inner),
        LocalBindingInsert::Inserted(_)
    ));
    assert_eq!(index.bindings().len(), 2);
}

#[test]
fn duplicate_local_binding_key_preserves_existing_binding() {
    let key = LocalBindingKey::new(LocalScopeId::from_raw(3), SymbolId::from_raw(13));
    let existing = LocalBinding::new(key, AstNodeId::from_raw(60), LocalBindingKind::Immutable);
    let attempted = LocalBinding::new(key, AstNodeId::from_raw(61), LocalBindingKind::Var);
    let mut index = LocalBindingIndex::new();

    assert_eq!(
        index.insert(existing.clone()),
        LocalBindingInsert::Inserted(existing.clone())
    );
    assert_eq!(
        index.insert(attempted.clone()),
        LocalBindingInsert::Duplicate {
            existing: existing.clone(),
            attempted
        }
    );
    assert_eq!(index.get(&key), Some(&existing));
    assert_eq!(index.bindings().len(), 1);
}

#[test]
fn builds_local_binding_index_from_parser_metadata() {
    let file = SourceFileId::from_raw(40);
    let parsed = parse_source(
        file,
        "fun run() { const first = one(); var second = first; }",
    );
    assert!(parsed.diagnostics.is_empty());
    let scope = LocalScopeId::from_raw(4);
    let mut interner = SymbolInterner::new();

    let built = build_local_binding_index(&parsed.local_binding_names, scope, &mut interner);

    assert_eq!(interner.symbols(), ["first", "second"]);
    assert_eq!(built.index().bindings().len(), 2);
    assert_eq!(built.inserts().len(), 2);
    assert!(built.diagnostics().is_empty());
    assert!(built
        .inserts()
        .iter()
        .all(|insert| matches!(insert, LocalBindingInsert::Inserted(_))));
    assert_eq!(built.index().bindings()[0].key().scope(), scope);
    assert_eq!(
        built.index().bindings()[0].kind(),
        LocalBindingKind::Immutable
    );
    assert_eq!(built.index().bindings()[1].kind(), LocalBindingKind::Var);
}

#[test]
fn local_binding_index_builder_reports_same_scope_duplicates() {
    let file = SourceFileId::from_raw(41);
    let parsed = parse_source(file, "fun run() { const same = one(); var same = two(); }");
    assert!(parsed.diagnostics.is_empty());
    let mut interner = SymbolInterner::new();

    let built = build_local_binding_index(
        &parsed.local_binding_names,
        LocalScopeId::from_raw(5),
        &mut interner,
    );

    assert_eq!(built.index().bindings().len(), 1);
    assert!(matches!(
        built.inserts()[0],
        LocalBindingInsert::Inserted(_)
    ));
    assert!(matches!(
        built.inserts()[1],
        LocalBindingInsert::Duplicate { .. }
    ));
    assert_eq!(built.diagnostics().len(), 1);
    assert_eq!(
        built.diagnostics()[0].kind(),
        ResolutionDiagnosticKind::DuplicateName
    );
    assert_eq!(
        built.diagnostics()[0].primary_span(),
        parsed.local_binding_names[1].name_span
    );
}

#[test]
fn local_scope_tree_allocates_stable_ids_in_insertion_order() {
    let mut tree = LocalScopeTree::new();
    let root_owner = AstNodeId::from_raw(70);
    let child_owner = AstNodeId::from_raw(71);

    let root = tree.add_scope(root_owner, None);
    let child = tree.add_scope(child_owner, Some(root));

    assert_eq!(root.index(), 0);
    assert_eq!(child.index(), 1);
    assert_eq!(tree.scopes()[0].id(), root);
    assert_eq!(tree.scopes()[1].id(), child);
}

#[test]
fn local_scope_tree_preserves_owner_and_parent() {
    let mut tree = LocalScopeTree::new();
    let root_owner = AstNodeId::from_raw(80);
    let child_owner = AstNodeId::from_raw(81);

    let root = tree.add_scope(root_owner, None);
    let child = tree.add_scope(child_owner, Some(root));

    assert_eq!(tree.get(root).unwrap().owner(), root_owner);
    assert_eq!(tree.get(root).unwrap().parent(), None);
    assert_eq!(tree.get(child).unwrap().owner(), child_owner);
    assert_eq!(tree.get(child).unwrap().parent(), Some(root));
}

#[test]
fn local_scope_tree_unknown_scope_id_returns_none() {
    let mut tree = LocalScopeTree::new();
    tree.add_scope(AstNodeId::from_raw(90), None);

    assert_eq!(tree.get(LocalScopeId::from_raw(99)), None);
}

#[test]
fn builds_local_scope_tree_for_parser_blocks_in_source_order() {
    let parsed = parse_source(
        SourceFileId::from_raw(50),
        "fun choose() { if (ready) { const inner = one(); } else { const other = two(); } }",
    );
    assert!(parsed.diagnostics.is_empty());

    let tree = build_local_scope_tree(&parsed.arena);

    assert_eq!(tree.scopes().len(), 3);
    assert!(tree.scopes()[0].parent().is_none());
    assert_eq!(tree.scopes()[1].parent(), Some(LocalScopeId::from_raw(0)));
    assert_eq!(tree.scopes()[2].parent(), Some(LocalScopeId::from_raw(0)));
    let owners: Vec<_> = tree.scopes().iter().map(|scope| scope.owner()).collect();
    assert!(
        parsed.arena.node(owners[0]).unwrap().span.start()
            < parsed.arena.node(owners[1]).unwrap().span.start()
    );
    assert!(
        parsed.arena.node(owners[1]).unwrap().span.start()
            < parsed.arena.node(owners[2]).unwrap().span.start()
    );
}

#[test]
fn local_scope_tree_builder_keeps_declaration_bodies_as_roots() {
    let parsed = parse_source(
        SourceFileId::from_raw(51),
        "struct Outer { struct Inner { fun run(); } }",
    );
    assert!(parsed.diagnostics.is_empty());

    let tree = build_local_scope_tree(&parsed.arena);

    assert_eq!(tree.scopes().len(), 2);
    assert!(tree.scopes().iter().all(|scope| scope.parent().is_none()));
}

#[test]
fn local_scope_tree_builder_ignores_non_scope_owner_nodes() {
    let file = SourceFileId::from_raw(52);
    let mut arena = AstArena::new();
    arena.add_source_file(ByteSpan::new(file, 0, 20).unwrap());
    arena.add_name_expression(ByteSpan::new(file, 1, 5).unwrap());
    arena.add_variable_declaration_statement(ByteSpan::new(file, 6, 15).unwrap());

    let tree = build_local_scope_tree(&arena);

    assert!(tree.scopes().is_empty());
}

#[test]
fn scoped_local_binding_builder_assigns_nearest_block_scope() {
    let parsed = parse_source(
        SourceFileId::from_raw(60),
        "fun run() { const outer = one(); if (ready) { const inner = outer; } }",
    );
    assert!(parsed.diagnostics.is_empty());
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();

    let built = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    assert_eq!(built.index().bindings().len(), 2);
    assert_eq!(
        built.index().bindings()[0].key().scope(),
        LocalScopeId::from_raw(0)
    );
    assert_eq!(
        built.index().bindings()[1].key().scope(),
        LocalScopeId::from_raw(1)
    );
    assert!(built.diagnostics().is_empty());
}

#[test]
fn scoped_local_binding_builder_allows_nested_shadowing() {
    let parsed = parse_source(
        SourceFileId::from_raw(61),
        "fun run() { const same = one(); if (ready) { var same = two(); } }",
    );
    assert!(parsed.diagnostics.is_empty());
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();

    let built = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    assert_eq!(built.index().bindings().len(), 2);
    assert!(built.diagnostics().is_empty());
    assert_eq!(
        built.index().bindings()[0].kind(),
        LocalBindingKind::Immutable
    );
    assert_eq!(built.index().bindings()[1].kind(), LocalBindingKind::Var);
}

#[test]
fn scoped_local_binding_builder_reports_same_block_duplicates() {
    let parsed = parse_source(
        SourceFileId::from_raw(62),
        "fun run() { const same = one(); var same = two(); }",
    );
    assert!(parsed.diagnostics.is_empty());
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();

    let built = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    assert_eq!(built.index().bindings().len(), 1);
    assert!(matches!(
        built.inserts()[1],
        LocalBindingInsert::Duplicate { .. }
    ));
    assert_eq!(built.diagnostics().len(), 1);
    assert_eq!(
        built.diagnostics()[0].kind(),
        ResolutionDiagnosticKind::DuplicateName
    );
    assert_eq!(
        built.diagnostics()[0].primary_span(),
        parsed.local_binding_names[1].name_span
    );
}

#[test]
fn local_binding_lookup_finds_visible_binding_after_declaration() {
    let source = "fun run() { const value = one(); value; }";
    let file = SourceFileId::from_raw(70);
    let parsed = parse_source(file, source);
    assert!(parsed.diagnostics.is_empty());
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let built = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );
    let value = interner.intern("value");
    let reference_start = source.rfind("value").unwrap();
    let reference_span =
        ByteSpan::new(file, reference_start, reference_start + "value".len()).unwrap();

    let result = built.index().lookup_local(
        &scopes,
        &parsed.arena,
        LocalNameLookup::new(LocalScopeId::from_raw(0), value, reference_span),
    );

    assert_eq!(
        result,
        LocalNameLookupResult::Found(built.index().bindings()[0].clone())
    );
}

#[test]
fn local_binding_lookup_rejects_reference_before_declaration() {
    let source = "fun run() { value; const value = one(); }";
    let file = SourceFileId::from_raw(71);
    let parsed = parse_source(file, source);
    assert!(parsed.diagnostics.is_empty());
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let built = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );
    let value = interner.intern("value");
    let reference_start = source.find("value").unwrap();
    let reference_span =
        ByteSpan::new(file, reference_start, reference_start + "value".len()).unwrap();

    let result = built.index().lookup_local(
        &scopes,
        &parsed.arena,
        LocalNameLookup::new(LocalScopeId::from_raw(0), value, reference_span),
    );

    assert!(matches!(result, LocalNameLookupResult::Unresolved(_)));
}

#[test]
fn local_binding_lookup_uses_nearest_visible_scope() {
    let source = "fun run() { const same = one(); if (ready) { var same = two(); same; } }";
    let file = SourceFileId::from_raw(72);
    let parsed = parse_source(file, source);
    assert!(parsed.diagnostics.is_empty());
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let built = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );
    let same = interner.intern("same");
    let reference_start = source.rfind("same").unwrap();
    let reference_span =
        ByteSpan::new(file, reference_start, reference_start + "same".len()).unwrap();

    let result = built.index().lookup_local(
        &scopes,
        &parsed.arena,
        LocalNameLookup::new(LocalScopeId::from_raw(1), same, reference_span),
    );

    assert_eq!(
        result,
        LocalNameLookupResult::Found(built.index().bindings()[1].clone())
    );
}

#[test]
fn local_binding_lookup_continues_past_not_yet_visible_inner_binding() {
    let source = "fun run() { const same = one(); if (ready) { same; var same = two(); } }";
    let file = SourceFileId::from_raw(73);
    let parsed = parse_source(file, source);
    assert!(parsed.diagnostics.is_empty());
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let built = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );
    let same = interner.intern("same");
    let reference_start = source.find("same;").unwrap();
    let reference_span =
        ByteSpan::new(file, reference_start, reference_start + "same".len()).unwrap();

    let result = built.index().lookup_local(
        &scopes,
        &parsed.arena,
        LocalNameLookup::new(LocalScopeId::from_raw(1), same, reference_span),
    );

    assert_eq!(
        result,
        LocalNameLookupResult::Found(built.index().bindings()[0].clone())
    );
}

#[test]
fn missing_local_binding_lookup_returns_unresolved_name_diagnostic() {
    let source = "fun run() { missing; }";
    let file = SourceFileId::from_raw(74);
    let parsed = parse_source(file, source);
    assert!(parsed.diagnostics.is_empty());
    let scopes = build_local_scope_tree(&parsed.arena);
    let interner = SymbolInterner::new();
    let index = LocalBindingIndex::new();
    let reference_start = source.find("missing").unwrap();
    let reference_span =
        ByteSpan::new(file, reference_start, reference_start + "missing".len()).unwrap();

    let result = index.lookup_local(
        &scopes,
        &parsed.arena,
        LocalNameLookup::new(
            LocalScopeId::from_raw(0),
            SymbolId::from_raw(interner.symbols().len()),
            reference_span,
        ),
    );

    match result {
        LocalNameLookupResult::Unresolved(diagnostic) => {
            assert_eq!(diagnostic.kind(), ResolutionDiagnosticKind::UnresolvedName);
            assert_eq!(diagnostic.primary_span(), reference_span);
        }
        LocalNameLookupResult::Found(_) => panic!("missing local lookup should be unresolved"),
    }
}

#[test]
fn local_reference_binding_records_visible_local_resolution() {
    let source = "fun run() { const value = 1; value; }";
    let file = SourceFileId::from_raw(80);
    let parsed = parse_source(file, source);
    assert!(parsed.diagnostics.is_empty());
    assert_eq!(parsed.name_references.len(), 1);
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let locals = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_local_name_references(
        &parsed.arena,
        &parsed.name_references,
        &scopes,
        locals.index(),
        &mut interner,
    );

    assert!(bound.diagnostics().is_empty());
    assert_eq!(bound.inserts().len(), 1);
    assert_eq!(bound.table().resolved_names().len(), 1);
    let resolved = bound.table().resolved_names()[0];
    assert_eq!(resolved.reference(), parsed.name_references[0].reference);
    assert_eq!(interner.resolve(resolved.symbol()), Some("value"));
}

#[test]
fn m0019_local_binding_resolution_identity_records_exact_binding() {
    let source = "fun run() { const value = 1; value; }";
    let file = SourceFileId::from_raw(83);
    let parsed = parse_source(file, source);
    assert!(parsed.diagnostics.is_empty());
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let locals = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_local_name_references(
        &parsed.arena,
        &parsed.name_references,
        &scopes,
        locals.index(),
        &mut interner,
    );

    assert!(bound.diagnostics().is_empty());
    assert_eq!(bound.resolved_local_bindings().len(), 1);
    let resolved = &bound.resolved_local_bindings()[0];
    assert_eq!(resolved.reference(), parsed.name_references[0].reference);
    assert_eq!(resolved.binding(), &locals.index().bindings()[0]);
}

#[test]
fn m0019_local_binding_resolution_identity_distinguishes_nested_shadowing() {
    let source = "fun run() { const same = 1; if (ready) { same; const same = 2; same; }; same; }";
    let file = SourceFileId::from_raw(84);
    let parsed = parse_source(file, source);
    assert!(
        parsed.diagnostics.is_empty(),
        "unexpected parser diagnostics: {:?}",
        parsed.diagnostics
    );
    let same_references = parsed
        .name_references
        .iter()
        .filter(|reference| reference.name == "same")
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(same_references.len(), 3);
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let locals = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );
    assert_eq!(locals.index().bindings().len(), 2);

    let bound = bind_local_name_references(
        &parsed.arena,
        &same_references,
        &scopes,
        locals.index(),
        &mut interner,
    );

    assert!(bound.diagnostics().is_empty());
    assert_eq!(bound.resolved_local_bindings().len(), 3);
    assert_eq!(
        bound.resolved_local_bindings()[0].binding(),
        &locals.index().bindings()[0]
    );
    assert_eq!(
        bound.resolved_local_bindings()[1].binding(),
        &locals.index().bindings()[1]
    );
    assert_eq!(
        bound.resolved_local_bindings()[2].binding(),
        &locals.index().bindings()[0]
    );
    assert_eq!(
        bound
            .resolved_local_bindings()
            .iter()
            .map(|resolved| resolved.reference())
            .collect::<Vec<_>>(),
        same_references
            .iter()
            .map(|reference| reference.reference)
            .collect::<Vec<_>>()
    );
}

#[test]
fn m0019_local_binding_resolution_identity_skips_unresolved_uses() {
    let source = "fun run() { missing; }";
    let file = SourceFileId::from_raw(85);
    let parsed = parse_source(file, source);
    assert!(parsed.diagnostics.is_empty());
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let locals = LocalBindingIndex::new();

    let bound = bind_local_name_references(
        &parsed.arena,
        &parsed.name_references,
        &scopes,
        &locals,
        &mut interner,
    );

    assert!(bound.resolved_local_bindings().is_empty());
    assert_eq!(bound.diagnostics().len(), 1);
    assert_eq!(
        bound.diagnostics()[0].kind(),
        ResolutionDiagnosticKind::UnresolvedName
    );
}

#[test]
fn local_reference_binding_reports_reference_before_declaration() {
    let source = "fun run() { value; const value = 1; }";
    let file = SourceFileId::from_raw(81);
    let parsed = parse_source(file, source);
    assert!(parsed.diagnostics.is_empty());
    assert_eq!(parsed.name_references.len(), 1);
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let locals = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_local_name_references(
        &parsed.arena,
        &parsed.name_references,
        &scopes,
        locals.index(),
        &mut interner,
    );

    assert!(bound.table().resolved_names().is_empty());
    assert!(bound.inserts().is_empty());
    assert_eq!(bound.diagnostics().len(), 1);
    assert_eq!(
        bound.diagnostics()[0].kind(),
        ResolutionDiagnosticKind::UnresolvedName
    );
    assert_eq!(
        bound.diagnostics()[0].primary_span(),
        parsed.name_references[0].name_span
    );
}

#[test]
fn local_reference_binding_does_not_use_top_level_fallback() {
    let source = "fun top(); fun run() { top; }";
    let file = SourceFileId::from_raw(82);
    let parsed = parse_source(file, source);
    assert!(parsed.diagnostics.is_empty());
    assert_eq!(parsed.name_references.len(), 1);
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let locals = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_local_name_references(
        &parsed.arena,
        &parsed.name_references,
        &scopes,
        locals.index(),
        &mut interner,
    );

    assert!(bound.table().resolved_names().is_empty());
    assert_eq!(bound.diagnostics().len(), 1);
    assert_eq!(
        bound.diagnostics()[0].kind(),
        ResolutionDiagnosticKind::UnresolvedName
    );
    assert_eq!(
        bound.diagnostics()[0].primary_span(),
        parsed.name_references[0].name_span
    );
}

#[test]
fn unqualified_function_reference_binding_uses_same_package_top_level_fallback() {
    let file = SourceFileId::from_raw(90);
    let parsed = parse_source(file, "fun helper(); fun run() { helper; }");
    assert!(parsed.diagnostics.is_empty());
    assert_eq!(parsed.name_references.len(), 1);
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [(file, PackageNamespace::parse("app").unwrap())],
    )
    .unwrap();
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let declarations = build_declaration_index(&metadata, &parsed.declaration_names, &mut interner);
    let locals = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_unqualified_function_references(
        &metadata,
        &parsed.arena,
        &parsed.name_references,
        &scopes,
        locals.index(),
        declarations.index(),
        &mut interner,
    );

    assert!(bound.diagnostics().is_empty());
    assert_eq!(bound.inserts().len(), 1);
    assert_eq!(bound.table().resolved_names().len(), 1);
    let resolved = bound.table().resolved_names()[0];
    assert_eq!(resolved.reference(), parsed.name_references[0].reference);
    assert_eq!(interner.resolve(resolved.symbol()), Some("helper"));
}

#[test]
fn unqualified_function_reference_binding_keeps_local_lookup_before_top_level() {
    let file = SourceFileId::from_raw(91);
    let parsed = parse_source(file, "fun value(); fun run() { const value = 1; value; }");
    assert!(parsed.diagnostics.is_empty());
    assert_eq!(parsed.name_references.len(), 1);
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [(file, PackageNamespace::root())],
    )
    .unwrap();
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let declarations = build_declaration_index(&metadata, &parsed.declaration_names, &mut interner);
    let locals = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_unqualified_function_references(
        &metadata,
        &parsed.arena,
        &parsed.name_references,
        &scopes,
        locals.index(),
        declarations.index(),
        &mut interner,
    );

    assert!(bound.diagnostics().is_empty());
    assert_eq!(bound.table().resolved_names().len(), 1);
    assert_eq!(
        bound.table().resolved_names()[0].reference(),
        parsed.name_references[0].reference
    );
    assert_eq!(
        interner.resolve(bound.table().resolved_names()[0].symbol()),
        Some("value")
    );
}

#[test]
fn unqualified_function_reference_binding_rejects_other_package_top_level() {
    let helper_file = SourceFileId::from_raw(92);
    let run_file = SourceFileId::from_raw(93);
    let helper = parse_source(helper_file, "fun helper();");
    let run = parse_source(run_file, "fun run() { helper; }");
    assert!(helper.diagnostics.is_empty());
    assert!(run.diagnostics.is_empty());
    assert_eq!(run.name_references.len(), 1);
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [
            (helper_file, PackageNamespace::parse("lib").unwrap()),
            (run_file, PackageNamespace::parse("app").unwrap()),
        ],
    )
    .unwrap();
    let mut declarations = helper.declaration_names.clone();
    declarations.extend(run.declaration_names.clone());
    let scopes = build_local_scope_tree(&run.arena);
    let mut interner = SymbolInterner::new();
    let declarations = build_declaration_index(&metadata, &declarations, &mut interner);
    let locals = build_scoped_local_binding_index(
        &run.arena,
        &run.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_unqualified_function_references(
        &metadata,
        &run.arena,
        &run.name_references,
        &scopes,
        locals.index(),
        declarations.index(),
        &mut interner,
    );

    assert!(bound.table().resolved_names().is_empty());
    assert_eq!(bound.diagnostics().len(), 1);
    assert_eq!(
        bound.diagnostics()[0].kind(),
        ResolutionDiagnosticKind::UnresolvedName
    );
    assert_eq!(
        bound.diagnostics()[0].primary_span(),
        run.name_references[0].name_span
    );
}

#[test]
fn unqualified_function_reference_binding_does_not_treat_types_as_function_fallback() {
    let file = SourceFileId::from_raw(94);
    let parsed = parse_source(file, "struct Box {} fun run() { Box; }");
    assert!(parsed.diagnostics.is_empty());
    assert_eq!(parsed.name_references.len(), 1);
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [(file, PackageNamespace::root())],
    )
    .unwrap();
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let declarations = build_declaration_index(&metadata, &parsed.declaration_names, &mut interner);
    let locals = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_unqualified_function_references(
        &metadata,
        &parsed.arena,
        &parsed.name_references,
        &scopes,
        locals.index(),
        declarations.index(),
        &mut interner,
    );

    assert!(bound.table().resolved_names().is_empty());
    assert_eq!(bound.diagnostics().len(), 1);
    assert_eq!(
        bound.diagnostics()[0].kind(),
        ResolutionDiagnosticKind::UnresolvedName
    );
}

#[test]
fn unqualified_type_reference_binding_uses_same_package_top_level_fallback() {
    let file = SourceFileId::from_raw(100);
    let parsed = parse_source(file, "struct Box {} fun run(): Box;");
    assert!(parsed.diagnostics.is_empty());
    assert_eq!(parsed.type_name_references.len(), 1);
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [(file, PackageNamespace::parse("app").unwrap())],
    )
    .unwrap();
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let declarations = build_declaration_index(&metadata, &parsed.declaration_names, &mut interner);
    let locals = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_unqualified_type_references(
        &metadata,
        &parsed.arena,
        &parsed.type_name_references,
        &scopes,
        locals.index(),
        declarations.index(),
        &mut interner,
    );

    assert!(bound.diagnostics().is_empty());
    assert_eq!(bound.inserts().len(), 1);
    assert_eq!(bound.table().resolved_names().len(), 1);
    let resolved = bound.table().resolved_names()[0];
    assert_eq!(
        resolved.reference(),
        parsed.type_name_references[0].reference
    );
    assert_eq!(interner.resolve(resolved.symbol()), Some("Box"));
}

#[test]
fn unqualified_type_reference_binding_keeps_local_lookup_before_top_level() {
    let file = SourceFileId::from_raw(101);
    let parsed = parse_source(
        file,
        "struct Box {} fun run() { const Box = make(); const item: Box = make(); }",
    );
    assert!(parsed.diagnostics.is_empty());
    assert_eq!(parsed.type_name_references.len(), 1);
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [(file, PackageNamespace::root())],
    )
    .unwrap();
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let declarations = build_declaration_index(&metadata, &parsed.declaration_names, &mut interner);
    let locals = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_unqualified_type_references(
        &metadata,
        &parsed.arena,
        &parsed.type_name_references,
        &scopes,
        locals.index(),
        declarations.index(),
        &mut interner,
    );

    assert!(bound.diagnostics().is_empty());
    assert_eq!(bound.table().resolved_names().len(), 1);
    assert_eq!(
        bound.table().resolved_names()[0].reference(),
        parsed.type_name_references[0].reference
    );
    assert_eq!(
        interner.resolve(bound.table().resolved_names()[0].symbol()),
        Some("Box")
    );
}

#[test]
fn unqualified_type_reference_binding_rejects_other_package_top_level() {
    let type_file = SourceFileId::from_raw(102);
    let run_file = SourceFileId::from_raw(103);
    let type_source = parse_source(type_file, "struct Box {}");
    let run = parse_source(run_file, "fun run(): Box;");
    assert!(type_source.diagnostics.is_empty());
    assert!(run.diagnostics.is_empty());
    assert_eq!(run.type_name_references.len(), 1);
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [
            (type_file, PackageNamespace::parse("lib").unwrap()),
            (run_file, PackageNamespace::parse("app").unwrap()),
        ],
    )
    .unwrap();
    let mut declarations = type_source.declaration_names.clone();
    declarations.extend(run.declaration_names.clone());
    let scopes = build_local_scope_tree(&run.arena);
    let mut interner = SymbolInterner::new();
    let declarations = build_declaration_index(&metadata, &declarations, &mut interner);
    let locals = build_scoped_local_binding_index(
        &run.arena,
        &run.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_unqualified_type_references(
        &metadata,
        &run.arena,
        &run.type_name_references,
        &scopes,
        locals.index(),
        declarations.index(),
        &mut interner,
    );

    assert!(bound.table().resolved_names().is_empty());
    assert_eq!(bound.diagnostics().len(), 1);
    assert_eq!(
        bound.diagnostics()[0].kind(),
        ResolutionDiagnosticKind::UnresolvedName
    );
    assert_eq!(
        bound.diagnostics()[0].primary_span(),
        run.type_name_references[0].name_span
    );
}

#[test]
fn unqualified_type_reference_binding_does_not_treat_functions_as_type_fallback() {
    let file = SourceFileId::from_raw(104);
    let parsed = parse_source(file, "fun Box(); fun run(): Box;");
    assert!(parsed.diagnostics.is_empty());
    assert_eq!(parsed.type_name_references.len(), 1);
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [(file, PackageNamespace::root())],
    )
    .unwrap();
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let declarations = build_declaration_index(&metadata, &parsed.declaration_names, &mut interner);
    let locals = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_unqualified_type_references(
        &metadata,
        &parsed.arena,
        &parsed.type_name_references,
        &scopes,
        locals.index(),
        declarations.index(),
        &mut interner,
    );

    assert!(bound.table().resolved_names().is_empty());
    assert_eq!(bound.diagnostics().len(), 1);
    assert_eq!(
        bound.diagnostics()[0].kind(),
        ResolutionDiagnosticKind::UnresolvedName
    );
}

#[test]
fn package_qualified_type_reference_binding_uses_explicit_package_namespace() {
    let type_file = SourceFileId::from_raw(110);
    let run_file = SourceFileId::from_raw(111);
    let type_source = parse_source(type_file, "struct Box {}");
    let run = parse_source(run_file, "fun run(): lib.Box;");
    assert!(type_source.diagnostics.is_empty());
    assert!(run.diagnostics.is_empty());
    assert_eq!(run.type_name_references.len(), 1);
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [
            (type_file, PackageNamespace::parse("lib").unwrap()),
            (run_file, PackageNamespace::parse("app").unwrap()),
        ],
    )
    .unwrap();
    let mut declarations = type_source.declaration_names.clone();
    declarations.extend(run.declaration_names.clone());
    let mut interner = SymbolInterner::new();
    let declarations = build_declaration_index(&metadata, &declarations, &mut interner);

    let bound = bind_package_qualified_type_references(
        &metadata,
        &run.type_name_references,
        declarations.index(),
        &mut interner,
    );

    assert!(bound.diagnostics().is_empty());
    assert_eq!(bound.inserts().len(), 1);
    assert_eq!(bound.table().resolved_names().len(), 1);
    let resolved = bound.table().resolved_names()[0];
    assert_eq!(resolved.reference(), run.type_name_references[0].reference);
    assert_eq!(interner.resolve(resolved.symbol()), Some("Box"));
}

#[test]
fn package_qualified_type_reference_binding_splits_nested_package_at_final_dot() {
    let type_file = SourceFileId::from_raw(112);
    let run_file = SourceFileId::from_raw(113);
    let type_source = parse_source(type_file, "struct Result {}");
    let run = parse_source(run_file, "fun run(): lib.core.Result;");
    assert!(type_source.diagnostics.is_empty());
    assert!(run.diagnostics.is_empty());
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [
            (type_file, PackageNamespace::parse("lib.core").unwrap()),
            (run_file, PackageNamespace::parse("app").unwrap()),
        ],
    )
    .unwrap();
    let mut declarations = type_source.declaration_names.clone();
    declarations.extend(run.declaration_names.clone());
    let mut interner = SymbolInterner::new();
    let declarations = build_declaration_index(&metadata, &declarations, &mut interner);

    let bound = bind_package_qualified_type_references(
        &metadata,
        &run.type_name_references,
        declarations.index(),
        &mut interner,
    );

    assert!(bound.diagnostics().is_empty());
    assert_eq!(bound.table().resolved_names().len(), 1);
    assert_eq!(
        interner.resolve(bound.table().resolved_names()[0].symbol()),
        Some("Result")
    );
}

#[test]
fn package_qualified_type_reference_binding_ignores_unqualified_type_names() {
    let file = SourceFileId::from_raw(114);
    let parsed = parse_source(file, "struct Box {} fun run(): Box;");
    assert!(parsed.diagnostics.is_empty());
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [(file, PackageNamespace::root())],
    )
    .unwrap();
    let mut interner = SymbolInterner::new();
    let declarations = build_declaration_index(&metadata, &parsed.declaration_names, &mut interner);

    let bound = bind_package_qualified_type_references(
        &metadata,
        &parsed.type_name_references,
        declarations.index(),
        &mut interner,
    );

    assert!(bound.table().resolved_names().is_empty());
    assert!(bound.inserts().is_empty());
    assert!(bound.diagnostics().is_empty());
}

#[test]
fn package_qualified_type_reference_binding_rejects_missing_and_function_candidates() {
    let file = SourceFileId::from_raw(115);
    let parsed = parse_source(file, "fun Box(); fun run(): lib.Box;");
    assert!(parsed.diagnostics.is_empty());
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [(file, PackageNamespace::parse("lib").unwrap())],
    )
    .unwrap();
    let mut interner = SymbolInterner::new();
    let declarations = build_declaration_index(&metadata, &parsed.declaration_names, &mut interner);

    let bound = bind_package_qualified_type_references(
        &metadata,
        &parsed.type_name_references,
        declarations.index(),
        &mut interner,
    );

    assert!(bound.table().resolved_names().is_empty());
    assert_eq!(bound.diagnostics().len(), 1);
    assert_eq!(
        bound.diagnostics()[0].kind(),
        ResolutionDiagnosticKind::UnresolvedName
    );
    assert_eq!(
        bound.diagnostics()[0].primary_span(),
        parsed.type_name_references[0].name_span
    );
}

#[test]
fn accepted_name_reference_binding_combines_expression_and_type_bindings() {
    let box_file = SourceFileId::from_raw(120);
    let run_file = SourceFileId::from_raw(121);
    let box_source = parse_source(box_file, "struct Box {}");
    let run = parse_source(
        run_file,
        "struct Item {} fun helper(); fun run(): lib.Box { const local: Item = helper(); local; }",
    );
    assert!(box_source.diagnostics.is_empty());
    assert!(run.diagnostics.is_empty());
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [
            (box_file, PackageNamespace::parse("lib").unwrap()),
            (run_file, PackageNamespace::parse("app").unwrap()),
        ],
    )
    .unwrap();
    let mut declarations = box_source.declaration_names.clone();
    declarations.extend(run.declaration_names.clone());
    let scopes = build_local_scope_tree(&run.arena);
    let mut interner = SymbolInterner::new();
    let declarations = build_declaration_index(&metadata, &declarations, &mut interner);
    let locals = build_scoped_local_binding_index(
        &run.arena,
        &run.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_accepted_name_references(
        &metadata,
        &run.arena,
        &run.name_references,
        &run.type_name_references,
        &scopes,
        locals.index(),
        declarations.index(),
        &mut interner,
    );

    assert!(bound.diagnostics().is_empty());
    assert_eq!(bound.inserts().len(), 4);
    assert_eq!(bound.table().resolved_names().len(), 4);
    let resolved: Vec<_> = bound
        .table()
        .resolved_names()
        .iter()
        .map(|resolved| interner.resolve(resolved.symbol()).unwrap())
        .collect();
    assert_eq!(resolved, ["helper", "local", "Item", "Box"]);
}

#[test]
fn accepted_name_reference_binding_does_not_duplicate_package_qualified_type_diagnostics() {
    let file = SourceFileId::from_raw(122);
    let parsed = parse_source(file, "fun run(): missing.Box;");
    assert!(parsed.diagnostics.is_empty());
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [(file, PackageNamespace::root())],
    )
    .unwrap();
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let declarations = build_declaration_index(&metadata, &parsed.declaration_names, &mut interner);
    let locals = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_accepted_name_references(
        &metadata,
        &parsed.arena,
        &parsed.name_references,
        &parsed.type_name_references,
        &scopes,
        locals.index(),
        declarations.index(),
        &mut interner,
    );

    assert!(bound.table().resolved_names().is_empty());
    assert_eq!(bound.diagnostics().len(), 1);
    assert_eq!(
        bound.diagnostics()[0].kind(),
        ResolutionDiagnosticKind::UnresolvedName
    );
    assert_eq!(
        bound.diagnostics()[0].primary_span(),
        parsed.type_name_references[0].name_span
    );
}

#[test]
fn accepted_name_reference_binding_collects_expression_and_type_unresolved_diagnostics() {
    let file = SourceFileId::from_raw(123);
    let parsed = parse_source(file, "fun run(): Missing { missing; }");
    assert!(parsed.diagnostics.is_empty());
    let metadata = newlang::module::ModuleMetadata::with_packages(
        ModuleName::parse("demo").unwrap(),
        [(file, PackageNamespace::root())],
    )
    .unwrap();
    let scopes = build_local_scope_tree(&parsed.arena);
    let mut interner = SymbolInterner::new();
    let declarations = build_declaration_index(&metadata, &parsed.declaration_names, &mut interner);
    let locals = build_scoped_local_binding_index(
        &parsed.arena,
        &parsed.local_binding_names,
        &scopes,
        &mut interner,
    );

    let bound = bind_accepted_name_references(
        &metadata,
        &parsed.arena,
        &parsed.name_references,
        &parsed.type_name_references,
        &scopes,
        locals.index(),
        declarations.index(),
        &mut interner,
    );

    assert!(bound.table().resolved_names().is_empty());
    assert_eq!(bound.diagnostics().len(), 2);
    assert!(bound
        .diagnostics()
        .iter()
        .all(|diagnostic| diagnostic.kind() == ResolutionDiagnosticKind::UnresolvedName));
}

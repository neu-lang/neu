use newlang::ast::AstNodeId;
use newlang::module::{ModuleName, PackageNamespace};
use newlang::name_resolution::{
    build_declaration_index, DeclarationIndex, DeclarationInsert, DeclarationKey, DeclarationKind,
    DeclaredName, ResolutionDiagnostic, ResolutionDiagnosticKind, ResolutionInsert,
    ResolutionTable, ResolvedName,
};
use newlang::parser::parse_source;
use newlang::source::{ByteSpan, SourceFileId};
use newlang::symbol::{SymbolId, SymbolInterner};

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
    assert!(built
        .inserts()
        .iter()
        .all(|insert| matches!(insert, DeclarationInsert::Inserted(_))));
}

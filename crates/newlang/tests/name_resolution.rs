use newlang::ast::AstNodeId;
use newlang::name_resolution::{
    ResolutionDiagnostic, ResolutionDiagnosticKind, ResolutionInsert, ResolutionTable, ResolvedName,
};
use newlang::source::{ByteSpan, SourceFileId};
use newlang::symbol::SymbolId;

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

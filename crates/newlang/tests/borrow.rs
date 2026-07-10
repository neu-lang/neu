use newlang::{
    ast::AstNodeId,
    borrow::{
        BorrowDiagnostic, BorrowDiagnosticKind, BorrowKind, BorrowRecord, LifetimeEscapeRecord,
        analyze_borrow_conflicts, analyze_lifetime_escapes,
    },
    name_resolution::{LocalBinding, LocalBindingKey, LocalBindingKind, LocalScopeId},
    symbol::SymbolId,
};

#[test]
fn m0023_shared_borrows_of_same_local_in_same_region_do_not_conflict() {
    let binding = local_binding(10, 100);
    let region = AstNodeId::from_raw(200);
    let borrows = [
        BorrowRecord::new(
            AstNodeId::from_raw(300),
            binding.clone(),
            BorrowKind::Shared,
            region,
        ),
        BorrowRecord::new(
            AstNodeId::from_raw(301),
            binding,
            BorrowKind::Shared,
            region,
        ),
    ];

    assert!(analyze_borrow_conflicts(&borrows).is_empty());
}

#[test]
fn m0023_exclusive_borrow_conflicts_with_shared_or_exclusive_in_same_region() {
    let binding = local_binding(10, 100);
    let region = AstNodeId::from_raw(200);
    let borrows = [
        BorrowRecord::new(
            AstNodeId::from_raw(300),
            binding.clone(),
            BorrowKind::Shared,
            region,
        ),
        BorrowRecord::new(
            AstNodeId::from_raw(301),
            binding.clone(),
            BorrowKind::Exclusive,
            region,
        ),
        BorrowRecord::new(
            AstNodeId::from_raw(302),
            binding,
            BorrowKind::Shared,
            region,
        ),
    ];

    let diagnostics = analyze_borrow_conflicts(&borrows);

    assert_eq!(
        diagnostics,
        [
            BorrowDiagnostic::borrow_conflict(AstNodeId::from_raw(301), AstNodeId::from_raw(300)),
            BorrowDiagnostic::borrow_conflict(AstNodeId::from_raw(302), AstNodeId::from_raw(301)),
        ]
    );
    assert_eq!(diagnostics[0].kind(), BorrowDiagnosticKind::BorrowConflict);
    assert_eq!(diagnostics[0].node(), AstNodeId::from_raw(301));
    assert_eq!(diagnostics[0].conflict_origin(), AstNodeId::from_raw(300));
}

#[test]
fn m0023_different_locals_or_regions_do_not_conflict() {
    let first = local_binding(10, 100);
    let second = local_binding(11, 101);
    let first_region = AstNodeId::from_raw(200);
    let second_region = AstNodeId::from_raw(201);
    let borrows = [
        BorrowRecord::new(
            AstNodeId::from_raw(300),
            first.clone(),
            BorrowKind::Exclusive,
            first_region,
        ),
        BorrowRecord::new(
            AstNodeId::from_raw(301),
            second,
            BorrowKind::Shared,
            first_region,
        ),
        BorrowRecord::new(
            AstNodeId::from_raw(302),
            first,
            BorrowKind::Shared,
            second_region,
        ),
    ];

    assert!(analyze_borrow_conflicts(&borrows).is_empty());
}

#[test]
fn m0023_lifetime_escape_diagnoses_uses_outside_borrow_region() {
    let first = local_binding(10, 100);
    let second = local_binding(11, 101);
    let borrow_region = AstNodeId::from_raw(200);
    let other_region = AstNodeId::from_raw(201);
    let records = [
        LifetimeEscapeRecord::new(
            AstNodeId::from_raw(300),
            first.clone(),
            AstNodeId::from_raw(400),
            borrow_region,
            borrow_region,
        ),
        LifetimeEscapeRecord::new(
            AstNodeId::from_raw(301),
            first,
            AstNodeId::from_raw(401),
            borrow_region,
            other_region,
        ),
        LifetimeEscapeRecord::new(
            AstNodeId::from_raw(302),
            second,
            AstNodeId::from_raw(402),
            other_region,
            borrow_region,
        ),
    ];

    let diagnostics = analyze_lifetime_escapes(&records);

    assert_eq!(
        diagnostics,
        [
            BorrowDiagnostic::lifetime_escape(AstNodeId::from_raw(301), AstNodeId::from_raw(401)),
            BorrowDiagnostic::lifetime_escape(AstNodeId::from_raw(302), AstNodeId::from_raw(402)),
        ]
    );
    assert_eq!(diagnostics[0].kind(), BorrowDiagnosticKind::LifetimeEscape);
    assert_eq!(diagnostics[0].node(), AstNodeId::from_raw(301));
    assert_eq!(diagnostics[0].conflict_origin(), AstNodeId::from_raw(401));
}

fn local_binding(binding_raw: usize, symbol_raw: usize) -> LocalBinding {
    LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(symbol_raw)),
        AstNodeId::from_raw(binding_raw),
        LocalBindingKind::Immutable,
    )
}

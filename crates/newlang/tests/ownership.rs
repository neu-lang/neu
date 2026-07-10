use newlang::{
    ast::AstNodeId,
    module::{ModuleName, PackageNamespace},
    name_resolution::{
        LocalBinding, LocalBindingKey, LocalBindingKind, LocalScopeId, ResolvedLocalBinding,
    },
    ownership::{
        OwnershipCategory, OwnershipDiagnostic, OwnershipDiagnosticKind, OwnershipTransfer,
        OwnershipTransferKind, analyze_use_after_move, classify_ownership_category,
        collect_ownership_transfers,
    },
    parser::{ParsedAssignmentStatement, ParsedLocalDeclaration},
    symbol::SymbolId,
    type_check::DeclarationSignature,
    types::{
        GenericParameterType, NominalTypeIdentity, NullableType, PrimitiveType, TypeArena,
        TypeRecord,
    },
};

#[test]
fn m0022_primitive_ownership_categories_follow_adr0035() {
    let mut arena = TypeArena::new();
    let bool_ty = arena.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let int_ty = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let unit_ty = arena.insert(TypeRecord::primitive(PrimitiveType::Unit));
    let null_ty = arena.insert(TypeRecord::primitive(PrimitiveType::Null));
    let string_ty = arena.insert(TypeRecord::primitive(PrimitiveType::String));

    for ty in [bool_ty, int_ty, unit_ty, null_ty] {
        assert_eq!(
            classify_ownership_category(&arena, ty),
            Some(OwnershipCategory::Copyable)
        );
    }
    assert_eq!(
        classify_ownership_category(&arena, string_ty),
        Some(OwnershipCategory::MoveOnly)
    );
}

#[test]
fn m0022_nominal_identities_are_move_only() {
    let mut arena = TypeArena::new();
    let nominal = arena.insert(TypeRecord::nominal(NominalTypeIdentity::new(
        ModuleName::parse("demo.domain").unwrap(),
        PackageNamespace::parse("model").unwrap(),
        AstNodeId::from_raw(20),
        SymbolId::from_raw(30),
    )));

    assert_eq!(
        classify_ownership_category(&arena, nominal),
        Some(OwnershipCategory::MoveOnly)
    );
}

#[test]
fn m0022_unsupported_type_categories_do_not_guess() {
    let mut arena = TypeArena::new();
    let generic = arena.insert(TypeRecord::generic_parameter(GenericParameterType::new(
        AstNodeId::from_raw(40),
        SymbolId::from_raw(50),
    )));
    let nullable = arena.insert(TypeRecord::nullable(NullableType::new(generic)));

    assert_eq!(classify_ownership_category(&arena, generic), None);
    assert_eq!(classify_ownership_category(&arena, nullable), None);
}

#[test]
fn m0022_transfer_sites_record_only_move_only_local_sources() {
    let mut arena = TypeArena::new();
    let string_ty = arena.insert(TypeRecord::primitive(PrimitiveType::String));
    let int_ty = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let generic_ty = arena.insert(TypeRecord::generic_parameter(GenericParameterType::new(
        AstNodeId::from_raw(900),
        SymbolId::from_raw(901),
    )));

    let string_binding = local_binding(10, 100);
    let int_binding = local_binding(11, 101);
    let generic_binding = local_binding(12, 102);
    let declarations = [
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(200),
            annotation: None,
            initializer: Some(AstNodeId::from_raw(300)),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(201),
            annotation: None,
            initializer: Some(AstNodeId::from_raw(301)),
        },
        ParsedLocalDeclaration {
            declaration: AstNodeId::from_raw(202),
            annotation: None,
            initializer: Some(AstNodeId::from_raw(302)),
        },
    ];
    let assignments = [ParsedAssignmentStatement {
        statement: AstNodeId::from_raw(400),
        target: AstNodeId::from_raw(401),
        value: AstNodeId::from_raw(402),
    }];
    let resolved = [
        ResolvedLocalBinding::new(AstNodeId::from_raw(300), string_binding.clone()),
        ResolvedLocalBinding::new(AstNodeId::from_raw(301), int_binding.clone()),
        ResolvedLocalBinding::new(AstNodeId::from_raw(302), generic_binding.clone()),
        ResolvedLocalBinding::new(AstNodeId::from_raw(402), string_binding.clone()),
    ];
    let signatures = [
        DeclarationSignature::new(string_binding.binding(), string_ty),
        DeclarationSignature::new(int_binding.binding(), int_ty),
        DeclarationSignature::new(generic_binding.binding(), generic_ty),
    ];

    let transfers =
        collect_ownership_transfers(&declarations, &assignments, &resolved, &signatures, &arena);

    assert_eq!(
        transfers,
        [
            OwnershipTransfer::new(
                OwnershipTransferKind::LocalInitializer,
                AstNodeId::from_raw(200),
                AstNodeId::from_raw(300),
                string_binding.clone(),
            ),
            OwnershipTransfer::new(
                OwnershipTransferKind::Assignment,
                AstNodeId::from_raw(400),
                AstNodeId::from_raw(402),
                string_binding,
            ),
        ]
    );
}

#[test]
fn m0022_use_after_move_diagnostics_report_later_uses_and_origin() {
    let moved = local_binding(10, 100);
    let other = local_binding(11, 101);
    let before = ResolvedLocalBinding::new(AstNodeId::from_raw(90), moved.clone());
    let origin = ResolvedLocalBinding::new(AstNodeId::from_raw(100), moved.clone());
    let later = ResolvedLocalBinding::new(AstNodeId::from_raw(110), moved.clone());
    let second_later = ResolvedLocalBinding::new(AstNodeId::from_raw(120), moved.clone());
    let other_later = ResolvedLocalBinding::new(AstNodeId::from_raw(130), other);
    let transfers = [OwnershipTransfer::new(
        OwnershipTransferKind::LocalInitializer,
        AstNodeId::from_raw(200),
        origin.reference(),
        moved,
    )];

    let diagnostics = analyze_use_after_move(
        &[before, origin, later, second_later, other_later],
        &transfers,
    );

    assert_eq!(
        diagnostics,
        [
            OwnershipDiagnostic::use_after_move(AstNodeId::from_raw(110), AstNodeId::from_raw(100)),
            OwnershipDiagnostic::use_after_move(AstNodeId::from_raw(120), AstNodeId::from_raw(100)),
        ]
    );
    assert_eq!(diagnostics[0].kind(), OwnershipDiagnosticKind::UseAfterMove);
    assert_eq!(diagnostics[0].node(), AstNodeId::from_raw(110));
    assert_eq!(diagnostics[0].move_origin(), AstNodeId::from_raw(100));
}

fn local_binding(binding_raw: usize, symbol_raw: usize) -> LocalBinding {
    LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(0), SymbolId::from_raw(symbol_raw)),
        AstNodeId::from_raw(binding_raw),
        LocalBindingKind::Immutable,
    )
}

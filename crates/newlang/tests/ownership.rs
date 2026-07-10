use newlang::{
    ast::AstNodeId,
    module::{ModuleName, PackageNamespace},
    ownership::{classify_ownership_category, OwnershipCategory},
    symbol::SymbolId,
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

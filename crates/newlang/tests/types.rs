use newlang::{
    ast::AstNodeId,
    module::{ModuleName, PackageNamespace},
    symbol::SymbolId,
    types::{GenericParameterType, NominalTypeIdentity, TypeArena, TypeKind, TypeRecord},
};

#[test]
fn type_ids_are_stable_in_insertion_order() {
    let module = ModuleName::parse("core.types").unwrap();
    let package = PackageNamespace::parse("collections").unwrap();
    let first_symbol = SymbolId::from_raw(1);
    let second_symbol = SymbolId::from_raw(2);

    let mut arena = TypeArena::new();
    let first = arena.insert(TypeRecord::nominal(NominalTypeIdentity::new(
        module.clone(),
        package.clone(),
        AstNodeId::from_raw(10),
        first_symbol,
    )));
    let second = arena.insert(TypeRecord::nominal(NominalTypeIdentity::new(
        module,
        package,
        AstNodeId::from_raw(11),
        second_symbol,
    )));

    assert_eq!(first.index(), 0);
    assert_eq!(second.index(), 1);
    assert_eq!(arena.records().len(), 2);
    assert_eq!(arena.get(first).unwrap().id(), first);
    assert_eq!(arena.get(second).unwrap().id(), second);
}

#[test]
fn nominal_type_identity_includes_module_package_declaration_and_symbol() {
    let module = ModuleName::parse("app.domain").unwrap();
    let package = PackageNamespace::parse("user.profile").unwrap();
    let declaration = AstNodeId::from_raw(42);
    let symbol = SymbolId::from_raw(7);

    let identity = NominalTypeIdentity::new(module.clone(), package.clone(), declaration, symbol);

    assert_eq!(identity.module(), &module);
    assert_eq!(identity.package(), &package);
    assert_eq!(identity.declaration(), declaration);
    assert_eq!(identity.symbol(), symbol);
}

#[test]
fn distinct_packages_produce_distinct_nominal_type_identities() {
    let module = ModuleName::parse("app.domain").unwrap();
    let public_package = PackageNamespace::parse("public").unwrap();
    let internal_package = PackageNamespace::parse("internal").unwrap();
    let declaration = AstNodeId::from_raw(5);
    let symbol = SymbolId::from_raw(9);

    let public_identity =
        NominalTypeIdentity::new(module.clone(), public_package, declaration, symbol);
    let internal_identity = NominalTypeIdentity::new(module, internal_package, declaration, symbol);

    assert_ne!(public_identity, internal_identity);
}

#[test]
fn generic_parameter_type_preserves_declaring_node_and_symbol() {
    let declaration = AstNodeId::from_raw(12);
    let symbol = SymbolId::from_raw(3);

    let generic = GenericParameterType::new(declaration, symbol);

    assert_eq!(generic.declaration(), declaration);
    assert_eq!(generic.symbol(), symbol);
}

#[test]
fn type_record_preserves_kind_and_id() {
    let generic = GenericParameterType::new(AstNodeId::from_raw(12), SymbolId::from_raw(3));
    let mut arena = TypeArena::new();

    let id = arena.insert(TypeRecord::generic_parameter(generic.clone()));
    let record = arena.get(id).unwrap();

    assert_eq!(record.id(), id);
    assert_eq!(record.kind(), &TypeKind::GenericParameter(generic));
}

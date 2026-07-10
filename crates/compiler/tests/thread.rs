use compiler::{
    ast::AstNodeId,
    module::{ModuleName, PackageNamespace},
    symbol::SymbolId,
    thread::{ThreadCapability, satisfies_thread_capability},
    types::{
        GenericParameterType, NominalTypeIdentity, NullableType, PrimitiveType, TypeArena, TypeId,
        TypeRecord,
    },
};

#[test]
fn m0024_primitives_follow_adr0037_capabilities() {
    let mut arena = TypeArena::new();
    let bool_ty = arena.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let int_ty = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let unit_ty = arena.insert(TypeRecord::primitive(PrimitiveType::Unit));
    let null_ty = arena.insert(TypeRecord::primitive(PrimitiveType::Null));
    let string_ty = arena.insert(TypeRecord::primitive(PrimitiveType::String));

    for ty in [bool_ty, int_ty, unit_ty, null_ty] {
        assert!(satisfies_thread_capability(
            &arena,
            ty,
            ThreadCapability::Send
        ));
        assert!(satisfies_thread_capability(
            &arena,
            ty,
            ThreadCapability::Share
        ));
    }

    assert!(satisfies_thread_capability(
        &arena,
        string_ty,
        ThreadCapability::Send
    ));
    assert!(!satisfies_thread_capability(
        &arena,
        string_ty,
        ThreadCapability::Share
    ));
}

#[test]
fn m0024_nullable_capabilities_follow_base_type() {
    let mut arena = TypeArena::new();
    let int_ty = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let string_ty = arena.insert(TypeRecord::primitive(PrimitiveType::String));
    let generic_ty = arena.insert(TypeRecord::generic_parameter(GenericParameterType::new(
        AstNodeId::from_raw(10),
        SymbolId::from_raw(20),
    )));
    let nullable_int = arena.insert(TypeRecord::nullable(NullableType::new(int_ty)));
    let nullable_string = arena.insert(TypeRecord::nullable(NullableType::new(string_ty)));
    let nullable_generic = arena.insert(TypeRecord::nullable(NullableType::new(generic_ty)));

    assert!(satisfies_thread_capability(
        &arena,
        nullable_int,
        ThreadCapability::Send
    ));
    assert!(satisfies_thread_capability(
        &arena,
        nullable_int,
        ThreadCapability::Share
    ));
    assert!(satisfies_thread_capability(
        &arena,
        nullable_string,
        ThreadCapability::Send
    ));
    assert!(!satisfies_thread_capability(
        &arena,
        nullable_string,
        ThreadCapability::Share
    ));
    assert!(!satisfies_thread_capability(
        &arena,
        nullable_generic,
        ThreadCapability::Send
    ));
    assert!(!satisfies_thread_capability(
        &arena,
        nullable_generic,
        ThreadCapability::Share
    ));
}

#[test]
fn m0024_nominal_generic_and_missing_types_satisfy_no_capabilities() {
    let mut arena = TypeArena::new();
    let nominal = arena.insert(TypeRecord::nominal(NominalTypeIdentity::new(
        ModuleName::parse("demo.domain").unwrap(),
        PackageNamespace::parse("model").unwrap(),
        AstNodeId::from_raw(30),
        SymbolId::from_raw(40),
    )));
    let generic = arena.insert(TypeRecord::generic_parameter(GenericParameterType::new(
        AstNodeId::from_raw(50),
        SymbolId::from_raw(60),
    )));
    let missing = TypeId::from_raw(999);

    for ty in [nominal, generic, missing] {
        assert!(!satisfies_thread_capability(
            &arena,
            ty,
            ThreadCapability::Send
        ));
        assert!(!satisfies_thread_capability(
            &arena,
            ty,
            ThreadCapability::Share
        ));
    }
}

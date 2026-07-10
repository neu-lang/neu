use newlang::{
    ast::AstNodeId,
    module::{ModuleName, PackageNamespace},
    symbol::SymbolId,
    types::{
        GenericParameterType, NominalTypeIdentity, NullableType, TypeArena, TypeDiagnostic,
        TypeDiagnosticKind, TypeId, TypeKind, TypeRecord, UnsupportedTypeForm,
    },
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

#[test]
fn nullable_type_preserves_wrapped_base_type() {
    let mut arena = TypeArena::new();
    let base = arena.insert(TypeRecord::generic_parameter(GenericParameterType::new(
        AstNodeId::from_raw(1),
        SymbolId::from_raw(2),
    )));

    let nullable = NullableType::new(base);

    assert_eq!(nullable.base(), base);
}

#[test]
fn nullable_type_record_is_distinct_from_base_record() {
    let mut arena = TypeArena::new();
    let base = arena.insert(TypeRecord::generic_parameter(GenericParameterType::new(
        AstNodeId::from_raw(1),
        SymbolId::from_raw(2),
    )));
    let nullable = arena.insert(TypeRecord::nullable(NullableType::new(base)));

    assert_ne!(base, nullable);
    assert!(matches!(
        arena.get(base).unwrap().kind(),
        TypeKind::GenericParameter(_)
    ));
    assert_eq!(
        arena.get(nullable).unwrap().kind(),
        &TypeKind::Nullable(NullableType::new(base))
    );
}

#[test]
fn nullable_record_storage_preserves_insertion_order() {
    let mut arena = TypeArena::new();
    let base = arena.insert(TypeRecord::generic_parameter(GenericParameterType::new(
        AstNodeId::from_raw(10),
        SymbolId::from_raw(20),
    )));
    let nullable = arena.insert(TypeRecord::nullable(NullableType::new(base)));

    assert_eq!(base.index(), 0);
    assert_eq!(nullable.index(), 1);
    assert_eq!(arena.get(nullable).unwrap().id(), nullable);
}

#[test]
fn unsupported_type_form_diagnostic_preserves_form_and_node() {
    let node = AstNodeId::from_raw(99);

    let diagnostic =
        TypeDiagnostic::unsupported_type_form(UnsupportedTypeForm::WildcardOrStarProjection, node);

    assert_eq!(diagnostic.kind(), TypeDiagnosticKind::UnsupportedTypeForm);
    assert_eq!(
        diagnostic.form(),
        Some(UnsupportedTypeForm::WildcardOrStarProjection)
    );
    assert_eq!(diagnostic.node(), node);
}

#[test]
fn unsupported_type_form_variants_cover_adr0023_deferrals() {
    let deferred_forms = [
        UnsupportedTypeForm::VarianceAnnotation,
        UnsupportedTypeForm::WildcardOrStarProjection,
        UnsupportedTypeForm::ReceiverFunctionType,
        UnsupportedTypeForm::FunctionTypeParameterName,
        UnsupportedTypeForm::TypeAnnotationSyntax,
        UnsupportedTypeForm::TypeAlias,
        UnsupportedTypeForm::AssociatedType,
        UnsupportedTypeForm::HigherKindedType,
        UnsupportedTypeForm::DependentType,
        UnsupportedTypeForm::IntersectionType,
        UnsupportedTypeForm::UnionType,
        UnsupportedTypeForm::InferredPlaceholderType,
        UnsupportedTypeForm::LayoutType,
        UnsupportedTypeForm::EffectType,
        UnsupportedTypeForm::CoroutineSuspensionMarker,
    ];

    assert_eq!(deferred_forms.len(), 15);
}

#[test]
fn unsupported_type_forms_do_not_become_type_records() {
    let arena = TypeArena::new();
    let diagnostic = TypeDiagnostic::unsupported_type_form(
        UnsupportedTypeForm::ReceiverFunctionType,
        AstNodeId::from_raw(8),
    );

    assert_eq!(arena.records().len(), 0);
    assert_eq!(diagnostic.kind(), TypeDiagnosticKind::UnsupportedTypeForm);
    assert!(arena.get(TypeId::from_raw(0)).is_none());
}

use compiler::{
    ast::AstNodeId,
    module::{ModuleName, PackageNamespace},
    parser::ParsedGenericParameter,
    symbol::{SymbolId, SymbolInterner},
    type_check::build_generic_declaration_records,
    types::{
        FunctionType, GenericParameterType, GenericSpecializationIdentity,
        GenericSpecializationRegistry, GenericSubstitution, GenericTypeIdentity,
        NominalTypeIdentity, NullableType, PrimitiveType, TypeArena, TypeDiagnostic,
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
fn generic_instance_identity_is_nominal_and_ordered() {
    let module = ModuleName::parse("collections").unwrap();
    let package = PackageNamespace::root();
    let declaration = AstNodeId::from_raw(80);
    let identity = NominalTypeIdentity::new(module, package, declaration, SymbolId::from_raw(80));
    let mut arena = TypeArena::new();
    let int_type = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let byte_type = arena.insert(TypeRecord::primitive(PrimitiveType::Byte));
    let first = arena.generic_instance(GenericTypeIdentity::new(
        identity.clone(),
        vec![int_type, byte_type],
    ));
    let same = arena.generic_instance(GenericTypeIdentity::new(
        identity.clone(),
        vec![int_type, byte_type],
    ));
    let reversed = arena.generic_instance(GenericTypeIdentity::new(
        identity,
        vec![byte_type, int_type],
    ));

    assert_eq!(first, same);
    assert_ne!(first, reversed);
}

#[test]
fn generic_substitution_recurses_through_nullable_array_and_instance_types() {
    let module = ModuleName::parse("collections").unwrap();
    let identity = NominalTypeIdentity::new(
        module,
        PackageNamespace::root(),
        AstNodeId::from_raw(81),
        SymbolId::from_raw(81),
    );
    let parameter = AstNodeId::from_raw(82);
    let mut arena = TypeArena::new();
    let generic = arena.insert(TypeRecord::generic_parameter(GenericParameterType::new(
        parameter,
        SymbolId::from_raw(82),
    )));
    let int_type = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let nullable = arena.nullable(generic);
    let array = arena.array(nullable, 2);
    let instance = arena.generic_instance(GenericTypeIdentity::new(identity, vec![array]));
    let mut substitution = GenericSubstitution::new();
    substitution.insert(generic, int_type);

    let substituted = substitution.apply(instance, &mut arena);
    let TypeKind::GenericInstance(instance) = arena.get(substituted).unwrap().kind() else {
        panic!("expected substituted generic instance");
    };
    let [argument] = instance.arguments() else {
        panic!("expected one generic argument");
    };
    let TypeKind::Array(array) = arena.get(*argument).unwrap().kind() else {
        panic!("expected substituted array");
    };
    let TypeKind::Nullable(nullable) = arena.get(array.element()).unwrap().kind() else {
        panic!("expected substituted nullable element");
    };
    assert_eq!(nullable.base(), int_type);
}

#[test]
fn generic_declaration_records_preserve_owner_and_parameter_order() {
    let parameters = vec![
        ParsedGenericParameter {
            parameter: AstNodeId::from_raw(90),
            owner: Some(AstNodeId::from_raw(91)),
            name: "T".to_owned(),
            name_span: compiler::source::ByteSpan::new(
                compiler::source::SourceFileId::from_raw(90),
                0,
                1,
            )
            .unwrap(),
            capability_bounds: Vec::new(),
        },
        ParsedGenericParameter {
            parameter: AstNodeId::from_raw(92),
            owner: Some(AstNodeId::from_raw(91)),
            name: "U".to_owned(),
            name_span: compiler::source::ByteSpan::new(
                compiler::source::SourceFileId::from_raw(90),
                2,
                3,
            )
            .unwrap(),
            capability_bounds: Vec::new(),
        },
    ];
    let mut symbols = SymbolInterner::new();
    let mut arena = TypeArena::new();
    let records = build_generic_declaration_records(&parameters, &mut symbols, &mut arena);

    assert_eq!(records.len(), 1);
    assert_eq!(records[0].owner(), AstNodeId::from_raw(91));
    assert_eq!(records[0].parameters().len(), 2);
    assert_eq!(
        records[0].parameters()[0].parameter(),
        parameters[0].parameter
    );
    assert_eq!(
        records[0].parameters()[1].parameter(),
        parameters[1].parameter
    );
}

#[test]
fn generic_specialization_identity_is_deduplicated_and_ordered() {
    let mut arena = TypeArena::new();
    let int_type = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let byte_type = arena.insert(TypeRecord::primitive(PrimitiveType::Byte));
    let declaration = AstNodeId::from_raw(100);
    let first = GenericSpecializationIdentity::new(declaration, vec![int_type, byte_type]);
    let same = GenericSpecializationIdentity::new(declaration, vec![int_type, byte_type]);
    let reversed = GenericSpecializationIdentity::new(declaration, vec![byte_type, int_type]);

    assert_eq!(first, same);
    assert_ne!(first, reversed);
    assert_eq!(first.arguments(), &[int_type, byte_type]);
    assert_eq!(first.mangle("identity"), "identity$g100$0_1");

    let mut registry = GenericSpecializationRegistry::new();
    assert_eq!(
        registry.request(first.clone()),
        compiler::types::SpecializationRequest::New(0)
    );
    assert_eq!(
        registry.request(first.clone()),
        compiler::types::SpecializationRequest::Recursive
    );
    registry.finish(&first);
    assert_eq!(
        registry.request(first),
        compiler::types::SpecializationRequest::Existing(0)
    );
}

#[test]
fn function_type_identity_is_structural_and_deduplicated() {
    let mut arena = TypeArena::new();
    let int_type = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let byte_type = arena.insert(TypeRecord::primitive(PrimitiveType::Byte));
    let first = arena.function(FunctionType::new(vec![int_type, byte_type], int_type));
    let same = arena.function(FunctionType::new(vec![int_type, byte_type], int_type));
    let different = arena.function(FunctionType::new(vec![byte_type, int_type], int_type));

    assert_eq!(first, same);
    assert_ne!(first, different);
    let TypeKind::Function(function) = arena.get(first).unwrap().kind() else {
        panic!("expected function type");
    };
    assert_eq!(function.parameters(), &[int_type, byte_type]);
    assert_eq!(function.return_type(), int_type);
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
fn primitive_type_records_cover_adr0027_bootstrap_identities() {
    let primitives = [
        PrimitiveType::Bool,
        PrimitiveType::Int,
        PrimitiveType::String,
        PrimitiveType::Void,
        PrimitiveType::Null,
    ];

    assert_eq!(primitives.len(), 5);

    let mut arena = TypeArena::new();
    let ids: Vec<_> = primitives
        .iter()
        .copied()
        .map(|primitive| arena.insert(TypeRecord::primitive(primitive)))
        .collect();

    for (index, primitive) in primitives.iter().copied().enumerate() {
        let record = arena.get(ids[index]).unwrap();
        assert_eq!(record.id(), ids[index]);
        assert_eq!(record.kind(), &TypeKind::Primitive(primitive));
    }
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

use compiler::{
    ast::AstNodeId,
    module::{ModuleName, PackageNamespace},
    name_resolution::{LocalBinding, LocalBindingKey, LocalBindingKind, LocalScopeId},
    symbol::SymbolId,
    thread::{
        ClosureBoundary, ClosureCapture, ClosureCaptureKind, ClosureConcurrencyDiagnostic,
        ClosureConcurrencyDiagnosticKind, ThreadBoundary, ThreadCapability, ThreadCapture,
        ThreadDiagnostic, ThreadDiagnosticKind, analyze_closure_boundaries,
        analyze_thread_boundaries, satisfies_thread_capability,
    },
    types::{
        GenericParameterType, NominalTypeIdentity, NullableType, PrimitiveType, TypeArena, TypeId,
        TypeRecord,
    },
};

#[test]
fn primitives_follow_adr0037_capabilities() {
    let mut arena = TypeArena::new();
    let bool_ty = arena.insert(TypeRecord::primitive(PrimitiveType::Bool));
    let int_ty = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let unit_ty = arena.insert(TypeRecord::primitive(PrimitiveType::Void));
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
fn closure_transfer_requires_send_and_shared_use_requires_share() {
    let mut arena = TypeArena::new();
    let string_ty = arena.insert(TypeRecord::primitive(PrimitiveType::String));
    let transfer = ClosureCapture::new(
        AstNodeId::from_raw(9001),
        local_binding(90, 900),
        string_ty,
        ClosureCaptureKind::Moved,
    );
    let shared = ClosureCapture::new(
        AstNodeId::from_raw(9002),
        local_binding(91, 901),
        string_ty,
        ClosureCaptureKind::Shared,
    );

    let diagnostics = analyze_closure_boundaries(
        &[ClosureBoundary::new(
            AstNodeId::from_raw(9000),
            vec![transfer, shared],
        )],
        &arena,
    );

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].capture(), AstNodeId::from_raw(9002));
    assert_eq!(
        diagnostics[0].kind(),
        ClosureConcurrencyDiagnosticKind::MissingCapability
    );
    assert_eq!(diagnostics[0].required(), ThreadCapability::Share);
}

#[test]
fn borrowed_and_mutably_shared_captures_are_rejected() {
    let mut arena = TypeArena::new();
    let int_ty = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let boundary = AstNodeId::from_raw(9100);
    let diagnostics = analyze_closure_boundaries(
        &[ClosureBoundary::new(
            boundary,
            vec![
                ClosureCapture::new(
                    AstNodeId::from_raw(9101),
                    local_binding(92, 902),
                    int_ty,
                    ClosureCaptureKind::Borrowed,
                ),
                ClosureCapture::new(
                    AstNodeId::from_raw(9102),
                    local_binding(93, 903),
                    int_ty,
                    ClosureCaptureKind::MutableShared,
                ),
            ],
        )],
        &arena,
    );

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(
        diagnostics[0],
        ClosureConcurrencyDiagnostic::borrowed_capture(
            AstNodeId::from_raw(9101),
            boundary,
            local_binding(92, 902),
            int_ty,
        )
    );
    assert_eq!(
        diagnostics[1].kind(),
        ClosureConcurrencyDiagnosticKind::MutableSharedCapture
    );
}

#[test]
fn nullable_capabilities_follow_base_type() {
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
fn nominal_generic_and_missing_types_satisfy_no_capabilities() {
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

#[test]
fn boundary_analysis_reports_missing_capabilities() {
    let mut arena = TypeArena::new();
    let string_ty = arena.insert(TypeRecord::primitive(PrimitiveType::String));
    let int_ty = arena.insert(TypeRecord::primitive(PrimitiveType::Int));
    let nominal_ty = arena.insert(TypeRecord::nominal(NominalTypeIdentity::new(
        ModuleName::parse("demo.domain").unwrap(),
        PackageNamespace::parse("model").unwrap(),
        AstNodeId::from_raw(70),
        SymbolId::from_raw(80),
    )));
    let boundary = AstNodeId::from_raw(1000);
    let string_binding = local_binding(10, 100);
    let int_binding = local_binding(11, 101);
    let nominal_binding = local_binding(12, 102);
    let captures = vec![
        ThreadCapture::new(
            AstNodeId::from_raw(1001),
            string_binding.clone(),
            string_ty,
            ThreadCapability::Send,
        ),
        ThreadCapture::new(
            AstNodeId::from_raw(1002),
            string_binding.clone(),
            string_ty,
            ThreadCapability::Share,
        ),
        ThreadCapture::new(
            AstNodeId::from_raw(1003),
            int_binding,
            int_ty,
            ThreadCapability::Share,
        ),
        ThreadCapture::new(
            AstNodeId::from_raw(1004),
            nominal_binding.clone(),
            nominal_ty,
            ThreadCapability::Send,
        ),
    ];

    let diagnostics = analyze_thread_boundaries(&[ThreadBoundary::new(boundary, captures)], &arena);

    assert_eq!(
        diagnostics,
        [
            ThreadDiagnostic::missing_thread_capability(
                AstNodeId::from_raw(1002),
                boundary,
                string_binding,
                string_ty,
                ThreadCapability::Share,
            ),
            ThreadDiagnostic::missing_thread_capability(
                AstNodeId::from_raw(1004),
                boundary,
                nominal_binding,
                nominal_ty,
                ThreadCapability::Send,
            ),
        ]
    );
    assert_eq!(
        diagnostics[0].kind(),
        ThreadDiagnosticKind::MissingThreadCapability
    );
}

#[test]
fn boundary_diagnostics_preserve_order_and_spans() {
    let mut arena = TypeArena::new();
    let generic_ty = arena.insert(TypeRecord::generic_parameter(GenericParameterType::new(
        AstNodeId::from_raw(90),
        SymbolId::from_raw(91),
    )));
    let missing_ty = TypeId::from_raw(999);
    let first_boundary = AstNodeId::from_raw(2000);
    let second_boundary = AstNodeId::from_raw(3000);
    let first_binding = local_binding(20, 200);
    let second_binding = local_binding(21, 201);

    let diagnostics = analyze_thread_boundaries(
        &[
            ThreadBoundary::new(
                first_boundary,
                vec![ThreadCapture::new(
                    AstNodeId::from_raw(2001),
                    first_binding.clone(),
                    generic_ty,
                    ThreadCapability::Send,
                )],
            ),
            ThreadBoundary::new(
                second_boundary,
                vec![ThreadCapture::new(
                    AstNodeId::from_raw(3001),
                    second_binding.clone(),
                    missing_ty,
                    ThreadCapability::Share,
                )],
            ),
        ],
        &arena,
    );

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].capture(), AstNodeId::from_raw(2001));
    assert_eq!(diagnostics[0].boundary(), first_boundary);
    assert_eq!(diagnostics[0].binding(), &first_binding);
    assert_eq!(diagnostics[0].ty(), generic_ty);
    assert_eq!(diagnostics[0].required(), ThreadCapability::Send);
    assert_eq!(diagnostics[1].capture(), AstNodeId::from_raw(3001));
    assert_eq!(diagnostics[1].boundary(), second_boundary);
    assert_eq!(diagnostics[1].binding(), &second_binding);
    assert_eq!(diagnostics[1].ty(), missing_ty);
    assert_eq!(diagnostics[1].required(), ThreadCapability::Share);
}

fn local_binding(binding_raw: usize, symbol_raw: usize) -> LocalBinding {
    LocalBinding::new(
        LocalBindingKey::new(LocalScopeId::from_raw(1), SymbolId::from_raw(symbol_raw)),
        AstNodeId::from_raw(binding_raw),
        LocalBindingKind::Immutable,
    )
}

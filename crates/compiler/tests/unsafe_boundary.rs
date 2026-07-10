use compiler::{
    ast::AstNodeId,
    unsafe_boundary::{
        SafetyBasis, UnsafeContext, UnsafeContextKind, UnsafeDiagnostic, UnsafeDiagnosticKind,
        UnsafeOperation, UnsafeOperationKind, analyze_unsafe_operations,
    },
};

#[test]
fn m0026_unsafe_analysis_accepts_proven_safe_operations_without_context() {
    let diagnostics = analyze_unsafe_operations(
        &[],
        &[UnsafeOperation::new(
            AstNodeId::from_raw(100),
            UnsafeOperationKind::RawPointerDereference,
            SafetyBasis::ProvenSafe,
            None,
        )],
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn m0026_unsafe_analysis_accepts_trusted_operations_in_matching_context() {
    let context = AstNodeId::from_raw(200);
    let diagnostics = analyze_unsafe_operations(
        &[UnsafeContext::new(context, UnsafeContextKind::Block)],
        &[UnsafeOperation::new(
            AstNodeId::from_raw(201),
            UnsafeOperationKind::ForeignCall,
            SafetyBasis::TrustedUnsafe,
            Some(context),
        )],
    );

    assert!(diagnostics.is_empty());
}

#[test]
fn m0026_unsafe_analysis_reports_trusted_operation_without_context() {
    let operation = AstNodeId::from_raw(300);
    let diagnostics = analyze_unsafe_operations(
        &[],
        &[UnsafeOperation::new(
            operation,
            UnsafeOperationKind::UnsafeCapabilityAssertion,
            SafetyBasis::TrustedUnsafe,
            None,
        )],
    );

    assert_eq!(
        diagnostics,
        [UnsafeDiagnostic::unsafe_operation_outside_context(
            operation,
            UnsafeOperationKind::UnsafeCapabilityAssertion,
            SafetyBasis::TrustedUnsafe,
            None,
        )]
    );
    assert_eq!(
        diagnostics[0].kind(),
        UnsafeDiagnosticKind::UnsafeOperationOutsideContext
    );
}

#[test]
fn m0026_unsafe_analysis_reports_non_matching_context_and_preserves_order() {
    let valid_context = AstNodeId::from_raw(400);
    let missing_context = AstNodeId::from_raw(401);
    let first_operation = AstNodeId::from_raw(402);
    let second_operation = AstNodeId::from_raw(403);

    let diagnostics = analyze_unsafe_operations(
        &[UnsafeContext::new(
            valid_context,
            UnsafeContextKind::Function,
        )],
        &[
            UnsafeOperation::new(
                first_operation,
                UnsafeOperationKind::ForeignCall,
                SafetyBasis::TrustedUnsafe,
                Some(missing_context),
            ),
            UnsafeOperation::new(
                AstNodeId::from_raw(404),
                UnsafeOperationKind::RawPointerDereference,
                SafetyBasis::TrustedUnsafe,
                Some(valid_context),
            ),
            UnsafeOperation::new(
                second_operation,
                UnsafeOperationKind::UnsafeCapabilityAssertion,
                SafetyBasis::TrustedUnsafe,
                None,
            ),
        ],
    );

    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].operation(), first_operation);
    assert_eq!(diagnostics[0].context(), Some(missing_context));
    assert_eq!(
        diagnostics[0].operation_kind(),
        UnsafeOperationKind::ForeignCall
    );
    assert_eq!(diagnostics[0].safety_basis(), SafetyBasis::TrustedUnsafe);
    assert_eq!(diagnostics[1].operation(), second_operation);
    assert_eq!(diagnostics[1].context(), None);
}

use newlang::{
    ast::AstNodeId,
    type_check::{
        AmbiguousTypeRule, TypeCheckDiagnostic, TypeCheckDiagnosticKind, TypeCheckReport,
    },
};

#[test]
fn ambiguous_type_rule_diagnostic_preserves_rule_and_node() {
    let node = AstNodeId::from_raw(17);

    let diagnostic =
        TypeCheckDiagnostic::ambiguous_type_rule(AmbiguousTypeRule::LiteralTyping, node);

    assert_eq!(
        diagnostic.kind(),
        TypeCheckDiagnosticKind::AmbiguousTypeRule
    );
    assert_eq!(diagnostic.rule(), AmbiguousTypeRule::LiteralTyping);
    assert_eq!(diagnostic.node(), node);
}

#[test]
fn ambiguous_type_rules_cover_m0018_blockers() {
    let blockers = [
        AmbiguousTypeRule::LiteralTyping,
        AmbiguousTypeRule::PrimitiveScalarCatalog,
        AmbiguousTypeRule::AssignmentCompatibility,
        AmbiguousTypeRule::CallResolution,
        AmbiguousTypeRule::FunctionTypeApplication,
    ];

    assert_eq!(blockers.len(), 5);
}

#[test]
fn type_check_report_records_blockers_without_successful_output() {
    let diagnostic = TypeCheckDiagnostic::ambiguous_type_rule(
        AmbiguousTypeRule::CallResolution,
        AstNodeId::from_raw(4),
    );
    let report = TypeCheckReport::blocked(vec![diagnostic.clone()]);

    assert!(report.is_blocked());
    assert_eq!(report.diagnostics(), &[diagnostic]);
}

use compiler::parser::{DiagnosticKind, parse_source};
use compiler::source::SourceFileId;

fn parser_kinds(source: &str) -> Vec<DiagnosticKind> {
    parse_source(SourceFileId::from_raw(40001), source)
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.kind)
        .collect()
}

#[test]
fn missing_binary_operand_reports_one_root_cause() {
    // One missing operand is one error. The enclosing binary expression must
    // not add a second diagnostic for the same failure.
    assert_eq!(
        parser_kinds("func main() { return x + ; }"),
        vec![DiagnosticKind::MissingExpression],
    );
}

#[test]
fn failed_lambda_body_reports_one_root_cause() {
    // A lambda body that is a statement, not an expression, is one error. The
    // statement parser must not re-read the rest of the body as declarations.
    assert_eq!(
        parser_kinds("func main() { val f = { x: Int -> val y: Int = x; y }; return 0; }"),
        vec![DiagnosticKind::MissingExpression],
    );
}

#[test]
fn independent_errors_in_separate_statements_are_each_reported() {
    // Suppression is local to one failure. Two broken statements are two errors.
    let kinds = parser_kinds("func broken() { target = ; return + ; }");

    assert!(
        kinds.len() >= 2,
        "expected one error per statement, got {kinds:?}"
    );
}

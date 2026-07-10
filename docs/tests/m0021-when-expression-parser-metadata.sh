#!/usr/bin/env sh
set -eu
grep -Eq 'pub struct ParsedWhenExpression' crates/newlang/src/parser.rs
grep -Eq 'pub struct ParsedMatchArm' crates/newlang/src/parser.rs
grep -Eq 'fn parse_when_expression' crates/newlang/src/parser.rs
grep -Eq 'WhenExpression' crates/newlang/src/ast.rs
grep -Eq 'm0021_when_expression_records_subject_and_ordered_arms' crates/newlang/tests/parser.rs
echo "m0021-when-expression-parser-metadata: contract validated"

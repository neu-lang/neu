//! Unknown-name diagnostics for the executable path (issue 50).
//!
//! A simple name reference is unknown when no declaration, binding, parameter,
//! field, class, enum variant, loop variable, or built-in in the program has
//! that name. The check is deliberately conservative: a name declared anywhere
//! in the program is never reported, so valid programs cannot gain errors from
//! it. Scope-sensitive misuse of a declared name is not covered here.

use std::collections::BTreeSet;

use crate::parser::ParseOutput;
use crate::source::ByteSpan;

/// Names the compiler provides without a declaration. Each is matched by name
/// in the type checker, HIR, or parser.
const BUILT_IN_NAMES: &[&str] = &[
    "assert",
    "await",
    "cancel",
    "channel",
    "clone",
    "close",
    "fail",
    "length",
    "precondition",
    "receive",
    "send",
    "spawn",
    "super",
    "this",
];

/// A simple name that the program never declares.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnknownName {
    pub name: String,
    pub span: ByteSpan,
}

/// Returns each unknown simple name reference, in source order.
pub fn find_unknown_names(parsed: &ParseOutput) -> Vec<UnknownName> {
    let mut declared: BTreeSet<&str> = BUILT_IN_NAMES.iter().copied().collect();
    declared.extend(parsed.declaration_names.iter().map(|d| d.name.as_str()));
    declared.extend(parsed.local_binding_names.iter().map(|b| b.name.as_str()));
    declared.extend(parsed.function_parameters.iter().map(|p| p.name.as_str()));
    declared.extend(parsed.pattern_bindings.iter().map(|b| b.name.as_str()));
    declared.extend(
        parsed
            .lambda_expressions
            .iter()
            .flat_map(|lambda| lambda.parameters.iter().map(|p| p.name.as_str())),
    );
    declared.extend(parsed.class_declarations.iter().map(|c| c.name.as_str()));
    declared.extend(
        parsed
            .class_declarations
            .iter()
            .flat_map(|c| c.constructor_parameters.iter().map(|p| p.name.as_str())),
    );
    declared.extend(parsed.field_declarations.iter().map(|f| f.name.as_str()));
    declared.extend(parsed.enum_variants.iter().map(|v| v.name.as_str()));
    declared.extend(
        parsed
            .for_statements
            .iter()
            .map(|f| f.binding_name.as_str()),
    );
    declared.extend(
        parsed
            .generic_parameters
            .iter()
            .map(|parameter| parameter.name.as_str()),
    );

    parsed
        .name_references
        .iter()
        .filter(|reference| !reference.name.contains('.'))
        .filter(|reference| !declared.contains(reference.name.as_str()))
        .map(|reference| UnknownName {
            name: reference.name.clone(),
            span: reference.name_span,
        })
        .collect()
}

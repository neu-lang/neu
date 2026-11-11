//! Class and interface types, constructors, dispatch, fields, and methods.

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassTypeRecord {
    pub(crate) declaration: AstNodeId,
    pub(crate) name: String,
    pub(crate) visibility: String,
    pub(crate) is_final: bool,
    pub(crate) is_abstract: bool,
    pub(crate) interface: bool,
    pub(crate) type_id: TypeId,
    pub(crate) superclass: Option<String>,
    pub(crate) interfaces: Vec<String>,
    pub(crate) constructor_parameter_count: usize,
}

impl ClassTypeRecord {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn visibility(&self) -> &str {
        &self.visibility
    }
    pub fn is_final(&self) -> bool {
        self.is_final
    }
    pub fn is_abstract(&self) -> bool {
        self.is_abstract
    }
    pub fn is_interface(&self) -> bool {
        self.interface
    }
    pub fn declaration(&self) -> AstNodeId {
        self.declaration
    }
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
    pub fn superclass(&self) -> Option<&str> {
        self.superclass.as_deref()
    }
    pub fn interfaces(&self) -> &[String] {
        &self.interfaces
    }
    pub fn constructor_parameter_count(&self) -> usize {
        self.constructor_parameter_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldTypeRecord {
    pub(crate) declaration: AstNodeId,
    pub(crate) owner: AstNodeId,
    pub(crate) type_id: TypeId,
    pub(crate) name: String,
    pub(crate) visibility: String,
    pub(crate) mutable: bool,
}

impl FieldTypeRecord {
    pub fn declaration(&self) -> AstNodeId {
        self.declaration
    }
    pub fn owner(&self) -> AstNodeId {
        self.owner
    }
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn visibility(&self) -> &str {
        &self.visibility
    }
    pub fn mutable(&self) -> bool {
        self.mutable
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClassTypeReport {
    pub(crate) classes: Vec<ClassTypeRecord>,
    pub(crate) fields: Vec<FieldTypeRecord>,
    pub(crate) diagnostics: Vec<TypeCheckDiagnostic>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConstructorDiagnosticKind {
    UnknownClass,
    ArgumentCountMismatch,
    SuperclassArgumentCountMismatch,
    AbstractClassConstruction,
    AbstractClassIncomplete,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConstructorDiagnostic {
    pub(crate) kind: ConstructorDiagnosticKind,
    pub(crate) span: ByteSpan,
}

impl ConstructorDiagnostic {
    pub fn kind(self) -> ConstructorDiagnosticKind {
        self.kind
    }
    pub fn span(self) -> ByteSpan {
        self.span
    }
}

pub fn check_constructor_calls(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
) -> Vec<ConstructorDiagnostic> {
    let mut diagnostics = Vec::new();
    for expression in &parsed.new_expressions {
        if expression.dynamic_array {
            continue;
        }
        let Some(class) = classes.classes.iter().find(|class| {
            parsed
                .class_declarations
                .iter()
                .find(|declaration| declaration.declaration == class.declaration)
                .is_some_and(|declaration| declaration.name == expression.type_name)
        }) else {
            diagnostics.push(ConstructorDiagnostic {
                kind: ConstructorDiagnosticKind::UnknownClass,
                span: expression.type_span,
            });
            continue;
        };
        if class.is_abstract {
            diagnostics.push(ConstructorDiagnostic {
                kind: ConstructorDiagnosticKind::AbstractClassConstruction,
                span: expression.type_span,
            });
        } else if class_has_unresolved_abstract_methods(parsed, class) {
            diagnostics.push(ConstructorDiagnostic {
                kind: ConstructorDiagnosticKind::AbstractClassIncomplete,
                span: expression.type_span,
            });
        }
        if expression.arguments.len() != class.constructor_parameter_count {
            diagnostics.push(ConstructorDiagnostic {
                kind: ConstructorDiagnosticKind::ArgumentCountMismatch,
                span: expression.span,
            });
        }
    }
    for class in parsed
        .class_declarations
        .iter()
        .filter(|class| !class.interface)
    {
        let Some(superclass_name) = class.superclass.as_deref() else {
            continue;
        };
        let Some(superclass) = parsed
            .class_declarations
            .iter()
            .find(|candidate| candidate.name == superclass_name)
        else {
            continue;
        };
        if class.superclass_arguments.len() != superclass.constructor_parameters.len()
            && let Some(node) = parsed.arena.node(class.declaration)
        {
            diagnostics.push(ConstructorDiagnostic {
                kind: ConstructorDiagnosticKind::SuperclassArgumentCountMismatch,
                span: node.span,
            });
        }
    }
    diagnostics
}

pub(crate) fn class_has_unresolved_abstract_methods(
    parsed: &ParseOutput,
    class: &ClassTypeRecord,
) -> bool {
    let mut hierarchy = Vec::new();
    let mut current = Some(class.name.clone());
    while let Some(name) = current {
        let Some(record) = parsed
            .class_declarations
            .iter()
            .find(|candidate| candidate.name == name)
        else {
            break;
        };
        hierarchy.push(record);
        current = record.superclass.clone();
    }
    parsed.function_declarations.iter().any(|requirement| {
        requirement.is_abstract
            && requirement
                .owner
                .is_some_and(|owner| hierarchy.iter().any(|record| record.declaration == owner))
            && !hierarchy.iter().any(|record| {
                parsed.function_declarations.iter().any(|candidate| {
                    candidate.owner == Some(record.declaration)
                        && candidate.name == requirement.name
                        && !candidate.is_abstract
                        && same_function_parameter_types(parsed, candidate, requirement)
                })
            })
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassLifecycleFacts {
    pub(crate) class: AstNodeId,
    pub(crate) initialization_order: Vec<String>,
    pub(crate) destruction_order: Vec<String>,
}

impl ClassLifecycleFacts {
    pub fn class(&self) -> AstNodeId {
        self.class
    }
    pub fn initialization_order(&self) -> &[String] {
        &self.initialization_order
    }
    pub fn destruction_order(&self) -> &[String] {
        &self.destruction_order
    }
}

pub fn class_lifecycle_facts(parsed: &ParseOutput) -> Vec<ClassLifecycleFacts> {
    parsed
        .class_declarations
        .iter()
        .map(|class| {
            let mut initialization_order = Vec::new();
            append_inherited_field_order(parsed, class, &mut initialization_order);
            /* Primary-constructor fields are initialized before body fields. */
            let mut destruction_order = initialization_order.clone();
            destruction_order.reverse();
            ClassLifecycleFacts {
                class: class.declaration,
                initialization_order,
                destruction_order,
            }
        })
        .collect()
}

pub(crate) fn append_inherited_field_order(
    parsed: &ParseOutput,
    class: &ParsedClassDeclaration,
    output: &mut Vec<String>,
) {
    if let Some(superclass) = class.superclass.as_deref()
        && let Some(parent) = parsed
            .class_declarations
            .iter()
            .find(|candidate| candidate.name == superclass)
    {
        append_inherited_field_order(parsed, parent, output);
    }
    output.extend(
        class
            .constructor_parameters
            .iter()
            .filter(|parameter| parameter.field)
            .map(|parameter| parameter.name.clone()),
    );
    output.extend(class.fields.iter().filter_map(|field| {
        parsed
            .field_declarations
            .iter()
            .find(|candidate| candidate.declaration == *field)
            .map(|field| field.name.clone())
    }));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DispatchDiagnosticKind {
    OverrideWithoutSuperclass,
    OverrideWithoutTarget,
    MissingOverrideMarker,
    OpenAndFinalMethod,
    FinalClassInheritance,
    FinalMethodOverride,
    InterfaceMethodFinal,
    MissingInterfaceMethod,
    AmbiguousInterfaceMethod,
    IncompatibleOverride,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DispatchDiagnostic {
    pub(crate) kind: DispatchDiagnosticKind,
    pub(crate) span: ByteSpan,
}

impl DispatchDiagnostic {
    pub fn kind(self) -> DispatchDiagnosticKind {
        self.kind
    }
    pub fn span(self) -> ByteSpan {
        self.span
    }
}

pub fn check_dispatch(parsed: &ParseOutput) -> Vec<DispatchDiagnostic> {
    let methods_by_owner: HashMap<AstNodeId, Vec<&ParsedFunctionDeclaration>> = parsed
        .function_declarations
        .iter()
        .filter_map(|function| function.owner.map(|owner| (owner, function)))
        .fold(HashMap::new(), |mut index, (owner, function)| {
            index.entry(owner).or_default().push(function);
            index
        });
    let parameter_types_by_function: HashMap<AstNodeId, Vec<_>> = parsed
        .function_declarations
        .iter()
        .map(|function| {
            (
                function.declaration,
                parsed
                    .function_parameters
                    .iter()
                    .filter(|parameter| parameter.function == function.declaration)
                    .map(|parameter| type_annotation_name(parsed, Some(parameter.annotation)))
                    .collect(),
            )
        })
        .collect();
    let same_parameters = |left: &ParsedFunctionDeclaration, right: &ParsedFunctionDeclaration| {
        parameter_types_by_function.get(&left.declaration)
            == parameter_types_by_function.get(&right.declaration)
    };
    let mut diagnostics = Vec::new();
    for function in &parsed.function_declarations {
        if function.is_final
            && function.owner.is_some_and(|owner| {
                parsed
                    .class_declarations
                    .iter()
                    .find(|class| class.declaration == owner)
                    .is_some_and(|class| class.interface)
            })
            && let Some(node) = parsed.arena.node(function.declaration)
        {
            diagnostics.push(DispatchDiagnostic {
                kind: DispatchDiagnosticKind::InterfaceMethodFinal,
                span: node.span,
            });
        }
    }
    for child in &parsed.class_declarations {
        if child.interface {
            continue;
        }
        if let Some(super_name) = child.superclass.as_deref()
            && let Some(parent) = parsed
                .class_declarations
                .iter()
                .find(|candidate| candidate.name == super_name)
            && parent.is_final
            && let Some(node) = parsed.arena.node(child.declaration)
        {
            diagnostics.push(DispatchDiagnostic {
                kind: DispatchDiagnosticKind::FinalClassInheritance,
                span: node.span,
            });
        }
        for method in methods_by_owner
            .get(&child.declaration)
            .into_iter()
            .flatten()
        {
            let targets = inherited_method_targets(parsed, child, &method.name)
                .into_iter()
                .filter(|target| same_parameters(method, target))
                .collect::<Vec<_>>();
            let Some(parent_method) = targets.first().copied() else {
                if method.is_override
                    && let Some(node) = parsed.arena.node(method.declaration)
                {
                    diagnostics.push(DispatchDiagnostic {
                        kind: DispatchDiagnosticKind::OverrideWithoutTarget,
                        span: node.span,
                    });
                }
                continue;
            };
            if !method.is_override
                && let Some(node) = parsed.arena.node(method.declaration)
            {
                diagnostics.push(DispatchDiagnostic {
                    kind: DispatchDiagnosticKind::MissingOverrideMarker,
                    span: node.span,
                });
            }
            if method.is_override
                && parent_method.is_final
                && let Some(node) = parsed.arena.node(method.declaration)
            {
                diagnostics.push(DispatchDiagnostic {
                    kind: DispatchDiagnosticKind::FinalMethodOverride,
                    span: node.span,
                });
            }
            let return_compatible = type_annotation_name(parsed, method.return_annotation)
                == type_annotation_name(parsed, parent_method.return_annotation);
            if method.is_override
                && (!return_compatible || !same_parameters(method, parent_method))
                && let Some(node) = parsed.arena.node(method.declaration)
            {
                diagnostics.push(DispatchDiagnostic {
                    kind: DispatchDiagnosticKind::IncompatibleOverride,
                    span: node.span,
                });
            }
        }
    }
    for class in parsed
        .class_declarations
        .iter()
        .filter(|class| !class.interface)
    {
        let mut interface_methods = Vec::new();
        for interface_name in &class.interfaces {
            for method in parsed.function_declarations.iter().filter(|method| {
                method.owner.is_some_and(|owner| {
                    parsed
                        .class_declarations
                        .iter()
                        .find(|candidate| candidate.declaration == owner)
                        .is_some_and(|candidate| candidate.interface)
                })
            }) {
                if method
                    .owner
                    .and_then(|owner| {
                        parsed
                            .class_declarations
                            .iter()
                            .find(|candidate| candidate.declaration == owner)
                    })
                    .is_some_and(|candidate| candidate.name == *interface_name)
                {
                    interface_methods.push(method);
                }
            }
        }
        for (index, method) in interface_methods.iter().enumerate() {
            if interface_methods[index + 1..].iter().any(|other| {
                other.name == method.name
                    && same_function_parameter_types(parsed, method, other)
                    && type_annotation_name(parsed, method.return_annotation)
                        != type_annotation_name(parsed, other.return_annotation)
            }) && let Some(node) = parsed.arena.node(class.declaration)
            {
                diagnostics.push(DispatchDiagnostic {
                    kind: DispatchDiagnosticKind::AmbiguousInterfaceMethod,
                    span: node.span,
                });
                break;
            }
        }
        for interface_name in &class.interfaces {
            let Some(interface) = parsed
                .class_declarations
                .iter()
                .find(|candidate| candidate.interface && candidate.name == *interface_name)
            else {
                continue;
            };
            for required in parsed
                .function_declarations
                .iter()
                .filter(|method| method.owner == Some(interface.declaration))
            {
                let implemented = parsed.function_declarations.iter().any(|method| {
                    method.owner == Some(class.declaration)
                        && method.name == required.name
                        && same_function_parameter_types(parsed, method, required)
                });
                if !implemented && let Some(node) = parsed.arena.node(class.declaration) {
                    diagnostics.push(DispatchDiagnostic {
                        kind: DispatchDiagnosticKind::MissingInterfaceMethod,
                        span: node.span,
                    });
                }
            }
        }
    }
    diagnostics
}

pub(crate) fn same_function_parameter_types(
    parsed: &ParseOutput,
    left: &ParsedFunctionDeclaration,
    right: &ParsedFunctionDeclaration,
) -> bool {
    let left_types = parsed
        .function_parameters
        .iter()
        .filter(|parameter| parameter.function == left.declaration)
        .map(|parameter| type_annotation_name(parsed, Some(parameter.annotation)))
        .collect::<Vec<_>>();
    let right_types = parsed
        .function_parameters
        .iter()
        .filter(|parameter| parameter.function == right.declaration)
        .map(|parameter| type_annotation_name(parsed, Some(parameter.annotation)))
        .collect::<Vec<_>>();
    left_types == right_types
}

pub(crate) fn inherited_method_targets<'a>(
    parsed: &'a ParseOutput,
    class: &ParsedClassDeclaration,
    method_name: &str,
) -> Vec<&'a ParsedFunctionDeclaration> {
    let mut targets = Vec::new();
    let mut superclass = class.superclass.as_deref();
    while let Some(name) = superclass {
        let Some(parent) = parsed
            .class_declarations
            .iter()
            .find(|candidate| candidate.name == name)
        else {
            break;
        };
        targets.extend(
            parsed
                .function_declarations
                .iter()
                .filter(|function| function.owner == Some(parent.declaration))
                .filter(|function| function.name == method_name),
        );
        superclass = parent.superclass.as_deref();
    }
    for interface_name in &class.interfaces {
        collect_interface_method_targets(parsed, interface_name, method_name, &mut targets);
    }
    targets
}

pub(crate) fn collect_interface_method_targets<'a>(
    parsed: &'a ParseOutput,
    interface_name: &str,
    method_name: &str,
    targets: &mut Vec<&'a ParsedFunctionDeclaration>,
) {
    let Some(interface) = parsed
        .class_declarations
        .iter()
        .find(|candidate| candidate.interface && candidate.name == interface_name)
    else {
        return;
    };
    targets.extend(
        parsed
            .function_declarations
            .iter()
            .filter(|function| function.owner == Some(interface.declaration))
            .filter(|function| function.name == method_name),
    );
    for parent in &interface.interfaces {
        collect_interface_method_targets(parsed, parent, method_name, targets);
    }
}

pub(crate) fn type_annotation_name(
    parsed: &ParseOutput,
    annotation: Option<AstNodeId>,
) -> Option<String> {
    annotation.and_then(|annotation| {
        parsed
            .type_name_references
            .iter()
            .find(|reference| reference.reference == annotation)
            .map(|reference| reference.name.clone())
    })
}

impl ClassTypeReport {
    pub fn classes(&self) -> &[ClassTypeRecord] {
        &self.classes
    }
    pub fn fields(&self) -> &[FieldTypeRecord] {
        &self.fields
    }
    pub fn diagnostics(&self) -> &[TypeCheckDiagnostic] {
        &self.diagnostics
    }
}

pub fn type_class_types(
    parsed: &ParseOutput,
    module: &ModuleName,
    package: &PackageNamespace,
) -> (TypeArena, ClassTypeReport) {
    let mut types = TypeArena::new();
    let report = type_class_types_in(&mut types, parsed, module, package);
    (types, report)
}

pub fn type_class_types_in(
    types: &mut TypeArena,
    parsed: &ParseOutput,
    module: &ModuleName,
    package: &PackageNamespace,
) -> ClassTypeReport {
    let primitives = if types.records().is_empty() {
        PrimitiveTypeIds::insert_into(types)
    } else {
        PrimitiveTypeIds::module_owned(types)
    };
    let mut interner = SymbolInterner::new();
    let mut report = ClassTypeReport::default();
    let mut class_ids = Vec::new();
    for declaration in &parsed.class_declarations {
        let symbol = interner.intern(&declaration.name);
        let type_id = types.nominal(crate::types::NominalTypeIdentity::new(
            module.clone(),
            package.clone(),
            declaration.declaration,
            symbol,
        ));
        class_ids.push((declaration.declaration, type_id));
        report.classes.push(ClassTypeRecord {
            declaration: declaration.declaration,
            name: declaration.name.clone(),
            visibility: declaration.visibility.clone(),
            is_final: declaration.is_final,
            is_abstract: declaration.is_abstract,
            interface: declaration.interface,
            type_id,
            superclass: declaration.superclass.clone(),
            interfaces: declaration.interfaces.clone(),
            constructor_parameter_count: declaration.constructor_parameters.len(),
        });
    }
    for field in &parsed.field_declarations {
        // Primitive annotations first. Other stdlib types, such as Array<T>,
        // resolve through the general annotation resolver (ADR-0126).
        let Some(type_id) = primitive_annotation_type_for_node(
            field.annotation,
            &parsed.type_name_references,
            primitives,
        )
        .or_else(|| {
            resolve_annotation_type(
                field.annotation,
                &parsed.type_name_references,
                &parsed.array_types,
                &primitives,
                &[],
                types,
            )
        }) else {
            continue;
        };
        if report.fields.iter().any(|existing: &FieldTypeRecord| {
            existing.owner == field.owner && existing.name == field.name
        }) {
            report
                .diagnostics
                .push(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::DuplicateField,
                    field.declaration,
                ));
            continue;
        }
        report.fields.push(FieldTypeRecord {
            declaration: field.declaration,
            owner: field.owner,
            type_id,
            name: field.name.clone(),
            visibility: field.visibility.clone(),
            mutable: field.mutable,
        });
    }
    for class in &parsed.class_declarations {
        for parameter in class
            .constructor_parameters
            .iter()
            .filter(|parameter| parameter.field)
        {
            let Some(type_id) = primitive_annotation_type_for_node(
                parameter.annotation,
                &parsed.type_name_references,
                primitives,
            )
            .or_else(|| {
                resolve_annotation_type(
                    parameter.annotation,
                    &parsed.type_name_references,
                    &parsed.array_types,
                    &primitives,
                    &[],
                    types,
                )
            }) else {
                continue;
            };
            if report
                .fields
                .iter()
                .any(|field| field.owner == class.declaration && field.name == parameter.name)
            {
                report
                    .diagnostics
                    .push(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::DuplicateField,
                        parameter.parameter,
                    ));
                continue;
            }
            report.fields.push(FieldTypeRecord {
                declaration: parameter.parameter,
                owner: class.declaration,
                type_id,
                name: parameter.name.clone(),
                visibility: "public".to_owned(),
                mutable: parameter.mutable,
            });
        }
    }
    for class in &parsed.class_declarations {
        let Some(superclass) = class.superclass.as_deref() else {
            continue;
        };
        let Some(parent) = parsed
            .class_declarations
            .iter()
            .find(|candidate| candidate.name == superclass)
        else {
            continue;
        };
        let inherited_names: Vec<_> = parent
            .constructor_parameters
            .iter()
            .filter(|parameter| parameter.field)
            .map(|parameter| parameter.name.as_str())
            .chain(parent.fields.iter().filter_map(|field| {
                parsed
                    .field_declarations
                    .iter()
                    .find(|candidate| candidate.declaration == *field)
                    .map(|field| field.name.as_str())
            }))
            .collect();
        for field in class.fields.iter().filter_map(|field| {
            parsed
                .field_declarations
                .iter()
                .find(|candidate| candidate.declaration == *field)
        }) {
            if inherited_names.iter().any(|name| *name == field.name) {
                report
                    .diagnostics
                    .push(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::FieldHiding,
                        field.declaration,
                    ));
            }
        }
    }
    for class in &parsed.class_declarations {
        if class.is_abstract && class.is_final {
            report
                .diagnostics
                .push(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::AbstractDeclarationInvalid,
                    class.declaration,
                ));
        }
    }
    for function in &parsed.function_declarations {
        if !function.is_abstract {
            continue;
        }
        let invalid = function.owner.is_none()
            || function.body.is_some()
            || function.is_final
            || function.is_static;
        if invalid {
            report
                .diagnostics
                .push(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::AbstractDeclarationInvalid,
                    function.declaration,
                ));
        }
    }
    let _ = class_ids;
    report
}

pub fn apply_class_type_facts(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    report: &mut TypeCheckReport,
) {
    for declaration in &parsed.local_declarations {
        let Some(annotation) = declaration.annotation else {
            continue;
        };
        let Some(reference) = parsed
            .type_name_references
            .iter()
            .find(|reference| reference.reference == annotation)
        else {
            continue;
        };
        let Some(class) = classes.classes.iter().find(|class| {
            parsed
                .class_declarations
                .iter()
                .find(|candidate| candidate.declaration == class.declaration)
                .is_some_and(|candidate| candidate.name == reference.name)
        }) else {
            continue;
        };
        report.replace_declaration_signature(DeclarationSignature::new(
            declaration.declaration,
            class.type_id,
        ));
    }
    for expression in &parsed.new_expressions {
        if let Some(class) = classes.classes.iter().find(|class| {
            parsed
                .class_declarations
                .iter()
                .find(|candidate| candidate.declaration == class.declaration)
                .is_some_and(|candidate| candidate.name == expression.type_name)
        }) {
            report
                .replace_expression_type(ExpressionType::new(expression.expression, class.type_id));
        }
    }

    for member in &parsed.member_expressions {
        let Some(receiver_name) = parsed
            .name_references
            .iter()
            .find(|name| name.reference == member.receiver)
            .map(|name| name.name.as_str())
        else {
            continue;
        };
        let Some(enum_class) = classes.classes.iter().find(|class| {
            class.name == receiver_name
                && parsed.enum_variants.iter().any(|variant| {
                    variant.enum_declaration == class.declaration && variant.name == member.name
                })
        }) else {
            continue;
        };
        report.replace_expression_type(ExpressionType::new(member.expression, enum_class.type_id));
        report.retain_diagnostics(|diagnostic| {
            diagnostic.node() != member.expression
                || diagnostic.rule() != TypeRuleDiagnostic::MemberExpressionDeferred
        });
    }

    for parameter in &parsed.function_parameters {
        let Some(reference) = parsed
            .type_name_references
            .iter()
            .find(|reference| reference.reference == parameter.annotation)
        else {
            continue;
        };
        let Some(class) = classes
            .classes
            .iter()
            .find(|class| class.name == reference.name)
        else {
            continue;
        };
        for name in parsed.name_references.iter().filter(|name| {
            name.name == parameter.name
                && name.name_span.file() == parameter.name_span.file()
                && name.name_span.start() >= parameter.name_span.end()
        }) {
            report.replace_expression_type(ExpressionType::new(name.reference, class.type_id));
        }
    }
    report.retain_diagnostics(|diagnostic| {
        if diagnostic.rule() != AmbiguousTypeRule::AssignmentCompatibility {
            return true;
        }
        let Some(declaration) = parsed
            .local_declarations
            .iter()
            .find(|declaration| declaration.declaration == diagnostic.node())
        else {
            return true;
        };
        let Some(interface_name) = declaration
            .annotation
            .and_then(|annotation| {
                parsed
                    .type_name_references
                    .iter()
                    .find(|reference| reference.reference == annotation)
            })
            .map(|reference| reference.name.as_str())
        else {
            return true;
        };
        let Some(initializer) = declaration.initializer.and_then(|initializer| {
            parsed
                .new_expressions
                .iter()
                .find(|expression| expression.expression == initializer)
        }) else {
            return true;
        };
        let Some(concrete) = parsed
            .class_declarations
            .iter()
            .find(|class| class.name == initializer.type_name)
        else {
            return true;
        };
        !(concrete
            .interfaces
            .iter()
            .any(|interface| interface == interface_name)
            && parsed
                .class_declarations
                .iter()
                .any(|class| class.interface && class.name == interface_name))
    });
    report.retain_diagnostics(|diagnostic| {
        !matches!(diagnostic.rule(), TypeRuleDiagnostic::MissingAnnotationType)
            || !parsed.local_declarations.iter().any(|declaration| {
                diagnostic.node() == declaration.declaration
                    && declaration.annotation.is_some_and(|annotation| {
                        parsed.type_name_references.iter().any(|reference| {
                            reference.reference == annotation
                                && classes.classes.iter().any(|class| {
                                    parsed.class_declarations.iter().any(|candidate| {
                                        candidate.declaration == class.declaration
                                            && candidate.name == reference.name
                                    })
                                })
                        })
                    })
            })
    });
}

pub fn apply_field_access_facts(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    report: &mut TypeCheckReport,
) {
    for member in &parsed.member_expressions {
        if member.name == "length" {
            continue;
        }
        let Some(receiver_type) = report.expression_type(member.receiver) else {
            continue;
        };
        let Some(mut class) = classes
            .classes
            .iter()
            .find(|class| class.type_id == receiver_type)
        else {
            continue;
        };
        if parsed
            .class_declarations
            .iter()
            .find(|candidate| candidate.declaration == class.declaration)
            .is_some_and(|candidate| candidate.interface)
            && let Some(name) = parsed
                .name_references
                .iter()
                .find(|name| name.reference == member.receiver)
            && let Some(binding) = parsed
                .local_binding_names
                .iter()
                .find(|binding| binding.name == name.name)
            && let Some(local) = parsed
                .local_declarations
                .iter()
                .find(|local| local.declaration == binding.binding)
            && let Some(initializer) = local.initializer.and_then(|initializer| {
                parsed
                    .new_expressions
                    .iter()
                    .find(|expression| expression.expression == initializer)
            })
            && let Some(concrete) = classes.classes.iter().find(|candidate| {
                parsed
                    .class_declarations
                    .iter()
                    .find(|class| class.declaration == candidate.declaration)
                    .is_some_and(|class| class.name == initializer.type_name)
            })
        {
            class = concrete;
        }
        let Some(field) = classes.fields.iter().find(|field| {
            field.name == member.name
                && class_or_superclass_owns_field(parsed, class.declaration, field.owner)
        }) else {
            continue;
        };
        report.replace_expression_type(ExpressionType::new(member.expression, field.type_id));
        if parsed
            .assignment_statements
            .iter()
            .any(|assignment| assignment.target == member.expression)
            && !field.mutable
        {
            report
                .diagnostics
                .push(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::ImmutableFieldMutation,
                    member.expression,
                ));
        }
        report.retain_diagnostics(|diagnostic| {
            diagnostic.node() != member.expression
                || diagnostic.rule() != TypeRuleDiagnostic::MemberExpressionDeferred
        });
    }
    for function in parsed
        .function_declarations
        .iter()
        .filter(|function| function.owner.is_some())
    {
        let Some(owner) = function.owner else {
            continue;
        };
        let Some(body) = function
            .body
            .and_then(|body| parsed.arena.node(body))
            .map(|node| node.span)
        else {
            continue;
        };
        for name in parsed.name_references.iter().filter(|name| {
            name.name != "this"
                && name.name != "super"
                && name.name_span.file() == body.file()
                && body.start() <= name.name_span.start()
                && name.name_span.end() <= body.end()
                && !parsed.local_binding_names.iter().any(|binding| {
                    binding.name == name.name
                        && parsed.executable_body_statements.iter().any(|statement| {
                            statement.function == function.declaration
                                && statement.statement == binding.binding
                        })
                })
                && !parsed.function_parameters.iter().any(|parameter| {
                    parameter.function == function.declaration && parameter.name == name.name
                })
        }) {
            let Some(field) = classes.fields.iter().find(|field| {
                field.name == name.name
                    && class_or_superclass_owns_field(parsed, owner, field.owner)
            }) else {
                continue;
            };
            report.replace_expression_type(ExpressionType::new(name.reference, field.type_id));
            report.retain_diagnostics(|diagnostic| {
                diagnostic.node() != name.reference
                    || diagnostic.rule() != TypeRuleDiagnostic::MissingResolvedNameType
            });
        }
    }
}

pub(crate) fn class_or_superclass_owns_field(
    parsed: &ParseOutput,
    class: AstNodeId,
    field_owner: AstNodeId,
) -> bool {
    if class == field_owner {
        return true;
    }
    parsed
        .class_declarations
        .iter()
        .find(|candidate| candidate.declaration == class)
        .and_then(|class| class.superclass.as_deref())
        .and_then(|superclass| {
            parsed
                .class_declarations
                .iter()
                .find(|candidate| candidate.name == superclass)
                .map(|parent| parent.declaration)
        })
        .is_some_and(|parent| class_or_superclass_owns_field(parsed, parent, field_owner))
}

pub fn apply_method_call_facts(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    report: &mut TypeCheckReport,
) {
    for call in &parsed.call_expressions {
        let Some(member) = parsed
            .member_expressions
            .iter()
            .find(|member| member.expression == call.callee)
        else {
            continue;
        };
        let Some(receiver_type) = report.expression_type(member.receiver) else {
            continue;
        };
        let Some(class) = classes
            .classes
            .iter()
            .find(|class| class.type_id == receiver_type)
        else {
            continue;
        };
        let Some(method) = parsed
            .function_declarations
            .iter()
            .find(|method| method.owner == Some(class.declaration) && method.name == member.name)
        else {
            continue;
        };
        let Some(return_annotation) = method.return_annotation else {
            continue;
        };
        let Some(reference) = parsed
            .type_name_references
            .iter()
            .find(|reference| reference.reference == return_annotation)
        else {
            continue;
        };
        let primitives = PrimitiveTypeIds::module_owned(&mut TypeArena::new());
        let Some(return_type) = primitives.type_for_primitive_name(&reference.name) else {
            continue;
        };
        report.replace_expression_type(ExpressionType::new(call.expression, return_type));
        report.retain_diagnostics(|diagnostic| {
            (diagnostic.node() != call.expression && diagnostic.node() != member.expression)
                || !matches!(
                    diagnostic.rule(),
                    TypeRuleDiagnostic::DirectCallDeferred
                        | TypeRuleDiagnostic::MemberExpressionDeferred
                )
        });
    }
}

pub fn apply_enum_method_call_facts(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    report: &mut TypeCheckReport,
    types: &TypeArena,
) {
    let bool_type = types.records().iter().find_map(|record| {
        (record.kind() == &TypeKind::Primitive(PrimitiveType::Bool)).then_some(record.id())
    });
    for call in &parsed.call_expressions {
        let Some(member) = parsed
            .member_expressions
            .iter()
            .find(|member| member.expression == call.callee)
        else {
            continue;
        };
        let Some(receiver_type) = report.expression_type(member.receiver) else {
            continue;
        };
        let declaration = match types.get(receiver_type).map(|record| record.kind()) {
            Some(TypeKind::GenericInstance(identity)) => identity.declaration().declaration(),
            Some(TypeKind::Nominal(identity)) => identity.declaration(),
            _ => continue,
        };
        let Some(class) = classes.classes.iter().find(|class| {
            class.declaration == declaration
                && parsed
                    .enum_variants
                    .iter()
                    .any(|variant| variant.enum_declaration == declaration)
        }) else {
            continue;
        };
        if parsed.function_declarations.iter().any(|function| {
            function.owner == Some(class.declaration)
                && function.name == member.name
                && function.return_annotation.is_some_and(|annotation| {
                    parsed.type_name_references.iter().any(|reference| {
                        reference.reference == annotation && reference.name == "Bool"
                    })
                })
        }) && let Some(bool_type) = bool_type
        {
            report.replace_expression_type(ExpressionType::new(call.expression, bool_type));
            report.retain_diagnostics(|diagnostic| {
                (diagnostic.node() != call.expression && diagnostic.node() != member.expression)
                    || !matches!(
                        diagnostic.rule(),
                        TypeRuleDiagnostic::DirectCallDeferred
                            | TypeRuleDiagnostic::MemberExpressionDeferred
                    )
            });
        }
    }
}

pub fn apply_receiver_name_facts(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    report: &mut TypeCheckReport,
) {
    for function in parsed
        .function_declarations
        .iter()
        .filter(|function| function.owner.is_some())
    {
        let Some(owner) = function.owner else {
            continue;
        };
        let Some(class) = classes
            .classes
            .iter()
            .find(|class| class.declaration == owner)
        else {
            continue;
        };
        let Some(body) = function
            .body
            .and_then(|body| parsed.arena.node(body))
            .map(|node| node.span)
        else {
            continue;
        };
        if function.is_static {
            if function.is_override {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::StaticFunctionOverride,
                    function.declaration,
                ));
            }
            for name in parsed.name_references.iter().filter(|name| {
                matches!(name.name.as_str(), "this" | "super")
                    && name.name_span.file() == body.file()
                    && body.start() <= name.name_span.start()
                    && name.name_span.end() <= body.end()
            }) {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::StaticFunctionInstanceAccess,
                    name.reference,
                ));
            }
            for member in parsed.member_expressions.iter().filter(|member| {
                parsed.name_references.iter().any(|name| {
                    name.reference == member.receiver
                        && matches!(name.name.as_str(), "this" | "super")
                }) && member.span.file() == body.file()
                    && body.start() <= member.span.start()
                    && member.span.end() <= body.end()
            }) {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::StaticFunctionInstanceAccess,
                    member.expression,
                ));
            }
            continue;
        }
        for name in parsed.name_references.iter().filter(|name| {
            matches!(name.name.as_str(), "this" | "super")
                && name.name_span.file() == body.file()
                && body.start() <= name.name_span.start()
                && name.name_span.end() <= body.end()
        }) {
            let type_id = if name.name == "super" {
                class.superclass.as_deref().and_then(|superclass| {
                    parsed
                        .class_declarations
                        .iter()
                        .find(|candidate| candidate.name == superclass)
                        .and_then(|candidate| {
                            classes
                                .classes
                                .iter()
                                .find(|class| class.declaration == candidate.declaration)
                        })
                        .map(|class| class.type_id)
                })
            } else {
                Some(class.type_id)
            };
            if let Some(type_id) = type_id {
                report.replace_expression_type(ExpressionType::new(name.reference, type_id));
            }
        }
    }
}

pub fn apply_receiver_signatures(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    signatures: &mut [FunctionSignature],
) {
    for function in &parsed.function_declarations {
        let Some(owner) = function.owner else {
            continue;
        };
        let Some(class) = classes
            .classes
            .iter()
            .find(|class| class.declaration == owner)
        else {
            continue;
        };
        if function.is_static {
            continue;
        }
        if let Some(signature) = signatures
            .iter_mut()
            .find(|signature| signature.declaration == function.declaration)
        {
            signature.prepend_parameter_type(class.type_id);
        }
    }
}

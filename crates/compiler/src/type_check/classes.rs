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
    /// The class's own type parameters, in declaration order.
    pub(crate) generic_parameters: Vec<TypeId>,
    /// The type arguments written on the superclass, resolved to types.
    pub(crate) superclass_arguments: Vec<TypeId>,
    /// The type arguments written on each entry of `interfaces`, in the same order.
    pub(crate) interface_arguments: Vec<Vec<TypeId>>,
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
    OpenAndFinalMethod,
    FinalClassInheritance,
    FinalMethodOverride,
    InterfaceMethodFinal,
    MissingInterfaceMethod,
    AmbiguousInterfaceMethod,
    IncompatibleOverride,
    IncompatibleInterfaceMethod,
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
                .filter(|target| {
                    // A superclass parameter `T` is compared as the subclass sees it.
                    let mapping = target
                        .owner
                        .map(|owner| inherited_name_mapping(parsed, child, owner))
                        .unwrap_or_default();
                    parameter_texts(parsed, target.declaration, &mapping)
                        == parameter_texts(parsed, method.declaration, &[])
                })
                .collect::<Vec<_>>();
            let Some(parent_method) = targets.first().copied() else {
                continue;
            };
            // A replacement is recognized by name and parameter types alone.
            let parent_is_interface = parsed.class_declarations.iter().any(|candidate| {
                candidate.interface && Some(candidate.declaration) == parent_method.owner
            });
            if parent_method.is_final
                && let Some(node) = parsed.arena.node(method.declaration)
            {
                diagnostics.push(DispatchDiagnostic {
                    kind: DispatchDiagnosticKind::FinalMethodOverride,
                    span: node.span,
                });
            }
            let parent_interface = parsed.class_declarations.iter().find(|candidate| {
                candidate.interface && Some(candidate.declaration) == parent_method.owner
            });
            let parent_return = match parent_interface.and_then(|interface| {
                let index = child
                    .interfaces
                    .iter()
                    .position(|name| *name == interface.name)?;
                let arguments = child
                    .interface_arguments
                    .get(index)
                    .map(|nodes| argument_texts(parsed, nodes))
                    .unwrap_or_default();
                Some((interface, arguments))
            }) {
                Some((interface, arguments)) => {
                    interface_return_name(parsed, interface, &arguments, parent_method)
                }
                None => {
                    let mapping = parent_method
                        .owner
                        .map(|owner| inherited_name_mapping(parsed, child, owner))
                        .unwrap_or_default();
                    parent_method
                        .return_annotation
                        .map(|annotation| type_text_with(parsed, annotation, &mapping))
                }
            };
            let return_compatible =
                type_annotation_name(parsed, method.return_annotation) == parent_return;
            // An interface requirement's return type is checked by the
            // conformance check above, so only superclass replacements are here.
            if !parent_is_interface
                && !return_compatible
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
            let parameters = parameter_texts(parsed, method.declaration, &[]);
            if interface_methods[index + 1..].iter().any(|other| {
                other.name == method.name
                    && other.owner != method.owner
                    && same_function_parameter_types(parsed, method, other)
            }) && !class_declares_method(parsed, class, &method.name, &parameters)
                && !superclass_concrete_method(parsed, class, &method.name, &parameters)
                && let Some(node) = parsed.arena.node(class.declaration)
            {
                diagnostics.push(DispatchDiagnostic {
                    kind: DispatchDiagnosticKind::AmbiguousInterfaceMethod,
                    span: node.span,
                });
                break;
            }
        }
        for (index, interface_name) in class.interfaces.iter().enumerate() {
            let Some(interface) = parsed
                .class_declarations
                .iter()
                .find(|candidate| candidate.interface && candidate.name == *interface_name)
            else {
                continue;
            };
            let arguments = class
                .interface_arguments
                .get(index)
                .map(|nodes| argument_texts(parsed, nodes))
                .unwrap_or_default();
            let interface_mapping =
                interface_parameter_mapping(parsed, interface.declaration, &arguments);
            for required in parsed
                .function_declarations
                .iter()
                .filter(|method| method.owner == Some(interface.declaration))
            {
                let required_return =
                    interface_return_name(parsed, interface, &arguments, required);
                let implementation = parsed.function_declarations.iter().find(|method| {
                    method.owner == Some(class.declaration)
                        && method.name == required.name
                        && parameter_texts(parsed, required.declaration, &interface_mapping)
                            == parameter_texts(parsed, method.declaration, &[])
                });
                let Some(node) = parsed.arena.node(class.declaration) else {
                    continue;
                };
                let parameters = parameter_texts(parsed, required.declaration, &interface_mapping);
                match implementation {
                    None if required.body.is_some()
                        || superclass_concrete_method(
                            parsed,
                            class,
                            &required.name,
                            &parameters,
                        ) => {}
                    None => diagnostics.push(DispatchDiagnostic {
                        kind: DispatchDiagnosticKind::MissingInterfaceMethod,
                        span: node.span,
                    }),
                    Some(method)
                        if type_annotation_name(parsed, method.return_annotation)
                            != required_return =>
                    {
                        diagnostics.push(DispatchDiagnostic {
                            kind: DispatchDiagnosticKind::IncompatibleInterfaceMethod,
                            span: node.span,
                        });
                    }
                    Some(_) => {}
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

/// The return type name of an interface method, with the interface's type
/// parameters replaced by the type arguments of the implementing class
/// (ADR-0083). A `T` return of `Counter<Int>` is `Int`.
pub(crate) fn interface_return_name(
    parsed: &ParseOutput,
    interface: &ParsedClassDeclaration,
    arguments: &[String],
    method: &ParsedFunctionDeclaration,
) -> Option<String> {
    let annotation = method.return_annotation?;
    let mapping = interface_parameter_mapping(parsed, interface.declaration, arguments);
    Some(type_text_with(parsed, annotation, &mapping))
}

/// The canonical spelling of each parameter of `function`, under `mapping`.
pub(crate) fn parameter_texts(
    parsed: &ParseOutput,
    function: AstNodeId,
    mapping: &[(String, String)],
) -> Vec<String> {
    parsed
        .function_parameters
        .iter()
        .filter(|parameter| parameter.function == function)
        .map(|parameter| type_text_with(parsed, parameter.annotation, mapping))
        .collect()
}

/// The canonical spelling of each type argument, in order (ADR-0128).
pub(crate) fn argument_texts(parsed: &ParseOutput, arguments: &[AstNodeId]) -> Vec<String> {
    arguments
        .iter()
        .map(|argument| type_text_with(parsed, *argument, &[]))
        .collect()
}

/// The mapping from the type parameters of `declaration` to the given arguments.
pub(crate) fn interface_parameter_mapping(
    parsed: &ParseOutput,
    declaration: AstNodeId,
    arguments: &[String],
) -> Vec<(String, String)> {
    parsed
        .generic_parameters
        .iter()
        .filter(|parameter| parameter.owner == Some(declaration))
        .map(|parameter| parameter.name.clone())
        .zip(arguments.iter().cloned())
        .collect()
}

/// The canonical spelling of a type annotation, with each name in `mapping`
/// replaced by its spelling. `Box<T>` under `T = Int` is `Box<Int>` (ADR-0128).
pub(crate) fn type_text_with(
    parsed: &ParseOutput,
    annotation: AstNodeId,
    mapping: &[(String, String)],
) -> String {
    let Some(reference) = parsed
        .type_name_references
        .iter()
        .find(|reference| reference.reference == annotation)
    else {
        return String::new();
    };
    if reference.generic_argument_references.is_empty() {
        return mapping
            .iter()
            .find(|(parameter, _)| *parameter == reference.name)
            .map_or_else(|| reference.name.clone(), |(_, text)| text.clone());
    }
    let arguments: Vec<String> = reference
        .generic_argument_references
        .iter()
        .map(|argument| type_text_with(parsed, *argument, mapping))
        .collect();
    format!("{}<{}>", reference.name, arguments.join(","))
}

/// Whether the class `class_name`, or a superclass of it, implements the interface
/// named by `annotation`, with the same spelling including type arguments
/// (ADR-0134). Only an interface type annotation can be satisfied this way.
pub(crate) fn class_implements_annotated_interface(
    parsed: &ParseOutput,
    class_name: &str,
    annotation: Option<AstNodeId>,
) -> bool {
    let Some(annotated) = type_annotation_name(parsed, annotation) else {
        return false;
    };
    let mut current = parsed
        .class_declarations
        .iter()
        .find(|class| class.name == class_name);
    while let Some(class) = current {
        let implemented = class.interfaces.iter().enumerate().any(|(index, name)| {
            let spelling = match class.interface_arguments.get(index) {
                Some(arguments) if !arguments.is_empty() => {
                    format!("{name}<{}>", argument_texts(parsed, arguments).join(","))
                }
                _ => name.clone(),
            };
            spelling == annotated
                && parsed
                    .class_declarations
                    .iter()
                    .any(|candidate| candidate.interface && candidate.name == *name)
        });
        if implemented {
            return true;
        }
        current = class.superclass.as_deref().and_then(|parent| {
            parsed
                .class_declarations
                .iter()
                .find(|candidate| candidate.name == parent)
        });
    }
    false
}

/// The signatures that HIR lowers against (ADR-0134). An implementation of a
/// generic interface method keeps a word at each slot that the interface declares
/// with its type parameter, so that one interface call can invoke any
/// implementation. A `Bool`, `Byte`, or `Float` slot is narrower than a word, so it
/// takes the interface's erased word in the implementation's signature. Type
/// checking keeps the declared signatures.
pub(crate) fn erased_interface_signatures(
    parsed: &ParseOutput,
    types: &TypeArena,
    signatures: &[FunctionSignature],
) -> Vec<FunctionSignature> {
    let is_erased = |ty: TypeId| {
        matches!(
            types.get(ty).map(|record| record.kind()),
            Some(TypeKind::GenericParameter(_))
        )
    };
    let is_narrow = |ty: TypeId| {
        matches!(
            types.get(ty).map(|record| record.kind()),
            Some(TypeKind::Primitive(
                PrimitiveType::Bool | PrimitiveType::Byte | PrimitiveType::Float
            ))
        )
    };
    let mut result = signatures.to_vec();
    for class in parsed
        .class_declarations
        .iter()
        .filter(|class| !class.interface)
    {
        for (index, interface_name) in class.interfaces.iter().enumerate() {
            let Some(interface) = parsed
                .class_declarations
                .iter()
                .find(|candidate| candidate.interface && candidate.name == *interface_name)
            else {
                continue;
            };
            let arguments = class
                .interface_arguments
                .get(index)
                .map(|arguments| argument_texts(parsed, arguments))
                .unwrap_or_default();
            let mapping = interface_parameter_mapping(parsed, interface.declaration, &arguments);
            for required in parsed
                .function_declarations
                .iter()
                .filter(|function| function.owner == Some(interface.declaration))
            {
                let Some(required_signature) = signatures
                    .iter()
                    .find(|signature| signature.declaration == required.declaration)
                else {
                    continue;
                };
                let required_parameters = parameter_texts(parsed, required.declaration, &mapping);
                for method in parsed.function_declarations.iter().filter(|function| {
                    function.owner == Some(class.declaration) && function.name == required.name
                }) {
                    if parameter_texts(parsed, method.declaration, &[]) != required_parameters {
                        continue;
                    }
                    let (Some(own), Some(erased)) = (
                        signatures
                            .iter()
                            .find(|signature| signature.declaration == method.declaration),
                        result
                            .iter_mut()
                            .find(|signature| signature.declaration == method.declaration),
                    ) else {
                        continue;
                    };
                    for ((slot, declared), required_slot) in own
                        .parameter_types
                        .iter()
                        .enumerate()
                        .zip(&required_signature.parameter_types)
                    {
                        if is_erased(*required_slot) && is_narrow(*declared) {
                            erased.parameter_types[slot] = *required_slot;
                        }
                    }
                    if is_erased(required_signature.return_type) && is_narrow(own.return_type) {
                        erased.return_type = required_signature.return_type;
                    }
                }
            }
        }
    }
    result
}

pub(crate) fn type_annotation_name(
    parsed: &ParseOutput,
    annotation: Option<AstNodeId>,
) -> Option<String> {
    annotation.and_then(|annotation| {
        parsed
            .type_name_references
            .iter()
            .any(|reference| reference.reference == annotation)
            .then(|| type_text_with(parsed, annotation, &[]))
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

/// The mapping from the type parameter names of `owner`, an ancestor of `child`
/// or an interface it implements, to the names `child` gives them (ADR-0083).
pub(crate) fn inherited_name_mapping(
    parsed: &ParseOutput,
    child: &ParsedClassDeclaration,
    owner: AstNodeId,
) -> Vec<(String, String)> {
    let parameter_names = |declaration: AstNodeId| -> Vec<String> {
        parsed
            .generic_parameters
            .iter()
            .filter(|parameter| parameter.owner == Some(declaration))
            .map(|parameter| parameter.name.clone())
            .collect()
    };
    if let Some(interface) = parsed
        .class_declarations
        .iter()
        .find(|candidate| candidate.declaration == owner && candidate.interface)
    {
        let Some(index) = child
            .interfaces
            .iter()
            .position(|name| *name == interface.name)
        else {
            return Vec::new();
        };
        let arguments = child
            .interface_arguments
            .get(index)
            .map(|nodes| argument_texts(parsed, nodes))
            .unwrap_or_default();
        return parameter_names(owner).into_iter().zip(arguments).collect();
    }
    let mut mapping: Vec<(String, String)> = Vec::new();
    let mut current = child;
    loop {
        let Some(parent) = current.superclass.as_deref().and_then(|name| {
            parsed
                .class_declarations
                .iter()
                .find(|candidate| candidate.name == name)
        }) else {
            return Vec::new();
        };
        let arguments: Vec<String> = current
            .superclass_type_arguments
            .iter()
            .map(|argument| type_text_with(parsed, *argument, &mapping))
            .collect();
        mapping = parameter_names(parent.declaration)
            .into_iter()
            .zip(arguments)
            .collect();
        if parent.declaration == owner {
            return mapping;
        }
        current = parent;
    }
}

/// Whether `class` itself declares an instance method `name` with these parameter
/// spellings.
pub(crate) fn class_declares_method(
    parsed: &ParseOutput,
    class: &ParsedClassDeclaration,
    name: &str,
    parameters: &[String],
) -> bool {
    parsed.function_declarations.iter().any(|method| {
        method.owner == Some(class.declaration)
            && method.name == name
            && parameter_texts(parsed, method.declaration, &[]) == parameters
    })
}

/// Whether a superclass of `class`, at any depth, provides a concrete (bodied)
/// method `name` with these parameter spellings (ADR-0133: a class's superclass
/// method wins over an interface default).
pub(crate) fn superclass_concrete_method(
    parsed: &ParseOutput,
    class: &ParsedClassDeclaration,
    name: &str,
    parameters: &[String],
) -> bool {
    let mut superclass = class.superclass.as_deref();
    while let Some(parent_name) = superclass {
        let Some(parent) = parsed
            .class_declarations
            .iter()
            .find(|candidate| candidate.name == parent_name)
        else {
            return false;
        };
        if parsed.function_declarations.iter().any(|method| {
            method.owner == Some(parent.declaration)
                && method.name == name
                && method.body.is_some()
                && parameter_texts(parsed, method.declaration, &[]) == parameters
        }) {
            return true;
        }
        superclass = parent.superclass.as_deref();
    }
    false
}

/// The nearest ancestor of `class` that declares the instance method `name`, or,
/// failing that, the interface whose default body supplies it (ADR-0133). A
/// superclass declaration wins over an interface default. `None` when `class`
/// declares the method itself, or no ancestor or interface supplies it.
pub(crate) fn inherited_method_owner(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    class: AstNodeId,
    name: &str,
) -> Option<AstNodeId> {
    let declares = |owner: AstNodeId| {
        parsed
            .function_declarations
            .iter()
            .any(|method| method.owner == Some(owner) && method.name == name && !method.is_static)
    };
    if declares(class) {
        return None;
    }
    let receiver = classes
        .classes()
        .iter()
        .find(|candidate| candidate.declaration == class)?;
    let mut lineage = vec![receiver];
    let mut current = receiver;
    while let Some(parent) = classes
        .classes()
        .iter()
        .find(|candidate| Some(candidate.name.as_str()) == current.superclass.as_deref())
    {
        if declares(parent.declaration) {
            return Some(parent.declaration);
        }
        lineage.push(parent);
        current = parent;
    }
    for record in lineage {
        for interface_name in &record.interfaces {
            let Some(interface) = classes
                .classes()
                .iter()
                .find(|candidate| candidate.interface && candidate.name == *interface_name)
            else {
                continue;
            };
            if parsed.function_declarations.iter().any(|method| {
                method.owner == Some(interface.declaration)
                    && method.name == name
                    && method.body.is_some()
                    && !method.is_static
            }) {
                return Some(interface.declaration);
            }
        }
    }
    None
}

/// The mapping from the type parameters of `interface` to the types that the class
/// `receiver` implements it with, through its own interfaces or those of an
/// ancestor (ADR-0134). Empty when the interface is not generic.
fn interface_parameter_types(
    classes: &ClassTypeReport,
    receiver: AstNodeId,
    interface: &ClassTypeRecord,
) -> Option<Vec<(TypeId, TypeId)>> {
    let mut current = classes
        .classes()
        .iter()
        .find(|class| class.declaration == receiver)?;
    // Maps the parameters of `current` to the types the receiver gives them. The
    // receiver's own parameters map to themselves, so the mapping starts empty.
    let mut mapping: Vec<(TypeId, TypeId)> = Vec::new();
    let substitute = |mapping: &[(TypeId, TypeId)], ty: TypeId| {
        mapping
            .iter()
            .find(|(parameter, _)| *parameter == ty)
            .map_or(ty, |(_, argument)| *argument)
    };
    loop {
        if let Some(index) = current
            .interfaces
            .iter()
            .position(|name| *name == interface.name)
        {
            let arguments: Vec<TypeId> = current
                .interface_arguments
                .get(index)
                .map(|arguments| {
                    arguments
                        .iter()
                        .map(|argument| substitute(&mapping, *argument))
                        .collect()
                })
                .unwrap_or_default();
            return Some(
                interface
                    .generic_parameters
                    .iter()
                    .copied()
                    .zip(arguments)
                    .collect(),
            );
        }
        let parent = classes
            .classes()
            .iter()
            .find(|class| Some(class.name.as_str()) == current.superclass.as_deref())?;
        let arguments: Vec<TypeId> = current
            .superclass_arguments
            .iter()
            .map(|argument| substitute(&mapping, *argument))
            .collect();
        mapping = parent
            .generic_parameters
            .iter()
            .copied()
            .zip(arguments)
            .collect();
        current = parent;
    }
}

/// The mapping from the type parameters of `owner`, an ancestor of the class
/// `receiver`, to the types that `receiver` inherits them as (ADR-0083). The
/// mapping is empty when `owner` is `receiver` itself, and `None` when `owner`
/// is not an ancestor.
pub(crate) fn inherited_parameter_types(
    classes: &ClassTypeReport,
    receiver: AstNodeId,
    owner: AstNodeId,
) -> Option<Vec<(TypeId, TypeId)>> {
    if let Some(interface) = classes
        .classes()
        .iter()
        .find(|candidate| candidate.declaration == owner && candidate.interface)
    {
        return interface_parameter_types(classes, receiver, interface);
    }
    let mut current = classes
        .classes()
        .iter()
        .find(|class| class.declaration == receiver)?;
    if current.declaration == owner {
        return Some(Vec::new());
    }
    let mut mapping: Vec<(TypeId, TypeId)> = Vec::new();
    loop {
        let parent = classes
            .classes()
            .iter()
            .find(|class| Some(class.name.as_str()) == current.superclass.as_deref())?;
        let arguments: Vec<TypeId> = current
            .superclass_arguments
            .iter()
            .map(|argument| {
                mapping
                    .iter()
                    .find(|(parameter, _)| parameter == argument)
                    .map_or(*argument, |(_, ty)| *ty)
            })
            .collect();
        mapping = parent
            .generic_parameters
            .iter()
            .copied()
            .zip(arguments)
            .collect();
        if parent.declaration == owner {
            return Some(mapping);
        }
        current = parent;
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
            generic_parameters: Vec::new(),
            superclass_arguments: Vec::new(),
            interface_arguments: Vec::new(),
        });
    }
    // Generic parameter types are built over the whole list, in declaration
    // order, as the signature builder builds them, so the same parameter gets
    // the same type identity everywhere (ADR-0083).
    let mut parameter_symbols = SymbolInterner::new();
    let parameter_records =
        build_generic_parameter_types(&parsed.generic_parameters, &mut parameter_symbols, types);
    let class_generic_types = |owner: AstNodeId| -> Vec<(&str, TypeId)> {
        parsed
            .generic_parameters
            .iter()
            .filter(|parameter| parameter.owner == Some(owner))
            .filter_map(|parameter| {
                parameter_records
                    .iter()
                    .find(|record| record.parameter() == parameter.parameter)
                    .map(|record| (parameter.name.as_str(), record.ty()))
            })
            .collect()
    };
    let class_snapshot = report.classes.clone();
    for record in &mut report.classes {
        record.generic_parameters = class_generic_types(record.declaration)
            .into_iter()
            .map(|(_, ty)| ty)
            .collect();
        let Some(declaration) = parsed
            .class_declarations
            .iter()
            .find(|candidate| candidate.declaration == record.declaration)
        else {
            continue;
        };
        let generic_types = class_generic_types(record.declaration);
        let arguments: Option<Vec<TypeId>> = declaration
            .superclass_type_arguments
            .iter()
            .map(|argument| {
                resolve_type_reference(
                    &parsed.type_name_references,
                    &parsed.array_types,
                    *argument,
                    &primitives,
                    &class_snapshot,
                    &generic_types,
                    types,
                )
            })
            .collect();
        record.superclass_arguments = arguments.unwrap_or_default();
        record.interface_arguments = declaration
            .interface_arguments
            .iter()
            .map(|arguments| {
                arguments
                    .iter()
                    .map(|argument| {
                        resolve_type_reference(
                            &parsed.type_name_references,
                            &parsed.array_types,
                            *argument,
                            &primitives,
                            &class_snapshot,
                            &generic_types,
                            types,
                        )
                    })
                    .collect::<Option<Vec<TypeId>>>()
                    .unwrap_or_default()
            })
            .collect();
    }
    for field in &parsed.field_declarations {
        // Primitive annotations first. Other annotations, such as T[],
        // resolve through the general annotation resolver with the class's
        // type parameters in scope (ADR-0126).
        let Some(type_id) = primitive_annotation_type_for_node(
            field.annotation,
            &parsed.type_name_references,
            primitives,
        )
        .or_else(|| {
            resolve_annotation_type_with_generics(
                field.annotation,
                &parsed.type_name_references,
                &parsed.array_types,
                &primitives,
                &[],
                types,
                &class_generic_types(field.owner),
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
                resolve_annotation_type_with_generics(
                    parameter.annotation,
                    &parsed.type_name_references,
                    &parsed.array_types,
                    &primitives,
                    &[],
                    types,
                    &class_generic_types(class.declaration),
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
                visibility: parameter.visibility.clone(),
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
        // A generic instance annotation such as `Box<Int>` is typed by the
        // generic-aware annotation pass, not by the bare class type.
        if !reference.generic_arguments.is_empty() {
            continue;
        }
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
        if !expression.type_argument_references.is_empty() {
            continue;
        }
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
        let Some(declaration) = parsed.local_declarations.iter().find(|declaration| {
            declaration.declaration == diagnostic.node()
                || declaration.initializer == Some(diagnostic.node())
        }) else {
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
        !class_implements_annotated_interface(
            parsed,
            &initializer.type_name,
            declaration.annotation,
        )
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

/// Whether a use of a field at `site` is allowed by its visibility. A private field
/// is used only inside its declaring class, not by a subclass or outside (ADR-0130).
pub(crate) fn private_field_accessible(
    parsed: &ParseOutput,
    field: &FieldTypeRecord,
    site: AstNodeId,
) -> bool {
    if field.visibility != "private" {
        return true;
    }
    let Some(site_span) = parsed.arena.node(site).map(|node| node.span) else {
        return false;
    };
    parsed
        .class_declarations
        .iter()
        .find(|class| class.declaration == field.owner)
        .is_some_and(|class| {
            class.span.file() == site_span.file()
                && class.span.start() <= site_span.start()
                && site_span.end() <= class.span.end()
        })
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
        if !private_field_accessible(parsed, field, member.expression) {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::PrivateFieldAccess,
                member.expression,
            ));
        }
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

/// The generic instance of the nominal class type `nominal` with `arguments`, or
/// `nominal` itself when there are no arguments.
fn instance_or_nominal(types: &mut TypeArena, nominal: TypeId, arguments: &[TypeId]) -> TypeId {
    if arguments.is_empty() {
        return nominal;
    }
    let Some(TypeKind::Nominal(identity)) = types.get(nominal).map(|record| record.kind()) else {
        return nominal;
    };
    let identity = identity.clone();
    types.generic_instance(GenericTypeIdentity::new(identity, arguments.to_vec()))
}

pub fn apply_receiver_name_facts(
    parsed: &ParseOutput,
    classes: &ClassTypeReport,
    types: &mut TypeArena,
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
                    let parent = parsed
                        .class_declarations
                        .iter()
                        .find(|candidate| candidate.name == superclass)
                        .and_then(|candidate| {
                            classes
                                .classes
                                .iter()
                                .find(|class| class.declaration == candidate.declaration)
                        })?;
                    // A superclass with type arguments is the instance it names, so
                    // its members are typed with those arguments (ADR-0083).
                    Some(instance_or_nominal(
                        types,
                        parent.type_id,
                        &class.superclass_arguments,
                    ))
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

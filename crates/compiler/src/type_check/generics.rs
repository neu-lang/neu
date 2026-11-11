//! Function signatures, generic declarations, and capability bounds.

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionSignature {
    pub(crate) declaration: AstNodeId,
    pub(crate) parameter_types: Vec<TypeId>,
    pub(crate) return_type: TypeId,
    pub(crate) is_suspend: bool,
    pub(crate) generic_parameters: Vec<TypeId>,
    pub(crate) generic_bounds: Vec<GenericCapabilityBound>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenericCapabilityBound {
    pub(crate) parameter_node: AstNodeId,
    pub(crate) parameter: TypeId,
    pub(crate) bound: AstNodeId,
    pub(crate) name: String,
}

impl GenericCapabilityBound {
    pub fn parameter_node(&self) -> AstNodeId {
        self.parameter_node
    }

    pub fn parameter(&self) -> TypeId {
        self.parameter
    }

    pub fn bound(&self) -> AstNodeId {
        self.bound
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl FunctionSignature {
    pub fn new(declaration: AstNodeId, parameter_types: Vec<TypeId>, return_type: TypeId) -> Self {
        Self {
            declaration,
            parameter_types,
            return_type,
            is_suspend: false,
            generic_parameters: Vec::new(),
            generic_bounds: Vec::new(),
        }
    }

    pub fn declaration(&self) -> AstNodeId {
        self.declaration
    }

    pub fn parameter_types(&self) -> &[TypeId] {
        &self.parameter_types
    }

    pub fn return_type(&self) -> TypeId {
        self.return_type
    }

    pub fn is_suspend(&self) -> bool {
        self.is_suspend
    }

    pub fn generic_parameters(&self) -> &[TypeId] {
        &self.generic_parameters
    }

    pub fn generic_bounds(&self) -> &[GenericCapabilityBound] {
        &self.generic_bounds
    }

    pub fn with_generic_parameters(mut self, parameters: Vec<TypeId>) -> Self {
        self.generic_parameters = parameters;
        self
    }

    pub fn specialize(&self, substitution: &GenericSubstitution, arena: &mut TypeArena) -> Self {
        Self {
            declaration: self.declaration,
            parameter_types: self
                .parameter_types
                .iter()
                .map(|ty| substitution.apply(*ty, arena))
                .collect(),
            return_type: substitution.apply(self.return_type, arena),
            is_suspend: self.is_suspend,
            generic_parameters: Vec::new(),
            generic_bounds: Vec::new(),
        }
    }

    pub fn with_suspend(mut self, is_suspend: bool) -> Self {
        self.is_suspend = is_suspend;
        self
    }

    pub fn prepend_parameter_type(&mut self, parameter_type: TypeId) {
        self.parameter_types.insert(0, parameter_type);
    }
}

pub fn specialize_function_signature_for_call(
    signature: &FunctionSignature,
    argument_types: &[TypeId],
    arena: &mut TypeArena,
) -> Option<FunctionSignature> {
    if signature.generic_parameters().len() != argument_types.len() {
        return None;
    }
    let mut substitution = GenericSubstitution::new();
    for (parameter, argument) in signature
        .generic_parameters()
        .iter()
        .copied()
        .zip(argument_types.iter().copied())
    {
        substitution.insert(parameter, argument);
    }
    Some(signature.specialize(&substitution, arena))
}

pub fn validate_function_signature_bounds(
    signature: &FunctionSignature,
    argument_types: &[TypeId],
    types: &TypeArena,
) -> Vec<GenericConstraintDiagnostic> {
    if signature.generic_parameters().len() != argument_types.len() {
        return Vec::new();
    }
    let mut substitution = GenericSubstitution::new();
    for (parameter, argument) in signature
        .generic_parameters()
        .iter()
        .copied()
        .zip(argument_types.iter().copied())
    {
        substitution.insert(parameter, argument);
    }
    signature
        .generic_bounds()
        .iter()
        .filter_map(|bound| {
            let capability = match bound.name() {
                "Copy" => ThreadCapability::Copy,
                "Send" => ThreadCapability::Send,
                "Share" => ThreadCapability::Share,
                _ => {
                    return Some(GenericConstraintDiagnostic {
                        parameter: bound.parameter_node(),
                        bound: bound.bound(),
                        kind: GenericConstraintFailureKind::UnsupportedCapability,
                    });
                }
            };
            let argument = substitution.get(bound.parameter())?;
            (!satisfies_thread_capability(types, argument, capability)).then_some(
                GenericConstraintDiagnostic {
                    parameter: bound.parameter_node(),
                    bound: bound.bound(),
                    kind: GenericConstraintFailureKind::UnsatisfiedCapability,
                },
            )
        })
        .collect()
}

pub fn specialize_function_signature_checked(
    signature: &FunctionSignature,
    argument_types: &[TypeId],
    arena: &mut TypeArena,
) -> Result<FunctionSignature, Vec<GenericConstraintDiagnostic>> {
    let diagnostics = validate_function_signature_bounds(signature, argument_types, arena);
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    specialize_function_signature_for_call(signature, argument_types, arena).ok_or_else(|| {
        vec![GenericConstraintDiagnostic {
            parameter: signature.declaration,
            bound: signature.declaration,
            kind: GenericConstraintFailureKind::ArgumentCountMismatch,
        }]
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GenericParameterTypeRecord {
    pub(crate) parameter: AstNodeId,
    pub(crate) ty: TypeId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenericDeclarationRecord {
    pub(crate) owner: AstNodeId,
    pub(crate) parameters: Vec<GenericParameterTypeRecord>,
}

impl GenericDeclarationRecord {
    pub fn new(owner: AstNodeId, parameters: Vec<GenericParameterTypeRecord>) -> Self {
        Self { owner, parameters }
    }

    pub fn owner(&self) -> AstNodeId {
        self.owner
    }

    pub fn parameters(&self) -> &[GenericParameterTypeRecord] {
        &self.parameters
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapabilityBoundRecord {
    pub(crate) parameter: AstNodeId,
    pub(crate) ty: TypeId,
    pub(crate) bound: AstNodeId,
    pub(crate) symbol: SymbolId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenericConstraintFailureKind {
    UnsatisfiedCapability,
    UnsupportedCapability,
    ArgumentCountMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenericConstraintDiagnostic {
    pub(crate) parameter: AstNodeId,
    pub(crate) bound: AstNodeId,
    pub(crate) kind: GenericConstraintFailureKind,
}

impl GenericConstraintDiagnostic {
    pub fn parameter(&self) -> AstNodeId {
        self.parameter
    }

    pub fn bound(&self) -> AstNodeId {
        self.bound
    }

    pub fn kind(&self) -> GenericConstraintFailureKind {
        self.kind
    }
}

impl CapabilityBoundRecord {
    pub fn new(parameter: AstNodeId, ty: TypeId, bound: AstNodeId, symbol: SymbolId) -> Self {
        Self {
            parameter,
            ty,
            bound,
            symbol,
        }
    }

    pub fn parameter(self) -> AstNodeId {
        self.parameter
    }

    pub fn ty(self) -> TypeId {
        self.ty
    }

    pub fn bound(self) -> AstNodeId {
        self.bound
    }

    pub fn symbol(self) -> SymbolId {
        self.symbol
    }
}

impl GenericParameterTypeRecord {
    pub fn new(parameter: AstNodeId, ty: TypeId) -> Self {
        Self { parameter, ty }
    }

    pub fn parameter(self) -> AstNodeId {
        self.parameter
    }

    pub fn ty(self) -> TypeId {
        self.ty
    }
}

pub fn build_generic_parameter_types(
    parameters: &[ParsedGenericParameter],
    symbols: &mut SymbolInterner,
    type_arena: &mut TypeArena,
) -> Vec<GenericParameterTypeRecord> {
    parameters
        .iter()
        .map(|parameter| {
            let symbol = symbols.intern(&parameter.name);
            let ty = type_arena.insert(TypeRecord::generic_parameter(GenericParameterType::new(
                parameter.parameter,
                symbol,
            )));
            GenericParameterTypeRecord::new(parameter.parameter, ty)
        })
        .collect()
}

pub fn build_capability_bound_records(
    parameters: &[ParsedGenericParameter],
    parameter_types: &[GenericParameterTypeRecord],
    symbols: &mut SymbolInterner,
) -> Vec<CapabilityBoundRecord> {
    let mut records = Vec::new();

    for parameter in parameters {
        let Some(parameter_type) = parameter_types
            .iter()
            .find(|record| record.parameter() == parameter.parameter)
        else {
            continue;
        };

        for bound in &parameter.capability_bounds {
            records.push(CapabilityBoundRecord::new(
                parameter.parameter,
                parameter_type.ty(),
                bound.bound,
                symbols.intern(&bound.name),
            ));
        }
    }

    records
}

pub fn validate_capability_bounds(
    bounds: &[CapabilityBoundRecord],
    substitution: &GenericSubstitution,
    types: &TypeArena,
    symbols: &SymbolInterner,
) -> Vec<GenericConstraintDiagnostic> {
    let mut diagnostics = Vec::new();
    for bound in bounds {
        let capability = match symbols.resolve(bound.symbol()) {
            Some("Copy") => ThreadCapability::Copy,
            Some("Send") => ThreadCapability::Send,
            Some("Share") => ThreadCapability::Share,
            Some(_) | None => {
                diagnostics.push(GenericConstraintDiagnostic {
                    parameter: bound.parameter(),
                    bound: bound.bound(),
                    kind: GenericConstraintFailureKind::UnsupportedCapability,
                });
                continue;
            }
        };
        let Some(argument) = substitution.get(bound.ty()) else {
            continue;
        };
        if !satisfies_thread_capability(types, argument, capability) {
            diagnostics.push(GenericConstraintDiagnostic {
                parameter: bound.parameter(),
                bound: bound.bound(),
                kind: GenericConstraintFailureKind::UnsatisfiedCapability,
            });
        }
    }
    diagnostics
}

pub fn build_generic_declaration_records(
    parameters: &[ParsedGenericParameter],
    symbols: &mut SymbolInterner,
    type_arena: &mut TypeArena,
) -> Vec<GenericDeclarationRecord> {
    let parameter_types = build_generic_parameter_types(parameters, symbols, type_arena);
    let mut records = Vec::new();
    for parameter in parameters {
        let Some(owner) = parameter.owner else {
            continue;
        };
        if records
            .iter()
            .any(|record: &GenericDeclarationRecord| record.owner == owner)
        {
            continue;
        }
        let owned = parameters
            .iter()
            .filter(|candidate| candidate.owner == Some(owner))
            .filter_map(|candidate| {
                parameter_types
                    .iter()
                    .find(|record| record.parameter() == candidate.parameter)
                    .copied()
            })
            .collect();
        records.push(GenericDeclarationRecord::new(owner, owned));
    }
    records
}

pub fn type_function_signatures(
    functions: &[ParsedFunctionDeclaration],
    parameters: &[ParsedFunctionParameter],
    type_name_references: &[ParsedTypeNameReference],
) -> (TypeArena, Vec<FunctionSignature>) {
    let mut arena = TypeArena::new();
    let signatures =
        type_function_signatures_in(&mut arena, functions, parameters, type_name_references);
    (arena, signatures)
}

pub fn type_function_signatures_in(
    arena: &mut TypeArena,
    functions: &[ParsedFunctionDeclaration],
    parameters: &[ParsedFunctionParameter],
    type_name_references: &[ParsedTypeNameReference],
) -> Vec<FunctionSignature> {
    type_function_signatures_with_arrays(arena, functions, parameters, type_name_references, &[])
}

pub fn type_function_signatures_with_arrays(
    arena: &mut TypeArena,
    functions: &[ParsedFunctionDeclaration],
    parameters: &[ParsedFunctionParameter],
    type_name_references: &[ParsedTypeNameReference],
    array_types: &[ParsedArrayType],
) -> Vec<FunctionSignature> {
    type_function_signatures_in_with_classes(
        arena,
        functions,
        parameters,
        type_name_references,
        array_types,
        &[],
    )
}

pub fn type_function_signatures_in_with_classes(
    arena: &mut TypeArena,
    functions: &[ParsedFunctionDeclaration],
    parameters: &[ParsedFunctionParameter],
    type_name_references: &[ParsedTypeNameReference],
    array_types: &[ParsedArrayType],
    classes: &[ClassTypeRecord],
) -> Vec<FunctionSignature> {
    type_function_signatures_in_with_classes_and_generics(
        arena,
        functions,
        parameters,
        type_name_references,
        array_types,
        classes,
        &[],
    )
}

pub fn type_function_signatures_in_with_generics(
    arena: &mut TypeArena,
    functions: &[ParsedFunctionDeclaration],
    parameters: &[ParsedFunctionParameter],
    type_name_references: &[ParsedTypeNameReference],
    array_types: &[ParsedArrayType],
    classes: &[ClassTypeRecord],
    generic_parameters: &[ParsedGenericParameter],
) -> Vec<FunctionSignature> {
    type_function_signatures_in_with_classes_and_generics(
        arena,
        functions,
        parameters,
        type_name_references,
        array_types,
        classes,
        generic_parameters,
    )
}

pub(crate) fn type_function_signatures_in_with_classes_and_generics(
    arena: &mut TypeArena,
    functions: &[ParsedFunctionDeclaration],
    parameters: &[ParsedFunctionParameter],
    type_name_references: &[ParsedTypeNameReference],
    array_types: &[ParsedArrayType],
    classes: &[ClassTypeRecord],
    generic_parameters: &[ParsedGenericParameter],
) -> Vec<FunctionSignature> {
    let primitives = PrimitiveTypeIds::module_owned(arena);
    let mut symbols = SymbolInterner::new();
    let generic_parameter_types =
        build_generic_parameter_types(generic_parameters, &mut symbols, arena);
    let mut signatures = Vec::new();

    for function in functions {
        let generic_types: Vec<_> = generic_parameters
            .iter()
            .filter(|parameter| {
                parameter.owner == Some(function.declaration) || function.owner == parameter.owner
            })
            .filter_map(|parameter| {
                generic_parameter_types
                    .iter()
                    .find(|record| record.parameter() == parameter.parameter)
                    .map(|record| (parameter.name.as_str(), record.ty()))
            })
            .collect();
        let return_type = function
            .return_annotation
            .map(|return_annotation| {
                resolve_annotation_type_with_generics(
                    return_annotation,
                    type_name_references,
                    array_types,
                    &primitives,
                    classes,
                    arena,
                    &generic_types,
                )
            })
            .unwrap_or(Some(primitives.void_id));
        let Some(return_type) = return_type else {
            continue;
        };
        let function_parameters: Vec<_> = parameters
            .iter()
            .filter(|parameter| parameter.function == function.declaration)
            .collect();
        let Some(parameter_types) = function_parameters
            .iter()
            .map(|parameter| {
                resolve_annotation_type_with_generics(
                    parameter.annotation,
                    type_name_references,
                    array_types,
                    &primitives,
                    classes,
                    arena,
                    &generic_types,
                )
            })
            .collect::<Option<Vec<_>>>()
        else {
            continue;
        };
        signatures.push(FunctionSignature {
            declaration: function.declaration,
            parameter_types,
            return_type,
            is_suspend: function.is_suspend,
            generic_parameters: generic_types.iter().map(|(_, ty)| *ty).collect(),
            generic_bounds: generic_parameters
                .iter()
                .filter(|parameter| {
                    parameter.owner == Some(function.declaration)
                        || function.owner == parameter.owner
                })
                .flat_map(|parameter| {
                    let Some(ty) = generic_parameter_types
                        .iter()
                        .find(|record| record.parameter() == parameter.parameter)
                        .map(|record| record.ty())
                    else {
                        return Vec::new();
                    };
                    parameter
                        .capability_bounds
                        .iter()
                        .map(|bound| GenericCapabilityBound {
                            parameter_node: parameter.parameter,
                            parameter: ty,
                            bound: bound.bound,
                            name: bound.name.clone(),
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
        });
    }

    signatures
}

pub(crate) fn resolve_annotation_type_with_generics(
    annotation: AstNodeId,
    type_name_references: &[ParsedTypeNameReference],
    array_types: &[ParsedArrayType],
    primitives: &PrimitiveTypeIds,
    classes: &[ClassTypeRecord],
    arena: &mut TypeArena,
    generic_types: &[(&str, TypeId)],
) -> Option<TypeId> {
    if let Some(reference) = type_name_references
        .iter()
        .find(|reference| reference.reference == annotation)
        && let Some((_, ty)) = generic_types
            .iter()
            .find(|(name, _)| *name == reference.name)
    {
        return Some(*ty);
    }
    if let Some(reference) = type_name_references
        .iter()
        .find(|reference| reference.reference == annotation)
        && !reference.generic_argument_names.is_empty()
    {
        if reference.name == "Array" && reference.generic_argument_names.len() == 1 {
            let name = &reference.generic_argument_names[0];
            let element = generic_types
                .iter()
                .find(|(generic, _)| *generic == name)
                .map(|(_, ty)| *ty)
                .or_else(|| primitives.type_for_primitive_name(name))?;
            return Some(arena.dynamic_array(element));
        }
        let class = classes.iter().find(|class| class.name == reference.name)?;
        let arguments = reference
            .generic_argument_names
            .iter()
            .map(|name| {
                generic_types
                    .iter()
                    .find(|(generic, _)| *generic == name)
                    .map(|(_, ty)| *ty)
                    .or_else(|| primitives.type_for_primitive_name(name))
                    .or_else(|| {
                        classes
                            .iter()
                            .find(|class| class.name == *name)
                            .map(|class| class.type_id)
                    })
            })
            .collect::<Option<Vec<_>>>()?;
        let identity = match arena.get(class.type_id)?.kind() {
            TypeKind::Nominal(identity) => identity.clone(),
            _ => return None,
        };
        return Some(arena.generic_instance(GenericTypeIdentity::new(identity, arguments)));
    }
    resolve_annotation_type(
        annotation,
        type_name_references,
        array_types,
        primitives,
        classes,
        arena,
    )
}

pub(crate) fn generic_enum_bindings<'a>(
    parsed: &'a ParseOutput,
    subject_type: TypeId,
    enum_class: &'a ClassTypeRecord,
    arena: &TypeArena,
) -> Vec<(&'a str, TypeId)> {
    let Some(TypeKind::GenericInstance(identity)) =
        arena.get(subject_type).map(|record| record.kind())
    else {
        return Vec::new();
    };
    let parameters = parsed
        .generic_parameters
        .iter()
        .filter(|parameter| parameter.owner == Some(enum_class.declaration()))
        .map(|parameter| parameter.name.as_str());
    parameters
        .zip(identity.arguments().iter().copied())
        .collect()
}

pub(crate) fn resolve_enum_payload_type(
    argument: AstNodeId,
    parsed: &ParseOutput,
    primitives: &PrimitiveTypeIds,
    classes: &[ClassTypeRecord],
    arena: &mut TypeArena,
    generic_bindings: &[(&str, TypeId)],
) -> Option<TypeId> {
    resolve_annotation_type_with_generics(
        argument,
        &parsed.type_name_references,
        &parsed.array_types,
        primitives,
        classes,
        arena,
        generic_bindings,
    )
    .or_else(|| {
        let name = parsed
            .name_references
            .iter()
            .find(|reference| reference.reference == argument)
            .map(|reference| reference.name.as_str())?;
        generic_bindings
            .iter()
            .find(|(parameter, _)| *parameter == name)
            .map(|(_, ty)| *ty)
    })
}

use crate::{
    ast::{AstArena, AstNodeId, AstNodeKind},
    module::{ModuleName, PackageNamespace},
    name_resolution::{LocalBinding, LocalBindingKind, ResolutionTable, ResolvedLocalBinding},
    parser::{
        ParseOutput, ParsedArrayType, ParsedAssignmentStatement, ParsedBinaryExpression,
        ParsedBinaryOperator, ParsedClassDeclaration, ParsedFunctionDeclaration,
        ParsedFunctionParameter, ParsedGenericParameter, ParsedGroupedExpression,
        ParsedIfExpression, ParsedIntegerLiteral, ParsedLiteralExpression, ParsedLiteralKind,
        ParsedLocalDeclaration, ParsedTypeNameReference, ParsedUnaryExpression,
        ParsedUnaryOperator,
    },
    source::ByteSpan,
    symbol::{SymbolId, SymbolInterner},
    types::{GenericParameterType, PrimitiveType, TypeArena, TypeId, TypeKind, TypeRecord},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AmbiguousTypeRule {
    LiteralTyping,
    PrimitiveScalarCatalog,
    AssignmentCompatibility,
    CallResolution,
    FunctionTypeApplication,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeCheckDiagnosticKind {
    AmbiguousTypeRule,
    UnresolvedTypeRule,
    UnsupportedTypeRule,
    TypeMismatch,
    InvalidNullableUse,
    InvalidatedRefinement,
    UnsupportedFlowRule,
    AmbiguousFlowRule,
    StaticIntegerDiagnostic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeRuleDiagnostic {
    Ambiguous(AmbiguousTypeRule),
    MissingAnnotationType,
    MissingResolvedNameType,
    DirectCallDeferred,
    FunctionTypeApplicationDeferred,
    MemberExpressionDeferred,
    BinaryExpressionDeferred,
    UnaryExpressionDeferred,
    IfValueDeferred,
    ConditionalConditionNotBool,
    ConditionalElseRequired,
    ConditionalBranchTypeMismatch,
    ConditionalMissingBranchValue,
    NullableValueWithoutRefinement,
    NullableAssignmentWithoutRefinement,
    AssignmentInvalidatedRefinement,
    RegionExitInvalidatedRefinement,
    MutableLocalRefinementDeferred,
    BooleanCombinationRefinementDeferred,
    MemberRefinementDeferred,
    CallResultRefinementDeferred,
    ExclusiveBorrowRefinementDeferred,
    AmbiguousLocalBindingFlow,
    AmbiguousNullTestRegion,
    IntegerLiteralOutOfRange,
    ByteLiteralOutOfRange,
    IntegerOverflow,
    DivisionByZero,
    NegativeExponent,
    InvalidShiftCount,
    ConstInitializerRequired,
    ConstInitializerNotConstant,
    ConstDependencyCycle,
    ArrayLiteralLengthMismatch,
    ArrayElementTypeMismatch,
    ArrayIndexTypeMismatch,
    ArrayIndexOutOfBounds,
    ImmutableArrayMutation,
    StringIndexTypeMismatch,
    StringIndexOutOfBounds,
    StringOperationUnsupported,
    DuplicateField,
    FieldHiding,
    ImmutableFieldMutation,
}

impl PartialEq<AmbiguousTypeRule> for TypeRuleDiagnostic {
    fn eq(&self, other: &AmbiguousTypeRule) -> bool {
        matches!(self, Self::Ambiguous(rule) if rule == other)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeCheckDiagnostic {
    kind: TypeCheckDiagnosticKind,
    rule: TypeRuleDiagnostic,
    node: AstNodeId,
    expected_type: Option<TypeId>,
    actual_type: Option<TypeId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompileTimeValue {
    Bool(bool),
    Int(i64),
    Float(u64),
    Byte(u8),
    Unit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompileTimeConstant {
    declaration: AstNodeId,
    ty: TypeId,
    value: CompileTimeValue,
}

impl CompileTimeConstant {
    pub fn new(declaration: AstNodeId, ty: TypeId, value: CompileTimeValue) -> Self {
        Self {
            declaration,
            ty,
            value,
        }
    }

    pub fn declaration(self) -> AstNodeId {
        self.declaration
    }

    pub fn ty(self) -> TypeId {
        self.ty
    }

    pub fn value(self) -> CompileTimeValue {
        self.value
    }
}

impl TypeCheckDiagnostic {
    pub fn static_integer(rule: TypeRuleDiagnostic, node: AstNodeId) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::StaticIntegerDiagnostic,
            rule,
            node,
            expected_type: None,
            actual_type: None,
        }
    }
    pub fn ambiguous_type_rule(rule: AmbiguousTypeRule, node: AstNodeId) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::AmbiguousTypeRule,
            rule: TypeRuleDiagnostic::Ambiguous(rule),
            node,
            expected_type: None,
            actual_type: None,
        }
    }

    pub fn unresolved_type_rule(rule: TypeRuleDiagnostic, node: AstNodeId) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::UnresolvedTypeRule,
            rule,
            node,
            expected_type: None,
            actual_type: None,
        }
    }

    pub fn unsupported_type_rule(rule: TypeRuleDiagnostic, node: AstNodeId) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::UnsupportedTypeRule,
            rule,
            node,
            expected_type: None,
            actual_type: None,
        }
    }

    pub fn type_mismatch(node: AstNodeId, expected_type: TypeId, actual_type: TypeId) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::TypeMismatch,
            rule: TypeRuleDiagnostic::Ambiguous(AmbiguousTypeRule::AssignmentCompatibility),
            node,
            expected_type: Some(expected_type),
            actual_type: Some(actual_type),
        }
    }

    pub fn invalid_nullable_use(
        rule: TypeRuleDiagnostic,
        node: AstNodeId,
        expected_type: TypeId,
        actual_type: TypeId,
    ) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::InvalidNullableUse,
            rule,
            node,
            expected_type: Some(expected_type),
            actual_type: Some(actual_type),
        }
    }

    pub fn invalidated_refinement(
        rule: TypeRuleDiagnostic,
        node: AstNodeId,
        expected_type: TypeId,
        actual_type: TypeId,
    ) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::InvalidatedRefinement,
            rule,
            node,
            expected_type: Some(expected_type),
            actual_type: Some(actual_type),
        }
    }

    pub fn unsupported_flow_rule(rule: TypeRuleDiagnostic, node: AstNodeId) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::UnsupportedFlowRule,
            rule,
            node,
            expected_type: None,
            actual_type: None,
        }
    }

    pub fn ambiguous_flow_rule(rule: TypeRuleDiagnostic, node: AstNodeId) -> Self {
        Self {
            kind: TypeCheckDiagnosticKind::AmbiguousFlowRule,
            rule,
            node,
            expected_type: None,
            actual_type: None,
        }
    }

    pub fn kind(&self) -> TypeCheckDiagnosticKind {
        self.kind
    }

    pub fn rule(&self) -> TypeRuleDiagnostic {
        self.rule
    }

    pub fn node(&self) -> AstNodeId {
        self.node
    }

    pub fn expected_type(&self) -> Option<TypeId> {
        self.expected_type
    }

    pub fn actual_type(&self) -> Option<TypeId> {
        self.actual_type
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntryPointDiagnosticKind {
    MissingEntryPoint,
    DuplicateEntryPoint,
    InvalidEntryPointSignature,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EntryPointDiagnosticLocation {
    Source(ByteSpan),
    EntryPackageInput(PackageNamespace),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryPointDiagnostic {
    kind: EntryPointDiagnosticKind,
    location: EntryPointDiagnosticLocation,
}

impl EntryPointDiagnostic {
    fn source(kind: EntryPointDiagnosticKind, span: ByteSpan) -> Self {
        Self {
            kind,
            location: EntryPointDiagnosticLocation::Source(span),
        }
    }

    fn entry_package_input(package: &PackageNamespace) -> Self {
        Self {
            kind: EntryPointDiagnosticKind::MissingEntryPoint,
            location: EntryPointDiagnosticLocation::EntryPackageInput(package.clone()),
        }
    }

    pub fn kind(&self) -> EntryPointDiagnosticKind {
        self.kind
    }

    pub fn source_span(&self) -> Option<ByteSpan> {
        match &self.location {
            EntryPointDiagnosticLocation::Source(span) => Some(*span),
            EntryPointDiagnosticLocation::EntryPackageInput(_) => None,
        }
    }

    pub fn location(&self) -> &EntryPointDiagnosticLocation {
        &self.location
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EntryPoint {
    declaration: AstNodeId,
    source_span: ByteSpan,
}

impl EntryPoint {
    pub fn declaration(self) -> AstNodeId {
        self.declaration
    }

    pub fn source_span(self) -> ByteSpan {
        self.source_span
    }
}

pub struct EntryPointFile<'a> {
    package: &'a PackageNamespace,
    parsed: &'a ParseOutput,
}

impl<'a> EntryPointFile<'a> {
    pub fn new(package: &'a PackageNamespace, parsed: &'a ParseOutput) -> Self {
        Self { package, parsed }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EntryPointReport {
    entry_point: Option<EntryPoint>,
    diagnostics: Vec<EntryPointDiagnostic>,
}

impl EntryPointReport {
    pub fn entry_point(&self) -> Option<EntryPoint> {
        self.entry_point
    }

    pub fn diagnostics(&self) -> &[EntryPointDiagnostic] {
        &self.diagnostics
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReturnPathDiagnosticKind {
    MissingReturn,
    UnreachableReturn,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReturnPathDiagnostic {
    kind: ReturnPathDiagnosticKind,
    node: AstNodeId,
}

impl ReturnPathDiagnostic {
    fn new(kind: ReturnPathDiagnosticKind, node: AstNodeId) -> Self {
        Self { kind, node }
    }

    pub fn kind(self) -> ReturnPathDiagnosticKind {
        self.kind
    }

    pub fn node(self) -> AstNodeId {
        self.node
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReturnPathReport {
    diagnostics: Vec<ReturnPathDiagnostic>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReturnTypeDiagnosticKind {
    ReturnTypeMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReturnTypeDiagnostic {
    kind: ReturnTypeDiagnosticKind,
    span: ByteSpan,
}

impl ReturnTypeDiagnostic {
    fn new(kind: ReturnTypeDiagnosticKind, span: ByteSpan) -> Self {
        Self { kind, span }
    }

    pub fn kind(self) -> ReturnTypeDiagnosticKind {
        self.kind
    }

    pub fn span(self) -> ByteSpan {
        self.span
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReturnTypeReport {
    diagnostics: Vec<ReturnTypeDiagnostic>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsupportedExecutableFormDiagnostic {
    span: ByteSpan,
}

impl UnsupportedExecutableFormDiagnostic {
    fn new(span: ByteSpan) -> Self {
        Self { span }
    }

    pub fn span(self) -> ByteSpan {
        self.span
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UnsupportedExecutableFormReport {
    diagnostics: Vec<UnsupportedExecutableFormDiagnostic>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionSignature {
    declaration: AstNodeId,
    parameter_types: Vec<TypeId>,
    return_type: TypeId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassTypeRecord {
    declaration: AstNodeId,
    name: String,
    is_final: bool,
    interface: bool,
    type_id: TypeId,
    superclass: Option<String>,
    interfaces: Vec<String>,
    constructor_parameter_count: usize,
}

impl ClassTypeRecord {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn is_final(&self) -> bool {
        self.is_final
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
    declaration: AstNodeId,
    owner: AstNodeId,
    type_id: TypeId,
    name: String,
    visibility: String,
    mutable: bool,
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
    classes: Vec<ClassTypeRecord>,
    fields: Vec<FieldTypeRecord>,
    diagnostics: Vec<TypeCheckDiagnostic>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConstructorDiagnosticKind {
    UnknownClass,
    ArgumentCountMismatch,
    SuperclassArgumentCountMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConstructorDiagnostic {
    kind: ConstructorDiagnosticKind,
    span: ByteSpan,
}

impl ConstructorDiagnostic {
    pub fn kind(self) -> ConstructorDiagnosticKind {
        self.kind
    }
    pub fn span(self) -> ByteSpan {
        self.span
    }
}

pub fn check_m0069_constructor_calls(
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassLifecycleFacts {
    class: AstNodeId,
    initialization_order: Vec<String>,
    destruction_order: Vec<String>,
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

fn append_inherited_field_order(
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
    kind: DispatchDiagnosticKind,
    span: ByteSpan,
}

impl DispatchDiagnostic {
    pub fn kind(self) -> DispatchDiagnosticKind {
        self.kind
    }
    pub fn span(self) -> ByteSpan {
        self.span
    }
}

pub fn check_m0070_dispatch(parsed: &ParseOutput) -> Vec<DispatchDiagnostic> {
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
        for method in parsed
            .function_declarations
            .iter()
            .filter(|method| method.owner == Some(child.declaration))
        {
            let targets = inherited_method_targets(parsed, child, &method.name)
                .into_iter()
                .filter(|target| same_function_parameter_types(parsed, method, target))
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
            let method_parameters = parsed
                .function_parameters
                .iter()
                .filter(|parameter| parameter.function == method.declaration)
                .map(|parameter| type_annotation_name(parsed, Some(parameter.annotation)))
                .collect::<Vec<_>>();
            let parent_parameters = parsed
                .function_parameters
                .iter()
                .filter(|parameter| parameter.function == parent_method.declaration)
                .map(|parameter| type_annotation_name(parsed, Some(parameter.annotation)))
                .collect::<Vec<_>>();
            if method.is_override
                && (!return_compatible || method_parameters != parent_parameters)
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

fn same_function_parameter_types(
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

fn inherited_method_targets<'a>(
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

fn collect_interface_method_targets<'a>(
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

fn type_annotation_name(parsed: &ParseOutput, annotation: Option<AstNodeId>) -> Option<String> {
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

pub fn type_m0068_class_types(
    parsed: &ParseOutput,
    module: &ModuleName,
    package: &PackageNamespace,
) -> (TypeArena, ClassTypeReport) {
    let mut types = TypeArena::new();
    let report = type_m0068_class_types_in(&mut types, parsed, module, package);
    (types, report)
}

pub fn type_m0068_class_types_in(
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
            is_final: declaration.is_final,
            interface: declaration.interface,
            type_id,
            superclass: declaration.superclass.clone(),
            interfaces: declaration.interfaces.clone(),
            constructor_parameter_count: declaration.constructor_parameters.len(),
        });
    }
    for field in &parsed.field_declarations {
        let Some(type_id) = primitive_annotation_type_for_node(
            field.annotation,
            &parsed.type_name_references,
            primitives,
        ) else {
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
            ) else {
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
    let _ = class_ids;
    report
}

pub fn apply_m0068_class_type_facts(
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

pub fn apply_m0068_field_access_facts(
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

fn class_or_superclass_owns_field(
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

pub fn apply_m0070_method_call_facts(
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

pub fn apply_m0070_receiver_name_facts(
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

pub fn apply_m0070_receiver_signatures(
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
        if let Some(signature) = signatures
            .iter_mut()
            .find(|signature| signature.declaration == function.declaration)
        {
            signature.prepend_parameter_type(class.type_id);
        }
    }
}

fn primitive_annotation_type_for_node(
    annotation: AstNodeId,
    references: &[ParsedTypeNameReference],
    primitives: PrimitiveTypeIds,
) -> Option<TypeId> {
    let reference = references
        .iter()
        .find(|reference| reference.reference == annotation)?;
    primitives.type_for_primitive_name(&reference.name)
}

pub struct ExecutableSourceTypes<'a> {
    package: &'a PackageNamespace,
    parsed: &'a ParseOutput,
    signatures: &'a [FunctionSignature],
    expression_types: &'a [ExpressionType],
    class_types: Option<&'a ClassTypeReport>,
}

impl<'a> ExecutableSourceTypes<'a> {
    pub fn new(
        package: &'a PackageNamespace,
        parsed: &'a ParseOutput,
        signatures: &'a [FunctionSignature],
        expression_types: &'a [ExpressionType],
    ) -> Self {
        Self {
            package,
            parsed,
            signatures,
            expression_types,
            class_types: None,
        }
    }

    pub fn with_class_types(mut self, class_types: &'a ClassTypeReport) -> Self {
        self.class_types = Some(class_types);
        self
    }

    pub fn package(&self) -> &PackageNamespace {
        self.package
    }

    pub fn parsed(&self) -> &ParseOutput {
        self.parsed
    }

    pub fn signatures(&self) -> &[FunctionSignature] {
        self.signatures
    }

    pub fn expression_types(&self) -> &[ExpressionType] {
        self.expression_types
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectCallDiagnosticKind {
    InvalidCallTarget,
    NoMatchingOverload,
    AmbiguousOverload,
    DuplicateOverload,
    ArgumentCountMismatch,
    ArgumentTypeMismatch,
    RecursiveCallUnsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirectCallDiagnostic {
    kind: DirectCallDiagnosticKind,
    span: ByteSpan,
}

impl DirectCallDiagnostic {
    pub fn new(kind: DirectCallDiagnosticKind, span: ByteSpan) -> Self {
        Self { kind, span }
    }

    pub fn kind(self) -> DirectCallDiagnosticKind {
        self.kind
    }

    pub fn span(self) -> ByteSpan {
        self.span
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirectCallExpressionType {
    expression_span: ByteSpan,
    ty: TypeId,
}

impl DirectCallExpressionType {
    pub fn expression_span(self) -> ByteSpan {
        self.expression_span
    }

    pub fn ty(self) -> TypeId {
        self.ty
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DirectCallReport {
    expression_types: Vec<ExpressionType>,
    source_expression_types: Vec<DirectCallExpressionType>,
    resolved_declarations: Vec<ResolvedCallDeclaration>,
    diagnostics: Vec<DirectCallDiagnostic>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedCallDeclaration {
    call: AstNodeId,
    declaration: AstNodeId,
}

impl ResolvedCallDeclaration {
    pub fn call(self) -> AstNodeId {
        self.call
    }

    pub fn declaration(self) -> AstNodeId {
        self.declaration
    }
}

impl DirectCallReport {
    pub fn expression_types(&self) -> &[ExpressionType] {
        &self.expression_types
    }

    pub fn source_expression_types(&self) -> &[DirectCallExpressionType] {
        &self.source_expression_types
    }

    pub fn resolved_declarations(&self) -> &[ResolvedCallDeclaration] {
        &self.resolved_declarations
    }

    pub fn diagnostics(&self) -> &[DirectCallDiagnostic] {
        &self.diagnostics
    }
}

pub fn check_m0028_direct_calls(sources: &[ExecutableSourceTypes<'_>]) -> DirectCallReport {
    let mut report = DirectCallReport::default();
    let mut resolved_calls = Vec::new();
    let mut call_edges: Vec<(ByteSpan, ByteSpan)> = Vec::new();

    for (source_index, source) in sources.iter().enumerate() {
        for (index, function) in source.parsed.function_declarations.iter().enumerate() {
            if source.parsed.function_declarations[index + 1..]
                .iter()
                .any(|other| {
                    other.owner == function.owner
                        && other.name == function.name
                        && same_function_parameter_types(source.parsed, function, other)
                })
                && let Some(node) = source.parsed.arena.node(function.declaration)
            {
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::DuplicateOverload,
                    node.span,
                ));
            }
        }
        for (call_index, call) in source.parsed.call_expressions.iter().enumerate() {
            let callee_name = source
                .parsed
                .name_references
                .iter()
                .find(|reference| reference.reference == call.callee)
                .map(|reference| (reference.name.clone(), reference.name_span));
            let member = source
                .parsed
                .member_expressions
                .iter()
                .find(|member| member.expression == call.callee);
            let Some((target_name, diagnostic_span)) =
                callee_name.or_else(|| member.map(|member| (member.name.clone(), member.span)))
            else {
                let span = source
                    .parsed
                    .arena
                    .node(call.callee)
                    .expect("parsed call callee")
                    .span;
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::InvalidCallTarget,
                    span,
                ));
                continue;
            };
            if member.is_none() && target_name == "clone" {
                continue;
            }
            if member
                .is_some_and(|member| matches!(member.name.as_str(), "size" | "add" | "remove"))
            {
                continue;
            }
            if member.is_some_and(|member| {
                source
                    .parsed
                    .index_expressions
                    .iter()
                    .any(|index| index.expression == member.receiver)
            }) {
                continue;
            }
            let targets: Vec<_> = sources
                .iter()
                .enumerate()
                .filter(|(_, candidate)| candidate.package == source.package)
                .flat_map(|(candidate_index, candidate)| {
                    candidate
                        .parsed
                        .function_declarations
                        .iter()
                        .filter(|function| {
                            function.name == target_name
                                && (member.is_none() || function.owner.is_some())
                                && member.is_none_or(|member| {
                                    member_receiver_class_name(
                                        source,
                                        member.receiver,
                                        call.function,
                                    )
                                    .and_then(|class_name| {
                                        function.owner.and_then(|owner| {
                                            candidate
                                                .parsed
                                                .class_declarations
                                                .iter()
                                                .find(|class| class.declaration == owner)
                                                .map(|class| class.name == class_name)
                                        })
                                    })
                                    .unwrap_or(true)
                                })
                        })
                        .filter_map(move |function| {
                            candidate
                                .signatures
                                .iter()
                                .position(|signature| signature.declaration == function.declaration)
                                .map(|signature_index| (candidate_index, signature_index))
                        })
                })
                .collect();
            let mut arguments = call.arguments.clone();
            if let Some(member) = member {
                arguments.insert(0, member.receiver);
            }
            let Some(argument_types) = arguments
                .iter()
                .enumerate()
                .map(|(index, argument)| {
                    if index == 0
                        && let Some(member) = member
                        && let Some(class_name) =
                            member_receiver_class_name(source, member.receiver, call.function)
                        && let Some(class_type) = source.class_types.and_then(|classes| {
                            classes
                                .classes
                                .iter()
                                .find(|class| class.name == class_name)
                        })
                    {
                        return Some(class_type.type_id);
                    }
                    source
                        .expression_types
                        .iter()
                        .find(|typed| typed.expression == *argument)
                        .map(|typed| typed.ty)
                })
                .collect::<Option<Vec<_>>>()
            else {
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::InvalidCallTarget,
                    diagnostic_span,
                ));
                continue;
            };
            let target_count = targets.len();
            let arity_matches = targets
                .into_iter()
                .filter(|(candidate_index, signature_index)| {
                    sources[*candidate_index].signatures[*signature_index]
                        .parameter_types()
                        .len()
                        == argument_types.len()
                })
                .collect::<Vec<_>>();
            if arity_matches.is_empty() {
                report.diagnostics.push(DirectCallDiagnostic::new(
                    if target_count == 0 {
                        DirectCallDiagnosticKind::InvalidCallTarget
                    } else if target_count == 1 {
                        DirectCallDiagnosticKind::ArgumentCountMismatch
                    } else {
                        DirectCallDiagnosticKind::NoMatchingOverload
                    },
                    diagnostic_span,
                ));
                continue;
            }
            let exact_matches = arity_matches
                .iter()
                .copied()
                .filter(|(candidate_index, signature_index)| {
                    sources[*candidate_index].signatures[*signature_index]
                        .parameter_types()
                        .iter()
                        .zip(&argument_types)
                        .all(|(expected, actual)| expected == actual)
                })
                .collect::<Vec<_>>();
            let compatible_matches = if exact_matches.is_empty() {
                arity_matches
                    .iter()
                    .copied()
                    .filter(|(candidate_index, signature_index)| {
                        sources[*candidate_index].signatures[*signature_index]
                            .parameter_types()
                            .iter()
                            .zip(&argument_types)
                            .all(|(expected, actual)| {
                                overload_type_compatible(source, *actual, *expected)
                            })
                    })
                    .collect::<Vec<_>>()
            } else {
                exact_matches
            };
            if compatible_matches.is_empty() {
                report.diagnostics.push(DirectCallDiagnostic::new(
                    if target_count == 1 {
                        DirectCallDiagnosticKind::ArgumentTypeMismatch
                    } else {
                        DirectCallDiagnosticKind::NoMatchingOverload
                    },
                    diagnostic_span,
                ));
                continue;
            }
            if compatible_matches.len() > 1 {
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::AmbiguousOverload,
                    diagnostic_span,
                ));
                continue;
            }
            let (target_source_index, target_signature_index) = compatible_matches[0];
            let target_source = &sources[target_source_index];
            let target = &target_source.signatures[target_signature_index];
            let has_body = target_source
                .parsed
                .function_declarations
                .iter()
                .any(|function| {
                    function.declaration == target.declaration && function.body.is_some()
                });
            let interface_contract = member.is_some_and(|member| {
                member_receiver_class_name(source, member.receiver, call.function)
                    .and_then(|name| {
                        source
                            .class_types?
                            .classes
                            .iter()
                            .find(|class| class.name == name)
                    })
                    .is_some_and(|class| class.interface)
            });
            if !has_body && !interface_contract {
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::InvalidCallTarget,
                    diagnostic_span,
                ));
                continue;
            }
            let source_function_span = source
                .parsed
                .arena
                .node(call.function)
                .expect("parsed call owner function")
                .span;
            let target_span = target_source
                .parsed
                .arena
                .node(target.declaration)
                .expect("parsed call target function")
                .span;
            call_edges.push((source_function_span, target_span));
            resolved_calls.push((
                source_index,
                call_index,
                source_function_span,
                target_span,
                target_source_index,
                target_signature_index,
            ));
        }
    }

    for (
        source_index,
        call_index,
        source_function_span,
        target_span,
        target_source_index,
        target_signature_index,
    ) in resolved_calls
    {
        let source = &sources[source_index];
        let call = &source.parsed.call_expressions[call_index];
        let target = &sources[target_source_index].signatures[target_signature_index];
        if m0028_call_path_exists(&call_edges, target_span, source_function_span) {
            let span = source
                .parsed
                .arena
                .node(call.expression)
                .expect("parsed call expression")
                .span;
            report.diagnostics.push(DirectCallDiagnostic::new(
                DirectCallDiagnosticKind::RecursiveCallUnsupported,
                span,
            ));
            continue;
        }
        let mut arguments = call.arguments.clone();
        if let Some(member) = source
            .parsed
            .member_expressions
            .iter()
            .find(|member| member.expression == call.callee)
        {
            arguments.insert(0, member.receiver);
        }
        if arguments.len() != target.parameter_types.len() {
            let span = source
                .parsed
                .arena
                .node(call.expression)
                .expect("parsed call expression")
                .span;
            report.diagnostics.push(DirectCallDiagnostic::new(
                DirectCallDiagnosticKind::ArgumentCountMismatch,
                span,
            ));
            continue;
        }
        let mut argument_error = false;
        for (index, (argument, parameter)) in
            arguments.iter().zip(&target.parameter_types).enumerate()
        {
            if index == 0
                && source
                    .parsed
                    .member_expressions
                    .iter()
                    .any(|member| member.expression == call.callee)
            {
                continue;
            }
            let actual_type = source
                .expression_types
                .iter()
                .find(|typed| typed.expression == *argument)
                .map(|typed| typed.ty);
            if actual_type
                .is_none_or(|actual| !overload_type_compatible(source, actual, *parameter))
            {
                let span = source
                    .parsed
                    .arena
                    .node(*argument)
                    .expect("parsed argument")
                    .span;
                report.diagnostics.push(DirectCallDiagnostic::new(
                    DirectCallDiagnosticKind::ArgumentTypeMismatch,
                    span,
                ));
                argument_error = true;
            }
        }
        if !argument_error {
            let call_span = source
                .parsed
                .arena
                .node(call.expression)
                .expect("parsed call expression")
                .span;
            report
                .expression_types
                .push(ExpressionType::new(call.expression, target.return_type));
            report
                .source_expression_types
                .push(DirectCallExpressionType {
                    expression_span: call_span,
                    ty: target.return_type,
                });
            report.resolved_declarations.push(ResolvedCallDeclaration {
                call: call.expression,
                declaration: target.declaration,
            });
        }
    }
    report
}

fn member_receiver_class_name(
    source: &ExecutableSourceTypes<'_>,
    receiver: AstNodeId,
    function_declaration: AstNodeId,
) -> Option<String> {
    let receiver_name = source
        .parsed
        .name_references
        .iter()
        .find(|name| name.reference == receiver)
        .map(|name| name.name.as_str());
    if matches!(receiver_name, Some("this" | "super")) {
        let owner = source
            .parsed
            .function_declarations
            .iter()
            .find(|function| function.declaration == function_declaration)?
            .owner?;
        let class = source
            .parsed
            .class_declarations
            .iter()
            .find(|class| class.declaration == owner)?;
        return if receiver_name == Some("super") {
            class.superclass.clone()
        } else {
            Some(class.name.clone())
        };
    }
    if let Some(expression) = source
        .parsed
        .new_expressions
        .iter()
        .find(|expression| expression.expression == receiver)
    {
        return Some(expression.type_name.clone());
    }
    let name = source
        .parsed
        .name_references
        .iter()
        .find(|name| name.reference == receiver)?;
    if let Some(parameter) =
        source.parsed.function_parameters.iter().find(|parameter| {
            parameter.function == function_declaration && parameter.name == name.name
        })
    {
        return source
            .parsed
            .type_name_references
            .iter()
            .find(|reference| reference.reference == parameter.annotation)
            .map(|reference| reference.name.clone());
    }
    let Some(binding) = source
        .parsed
        .local_binding_names
        .iter()
        .find(|binding| binding.name == name.name)
    else {
        return source
            .expression_types
            .iter()
            .find(|typed| typed.expression == receiver)
            .and_then(|typed| {
                source
                    .class_types?
                    .classes
                    .iter()
                    .find(|class| class.type_id == typed.ty)
            })
            .map(|class| class.name.clone());
    };
    let declaration = source
        .parsed
        .local_declarations
        .iter()
        .find(|declaration| declaration.declaration == binding.binding)?;
    let local_name = declaration.initializer.and_then(|initializer| {
        source
            .parsed
            .new_expressions
            .iter()
            .find(|expression| expression.expression == initializer)
            .map(|expression| expression.type_name.clone())
    });
    if local_name.is_some() {
        return local_name;
    }
    source
        .expression_types
        .iter()
        .find(|typed| typed.expression == receiver)
        .and_then(|typed| {
            source
                .class_types?
                .classes
                .iter()
                .find(|class| class.type_id == typed.ty)
        })
        .map(|class| class.name.clone())
}

fn overload_type_compatible(
    source: &ExecutableSourceTypes<'_>,
    actual: TypeId,
    expected: TypeId,
) -> bool {
    if actual == expected {
        return true;
    }
    let Some(classes) = source.class_types else {
        return false;
    };
    let Some(actual) = classes.classes.iter().find(|class| class.type_id == actual) else {
        return false;
    };
    let Some(expected) = classes
        .classes
        .iter()
        .find(|class| class.type_id == expected)
    else {
        return false;
    };
    if expected.interface {
        return actual.interfaces.iter().any(|name| name == &expected.name)
            || actual.superclass.as_deref().is_some_and(|parent| {
                classes
                    .classes
                    .iter()
                    .find(|class| class.name == parent)
                    .is_some_and(|parent| {
                        overload_type_compatible(source, parent.type_id, expected.type_id)
                    })
            });
    }
    actual.superclass.as_deref().is_some_and(|parent| {
        classes
            .classes
            .iter()
            .find(|class| class.name == parent)
            .is_some_and(|parent| {
                overload_type_compatible(source, parent.type_id, expected.type_id)
            })
    })
}

pub fn apply_m0028_direct_call_results(
    report: &mut TypeCheckReport,
    parsed: &ParseOutput,
    direct_calls: &DirectCallReport,
) {
    for call in &parsed.call_expressions {
        let span = parsed
            .arena
            .node(call.expression)
            .expect("parsed call expression")
            .span;
        let Some(ty) = direct_calls
            .source_expression_types
            .iter()
            .find(|typed| typed.expression_span == span)
            .map(|typed| typed.ty)
        else {
            continue;
        };
        report.record_expression_type(ExpressionType::new(call.expression, ty));
        report.diagnostics.retain(|diagnostic| {
            diagnostic.node() != call.expression
                || diagnostic.rule() != TypeRuleDiagnostic::DirectCallDeferred
        });
    }
}

fn m0028_call_path_exists(
    call_edges: &[(ByteSpan, ByteSpan)],
    start: ByteSpan,
    goal: ByteSpan,
) -> bool {
    let mut pending = vec![start];
    let mut visited = Vec::new();

    while let Some(current) = pending.pop() {
        if current == goal {
            return true;
        }
        if visited.contains(&current) {
            continue;
        }
        visited.push(current);
        pending.extend(
            call_edges
                .iter()
                .filter_map(|(owner, target)| (*owner == current).then_some(*target)),
        );
    }

    false
}

impl FunctionSignature {
    pub fn new(declaration: AstNodeId, parameter_types: Vec<TypeId>, return_type: TypeId) -> Self {
        Self {
            declaration,
            parameter_types,
            return_type,
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

    pub fn prepend_parameter_type(&mut self, parameter_type: TypeId) {
        self.parameter_types.insert(0, parameter_type);
    }
}

impl ReturnPathReport {
    pub fn diagnostics(&self) -> &[ReturnPathDiagnostic] {
        &self.diagnostics
    }
}

impl ReturnTypeReport {
    pub fn diagnostics(&self) -> &[ReturnTypeDiagnostic] {
        &self.diagnostics
    }
}

impl UnsupportedExecutableFormReport {
    pub fn diagnostics(&self) -> &[UnsupportedExecutableFormDiagnostic] {
        &self.diagnostics
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExpressionType {
    expression: AstNodeId,
    ty: TypeId,
}

impl ExpressionType {
    pub fn new(expression: AstNodeId, ty: TypeId) -> Self {
        Self { expression, ty }
    }

    pub fn expression(self) -> AstNodeId {
        self.expression
    }

    pub fn ty(self) -> TypeId {
        self.ty
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSignature {
    declaration: AstNodeId,
    ty: TypeId,
}

impl DeclarationSignature {
    pub fn new(declaration: AstNodeId, ty: TypeId) -> Self {
        Self { declaration, ty }
    }

    pub fn declaration(self) -> AstNodeId {
        self.declaration
    }

    pub fn ty(self) -> TypeId {
        self.ty
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssignmentCheck {
    statement: AstNodeId,
    target: TypeId,
    value: TypeId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefinementRecord {
    region: AstNodeId,
    binding_use: AstNodeId,
    originating_null_test: AstNodeId,
    binding: LocalBinding,
    original_nullable_type: TypeId,
    refined_non_null_type: TypeId,
}

impl RefinementRecord {
    pub fn new(
        region: AstNodeId,
        binding_use: AstNodeId,
        originating_null_test: AstNodeId,
        binding: LocalBinding,
        original_nullable_type: TypeId,
        refined_non_null_type: TypeId,
    ) -> Self {
        Self {
            region,
            binding_use,
            originating_null_test,
            binding,
            original_nullable_type,
            refined_non_null_type,
        }
    }

    pub fn region(&self) -> AstNodeId {
        self.region
    }

    pub fn binding_use(&self) -> AstNodeId {
        self.binding_use
    }

    pub fn originating_null_test(&self) -> AstNodeId {
        self.originating_null_test
    }

    pub fn binding(&self) -> &LocalBinding {
        &self.binding
    }

    pub fn original_nullable_type(&self) -> TypeId {
        self.original_nullable_type
    }

    pub fn refined_non_null_type(&self) -> TypeId {
        self.refined_non_null_type
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RefinedExpressionType {
    expression: AstNodeId,
    refinement: AstNodeId,
    original_nullable_type: TypeId,
    refined_non_null_type: TypeId,
}

impl RefinedExpressionType {
    pub fn new(
        expression: AstNodeId,
        refinement: AstNodeId,
        original_nullable_type: TypeId,
        refined_non_null_type: TypeId,
    ) -> Self {
        Self {
            expression,
            refinement,
            original_nullable_type,
            refined_non_null_type,
        }
    }

    pub fn expression(self) -> AstNodeId {
        self.expression
    }

    pub fn refinement(self) -> AstNodeId {
        self.refinement
    }

    pub fn original_nullable_type(self) -> TypeId {
        self.original_nullable_type
    }

    pub fn refined_non_null_type(self) -> TypeId {
        self.refined_non_null_type
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NullTestRefinedBranch {
    Then,
    Else,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecognizedNullTest {
    expression: AstNodeId,
    name_expression: AstNodeId,
    null_literal: AstNodeId,
    operator: ParsedBinaryOperator,
    refined_branch: NullTestRefinedBranch,
}

impl RecognizedNullTest {
    pub fn new(
        expression: AstNodeId,
        name_expression: AstNodeId,
        null_literal: AstNodeId,
        operator: ParsedBinaryOperator,
        refined_branch: NullTestRefinedBranch,
    ) -> Self {
        Self {
            expression,
            name_expression,
            null_literal,
            operator,
            refined_branch,
        }
    }

    pub fn expression(self) -> AstNodeId {
        self.expression
    }

    pub fn name_expression(self) -> AstNodeId {
        self.name_expression
    }

    pub fn null_literal(self) -> AstNodeId {
        self.null_literal
    }

    pub fn operator(self) -> ParsedBinaryOperator {
        self.operator
    }

    pub fn refined_branch(self) -> NullTestRefinedBranch {
        self.refined_branch
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EligibleNullTestRefinement {
    null_test: RecognizedNullTest,
    binding: LocalBinding,
    original_nullable_type: TypeId,
    refined_non_null_type: TypeId,
}

impl EligibleNullTestRefinement {
    pub fn new(
        null_test: RecognizedNullTest,
        binding: LocalBinding,
        original_nullable_type: TypeId,
        refined_non_null_type: TypeId,
    ) -> Self {
        Self {
            null_test,
            binding,
            original_nullable_type,
            refined_non_null_type,
        }
    }

    pub fn null_test(&self) -> RecognizedNullTest {
        self.null_test
    }

    pub fn binding(&self) -> &LocalBinding {
        &self.binding
    }

    pub fn original_nullable_type(&self) -> TypeId {
        self.original_nullable_type
    }

    pub fn refined_non_null_type(&self) -> TypeId {
        self.refined_non_null_type
    }
}

impl AssignmentCheck {
    pub fn new(statement: AstNodeId, target: TypeId, value: TypeId) -> Self {
        Self {
            statement,
            target,
            value,
        }
    }

    pub fn statement(self) -> AstNodeId {
        self.statement
    }

    pub fn target(self) -> TypeId {
        self.target
    }

    pub fn value(self) -> TypeId {
        self.value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiteralKind {
    BoolTrue,
    BoolFalse,
    AcceptedInteger,
    AcceptedString,
    Float,
    Unit,
    Null,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LiteralExpressionInput {
    expression: AstNodeId,
    kind: LiteralKind,
}

impl LiteralExpressionInput {
    pub fn new(expression: AstNodeId, kind: LiteralKind) -> Self {
        Self { expression, kind }
    }

    pub fn expression(self) -> AstNodeId {
        self.expression
    }

    pub fn kind(self) -> LiteralKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KnownSymbolType {
    symbol: SymbolId,
    ty: TypeId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GenericParameterTypeRecord {
    parameter: AstNodeId,
    ty: TypeId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapabilityBoundRecord {
    parameter: AstNodeId,
    ty: TypeId,
    bound: AstNodeId,
    symbol: SymbolId,
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

impl KnownSymbolType {
    pub fn new(symbol: SymbolId, ty: TypeId) -> Self {
        Self { symbol, ty }
    }

    pub fn symbol(self) -> SymbolId {
        self.symbol
    }

    pub fn ty(self) -> TypeId {
        self.ty
    }
}

pub fn build_m0020_generic_parameter_types(
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

pub fn build_m0020_capability_bound_records(
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeCheckReport {
    expression_types: Vec<ExpressionType>,
    declaration_signatures: Vec<DeclarationSignature>,
    assignment_checks: Vec<AssignmentCheck>,
    refinements: Vec<RefinementRecord>,
    refined_expression_types: Vec<RefinedExpressionType>,
    diagnostics: Vec<TypeCheckDiagnostic>,
    compile_time_constants: Vec<CompileTimeConstant>,
}

impl TypeCheckReport {
    pub fn new() -> Self {
        Self {
            expression_types: Vec::new(),
            declaration_signatures: Vec::new(),
            assignment_checks: Vec::new(),
            refinements: Vec::new(),
            refined_expression_types: Vec::new(),
            diagnostics: Vec::new(),
            compile_time_constants: Vec::new(),
        }
    }

    pub fn blocked(diagnostics: Vec<TypeCheckDiagnostic>) -> Self {
        Self {
            diagnostics,
            ..Self::new()
        }
    }

    pub fn is_blocked(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    pub fn record_expression_type(&mut self, expression_type: ExpressionType) {
        self.expression_types.push(expression_type);
    }

    pub fn replace_expression_type(&mut self, expression_type: ExpressionType) {
        self.expression_types
            .retain(|entry| entry.expression() != expression_type.expression());
        self.expression_types.push(expression_type);
    }

    pub fn expression_types(&self) -> &[ExpressionType] {
        &self.expression_types
    }

    pub fn expression_type(&self, expression: AstNodeId) -> Option<TypeId> {
        self.expression_types
            .iter()
            .find(|entry| entry.expression() == expression)
            .map(|entry| entry.ty())
    }

    pub fn record_declaration_signature(&mut self, signature: DeclarationSignature) {
        self.declaration_signatures.push(signature);
    }

    pub fn replace_declaration_signature(&mut self, signature: DeclarationSignature) {
        self.declaration_signatures
            .retain(|entry| entry.declaration() != signature.declaration());
        self.declaration_signatures.push(signature);
    }

    pub fn declaration_signatures(&self) -> &[DeclarationSignature] {
        &self.declaration_signatures
    }

    pub fn declaration_signature(&self, declaration: AstNodeId) -> Option<TypeId> {
        self.declaration_signatures
            .iter()
            .find(|entry| entry.declaration() == declaration)
            .map(|entry| entry.ty())
    }

    pub fn record_assignment_check(&mut self, assignment_check: AssignmentCheck) {
        self.assignment_checks.push(assignment_check);
    }

    pub fn record_refinement(&mut self, refinement: RefinementRecord) {
        self.refinements.push(refinement);
    }

    pub fn refinements(&self) -> &[RefinementRecord] {
        &self.refinements
    }

    pub fn refinement(&self, region: AstNodeId) -> Option<&RefinementRecord> {
        self.refinements
            .iter()
            .find(|entry| entry.region() == region)
    }

    pub fn record_refined_expression_type(&mut self, refined_type: RefinedExpressionType) {
        self.refined_expression_types.push(refined_type);
    }

    pub fn refined_expression_types(&self) -> &[RefinedExpressionType] {
        &self.refined_expression_types
    }

    pub fn refined_expression_type(&self, expression: AstNodeId) -> Option<TypeId> {
        self.refined_expression_types
            .iter()
            .find(|entry| entry.expression() == expression)
            .map(|entry| entry.refined_non_null_type())
    }

    pub fn record_diagnostic(&mut self, diagnostic: TypeCheckDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn assignment_checks(&self) -> &[AssignmentCheck] {
        &self.assignment_checks
    }

    pub fn assignment_check(&self, statement: AstNodeId) -> Option<AssignmentCheck> {
        self.assignment_checks
            .iter()
            .find(|entry| entry.statement() == statement)
            .cloned()
    }

    pub fn diagnostics(&self) -> &[TypeCheckDiagnostic] {
        &self.diagnostics
    }

    pub fn retain_diagnostics(&mut self, keep: impl Fn(&TypeCheckDiagnostic) -> bool) {
        self.diagnostics.retain(keep);
    }

    pub fn record_compile_time_constant(&mut self, constant: CompileTimeConstant) {
        self.compile_time_constants.push(constant);
    }

    pub fn compile_time_constants(&self) -> &[CompileTimeConstant] {
        &self.compile_time_constants
    }
}

impl Default for TypeCheckReport {
    fn default() -> Self {
        Self::new()
    }
}

pub fn check_m0028_entry_point(
    entry_package: &PackageNamespace,
    files: &[EntryPointFile<'_>],
) -> EntryPointReport {
    let candidates: Vec<_> = files
        .iter()
        .filter(|file| file.package == entry_package)
        .flat_map(|file| {
            file.parsed
                .function_declarations
                .iter()
                .filter(move |function| {
                    function.top_level
                        && file.parsed.declaration_names.iter().any(|name| {
                            name.declaration == function.declaration && name.name == "main"
                        })
                })
                .map(move |function| (file.parsed, function))
        })
        .collect();

    if candidates.is_empty() {
        return EntryPointReport {
            entry_point: None,
            diagnostics: vec![EntryPointDiagnostic::entry_package_input(entry_package)],
        };
    }

    if candidates.len() > 1 {
        return EntryPointReport {
            entry_point: None,
            diagnostics: candidates
                .iter()
                .map(|(parsed, function)| {
                    EntryPointDiagnostic::source(
                        EntryPointDiagnosticKind::DuplicateEntryPoint,
                        parsed
                            .arena
                            .node(function.declaration)
                            .expect("parsed function declaration is in its arena")
                            .span,
                    )
                })
                .collect(),
        };
    }

    let (parsed, candidate) = candidates[0];
    let has_int_return = candidate.return_annotation.is_some_and(|annotation| {
        parsed
            .type_name_references
            .iter()
            .any(|reference| reference.reference == annotation && reference.name == "Int")
    });
    let span = parsed
        .arena
        .node(candidate.declaration)
        .expect("parsed function declaration is in its arena")
        .span;
    if candidate.body.is_none() || !candidate.parameters.is_empty() || !has_int_return {
        return EntryPointReport {
            entry_point: None,
            diagnostics: vec![EntryPointDiagnostic::source(
                EntryPointDiagnosticKind::InvalidEntryPointSignature,
                span,
            )],
        };
    }

    EntryPointReport {
        entry_point: Some(EntryPoint {
            declaration: candidate.declaration,
            source_span: span,
        }),
        diagnostics: Vec::new(),
    }
}

pub fn check_m0028_straight_line_returns(parsed: &ParseOutput) -> ReturnPathReport {
    let mut diagnostics = Vec::new();
    for function in &parsed.function_declarations {
        let is_int_function = function.return_annotation.is_some_and(|annotation| {
            parsed
                .type_name_references
                .iter()
                .any(|reference| reference.reference == annotation && reference.name == "Int")
        });
        let Some(body) = function.body else {
            continue;
        };
        if !is_int_function {
            continue;
        }

        let direct_returns: Vec<_> = parsed
            .return_statements
            .iter()
            .filter(|returned| returned.function == function.declaration && returned.block == body)
            .collect();
        let Some((first, later)) = direct_returns.split_first() else {
            diagnostics.push(ReturnPathDiagnostic::new(
                ReturnPathDiagnosticKind::MissingReturn,
                function.declaration,
            ));
            continue;
        };
        let _ = first;
        diagnostics.extend(later.iter().map(|returned| {
            ReturnPathDiagnostic::new(
                ReturnPathDiagnosticKind::UnreachableReturn,
                returned.statement,
            )
        }));
    }
    ReturnPathReport { diagnostics }
}

pub fn check_m0028_return_expression_types(
    parsed: &ParseOutput,
    signatures: &[FunctionSignature],
    expression_types: &[ExpressionType],
) -> ReturnTypeReport {
    let mut diagnostics = Vec::new();

    for returned in &parsed.return_statements {
        let Some(value) = returned.value else {
            continue;
        };
        let Some(expected) = signatures
            .iter()
            .find(|signature| signature.declaration == returned.function)
            .map(|signature| signature.return_type)
        else {
            continue;
        };
        let Some(actual) = expression_types
            .iter()
            .find(|typed| typed.expression == value)
            .map(|typed| typed.ty)
        else {
            continue;
        };
        if actual != expected {
            let span = parsed.arena.node(value).expect("parsed return value").span;
            diagnostics.push(ReturnTypeDiagnostic::new(
                ReturnTypeDiagnosticKind::ReturnTypeMismatch,
                span,
            ));
        }
    }

    ReturnTypeReport { diagnostics }
}

pub fn check_m0028_unsupported_executable_forms(
    parsed: &ParseOutput,
) -> UnsupportedExecutableFormReport {
    let specific_form_spans: Vec<_> = parsed
        .arena
        .nodes()
        .iter()
        .filter(|node| {
            let dynamic_generic = node.kind == AstNodeKind::GenericArgument
                && parsed.type_name_references.iter().any(|reference| {
                    reference.name == "Array" && reference.generic_arguments.contains(&node.id)
                });
            !dynamic_generic
                && matches!(
                    node.kind,
                    AstNodeKind::BinaryExpression
                        | AstNodeKind::UnaryExpression
                        | AstNodeKind::CallExpression
                        | AstNodeKind::MemberExpression
                )
        })
        .map(|node| node.span)
        .collect();
    let mut candidates: Vec<_> = parsed
        .arena
        .nodes()
        .iter()
        .filter(|node| {
            matches!(
                node.kind,
                AstNodeKind::ImportDeclaration
                    | AstNodeKind::StructDeclaration
                    | AstNodeKind::EnumDeclaration
                    | AstNodeKind::NullableType
                    | AstNodeKind::GenericParameter
                    | AstNodeKind::CapabilityBound
                    | AstNodeKind::FunctionType
                    | AstNodeKind::GroupedType
                    | AstNodeKind::WhenExpression
            )
        })
        .map(|node| node.span)
        .collect();

    candidates.extend(
        parsed
            .literal_expressions
            .iter()
            .filter(|literal| matches!(literal.kind, ParsedLiteralKind::Null))
            .map(|literal| literal.span),
    );
    candidates.extend(
        parsed
            .type_name_references
            .iter()
            .filter(|reference| {
                reference.name != "Array"
                    && !matches!(
                        reference.name.as_str(),
                        "Bool" | "Int" | "String" | "Unit" | "Float" | "Byte"
                    )
                    && !parsed
                        .class_declarations
                        .iter()
                        .any(|class| class.name == reference.name)
            })
            .map(|reference| reference.name_span),
    );
    candidates.extend(
        parsed
            .function_declarations
            .iter()
            .filter(|function| function.body.is_none() && function.owner.is_none())
            .filter_map(|function| {
                parsed
                    .arena
                    .node(function.declaration)
                    .map(|node| node.span)
            }),
    );
    candidates.extend(
        parsed
            .local_declarations
            .iter()
            .filter(|declaration| declaration.initializer.is_none())
            .filter_map(|declaration| {
                parsed
                    .arena
                    .node(declaration.declaration)
                    .map(|node| node.span)
            }),
    );
    candidates.extend(
        parsed
            .return_statements
            .iter()
            .filter(|returned| returned.value.is_none())
            .filter_map(|returned| parsed.arena.node(returned.statement).map(|node| node.span)),
    );

    candidates.sort_by(|left, right| {
        left.file()
            .index()
            .cmp(&right.file().index())
            .then(left.start().cmp(&right.start()))
            .then(right.end().cmp(&left.end()))
    });
    candidates.dedup();

    let mut diagnostics = Vec::new();
    for span in candidates {
        if specific_form_spans
            .iter()
            .any(|specific| m0028_span_contains(*specific, span))
        {
            continue;
        }
        if diagnostics
            .iter()
            .any(|diagnostic: &UnsupportedExecutableFormDiagnostic| {
                m0028_span_contains(diagnostic.span, span)
            })
        {
            continue;
        }
        diagnostics.push(UnsupportedExecutableFormDiagnostic::new(span));
    }

    UnsupportedExecutableFormReport { diagnostics }
}

fn m0028_span_contains(outer: ByteSpan, inner: ByteSpan) -> bool {
    outer.file() == inner.file()
        && outer.start() <= inner.start()
        && inner.end() <= outer.end()
        && outer != inner
}

pub fn type_m0028_function_signatures(
    functions: &[ParsedFunctionDeclaration],
    parameters: &[ParsedFunctionParameter],
    type_name_references: &[ParsedTypeNameReference],
) -> (TypeArena, Vec<FunctionSignature>) {
    let mut arena = TypeArena::new();
    let signatures =
        type_m0028_function_signatures_in(&mut arena, functions, parameters, type_name_references);
    (arena, signatures)
}

pub fn type_m0028_function_signatures_in(
    arena: &mut TypeArena,
    functions: &[ParsedFunctionDeclaration],
    parameters: &[ParsedFunctionParameter],
    type_name_references: &[ParsedTypeNameReference],
) -> Vec<FunctionSignature> {
    type_m0063_function_signatures_in(arena, functions, parameters, type_name_references, &[])
}

pub fn type_m0063_function_signatures_in(
    arena: &mut TypeArena,
    functions: &[ParsedFunctionDeclaration],
    parameters: &[ParsedFunctionParameter],
    type_name_references: &[ParsedTypeNameReference],
    array_types: &[ParsedArrayType],
) -> Vec<FunctionSignature> {
    type_m0063_function_signatures_in_with_classes(
        arena,
        functions,
        parameters,
        type_name_references,
        array_types,
        &[],
    )
}

pub fn type_m0063_function_signatures_in_with_classes(
    arena: &mut TypeArena,
    functions: &[ParsedFunctionDeclaration],
    parameters: &[ParsedFunctionParameter],
    type_name_references: &[ParsedTypeNameReference],
    array_types: &[ParsedArrayType],
    classes: &[ClassTypeRecord],
) -> Vec<FunctionSignature> {
    let primitives = PrimitiveTypeIds::module_owned(arena);
    let mut signatures = Vec::new();

    for function in functions {
        let Some(return_annotation) = function.return_annotation else {
            continue;
        };
        let Some(return_type) = resolve_m0063_annotation_type(
            return_annotation,
            type_name_references,
            array_types,
            &primitives,
            classes,
            arena,
        ) else {
            continue;
        };
        let function_parameters: Vec<_> = parameters
            .iter()
            .filter(|parameter| parameter.function == function.declaration)
            .collect();
        let Some(parameter_types) = function_parameters
            .iter()
            .map(|parameter| {
                resolve_m0063_annotation_type(
                    parameter.annotation,
                    type_name_references,
                    array_types,
                    &primitives,
                    classes,
                    arena,
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
        });
    }

    signatures
}

fn resolve_m0063_annotation_type(
    annotation: AstNodeId,
    type_name_references: &[ParsedTypeNameReference],
    array_types: &[ParsedArrayType],
    primitives: &PrimitiveTypeIds,
    classes: &[ClassTypeRecord],
    arena: &mut TypeArena,
) -> Option<TypeId> {
    if let Some(reference) = type_name_references
        .iter()
        .find(|reference| reference.reference == annotation)
    {
        if reference.name == "Array" && reference.generic_argument_names.len() == 1 {
            let element_name = &reference.generic_argument_names[0];
            let element = primitives
                .type_for_primitive_name(element_name)
                .or_else(|| {
                    classes
                        .iter()
                        .find(|class| class.name == *element_name)
                        .map(|class| class.type_id)
                })?;
            return Some(arena.dynamic_array(element));
        }
        return primitives
            .type_for_primitive_name(&reference.name)
            .or_else(|| {
                classes
                    .iter()
                    .find(|class| class.name == reference.name)
                    .map(|class| class.type_id)
            });
    }
    let array = array_types.iter().find(|array| array.array == annotation)?;
    let element = resolve_m0063_annotation_type(
        array.element_type,
        type_name_references,
        array_types,
        primitives,
        classes,
        arena,
    )?;
    Some(arena.array(element, array.length?))
}

pub fn type_m0063_array_expressions(
    types: &mut TypeArena,
    parsed: &ParseOutput,
    report: &mut TypeCheckReport,
) {
    type_m0063_array_expressions_with_classes(types, parsed, report, &[]);
}

pub fn type_m0063_array_expressions_with_classes(
    types: &mut TypeArena,
    parsed: &ParseOutput,
    report: &mut TypeCheckReport,
    classes: &[ClassTypeRecord],
) {
    let primitives = PrimitiveTypeIds::module_owned(types);
    let const_lengths = report
        .compile_time_constants()
        .iter()
        .filter_map(|constant| match constant.value() {
            CompileTimeValue::Int(value) => u64::try_from(value)
                .ok()
                .map(|value| (constant.declaration(), value)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let resolve_type = |node: AstNodeId, types: &mut TypeArena| -> Option<TypeId> {
        if let Some(reference) = parsed
            .type_name_references
            .iter()
            .find(|reference| reference.reference == node)
        {
            if reference.name == "Array" && reference.generic_argument_names.len() == 1 {
                let element =
                    primitives.type_for_primitive_name(&reference.generic_argument_names[0])?;
                return Some(types.dynamic_array(element));
            }
            return primitives.type_for_primitive_name(&reference.name);
        }
        let array = parsed
            .array_types
            .iter()
            .find(|array| array.array == node)?;
        let element =
            resolve_array_element_type(array.element_type, parsed, types, primitives, classes)?;
        Some(types.array(
            element,
            array.length.or_else(|| {
                resolve_named_const_length(array.length_name.as_deref(), parsed, &const_lengths)
            })?,
        ))
    };

    for declaration in &parsed.local_declarations {
        let Some(annotation) = declaration.annotation else {
            continue;
        };
        let Some(ty) = resolve_type(annotation, types) else {
            continue;
        };
        report.record_declaration_signature(DeclarationSignature::new(declaration.declaration, ty));
        if let (Some(initializer), Some(TypeKind::Array(array))) = (
            declaration.initializer,
            types.get(ty).map(|record| record.kind()),
        ) && let Some(literal) = parsed
            .array_literals
            .iter()
            .find(|literal| literal.expression == initializer)
        {
            if literal.elements.len() as u64 != array.length() {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::ArrayLiteralLengthMismatch,
                    literal.expression,
                ));
            }
            for element in &literal.elements {
                if let Some(element_type) = report.expression_type(*element)
                    && !array_element_assignable(element_type, array.element(), classes)
                {
                    report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                        TypeRuleDiagnostic::ArrayElementTypeMismatch,
                        *element,
                    ));
                }
            }
        }
    }

    for expression in &parsed.new_expressions {
        if !expression.dynamic_array {
            continue;
        }
        if let Some(ty) = parsed
            .local_declarations
            .iter()
            .find(|declaration| declaration.initializer == Some(expression.expression))
            .and_then(|declaration| declaration.annotation)
            .and_then(|annotation| resolve_type(annotation, types))
        {
            report.replace_expression_type(ExpressionType::new(expression.expression, ty));
        }
    }

    for reference in &parsed.name_references {
        let Some(binding) = parsed
            .local_binding_names
            .iter()
            .find(|binding| binding.name == reference.name)
        else {
            continue;
        };
        if let Some(ty) = report.declaration_signature(binding.binding) {
            report.replace_expression_type(ExpressionType::new(reference.reference, ty));
        }
    }

    for literal in &parsed.array_literals {
        let ty = parsed
            .local_declarations
            .iter()
            .find(|declaration| declaration.initializer == Some(literal.expression))
            .and_then(|declaration| declaration.annotation)
            .and_then(|annotation| resolve_type(annotation, types))
            .or_else(|| {
                let element = literal
                    .elements
                    .first()
                    .and_then(|element| report.expression_type(*element))?;
                Some(types.array(element, literal.elements.len() as u64))
            });
        if let Some(ty) = ty {
            report.replace_expression_type(ExpressionType::new(literal.expression, ty));
        }
    }

    for index in &parsed.index_expressions {
        let Some(array_ty) = report.expression_type(index.array) else {
            continue;
        };
        let Some(index_ty) = report.expression_type(index.index) else {
            continue;
        };
        if index_ty != primitives.int_id {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ArrayIndexTypeMismatch,
                index.index,
            ));
            continue;
        }
        if let Some(TypeKind::Array(array)) = types.get(array_ty).map(|record| record.kind()) {
            report.replace_expression_type(ExpressionType::new(index.expression, array.element()));
            if let Some(value) = parsed
                .integer_literals
                .iter()
                .find(|literal| literal.expression == index.index)
                .and_then(|literal| literal.value)
                && value >= array.length()
            {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::ArrayIndexOutOfBounds,
                    index.index,
                ));
            }
        }
    }

    for assignment in &parsed.assignment_statements {
        let Some(index) = parsed
            .index_expressions
            .iter()
            .find(|index| index.expression == assignment.target)
        else {
            continue;
        };
        let Some(binding) = parsed.local_binding_names.iter().find(|binding| {
            binding.name
                == parsed
                    .name_references
                    .iter()
                    .find(|name| name.reference == index.array)
                    .map(|name| name.name.as_str())
                    .unwrap_or_default()
        }) else {
            continue;
        };
        if binding.kind == LocalBindingKind::Immutable {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ImmutableArrayMutation,
                assignment.target,
            ));
        }
    }
}

fn array_element_assignable(actual: TypeId, expected: TypeId, classes: &[ClassTypeRecord]) -> bool {
    if actual == expected {
        return true;
    }
    let Some(actual) = classes.iter().find(|class| class.type_id == actual) else {
        return false;
    };
    let Some(expected) = classes.iter().find(|class| class.type_id == expected) else {
        return false;
    };
    if expected.interface {
        return actual.interfaces.iter().any(|name| name == &expected.name)
            || actual.superclass.as_deref().is_some_and(|parent| {
                classes
                    .iter()
                    .find(|class| class.name == parent)
                    .is_some_and(|parent| {
                        array_element_assignable(parent.type_id, expected.type_id, classes)
                    })
            });
    }
    actual.superclass.as_deref().is_some_and(|parent| {
        classes
            .iter()
            .find(|class| class.name == parent)
            .is_some_and(|parent| {
                array_element_assignable(parent.type_id, expected.type_id, classes)
            })
    })
}

pub fn type_m0064_string_operations(
    parsed: &ParseOutput,
    report: &mut TypeCheckReport,
    types: &mut TypeArena,
    array_types: &[ParsedArrayType],
) {
    let Some(string_type) = types.records().iter().find_map(|record| {
        (record.kind() == &TypeKind::Primitive(PrimitiveType::String)).then_some(record.id())
    }) else {
        return;
    };
    let byte_type = types.records().iter().find_map(|record| {
        (record.kind() == &TypeKind::Primitive(PrimitiveType::Byte)).then_some(record.id())
    });
    let int_type = types.records().iter().find_map(|record| {
        (record.kind() == &TypeKind::Primitive(PrimitiveType::Int)).then_some(record.id())
    });
    let bool_type = types.records().iter().find_map(|record| {
        (record.kind() == &TypeKind::Primitive(PrimitiveType::Bool)).then_some(record.id())
    });

    for literal in &parsed.string_literals {
        report.replace_expression_type(ExpressionType::new(literal.expression, string_type));
    }
    report.retain_diagnostics(|diagnostic| {
        diagnostic.kind() != TypeCheckDiagnosticKind::TypeMismatch
            || !parsed
                .string_literals
                .iter()
                .any(|literal| literal.expression == diagnostic.node())
    });

    for name in &parsed.name_references {
        let Some(binding) = parsed
            .local_binding_names
            .iter()
            .find(|binding| binding.name == name.name)
        else {
            continue;
        };
        if let Some(ty) = report.declaration_signature(binding.binding) {
            report.replace_expression_type(ExpressionType::new(name.reference, ty));
        }
    }

    for parameter in &parsed.function_parameters {
        let primitives = PrimitiveTypeIds::module_owned(types);
        let Some(parameter_type) = resolve_m0063_annotation_type(
            parameter.annotation,
            &parsed.type_name_references,
            array_types,
            &primitives,
            &[],
            types,
        ) else {
            continue;
        };
        for reference in parsed.name_references.iter().filter(|reference| {
            reference.name == parameter.name
                && parsed
                    .arena
                    .node(reference.reference)
                    .is_some_and(|node| node.span.start() >= parameter.name_span.start())
        }) {
            report
                .replace_expression_type(ExpressionType::new(reference.reference, parameter_type));
        }
    }

    for member in &parsed.member_expressions {
        if report
            .expression_type(member.receiver)
            .is_some_and(|receiver| {
                matches!(
                    types.get(receiver).map(|record| record.kind()),
                    Some(TypeKind::DynamicArray(_))
                )
            })
        {
            continue;
        }
        if member.name == "length" && report.expression_type(member.receiver) == Some(string_type) {
            if let Some(int_type) = int_type {
                report.replace_expression_type(ExpressionType::new(member.expression, int_type));
            }
            report.retain_diagnostics(|diagnostic| {
                diagnostic.node() != member.expression
                    || diagnostic.rule() != TypeRuleDiagnostic::MemberExpressionDeferred
            });
            continue;
        }
        report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
            TypeRuleDiagnostic::MemberExpressionDeferred,
            member.expression,
        ));
    }

    for index in &parsed.index_expressions {
        if report.expression_type(index.array) != Some(string_type) {
            continue;
        }
        if report.expression_type(index.index) != int_type {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::StringIndexTypeMismatch,
                index.index,
            ));
            continue;
        }
        if let Some(byte_type) = byte_type {
            report.replace_expression_type(ExpressionType::new(index.expression, byte_type));
        }
        let literal_length = parsed
            .string_literals
            .iter()
            .find(|literal| literal.expression == index.array)
            .map(|literal| literal.bytes.len())
            .or_else(|| {
                let name = parsed
                    .name_references
                    .iter()
                    .find(|name| name.reference == index.array)?;
                let binding = parsed
                    .local_binding_names
                    .iter()
                    .find(|binding| binding.name == name.name)?;
                let declaration = parsed
                    .local_declarations
                    .iter()
                    .find(|declaration| declaration.declaration == binding.binding)?;
                let initializer = declaration.initializer?;
                parsed
                    .string_literals
                    .iter()
                    .find(|literal| literal.expression == initializer)
                    .map(|literal| literal.bytes.len())
            });
        let static_index = parsed
            .integer_literals
            .iter()
            .find(|literal| literal.expression == index.index)
            .and_then(|literal| literal.value.map(|value| value as i128))
            .or_else(|| {
                let unary = parsed
                    .unary_expressions
                    .iter()
                    .find(|unary| unary.expression == index.index)?;
                if unary.operator != crate::parser::ParsedUnaryOperator::Minus {
                    return None;
                }
                parsed
                    .integer_literals
                    .iter()
                    .find(|literal| literal.expression == unary.operand)
                    .and_then(|literal| literal.value.map(|value| -(value as i128)))
            });
        if let (Some(length), Some(value)) = (literal_length, static_index)
            && (value < 0 || value >= length as i128)
        {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::StringIndexOutOfBounds,
                index.index,
            ));
        }
    }

    for binary in &parsed.binary_expressions {
        let left = report.expression_type(binary.left);
        let right = report.expression_type(binary.right);
        if left != Some(string_type) || right != Some(string_type) {
            continue;
        }
        match binary.operator {
            ParsedBinaryOperator::Equal | ParsedBinaryOperator::NotEqual => {
                if let Some(bool_type) = bool_type {
                    report
                        .replace_expression_type(ExpressionType::new(binary.expression, bool_type));
                }
                report.retain_diagnostics(|diagnostic| {
                    let declaration = parsed
                        .local_declarations
                        .iter()
                        .find(|declaration| declaration.initializer == Some(binary.expression))
                        .map(|declaration| declaration.declaration);
                    diagnostic.kind() != TypeCheckDiagnosticKind::TypeMismatch
                        || (diagnostic.node() != binary.expression
                            && Some(diagnostic.node()) != declaration)
                });
            }
            ParsedBinaryOperator::Plus => {
                report.replace_expression_type(ExpressionType::new(binary.expression, string_type));
                report.retain_diagnostics(|diagnostic| {
                    let declaration = parsed
                        .local_declarations
                        .iter()
                        .find(|declaration| declaration.initializer == Some(binary.expression))
                        .map(|declaration| declaration.declaration);
                    diagnostic.kind() != TypeCheckDiagnosticKind::TypeMismatch
                        || (diagnostic.node() != binary.expression
                            && Some(diagnostic.node()) != declaration)
                });
            }
            _ => report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::StringOperationUnsupported,
                binary.expression,
            )),
        }
    }

    for call in &parsed.call_expressions {
        let is_clone = parsed
            .name_references
            .iter()
            .find(|name| name.reference == call.callee)
            .is_some_and(|name| name.name == "clone");
        if is_clone
            && call.arguments.len() == 1
            && report.expression_type(call.arguments[0]) == Some(string_type)
        {
            report.replace_expression_type(ExpressionType::new(call.expression, string_type));
            report.retain_diagnostics(|diagnostic| {
                diagnostic.node() != call.expression
                    || diagnostic.rule() != TypeRuleDiagnostic::DirectCallDeferred
            });
        }
    }

    for binary in &parsed.binary_expressions {
        let left = report.expression_type(binary.left);
        let right = report.expression_type(binary.right);
        if left != Some(string_type) || right != Some(string_type) {
            continue;
        }
        match binary.operator {
            ParsedBinaryOperator::Equal | ParsedBinaryOperator::NotEqual => {
                if let Some(bool_type) = bool_type {
                    report
                        .replace_expression_type(ExpressionType::new(binary.expression, bool_type));
                }
            }
            ParsedBinaryOperator::Plus => {
                report.replace_expression_type(ExpressionType::new(binary.expression, string_type));
            }
            _ => {}
        }
    }
}

pub fn type_m0073_dynamic_array_operations(
    parsed: &ParseOutput,
    report: &mut TypeCheckReport,
    types: &TypeArena,
) {
    let dynamic_declarations = parsed
        .local_declarations
        .iter()
        .filter(|declaration| {
            declaration
                .annotation
                .and_then(|annotation| {
                    parsed
                        .type_name_references
                        .iter()
                        .find(|reference| reference.reference == annotation)
                })
                .is_some_and(|reference| reference.name == "Array")
        })
        .map(|declaration| declaration.declaration)
        .collect::<Vec<_>>();
    report.retain_diagnostics(|diagnostic| {
        !dynamic_declarations.contains(&diagnostic.node())
            || diagnostic.rule() != TypeRuleDiagnostic::MissingAnnotationType
    });
    let int_type = types.records().iter().find_map(|record| {
        (record.kind() == &TypeKind::Primitive(PrimitiveType::Int)).then_some(record.id())
    });
    let unit_type = types.records().iter().find_map(|record| {
        (record.kind() == &TypeKind::Primitive(PrimitiveType::Unit)).then_some(record.id())
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
        if !matches!(
            types.get(receiver_type).map(|record| record.kind()),
            Some(TypeKind::DynamicArray(_))
        ) {
            continue;
        }
        let (result, mutating, valid_arity) = match member.name.as_str() {
            "size" => (int_type, false, call.arguments.is_empty()),
            "add" => (unit_type, true, matches!(call.arguments.len(), 1 | 2)),
            "remove" => (unit_type, true, call.arguments.len() == 1),
            _ => (None, false, false),
        };
        if !valid_arity {
            continue;
        }
        if mutating
            && let Some(name) = parsed
                .name_references
                .iter()
                .find(|name| name.reference == member.receiver)
            && let Some(binding) = parsed
                .local_binding_names
                .iter()
                .find(|binding| binding.name == name.name)
            && binding.kind == LocalBindingKind::Immutable
        {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ImmutableArrayMutation,
                member.expression,
            ));
            continue;
        }
        if let Some(result) = result {
            report.replace_expression_type(ExpressionType::new(call.expression, result));
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

fn resolve_array_element_type(
    node: AstNodeId,
    parsed: &ParseOutput,
    types: &mut TypeArena,
    primitives: PrimitiveTypeIds,
    classes: &[ClassTypeRecord],
) -> Option<TypeId> {
    if let Some(reference) = parsed
        .type_name_references
        .iter()
        .find(|reference| reference.reference == node)
    {
        return primitives
            .type_for_primitive_name(&reference.name)
            .or_else(|| {
                classes
                    .iter()
                    .find(|class| class.name == reference.name)
                    .map(|class| class.type_id)
            });
    }
    let array = parsed
        .array_types
        .iter()
        .find(|array| array.array == node)?;
    let element =
        resolve_array_element_type(array.element_type, parsed, types, primitives, classes)?;
    Some(
        types.array(
            element,
            array.length.or_else(|| {
                resolve_named_const_length(array.length_name.as_deref(), parsed, &[])
            })?,
        ),
    )
}

fn resolve_named_const_length(
    name: Option<&str>,
    parsed: &ParseOutput,
    const_lengths: &[(AstNodeId, u64)],
) -> Option<u64> {
    let name = name?;
    let binding = parsed.local_binding_names.iter().find(|binding| {
        binding.name == name && parsed.const_declarations.contains(&binding.binding)
    })?;
    let declaration = parsed
        .local_declarations
        .iter()
        .find(|declaration| declaration.declaration == binding.binding)?;
    let initializer = declaration.initializer?;
    const_lengths
        .iter()
        .find(|(declaration, _)| *declaration == binding.binding)
        .map(|(_, value)| *value)
        .or_else(|| {
            parsed
                .integer_literals
                .iter()
                .find(|literal| literal.expression == initializer)
                .and_then(|literal| literal.value)
        })
}

pub fn recognize_m0019_null_tests(
    binary_expressions: &[ParsedBinaryExpression],
    literal_expressions: &[ParsedLiteralExpression],
    arena: &AstArena,
) -> Vec<RecognizedNullTest> {
    let mut recognized = Vec::new();

    for binary in binary_expressions {
        let refined_branch = match binary.operator {
            ParsedBinaryOperator::NotEqual => NullTestRefinedBranch::Then,
            ParsedBinaryOperator::Equal => NullTestRefinedBranch::Else,
            _ => continue,
        };

        let left_is_null = is_null_literal(binary.left, literal_expressions);
        let right_is_null = is_null_literal(binary.right, literal_expressions);
        let (name_expression, null_literal) = match (left_is_null, right_is_null) {
            (true, false) if is_name_expression(binary.right, arena) => (binary.right, binary.left),
            (false, true) if is_name_expression(binary.left, arena) => (binary.left, binary.right),
            _ => continue,
        };

        recognized.push(RecognizedNullTest::new(
            binary.expression,
            name_expression,
            null_literal,
            binary.operator,
            refined_branch,
        ));
    }

    recognized
}

fn is_null_literal(expression: AstNodeId, literal_expressions: &[ParsedLiteralExpression]) -> bool {
    literal_expressions
        .iter()
        .any(|literal| literal.expression == expression && literal.kind == ParsedLiteralKind::Null)
}

fn is_name_expression(expression: AstNodeId, arena: &AstArena) -> bool {
    arena
        .node(expression)
        .is_some_and(|node| node.kind == AstNodeKind::NameExpression)
}

pub fn select_m0019_eligible_null_tests(
    null_tests: &[RecognizedNullTest],
    resolutions: &ResolutionTable,
    local_bindings: &[LocalBinding],
    declaration_signatures: &[DeclarationSignature],
    arena: &TypeArena,
) -> (Vec<EligibleNullTestRefinement>, TypeCheckReport) {
    let mut eligible = Vec::new();
    let mut report = TypeCheckReport::new();

    for null_test in null_tests {
        let Some(resolved) = resolutions.get(null_test.name_expression()) else {
            continue;
        };
        let matching_bindings: Vec<_> = local_bindings
            .iter()
            .filter(|binding| binding.key().name() == resolved.symbol())
            .collect();

        let [binding] = matching_bindings.as_slice() else {
            if matching_bindings.len() > 1 {
                report.record_diagnostic(TypeCheckDiagnostic::ambiguous_flow_rule(
                    TypeRuleDiagnostic::AmbiguousLocalBindingFlow,
                    null_test.name_expression(),
                ));
            }
            continue;
        };

        match binding.kind() {
            LocalBindingKind::Immutable => {}
            LocalBindingKind::Var => {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_flow_rule(
                    TypeRuleDiagnostic::MutableLocalRefinementDeferred,
                    null_test.name_expression(),
                ));
                continue;
            }
        }

        let Some(signature) = declaration_signatures
            .iter()
            .find(|signature| signature.declaration() == binding.binding())
        else {
            continue;
        };
        let Some(record) = arena.get(signature.ty()) else {
            continue;
        };
        let TypeKind::Nullable(nullable) = record.kind() else {
            continue;
        };

        eligible.push(EligibleNullTestRefinement::new(
            *null_test,
            (*binding).clone(),
            signature.ty(),
            nullable.base(),
        ));
    }

    (eligible, report)
}

pub fn record_m0019_branch_refinements(
    eligible_refinements: &[EligibleNullTestRefinement],
    if_expressions: &[ParsedIfExpression],
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for eligible in eligible_refinements {
        let Some(if_expression) = if_expressions
            .iter()
            .find(|if_expression| if_expression.condition == eligible.null_test().expression())
        else {
            continue;
        };

        let Some(region) = m0019_refined_branch_region(eligible.null_test(), if_expression) else {
            continue;
        };

        report.record_refinement(RefinementRecord::new(
            region,
            eligible.null_test().name_expression(),
            eligible.null_test().expression(),
            eligible.binding().clone(),
            eligible.original_nullable_type(),
            eligible.refined_non_null_type(),
        ));
    }

    report
}

fn m0019_refined_branch_region(
    null_test: RecognizedNullTest,
    if_expression: &ParsedIfExpression,
) -> Option<AstNodeId> {
    match null_test.refined_branch() {
        NullTestRefinedBranch::Then => Some(if_expression.then_block),
        NullTestRefinedBranch::Else => if_expression.else_block,
    }
}

pub fn record_m0019_refined_expression_types(
    report: &mut TypeCheckReport,
    arena: &AstArena,
    resolved_local_bindings: &[ResolvedLocalBinding],
) {
    let mut refined_types = Vec::new();
    let mut diagnostics = Vec::new();

    for resolved in resolved_local_bindings {
        let Some(expression) = arena.node(resolved.reference()) else {
            continue;
        };
        if expression.kind != AstNodeKind::NameExpression {
            continue;
        }

        let mut matching_refinement = None;
        let mut ambiguous = false;
        for refinement in report.refinements() {
            if refinement.binding() != resolved.binding()
                || !m0019_expression_is_inside_refinement_region(
                    arena,
                    resolved.reference(),
                    refinement.region(),
                )
            {
                continue;
            }

            if matching_refinement.is_some() {
                ambiguous = true;
                break;
            }
            matching_refinement = Some(refinement);
        }

        if ambiguous {
            diagnostics.push(TypeCheckDiagnostic::ambiguous_flow_rule(
                TypeRuleDiagnostic::AmbiguousNullTestRegion,
                resolved.reference(),
            ));
        } else if let Some(refinement) = matching_refinement {
            refined_types.push(RefinedExpressionType::new(
                resolved.reference(),
                refinement.region(),
                refinement.original_nullable_type(),
                refinement.refined_non_null_type(),
            ));
        }
    }

    for refined_type in refined_types {
        report.record_refined_expression_type(refined_type);
    }
    for diagnostic in diagnostics {
        report.record_diagnostic(diagnostic);
    }
}

fn m0019_expression_is_inside_refinement_region(
    arena: &AstArena,
    expression: AstNodeId,
    region: AstNodeId,
) -> bool {
    let Some(expression) = arena.node(expression) else {
        return false;
    };
    let Some(region) = arena.node(region) else {
        return false;
    };

    expression.kind == AstNodeKind::NameExpression
        && region.kind == AstNodeKind::Block
        && expression.span.file() == region.span.file()
        && region.span.start() <= expression.span.start()
        && expression.span.end() <= region.span.end()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PrimitiveTypeIds {
    bool_id: TypeId,
    int_id: TypeId,
    string_id: TypeId,
    unit_id: TypeId,
    null_id: TypeId,
    float_id: TypeId,
    byte_id: TypeId,
}

impl PrimitiveTypeIds {
    fn insert_into(arena: &mut TypeArena) -> Self {
        Self {
            bool_id: arena.insert(TypeRecord::primitive(PrimitiveType::Bool)),
            int_id: arena.insert(TypeRecord::primitive(PrimitiveType::Int)),
            string_id: arena.insert(TypeRecord::primitive(PrimitiveType::String)),
            unit_id: arena.insert(TypeRecord::primitive(PrimitiveType::Unit)),
            null_id: arena.insert(TypeRecord::primitive(PrimitiveType::Null)),
            float_id: arena.insert(TypeRecord::primitive(PrimitiveType::Float)),
            byte_id: arena.insert(TypeRecord::primitive(PrimitiveType::Byte)),
        }
    }

    fn module_owned(arena: &mut TypeArena) -> Self {
        if arena.records().is_empty() {
            return Self::insert_into(arena);
        }

        let primitive = |expected| {
            arena
                .records()
                .iter()
                .position(|record| record.kind() == &TypeKind::Primitive(expected))
                .map(TypeId::from_raw)
                .expect("module TypeArena contains every bootstrap primitive")
        };
        Self {
            bool_id: primitive(PrimitiveType::Bool),
            int_id: primitive(PrimitiveType::Int),
            string_id: primitive(PrimitiveType::String),
            unit_id: primitive(PrimitiveType::Unit),
            null_id: primitive(PrimitiveType::Null),
            float_id: primitive(PrimitiveType::Float),
            byte_id: primitive(PrimitiveType::Byte),
        }
    }

    fn type_for_literal(self, kind: LiteralKind) -> TypeId {
        match kind {
            LiteralKind::BoolTrue | LiteralKind::BoolFalse => self.bool_id,
            LiteralKind::AcceptedInteger => self.int_id,
            LiteralKind::AcceptedString => self.string_id,
            LiteralKind::Float => self.float_id,
            LiteralKind::Unit => self.unit_id,
            LiteralKind::Null => self.null_id,
        }
    }

    fn type_for_primitive_name(self, name: &str) -> Option<TypeId> {
        match name {
            "Bool" => Some(self.bool_id),
            "Int" => Some(self.int_id),
            "String" => Some(self.string_id),
            "Unit" => Some(self.unit_id),
            "Null" => Some(self.null_id),
            "Float" => Some(self.float_id),
            "Byte" => Some(self.byte_id),
            _ => None,
        }
    }
}

pub fn type_literal_expressions(inputs: &[LiteralExpressionInput]) -> (TypeArena, TypeCheckReport) {
    let mut arena = TypeArena::new();
    let primitives = PrimitiveTypeIds::insert_into(&mut arena);
    let mut report = TypeCheckReport::new();

    for input in inputs {
        report.record_expression_type(ExpressionType::new(
            input.expression(),
            primitives.type_for_literal(input.kind()),
        ));
    }

    (arena, report)
}

pub fn type_parser_literals(literals: &[ParsedLiteralExpression]) -> (TypeArena, TypeCheckReport) {
    let inputs: Vec<_> = literals
        .iter()
        .map(|literal| {
            LiteralExpressionInput::new(literal.expression, literal_kind_from_parser(literal.kind))
        })
        .collect();
    type_literal_expressions(&inputs)
}

fn literal_kind_from_parser(kind: ParsedLiteralKind) -> LiteralKind {
    match kind {
        ParsedLiteralKind::BoolTrue => LiteralKind::BoolTrue,
        ParsedLiteralKind::BoolFalse => LiteralKind::BoolFalse,
        ParsedLiteralKind::AcceptedInteger => LiteralKind::AcceptedInteger,
        ParsedLiteralKind::AcceptedString => LiteralKind::AcceptedString,
        ParsedLiteralKind::Float => LiteralKind::Float,
        ParsedLiteralKind::Unit => LiteralKind::Unit,
        ParsedLiteralKind::Null => LiteralKind::Null,
    }
}

pub fn type_primitive_local_declarations(
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
) -> (TypeArena, TypeCheckReport) {
    let mut arena = TypeArena::new();
    let primitives = PrimitiveTypeIds::insert_into(&mut arena);
    let mut report = TypeCheckReport::new();

    for declaration in declarations {
        let Some(annotation_type) =
            primitive_annotation_type(declaration, type_name_references, primitives)
        else {
            report.record_diagnostic(TypeCheckDiagnostic::unresolved_type_rule(
                TypeRuleDiagnostic::MissingAnnotationType,
                declaration.declaration,
            ));
            continue;
        };
        report.record_declaration_signature(DeclarationSignature::new(
            declaration.declaration,
            annotation_type,
        ));
    }

    (arena, report)
}

pub fn type_primitive_local_initializer_declarations(
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    integer_literals: &[ParsedIntegerLiteral],
) -> (TypeArena, TypeCheckReport) {
    let mut arena = TypeArena::new();
    let primitives = PrimitiveTypeIds::insert_into(&mut arena);
    let mut report = TypeCheckReport::new();

    for literal in literals {
        let kind = literal_kind_from_parser(literal.kind);
        report.record_expression_type(ExpressionType::new(
            literal.expression,
            primitives.type_for_literal(kind),
        ));
    }

    for declaration in declarations {
        let Some(annotation_type) =
            primitive_annotation_type(declaration, type_name_references, primitives)
        else {
            report.record_diagnostic(TypeCheckDiagnostic::unresolved_type_rule(
                TypeRuleDiagnostic::MissingAnnotationType,
                declaration.declaration,
            ));
            continue;
        };
        report.record_declaration_signature(DeclarationSignature::new(
            declaration.declaration,
            annotation_type,
        ));

        let Some(initializer) = declaration.initializer else {
            continue;
        };
        let Some(initializer_type) = report.expression_type(initializer) else {
            continue;
        };

        if annotation_type == primitives.byte_id
            && let Some(integer) = integer_literals
                .iter()
                .find(|literal| literal.expression == initializer)
        {
            match integer.value {
                Some(value) if value <= u64::from(u8::MAX) => {
                    report
                        .replace_expression_type(ExpressionType::new(initializer, annotation_type));
                    report.record_assignment_check(AssignmentCheck::new(
                        declaration.declaration,
                        annotation_type,
                        annotation_type,
                    ));
                    continue;
                }
                Some(_) | None => {
                    report.record_diagnostic(TypeCheckDiagnostic::static_integer(
                        TypeRuleDiagnostic::ByteLiteralOutOfRange,
                        initializer,
                    ));
                    continue;
                }
            }
        }

        if initializer_type == annotation_type {
            report.record_assignment_check(AssignmentCheck::new(
                declaration.declaration,
                annotation_type,
                initializer_type,
            ));
        } else {
            report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                initializer,
                annotation_type,
                initializer_type,
            ));
        }
    }

    (arena, report)
}

fn primitive_annotation_type(
    declaration: &ParsedLocalDeclaration,
    type_name_references: &[ParsedTypeNameReference],
    primitives: PrimitiveTypeIds,
) -> Option<TypeId> {
    let annotation = declaration.annotation?;
    let type_name = type_name_references
        .iter()
        .find(|reference| reference.reference == annotation)?;
    primitives.type_for_primitive_name(type_name.name.as_str())
}

pub fn type_resolved_name_expressions(
    resolutions: &ResolutionTable,
    known_symbols: &[KnownSymbolType],
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for resolved in resolutions.resolved_names() {
        let Some(known) = known_symbols
            .iter()
            .find(|known| known.symbol() == resolved.symbol())
        else {
            report.record_diagnostic(TypeCheckDiagnostic::unresolved_type_rule(
                TypeRuleDiagnostic::MissingResolvedNameType,
                resolved.reference(),
            ));
            continue;
        };
        report.record_expression_type(ExpressionType::new(resolved.reference(), known.ty()));
    }

    report
}

pub fn known_local_symbol_types(
    bindings: &[LocalBinding],
    signatures: &[DeclarationSignature],
) -> Vec<KnownSymbolType> {
    let mut known = Vec::new();

    for binding in bindings {
        let Some(signature) = signatures
            .iter()
            .find(|signature| signature.declaration() == binding.binding())
        else {
            continue;
        };
        known.push(KnownSymbolType::new(binding.key().name(), signature.ty()));
    }

    known
}

pub fn type_grouped_expressions(
    grouped_expressions: &[ParsedGroupedExpression],
    known_expression_types: &[ExpressionType],
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for grouped in grouped_expressions {
        let Some(inner) = known_expression_types
            .iter()
            .find(|entry| entry.expression() == grouped.inner)
        else {
            continue;
        };
        report.record_expression_type(ExpressionType::new(grouped.expression, inner.ty()));
    }

    report
}

pub fn type_m0028_executable_int_operators(
    unary_expressions: &[ParsedUnaryExpression],
    binary_expressions: &[ParsedBinaryExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    known_expression_types: &[ExpressionType],
    int_type: TypeId,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();
    for expression_type in known_expression_types {
        report.record_expression_type(*expression_type);
    }
    let mut completed = Vec::new();
    loop {
        let mut changed = false;

        for unary in unary_expressions {
            if completed.contains(&unary.expression)
                || !is_m0028_executable_int_unary_operator(unary.operator)
            {
                continue;
            }
            let Some(operand_type) = report.expression_type(unary.operand) else {
                continue;
            };

            completed.push(unary.expression);
            changed = true;
            if operand_type == int_type {
                report.record_expression_type(ExpressionType::new(unary.expression, int_type));
            } else {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    unary.operand,
                    int_type,
                    operand_type,
                ));
            }
        }

        for binary in binary_expressions {
            if completed.contains(&binary.expression)
                || !is_m0028_executable_int_operator(binary.operator)
            {
                continue;
            }
            let (Some(left_type), Some(right_type)) = (
                report.expression_type(binary.left),
                report.expression_type(binary.right),
            ) else {
                continue;
            };

            completed.push(binary.expression);
            changed = true;
            if left_type != int_type {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    binary.left,
                    int_type,
                    left_type,
                ));
            } else if right_type != int_type {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    binary.right,
                    int_type,
                    right_type,
                ));
            } else {
                report.record_expression_type(ExpressionType::new(binary.expression, int_type));
            }
        }

        for grouped in grouped_expressions {
            if completed.contains(&grouped.expression) {
                continue;
            }
            let Some(inner_type) = report.expression_type(grouped.inner) else {
                continue;
            };

            completed.push(grouped.expression);
            changed = true;
            report.record_expression_type(ExpressionType::new(grouped.expression, inner_type));
        }

        if !changed {
            break;
        }
    }

    report
}

fn is_m0028_executable_int_operator(operator: ParsedBinaryOperator) -> bool {
    matches!(
        operator,
        ParsedBinaryOperator::Plus
            | ParsedBinaryOperator::Minus
            | ParsedBinaryOperator::Star
            | ParsedBinaryOperator::Slash
            | ParsedBinaryOperator::Percent
            | ParsedBinaryOperator::Exponent
            | ParsedBinaryOperator::BitwiseAnd
            | ParsedBinaryOperator::BitwiseOr
            | ParsedBinaryOperator::BitwiseXor
            | ParsedBinaryOperator::ShiftLeft
            | ParsedBinaryOperator::ShiftRight
    )
}

fn is_m0028_executable_int_unary_operator(operator: crate::parser::ParsedUnaryOperator) -> bool {
    matches!(
        operator,
        crate::parser::ParsedUnaryOperator::Plus
            | crate::parser::ParsedUnaryOperator::Minus
            | crate::parser::ParsedUnaryOperator::BitwiseNot
    )
}

pub fn type_m0035_primitive_operators(
    unary_expressions: &[ParsedUnaryExpression],
    binary_expressions: &[ParsedBinaryExpression],
    known_expression_types: &[ExpressionType],
    bool_type: TypeId,
    float_type: TypeId,
    byte_type: TypeId,
    unit_type: TypeId,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();
    for expression_type in known_expression_types {
        report.record_expression_type(*expression_type);
    }

    for unary in unary_expressions {
        let Some(operand_type) = report.expression_type(unary.operand) else {
            continue;
        };
        if unary.operator == crate::parser::ParsedUnaryOperator::Not {
            if operand_type == bool_type {
                report.record_expression_type(ExpressionType::new(unary.expression, bool_type));
            } else {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    unary.operand,
                    bool_type,
                    operand_type,
                ));
            }
        } else if matches!(
            unary.operator,
            crate::parser::ParsedUnaryOperator::Plus | crate::parser::ParsedUnaryOperator::Minus
        ) && operand_type == float_type
        {
            report.record_expression_type(ExpressionType::new(unary.expression, float_type));
        }
    }

    for binary in binary_expressions {
        let (Some(left), Some(right)) = (
            report.expression_type(binary.left),
            report.expression_type(binary.right),
        ) else {
            continue;
        };
        let (expected, result) = match binary.operator {
            ParsedBinaryOperator::LogicalAnd | ParsedBinaryOperator::LogicalOr => {
                (Some(bool_type), Some(bool_type))
            }
            ParsedBinaryOperator::Equal | ParsedBinaryOperator::NotEqual
                if left == right && left != unit_type =>
            {
                (None, Some(bool_type))
            }
            ParsedBinaryOperator::Plus
            | ParsedBinaryOperator::Minus
            | ParsedBinaryOperator::Star
            | ParsedBinaryOperator::Slash
                if left == float_type && right == float_type =>
            {
                (Some(float_type), Some(float_type))
            }
            ParsedBinaryOperator::Plus
            | ParsedBinaryOperator::Minus
            | ParsedBinaryOperator::Star
            | ParsedBinaryOperator::Slash
                if left == float_type || right == float_type =>
            {
                (Some(float_type), Some(float_type))
            }
            ParsedBinaryOperator::Less
            | ParsedBinaryOperator::Greater
            | ParsedBinaryOperator::LessEqual
            | ParsedBinaryOperator::GreaterEqual
                if left == float_type && right == float_type =>
            {
                (Some(float_type), Some(bool_type))
            }
            ParsedBinaryOperator::Less
            | ParsedBinaryOperator::Greater
            | ParsedBinaryOperator::LessEqual
            | ParsedBinaryOperator::GreaterEqual
                if left == float_type || right == float_type =>
            {
                (Some(float_type), Some(bool_type))
            }
            ParsedBinaryOperator::Plus
            | ParsedBinaryOperator::Minus
            | ParsedBinaryOperator::Star
            | ParsedBinaryOperator::Slash
            | ParsedBinaryOperator::Percent
            | ParsedBinaryOperator::ShiftLeft
            | ParsedBinaryOperator::ShiftRight
            | ParsedBinaryOperator::BitwiseAnd
            | ParsedBinaryOperator::BitwiseOr
            | ParsedBinaryOperator::BitwiseXor
                if left == byte_type && right == byte_type =>
            {
                (Some(byte_type), Some(byte_type))
            }
            ParsedBinaryOperator::Plus
            | ParsedBinaryOperator::Minus
            | ParsedBinaryOperator::Star
            | ParsedBinaryOperator::Slash
            | ParsedBinaryOperator::Percent
            | ParsedBinaryOperator::ShiftLeft
            | ParsedBinaryOperator::ShiftRight
            | ParsedBinaryOperator::BitwiseAnd
            | ParsedBinaryOperator::BitwiseOr
            | ParsedBinaryOperator::BitwiseXor
                if left == byte_type || right == byte_type =>
            {
                (Some(byte_type), Some(byte_type))
            }
            _ => (None, None),
        };
        let Some(result) = result else {
            continue;
        };
        if let Some(expected) = expected {
            if left != expected {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    binary.left,
                    expected,
                    left,
                ));
                continue;
            }
            if right != expected {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    binary.right,
                    expected,
                    right,
                ));
                continue;
            }
        } else if left != right {
            continue;
        }
        report.record_expression_type(ExpressionType::new(binary.expression, result));
    }
    report
}

pub fn type_m0060_control_flow(
    parsed: &ParseOutput,
    known_expression_types: &[ExpressionType],
    int_type: TypeId,
    bool_type: TypeId,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();
    for expression_type in known_expression_types {
        report.record_expression_type(*expression_type);
    }
    let statement_conditionals: Vec<_> = parsed
        .if_statements
        .iter()
        .map(|statement| statement.expression)
        .collect();
    report.diagnostics.retain(|diagnostic| {
        !(diagnostic.rule() == TypeRuleDiagnostic::IfValueDeferred
            && statement_conditionals.contains(&diagnostic.node()))
    });

    for loop_statement in &parsed.for_statements {
        let Some(binding_span) = parsed.arena.node(loop_statement.body).map(|node| node.span)
        else {
            continue;
        };
        for reference in parsed.name_references.iter().filter(|reference| {
            reference.name == loop_statement.binding_name
                && binding_span.file() == reference.name_span.file()
                && binding_span.start() <= reference.name_span.start()
                && reference.name_span.end() <= binding_span.end()
        }) {
            report.record_expression_type(ExpressionType::new(reference.reference, int_type));
        }
        for expression in [loop_statement.start, loop_statement.end] {
            if let Some(actual) = report.expression_type(expression)
                && actual != int_type
            {
                report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                    expression, int_type, actual,
                ));
            }
        }
    }

    for if_statement in &parsed.if_statements {
        let Some(if_expression) = parsed
            .if_expressions
            .iter()
            .find(|expression| expression.expression == if_statement.expression)
        else {
            continue;
        };
        if let Some(actual) = report.expression_type(if_expression.condition)
            && actual != bool_type
        {
            report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                if_expression.condition,
                bool_type,
                actual,
            ));
        }
    }

    let int_unary = parsed
        .unary_expressions
        .iter()
        .filter(|expression| report.expression_type(expression.operand) == Some(int_type))
        .cloned()
        .collect::<Vec<_>>();
    let int_binary = parsed
        .binary_expressions
        .iter()
        .filter(|expression| {
            report.expression_type(expression.left) == Some(int_type)
                || report.expression_type(expression.right) == Some(int_type)
        })
        .cloned()
        .collect::<Vec<_>>();
    let int_report = type_m0028_executable_int_operators(
        &int_unary,
        &int_binary,
        &parsed.grouped_expressions,
        report.expression_types(),
        int_type,
    );
    merge_type_check_report(&mut report, int_report);
    for binary in &parsed.binary_expressions {
        let Some(left) = report.expression_type(binary.left) else {
            continue;
        };
        let Some(right) = report.expression_type(binary.right) else {
            continue;
        };
        if left == int_type
            && right == int_type
            && matches!(
                binary.operator,
                ParsedBinaryOperator::Equal
                    | ParsedBinaryOperator::NotEqual
                    | ParsedBinaryOperator::Less
                    | ParsedBinaryOperator::Greater
                    | ParsedBinaryOperator::LessEqual
                    | ParsedBinaryOperator::GreaterEqual
            )
        {
            report.record_expression_type(ExpressionType::new(binary.expression, bool_type));
        }
    }
    report
}

pub fn type_m0077_value_conditionals(
    parsed: &ParseOutput,
    known_expression_types: &[ExpressionType],
    type_arena: &TypeArena,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();
    for expression_type in known_expression_types {
        report.record_expression_type(*expression_type);
    }
    let statement_conditionals = parsed
        .if_statements
        .iter()
        .map(|statement| statement.expression)
        .collect::<Vec<_>>();
    let bool_type = type_arena
        .records()
        .iter()
        .find(|record| record.kind() == &TypeKind::Primitive(PrimitiveType::Bool))
        .map(|record| record.id());
    for conditional in &parsed.if_expressions {
        if statement_conditionals.contains(&conditional.expression) {
            continue;
        }
        let Some(bool_type) = bool_type else {
            continue;
        };
        if report.expression_type(conditional.condition) != Some(bool_type) {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ConditionalConditionNotBool,
                conditional.condition,
            ));
        }
        let Some(else_block) = conditional.else_block else {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ConditionalElseRequired,
                conditional.expression,
            ));
            continue;
        };
        let then_value = conditional_branch_value(parsed, conditional.then_block);
        let else_value = conditional_branch_value(parsed, else_block);
        let (Some(then_value), Some(else_value)) = (then_value, else_value) else {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ConditionalMissingBranchValue,
                conditional.expression,
            ));
            continue;
        };
        let (Some(then_type), Some(else_type)) = (
            report.expression_type(then_value),
            report.expression_type(else_value),
        ) else {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ConditionalMissingBranchValue,
                conditional.expression,
            ));
            continue;
        };
        if then_type != else_type {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ConditionalBranchTypeMismatch,
                conditional.expression,
            ));
            continue;
        }
        report.record_expression_type(ExpressionType::new(conditional.expression, then_type));
    }
    report
}

pub fn apply_m0077_value_conditional_results(
    target: &mut TypeCheckReport,
    source: &TypeCheckReport,
) {
    merge_type_check_report(target, source.clone());
}

fn conditional_branch_value(parsed: &ParseOutput, block: AstNodeId) -> Option<AstNodeId> {
    let block_span = parsed.arena.node(block)?.span;
    parsed
        .executable_body_statements
        .iter()
        .filter(|statement| {
            block_span.file() == statement.span.file()
                && block_span.start() <= statement.span.start()
                && statement.span.end() <= block_span.end()
                && parsed
                    .arena
                    .node(statement.statement)
                    .is_some_and(|node| node.kind == AstNodeKind::ExpressionStatement)
        })
        .filter_map(|statement| {
            parsed
                .arena
                .nodes()
                .iter()
                .filter(|node| {
                    node.span.file() == statement.span.file()
                        && node.span.start() == statement.span.start()
                        && node.span.end() <= statement.span.end()
                        && !matches!(
                            node.kind,
                            AstNodeKind::ExpressionStatement | AstNodeKind::Block
                        )
                })
                .max_by_key(|node| node.span.end())
                .map(|node| node.id)
        })
        .max_by_key(|expression| {
            parsed
                .arena
                .node(*expression)
                .map_or(0, |node| node.span.end())
        })
}

pub fn apply_m0060_control_flow_results(
    target: &mut TypeCheckReport,
    parsed: &ParseOutput,
    source: &TypeCheckReport,
) {
    let statement_conditionals: Vec<_> = parsed
        .if_statements
        .iter()
        .map(|statement| statement.expression)
        .collect();
    target.diagnostics.retain(|diagnostic| {
        !(diagnostic.rule() == TypeRuleDiagnostic::IfValueDeferred
            && statement_conditionals.contains(&diagnostic.node()))
    });
    merge_type_check_report(target, source.clone());
}

pub fn type_m0028_static_integer_diagnostics(
    literals: &[ParsedIntegerLiteral],
    grouped: &[ParsedGroupedExpression],
    unary: &[ParsedUnaryExpression],
    binaries: &[ParsedBinaryExpression],
) -> Vec<TypeCheckDiagnostic> {
    fn is_constant_expression(
        node: AstNodeId,
        literals: &[ParsedIntegerLiteral],
        grouped: &[ParsedGroupedExpression],
        unary: &[ParsedUnaryExpression],
        binaries: &[ParsedBinaryExpression],
    ) -> bool {
        if literals.iter().any(|literal| literal.expression == node) {
            return true;
        }
        if let Some(group) = grouped.iter().find(|group| group.expression == node) {
            return is_constant_expression(group.inner, literals, grouped, unary, binaries);
        }
        if let Some(expression) = unary
            .iter()
            .find(|expression| expression.expression == node)
        {
            return matches!(
                expression.operator,
                crate::parser::ParsedUnaryOperator::Plus
                    | crate::parser::ParsedUnaryOperator::Minus
                    | crate::parser::ParsedUnaryOperator::BitwiseNot
            ) && is_constant_expression(
                expression.operand,
                literals,
                grouped,
                unary,
                binaries,
            );
        }
        let Some(expression) = binaries
            .iter()
            .find(|expression| expression.expression == node)
        else {
            return false;
        };
        is_m0028_executable_int_operator(expression.operator)
            && is_constant_expression(expression.left, literals, grouped, unary, binaries)
            && is_constant_expression(expression.right, literals, grouped, unary, binaries)
    }

    fn is_min_int_magnitude(
        node: AstNodeId,
        literals: &[ParsedIntegerLiteral],
        grouped: &[ParsedGroupedExpression],
    ) -> bool {
        if let Some(literal) = literals.iter().find(|literal| literal.expression == node) {
            return literal.value == Some((i64::MAX as u64) + 1);
        }
        grouped
            .iter()
            .find(|group| group.expression == node)
            .is_some_and(|group| is_min_int_magnitude(group.inner, literals, grouped))
    }

    fn checked_power(base: i64, mut exponent: i64) -> Option<i64> {
        let mut result = 1_i64;
        let mut factor = base;
        while exponent > 0 {
            if exponent % 2 != 0 {
                result = result.checked_mul(factor)?;
            }
            exponent /= 2;
            if exponent > 0 {
                factor = factor.checked_mul(factor)?;
            }
        }
        Some(result)
    }

    fn evaluate(
        node: AstNodeId,
        literals: &[ParsedIntegerLiteral],
        grouped: &[ParsedGroupedExpression],
        unary: &[ParsedUnaryExpression],
        binaries: &[ParsedBinaryExpression],
    ) -> Result<Option<i64>, TypeRuleDiagnostic> {
        if let Some(literal) = literals.iter().find(|literal| literal.expression == node) {
            return literal
                .value
                .and_then(|value| i64::try_from(value).ok())
                .map(Some)
                .ok_or(TypeRuleDiagnostic::IntegerLiteralOutOfRange);
        }
        if let Some(group) = grouped.iter().find(|group| group.expression == node) {
            return evaluate(group.inner, literals, grouped, unary, binaries);
        }
        if let Some(expression) = unary
            .iter()
            .find(|expression| expression.expression == node)
        {
            if expression.operator == crate::parser::ParsedUnaryOperator::Minus
                && is_min_int_magnitude(expression.operand, literals, grouped)
            {
                return Ok(Some(i64::MIN));
            }
            let value = evaluate(expression.operand, literals, grouped, unary, binaries)?;
            return match (expression.operator, value) {
                (_, None) => Ok(None),
                (crate::parser::ParsedUnaryOperator::Plus, Some(value)) => Ok(Some(value)),
                (crate::parser::ParsedUnaryOperator::Minus, Some(value)) => value
                    .checked_neg()
                    .map(Some)
                    .ok_or(TypeRuleDiagnostic::IntegerOverflow),
                (crate::parser::ParsedUnaryOperator::BitwiseNot, Some(value)) => Ok(Some(!value)),
                (crate::parser::ParsedUnaryOperator::Not, Some(_)) => Ok(None),
            };
        }
        let Some(expression) = binaries
            .iter()
            .find(|expression| expression.expression == node)
        else {
            return Ok(None);
        };
        let (Some(left), Some(right)) = (
            evaluate(expression.left, literals, grouped, unary, binaries)?,
            evaluate(expression.right, literals, grouped, unary, binaries)?,
        ) else {
            return Ok(None);
        };
        let value = match expression.operator {
            ParsedBinaryOperator::Plus => left.checked_add(right),
            ParsedBinaryOperator::Minus => left.checked_sub(right),
            ParsedBinaryOperator::Star => left.checked_mul(right),
            ParsedBinaryOperator::Slash => {
                return if right == 0 {
                    Err(TypeRuleDiagnostic::DivisionByZero)
                } else {
                    left.checked_div(right)
                        .map(Some)
                        .ok_or(TypeRuleDiagnostic::IntegerOverflow)
                };
            }
            ParsedBinaryOperator::Percent => {
                return if right == 0 {
                    Err(TypeRuleDiagnostic::DivisionByZero)
                } else {
                    left.checked_rem(right)
                        .map(Some)
                        .ok_or(TypeRuleDiagnostic::IntegerOverflow)
                };
            }
            ParsedBinaryOperator::Exponent => {
                if right < 0 {
                    return Err(TypeRuleDiagnostic::NegativeExponent);
                }
                checked_power(left, right)
            }
            ParsedBinaryOperator::ShiftLeft | ParsedBinaryOperator::ShiftRight
                if !(0..64).contains(&right) =>
            {
                return Err(TypeRuleDiagnostic::InvalidShiftCount);
            }
            ParsedBinaryOperator::ShiftLeft => left.checked_shl(right as u32),
            ParsedBinaryOperator::ShiftRight => Some(left >> right),
            ParsedBinaryOperator::BitwiseAnd => Some(left & right),
            ParsedBinaryOperator::BitwiseOr => Some(left | right),
            ParsedBinaryOperator::BitwiseXor => Some(left ^ right),
            _ => return Ok(None),
        };
        value.map(Some).ok_or(TypeRuleDiagnostic::IntegerOverflow)
    }

    let expressions: Vec<_> = literals
        .iter()
        .map(|literal| literal.expression)
        .chain(grouped.iter().map(|expression| expression.expression))
        .chain(unary.iter().map(|expression| expression.expression))
        .chain(binaries.iter().map(|expression| expression.expression))
        .filter(|expression| {
            is_constant_expression(*expression, literals, grouped, unary, binaries)
        })
        .collect();
    let children: Vec<_> = grouped
        .iter()
        .filter(|expression| {
            is_constant_expression(expression.expression, literals, grouped, unary, binaries)
        })
        .map(|expression| expression.inner)
        .chain(
            unary
                .iter()
                .filter(|expression| {
                    is_constant_expression(
                        expression.expression,
                        literals,
                        grouped,
                        unary,
                        binaries,
                    )
                })
                .map(|expression| expression.operand),
        )
        .chain(
            binaries
                .iter()
                .filter(|expression| {
                    is_constant_expression(
                        expression.expression,
                        literals,
                        grouped,
                        unary,
                        binaries,
                    )
                })
                .flat_map(|expression| [expression.left, expression.right]),
        )
        .collect();

    let mut diagnostics = Vec::new();
    for expression in expressions
        .into_iter()
        .filter(|expression| !children.contains(expression))
    {
        if let Err(rule) = evaluate(expression, literals, grouped, unary, binaries) {
            diagnostics.push(TypeCheckDiagnostic::static_integer(rule, expression));
        }
    }
    diagnostics
}

pub fn type_assignment_statements(
    assignments: &[ParsedAssignmentStatement],
    known_expression_types: &[ExpressionType],
    arena: &TypeArena,
) -> TypeCheckReport {
    type_assignment_statements_with_refinements(
        assignments,
        known_expression_types,
        None,
        arena,
        false,
    )
}

pub fn type_m0019_assignment_statements(
    assignments: &[ParsedAssignmentStatement],
    known_expression_types: &[ExpressionType],
    flow_report: &TypeCheckReport,
    resolved_local_bindings: &[ResolvedLocalBinding],
    ast_arena: &AstArena,
    type_arena: &TypeArena,
) -> TypeCheckReport {
    type_assignment_statements_with_refinements(
        assignments,
        known_expression_types,
        Some((flow_report, resolved_local_bindings, ast_arena)),
        type_arena,
        true,
    )
}

pub fn type_m0019_local_declaration_initializers(
    declarations: &[ParsedLocalDeclaration],
    declaration_signatures: &[DeclarationSignature],
    known_expression_types: &[ExpressionType],
    flow_report: &TypeCheckReport,
    resolved_local_bindings: &[ResolvedLocalBinding],
    ast_arena: &AstArena,
    type_arena: &TypeArena,
) -> TypeCheckReport {
    type_m0019_local_declaration_initializers_with_region_exit_invalidations(
        declarations,
        declaration_signatures,
        known_expression_types,
        flow_report,
        resolved_local_bindings,
        None,
        ast_arena,
        type_arena,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn type_m0019_region_exit_refinement_invalidations(
    declarations: &[ParsedLocalDeclaration],
    declaration_signatures: &[DeclarationSignature],
    known_expression_types: &[ExpressionType],
    flow_report: &TypeCheckReport,
    resolved_local_bindings: &[ResolvedLocalBinding],
    if_expressions: &[ParsedIfExpression],
    ast_arena: &AstArena,
    type_arena: &TypeArena,
) -> TypeCheckReport {
    type_m0019_local_declaration_initializers_with_region_exit_invalidations(
        declarations,
        declaration_signatures,
        known_expression_types,
        flow_report,
        resolved_local_bindings,
        Some(if_expressions),
        ast_arena,
        type_arena,
    )
}

#[allow(clippy::too_many_arguments)]
fn type_m0019_local_declaration_initializers_with_region_exit_invalidations(
    declarations: &[ParsedLocalDeclaration],
    declaration_signatures: &[DeclarationSignature],
    known_expression_types: &[ExpressionType],
    flow_report: &TypeCheckReport,
    resolved_local_bindings: &[ResolvedLocalBinding],
    if_expressions: Option<&[ParsedIfExpression]>,
    ast_arena: &AstArena,
    type_arena: &TypeArena,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for signature in declaration_signatures {
        report.record_declaration_signature(*signature);
    }
    for expression_type in known_expression_types {
        report.record_expression_type(*expression_type);
    }

    for declaration in declarations {
        if declaration.annotation.is_none() {
            continue;
        }
        let Some(target_type) = declaration_signatures
            .iter()
            .find(|signature| signature.declaration() == declaration.declaration)
            .map(|signature| signature.ty())
        else {
            continue;
        };
        let Some(initializer) = declaration.initializer else {
            continue;
        };
        let Some(original_initializer_type) =
            expression_type_in(known_expression_types, initializer)
        else {
            continue;
        };
        let refined_initializer_type = valid_m0019_refined_value_type(
            initializer,
            original_initializer_type,
            Some((flow_report, resolved_local_bindings, ast_arena)),
            type_arena,
        );
        let effective_initializer_type =
            refined_initializer_type.unwrap_or(original_initializer_type);

        if assignment_compatible(target_type, effective_initializer_type, type_arena) {
            report.record_assignment_check(AssignmentCheck::new(
                declaration.declaration,
                target_type,
                effective_initializer_type,
            ));
        } else if refined_initializer_type.is_none()
            && exact_m0019_nullable_name_initializer(
                initializer,
                original_initializer_type,
                target_type,
                resolved_local_bindings,
                ast_arena,
                type_arena,
            )
        {
            let diagnostic = if if_expressions.is_some_and(|if_expressions| {
                exact_m0019_region_exit_nullable_name_initializer(
                    initializer,
                    original_initializer_type,
                    target_type,
                    flow_report,
                    resolved_local_bindings,
                    if_expressions,
                    ast_arena,
                    type_arena,
                )
            }) {
                TypeCheckDiagnostic::invalidated_refinement(
                    TypeRuleDiagnostic::RegionExitInvalidatedRefinement,
                    initializer,
                    target_type,
                    original_initializer_type,
                )
            } else {
                TypeCheckDiagnostic::invalid_nullable_use(
                    TypeRuleDiagnostic::NullableAssignmentWithoutRefinement,
                    initializer,
                    target_type,
                    original_initializer_type,
                )
            };
            report.record_diagnostic(diagnostic);
        } else {
            report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                initializer,
                target_type,
                effective_initializer_type,
            ));
        }
    }

    report
}

fn type_assignment_statements_with_refinements(
    assignments: &[ParsedAssignmentStatement],
    known_expression_types: &[ExpressionType],
    flow_context: Option<(&TypeCheckReport, &[ResolvedLocalBinding], &AstArena)>,
    arena: &TypeArena,
    diagnose_nullable_use: bool,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for assignment in assignments {
        let Some(target_type) = expression_type_in(known_expression_types, assignment.target)
        else {
            continue;
        };
        let Some(original_value_type) =
            expression_type_in(known_expression_types, assignment.value)
        else {
            continue;
        };
        let refined_value_type = valid_m0019_refined_value_type(
            assignment.value,
            original_value_type,
            flow_context,
            arena,
        );
        let effective_value_type = refined_value_type.unwrap_or(original_value_type);

        if assignment_compatible(target_type, effective_value_type, arena) {
            report.record_assignment_check(AssignmentCheck::new(
                assignment.statement,
                target_type,
                effective_value_type,
            ));
        } else if diagnose_nullable_use
            && refined_value_type.is_none()
            && nullable_base_type(original_value_type, arena) == Some(target_type)
        {
            report.record_diagnostic(TypeCheckDiagnostic::invalid_nullable_use(
                TypeRuleDiagnostic::NullableAssignmentWithoutRefinement,
                assignment.value,
                target_type,
                original_value_type,
            ));
        } else {
            report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                assignment.value,
                target_type,
                effective_value_type,
            ));
        }
    }

    report
}

fn valid_m0019_refined_value_type(
    expression: AstNodeId,
    original_type: TypeId,
    flow_context: Option<(&TypeCheckReport, &[ResolvedLocalBinding], &AstArena)>,
    type_arena: &TypeArena,
) -> Option<TypeId> {
    let (flow_report, resolved_local_bindings, ast_arena) = flow_context?;
    let mut matching = flow_report
        .refined_expression_types()
        .iter()
        .filter(|entry| entry.expression() == expression);
    let refined = matching.next()?;
    if matching.next().is_some() || refined.original_nullable_type() != original_type {
        return None;
    }

    let mut matching_refinements = flow_report
        .refinements()
        .iter()
        .filter(|entry| entry.region() == refined.refinement());
    let provenance = matching_refinements.next()?;
    if matching_refinements.next().is_some()
        || provenance.original_nullable_type() != refined.original_nullable_type()
        || provenance.refined_non_null_type() != refined.refined_non_null_type()
        || provenance.binding().kind() != LocalBindingKind::Immutable
        || !m0019_expression_is_inside_refinement_region(ast_arena, expression, provenance.region())
    {
        return None;
    }

    let mut matching_resolutions = resolved_local_bindings
        .iter()
        .filter(|resolved| resolved.reference() == expression);
    let resolved = matching_resolutions.next()?;
    if matching_resolutions.next().is_some() || resolved.binding() != provenance.binding() {
        return None;
    }

    let base = nullable_base_type(original_type, type_arena)?;
    (refined.refined_non_null_type() == base).then_some(base)
}

fn exact_m0019_nullable_name_initializer(
    initializer: AstNodeId,
    original_type: TypeId,
    target_type: TypeId,
    resolved_local_bindings: &[ResolvedLocalBinding],
    ast_arena: &AstArena,
    type_arena: &TypeArena,
) -> bool {
    if ast_arena.node(initializer).map(|node| node.kind) != Some(AstNodeKind::NameExpression)
        || nullable_base_type(original_type, type_arena) != Some(target_type)
    {
        return false;
    }

    let mut matching = resolved_local_bindings
        .iter()
        .filter(|resolved| resolved.reference() == initializer);
    matching.next().is_some() && matching.next().is_none()
}

#[allow(clippy::too_many_arguments)]
fn exact_m0019_region_exit_nullable_name_initializer(
    initializer: AstNodeId,
    original_type: TypeId,
    target_type: TypeId,
    flow_report: &TypeCheckReport,
    resolved_local_bindings: &[ResolvedLocalBinding],
    if_expressions: &[ParsedIfExpression],
    ast_arena: &AstArena,
    type_arena: &TypeArena,
) -> bool {
    if ast_arena.node(initializer).map(|node| node.kind) != Some(AstNodeKind::NameExpression)
        || nullable_base_type(original_type, type_arena) != Some(target_type)
    {
        return false;
    }
    let Some(initializer_span) = ast_arena.node(initializer).map(|node| node.span) else {
        return false;
    };

    let mut matching_resolutions = resolved_local_bindings
        .iter()
        .filter(|resolved| resolved.reference() == initializer);
    let Some(resolved) = matching_resolutions.next() else {
        return false;
    };
    if matching_resolutions.next().is_some()
        || resolved.binding().kind() != LocalBindingKind::Immutable
    {
        return false;
    }

    flow_report.refinements().iter().any(|refinement| {
        if refinement.binding() != resolved.binding()
            || refinement.binding().kind() != LocalBindingKind::Immutable
            || refinement.original_nullable_type() != original_type
            || refinement.refined_non_null_type() != target_type
        {
            return false;
        }

        let Some(if_expression) = if_expressions
            .iter()
            .find(|if_expression| if_expression.condition == refinement.originating_null_test())
        else {
            return false;
        };
        let is_refined_branch = refinement.region() == if_expression.then_block
            || if_expression.else_block == Some(refinement.region());

        is_refined_branch
            && if_expression.span.file() == initializer_span.file()
            && if_expression.span.end() <= initializer_span.start()
            && m0019_immediate_containing_block(ast_arena, if_expression.span)
                == m0019_immediate_containing_block(ast_arena, initializer_span)
    })
}

fn m0019_immediate_containing_block(
    arena: &AstArena,
    span: crate::source::ByteSpan,
) -> Option<AstNodeId> {
    arena
        .nodes()
        .iter()
        .filter(|node| {
            node.kind == AstNodeKind::Block
                && node.span.file() == span.file()
                && node.span.start() <= span.start()
                && span.end() <= node.span.end()
        })
        .min_by_key(|node| node.span.end() - node.span.start())
        .map(|node| node.id)
}

fn nullable_base_type(ty: TypeId, arena: &TypeArena) -> Option<TypeId> {
    let record = arena.get(ty)?;
    let TypeKind::Nullable(nullable) = record.kind() else {
        return None;
    };
    Some(nullable.base())
}

fn expression_type_in(
    expression_types: &[ExpressionType],
    expression: AstNodeId,
) -> Option<TypeId> {
    expression_types
        .iter()
        .find(|entry| entry.expression() == expression)
        .map(|entry| entry.ty())
}

fn assignment_compatible(target: TypeId, value: TypeId, arena: &TypeArena) -> bool {
    if target == value {
        return true;
    }

    let Some(target_record) = arena.get(target) else {
        return false;
    };
    let Some(value_record) = arena.get(value) else {
        return false;
    };

    match (target_record.kind(), value_record.kind()) {
        (TypeKind::Nullable(_), TypeKind::Primitive(PrimitiveType::Null)) => true,
        (TypeKind::Nullable(nullable), _) => nullable.base() == value,
        _ => false,
    }
}

pub fn type_unsupported_m0018_expressions(arena: &AstArena) -> TypeCheckReport {
    type_unsupported_expressions(arena, None)
}

fn type_unsupported_expressions(
    arena: &AstArena,
    executable_operators: Option<(&[ParsedUnaryExpression], &[ParsedBinaryExpression])>,
) -> TypeCheckReport {
    let mut report = TypeCheckReport::new();

    for node in arena.nodes() {
        if executable_operators.is_some_and(|(unary, binary)| {
            unary.iter().any(|expression| {
                expression.expression == node.id
                    && (is_m0028_executable_int_unary_operator(expression.operator)
                        || expression.operator == crate::parser::ParsedUnaryOperator::Not)
            }) || binary.iter().any(|expression| {
                expression.expression == node.id
                    && (is_m0028_executable_int_operator(expression.operator)
                        || is_m0035_primitive_binary_operator(expression.operator))
            })
        }) {
            continue;
        }
        let Some(rule) = unsupported_expression_rule(node.kind) else {
            continue;
        };
        report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(rule, node.id));
    }

    report
}

fn is_m0035_primitive_binary_operator(operator: ParsedBinaryOperator) -> bool {
    matches!(
        operator,
        ParsedBinaryOperator::LogicalOr
            | ParsedBinaryOperator::LogicalAnd
            | ParsedBinaryOperator::Equal
            | ParsedBinaryOperator::NotEqual
            | ParsedBinaryOperator::Less
            | ParsedBinaryOperator::Greater
            | ParsedBinaryOperator::LessEqual
            | ParsedBinaryOperator::GreaterEqual
            | ParsedBinaryOperator::Plus
            | ParsedBinaryOperator::Minus
            | ParsedBinaryOperator::Star
            | ParsedBinaryOperator::Slash
            | ParsedBinaryOperator::Percent
            | ParsedBinaryOperator::BitwiseAnd
            | ParsedBinaryOperator::BitwiseOr
            | ParsedBinaryOperator::BitwiseXor
            | ParsedBinaryOperator::ShiftLeft
            | ParsedBinaryOperator::ShiftRight
    )
}

fn unsupported_expression_rule(kind: AstNodeKind) -> Option<TypeRuleDiagnostic> {
    match kind {
        AstNodeKind::CallExpression => Some(TypeRuleDiagnostic::DirectCallDeferred),
        AstNodeKind::MemberExpression => Some(TypeRuleDiagnostic::MemberExpressionDeferred),
        AstNodeKind::BinaryExpression => Some(TypeRuleDiagnostic::BinaryExpressionDeferred),
        AstNodeKind::UnaryExpression => Some(TypeRuleDiagnostic::UnaryExpressionDeferred),
        AstNodeKind::IfExpression => Some(TypeRuleDiagnostic::IfValueDeferred),
        _ => None,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn type_m0018_core(
    arena: &AstArena,
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    assignments: &[ParsedAssignmentStatement],
    resolutions: &ResolutionTable,
    local_bindings: &[LocalBinding],
) -> (TypeArena, TypeCheckReport) {
    type_core(
        arena,
        declarations,
        type_name_references,
        literals,
        None,
        grouped_expressions,
        assignments,
        resolutions,
        local_bindings,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn type_m0028_executable_core(
    arena: &AstArena,
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    integer_literals: &[ParsedIntegerLiteral],
    grouped_expressions: &[ParsedGroupedExpression],
    unary_expressions: &[ParsedUnaryExpression],
    binary_expressions: &[ParsedBinaryExpression],
    assignments: &[ParsedAssignmentStatement],
    resolutions: &ResolutionTable,
    local_bindings: &[LocalBinding],
) -> (TypeArena, TypeCheckReport) {
    let mut type_arena = TypeArena::new();
    let report = type_m0028_executable_core_in(
        &mut type_arena,
        arena,
        declarations,
        type_name_references,
        literals,
        integer_literals,
        grouped_expressions,
        unary_expressions,
        binary_expressions,
        assignments,
        resolutions,
        local_bindings,
    );
    (type_arena, report)
}

#[allow(clippy::too_many_arguments)]
pub fn type_m0028_executable_core_in(
    type_arena: &mut TypeArena,
    arena: &AstArena,
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    integer_literals: &[ParsedIntegerLiteral],
    grouped_expressions: &[ParsedGroupedExpression],
    unary_expressions: &[ParsedUnaryExpression],
    binary_expressions: &[ParsedBinaryExpression],
    assignments: &[ParsedAssignmentStatement],
    resolutions: &ResolutionTable,
    local_bindings: &[LocalBinding],
) -> TypeCheckReport {
    let mut report = type_core_with_arena(
        type_arena,
        arena,
        declarations,
        type_name_references,
        literals,
        Some(integer_literals),
        grouped_expressions,
        assignments,
        resolutions,
        local_bindings,
        Some((unary_expressions, binary_expressions)),
    );
    for diagnostic in type_m0028_static_integer_diagnostics(
        integer_literals,
        grouped_expressions,
        unary_expressions,
        binary_expressions,
    ) {
        report.record_diagnostic(diagnostic);
    }
    report
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CompileTimeEvaluationError {
    NotConstant,
    DependencyCycle,
}

pub fn validate_m0061_compile_time_constants(
    parsed: &ParseOutput,
    expression_types: &[ExpressionType],
    type_arena: &TypeArena,
    report: &mut TypeCheckReport,
) {
    for declaration_id in &parsed.const_declarations {
        let Some(declaration) = parsed
            .local_declarations
            .iter()
            .find(|declaration| declaration.declaration == *declaration_id)
        else {
            continue;
        };
        let Some(initializer) = declaration.initializer else {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ConstInitializerRequired,
                *declaration_id,
            ));
            continue;
        };
        let Some(ty) = expression_types
            .iter()
            .find(|entry| entry.expression() == initializer)
            .map(|entry| entry.ty())
        else {
            report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                TypeRuleDiagnostic::ConstInitializerNotConstant,
                initializer,
            ));
            continue;
        };

        match evaluate_m0061_constant(initializer, parsed, expression_types, type_arena) {
            Ok(value) => report.record_compile_time_constant(CompileTimeConstant::new(
                *declaration_id,
                ty,
                value,
            )),
            Err(CompileTimeEvaluationError::DependencyCycle) => {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::ConstDependencyCycle,
                    initializer,
                ));
            }
            Err(CompileTimeEvaluationError::NotConstant) => {
                report.record_diagnostic(TypeCheckDiagnostic::unsupported_type_rule(
                    TypeRuleDiagnostic::ConstInitializerNotConstant,
                    initializer,
                ));
            }
        }
    }
}

fn evaluate_m0061_constant(
    expression: AstNodeId,
    parsed: &ParseOutput,
    expression_types: &[ExpressionType],
    type_arena: &TypeArena,
) -> Result<CompileTimeValue, CompileTimeEvaluationError> {
    if let Some(literal) = parsed
        .literal_expressions
        .iter()
        .find(|literal| literal.expression == expression)
    {
        return match literal.kind {
            ParsedLiteralKind::BoolTrue => Ok(CompileTimeValue::Bool(true)),
            ParsedLiteralKind::BoolFalse => Ok(CompileTimeValue::Bool(false)),
            ParsedLiteralKind::Unit => Ok(CompileTimeValue::Unit),
            ParsedLiteralKind::Float => parsed
                .float_literals
                .iter()
                .find(|literal| literal.expression == expression)
                .and_then(|literal| literal.bits)
                .map(CompileTimeValue::Float)
                .ok_or(CompileTimeEvaluationError::NotConstant),
            ParsedLiteralKind::AcceptedInteger => parsed
                .integer_literals
                .iter()
                .find(|literal| literal.expression == expression)
                .and_then(|literal| literal.value)
                .and_then(|value| i64::try_from(value).ok())
                .map(|value| {
                    if expression_type_is(
                        expression,
                        expression_types,
                        type_arena,
                        PrimitiveType::Byte,
                    ) {
                        CompileTimeValue::Byte(value as u8)
                    } else {
                        CompileTimeValue::Int(value)
                    }
                })
                .ok_or(CompileTimeEvaluationError::NotConstant),
            ParsedLiteralKind::AcceptedString | ParsedLiteralKind::Null => {
                Err(CompileTimeEvaluationError::NotConstant)
            }
        };
    }
    if let Some(grouped) = parsed
        .grouped_expressions
        .iter()
        .find(|grouped| grouped.expression == expression)
    {
        return evaluate_m0061_constant(grouped.inner, parsed, expression_types, type_arena);
    }
    if let Some(unary) = parsed
        .unary_expressions
        .iter()
        .find(|unary| unary.expression == expression)
    {
        let operand = evaluate_m0061_constant(unary.operand, parsed, expression_types, type_arena)?;
        return evaluate_m0061_unary(unary.operator, operand);
    }
    if let Some(binary) = parsed
        .binary_expressions
        .iter()
        .find(|binary| binary.expression == expression)
    {
        let left = evaluate_m0061_constant(binary.left, parsed, expression_types, type_arena)?;
        let right = evaluate_m0061_constant(binary.right, parsed, expression_types, type_arena)?;
        return evaluate_m0061_binary(binary.operator, left, right);
    }
    if let Some(name) = parsed
        .name_references
        .iter()
        .find(|name| name.reference == expression)
        && parsed.local_binding_names.iter().any(|binding| {
            parsed.const_declarations.contains(&binding.binding) && binding.name == name.name
        })
    {
        return Err(CompileTimeEvaluationError::DependencyCycle);
    }
    Err(CompileTimeEvaluationError::NotConstant)
}

fn expression_type_is(
    expression: AstNodeId,
    expression_types: &[ExpressionType],
    type_arena: &TypeArena,
    primitive: PrimitiveType,
) -> bool {
    expression_types
        .iter()
        .find(|entry| entry.expression() == expression)
        .and_then(|entry| type_arena.get(entry.ty()))
        .is_some_and(|record| record.kind() == &TypeKind::Primitive(primitive))
}

fn evaluate_m0061_unary(
    operator: ParsedUnaryOperator,
    value: CompileTimeValue,
) -> Result<CompileTimeValue, CompileTimeEvaluationError> {
    match (operator, value) {
        (ParsedUnaryOperator::Not, CompileTimeValue::Bool(value)) => {
            Ok(CompileTimeValue::Bool(!value))
        }
        (ParsedUnaryOperator::Plus, CompileTimeValue::Int(value)) => {
            Ok(CompileTimeValue::Int(value))
        }
        (ParsedUnaryOperator::Minus, CompileTimeValue::Int(value)) => value
            .checked_neg()
            .map(CompileTimeValue::Int)
            .ok_or(CompileTimeEvaluationError::NotConstant),
        (ParsedUnaryOperator::BitwiseNot, CompileTimeValue::Int(value)) => {
            Ok(CompileTimeValue::Int(!value))
        }
        (ParsedUnaryOperator::Plus, CompileTimeValue::Float(value)) => {
            Ok(CompileTimeValue::Float(value))
        }
        (ParsedUnaryOperator::Minus, CompileTimeValue::Float(value)) => {
            Ok(CompileTimeValue::Float((-f64::from_bits(value)).to_bits()))
        }
        (ParsedUnaryOperator::BitwiseNot, CompileTimeValue::Byte(value)) => {
            Ok(CompileTimeValue::Byte(!value))
        }
        _ => Err(CompileTimeEvaluationError::NotConstant),
    }
}

fn evaluate_m0061_binary(
    operator: ParsedBinaryOperator,
    left: CompileTimeValue,
    right: CompileTimeValue,
) -> Result<CompileTimeValue, CompileTimeEvaluationError> {
    match (operator, left, right) {
        (
            ParsedBinaryOperator::LogicalAnd,
            CompileTimeValue::Bool(left),
            CompileTimeValue::Bool(right),
        ) => Ok(CompileTimeValue::Bool(left && right)),
        (
            ParsedBinaryOperator::LogicalOr,
            CompileTimeValue::Bool(left),
            CompileTimeValue::Bool(right),
        ) => Ok(CompileTimeValue::Bool(left || right)),
        (ParsedBinaryOperator::Equal, left, right) => Ok(CompileTimeValue::Bool(left == right)),
        (ParsedBinaryOperator::NotEqual, left, right) => Ok(CompileTimeValue::Bool(left != right)),
        (operator, CompileTimeValue::Int(left), CompileTimeValue::Int(right)) => {
            let value = match operator {
                ParsedBinaryOperator::Plus => left.checked_add(right),
                ParsedBinaryOperator::Minus => left.checked_sub(right),
                ParsedBinaryOperator::Star => left.checked_mul(right),
                ParsedBinaryOperator::Slash => left.checked_div(right),
                ParsedBinaryOperator::Percent => left.checked_rem(right),
                ParsedBinaryOperator::Exponent => u32::try_from(right)
                    .ok()
                    .and_then(|right| left.checked_pow(right)),
                ParsedBinaryOperator::BitwiseAnd => Some(left & right),
                ParsedBinaryOperator::BitwiseOr => Some(left | right),
                ParsedBinaryOperator::BitwiseXor => Some(left ^ right),
                ParsedBinaryOperator::ShiftLeft => u32::try_from(right)
                    .ok()
                    .and_then(|right| left.checked_shl(right)),
                ParsedBinaryOperator::ShiftRight => u32::try_from(right)
                    .ok()
                    .and_then(|right| left.checked_shr(right)),
                ParsedBinaryOperator::Less => return Ok(CompileTimeValue::Bool(left < right)),
                ParsedBinaryOperator::Greater => return Ok(CompileTimeValue::Bool(left > right)),
                ParsedBinaryOperator::LessEqual => {
                    return Ok(CompileTimeValue::Bool(left <= right));
                }
                ParsedBinaryOperator::GreaterEqual => {
                    return Ok(CompileTimeValue::Bool(left >= right));
                }
                _ => None,
            };
            value
                .map(CompileTimeValue::Int)
                .ok_or(CompileTimeEvaluationError::NotConstant)
        }
        (operator, CompileTimeValue::Float(left), CompileTimeValue::Float(right)) => {
            let left = f64::from_bits(left);
            let right = f64::from_bits(right);
            let value = match operator {
                ParsedBinaryOperator::Plus => CompileTimeValue::Float((left + right).to_bits()),
                ParsedBinaryOperator::Minus => CompileTimeValue::Float((left - right).to_bits()),
                ParsedBinaryOperator::Star => CompileTimeValue::Float((left * right).to_bits()),
                ParsedBinaryOperator::Slash => CompileTimeValue::Float((left / right).to_bits()),
                ParsedBinaryOperator::Less => CompileTimeValue::Bool(left < right),
                ParsedBinaryOperator::Greater => CompileTimeValue::Bool(left > right),
                ParsedBinaryOperator::LessEqual => CompileTimeValue::Bool(left <= right),
                ParsedBinaryOperator::GreaterEqual => CompileTimeValue::Bool(left >= right),
                _ => return Err(CompileTimeEvaluationError::NotConstant),
            };
            Ok(value)
        }
        (operator, CompileTimeValue::Byte(left), CompileTimeValue::Byte(right)) => {
            let value = match operator {
                ParsedBinaryOperator::Plus => left.checked_add(right),
                ParsedBinaryOperator::Minus => left.checked_sub(right),
                ParsedBinaryOperator::Star => left.checked_mul(right),
                ParsedBinaryOperator::Slash => left.checked_div(right),
                ParsedBinaryOperator::Percent => left.checked_rem(right),
                ParsedBinaryOperator::BitwiseAnd => Some(left & right),
                ParsedBinaryOperator::BitwiseOr => Some(left | right),
                ParsedBinaryOperator::BitwiseXor => Some(left ^ right),
                ParsedBinaryOperator::ShiftLeft => left.checked_shl(u32::from(right)),
                ParsedBinaryOperator::ShiftRight => left.checked_shr(u32::from(right)),
                _ => None,
            };
            value
                .map(CompileTimeValue::Byte)
                .ok_or(CompileTimeEvaluationError::NotConstant)
        }
        _ => Err(CompileTimeEvaluationError::NotConstant),
    }
}

#[allow(clippy::too_many_arguments)]
fn type_core(
    arena: &AstArena,
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    integer_literals: Option<&[ParsedIntegerLiteral]>,
    grouped_expressions: &[ParsedGroupedExpression],
    assignments: &[ParsedAssignmentStatement],
    resolutions: &ResolutionTable,
    local_bindings: &[LocalBinding],
    executable_operators: Option<(&[ParsedUnaryExpression], &[ParsedBinaryExpression])>,
) -> (TypeArena, TypeCheckReport) {
    let mut type_arena = TypeArena::new();
    let report = type_core_with_arena(
        &mut type_arena,
        arena,
        declarations,
        type_name_references,
        literals,
        integer_literals,
        grouped_expressions,
        assignments,
        resolutions,
        local_bindings,
        executable_operators,
    );
    (type_arena, report)
}

#[allow(clippy::too_many_arguments)]
fn type_core_with_arena(
    type_arena: &mut TypeArena,
    arena: &AstArena,
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    integer_literals: Option<&[ParsedIntegerLiteral]>,
    grouped_expressions: &[ParsedGroupedExpression],
    assignments: &[ParsedAssignmentStatement],
    resolutions: &ResolutionTable,
    local_bindings: &[LocalBinding],
    executable_operators: Option<(&[ParsedUnaryExpression], &[ParsedBinaryExpression])>,
) -> TypeCheckReport {
    let primitives = PrimitiveTypeIds::module_owned(type_arena);
    let mut report = TypeCheckReport::new();

    for declaration in declarations {
        let Some(annotation_type) =
            primitive_annotation_type(declaration, type_name_references, primitives)
        else {
            report.record_diagnostic(TypeCheckDiagnostic::unresolved_type_rule(
                TypeRuleDiagnostic::MissingAnnotationType,
                declaration.declaration,
            ));
            continue;
        };
        report.record_declaration_signature(DeclarationSignature::new(
            declaration.declaration,
            annotation_type,
        ));
    }

    let known_symbols = known_local_symbol_types(local_bindings, report.declaration_signatures());
    record_m0018_accepted_expression_types(
        literals,
        grouped_expressions,
        resolutions,
        &known_symbols,
        primitives,
        &mut report,
    );

    // A Byte annotation gives a directly written integer literal its exact
    // contextual type before operator propagation and assignment checking.
    for declaration in declarations {
        if report.declaration_signature(declaration.declaration) != Some(primitives.byte_id) {
            continue;
        }
        let Some(initializer) = declaration.initializer else {
            continue;
        };
        let Some(integer) = integer_literals.and_then(|literals| {
            literals
                .iter()
                .find(|literal| literal.expression == initializer)
        }) else {
            continue;
        };
        if integer
            .value
            .is_some_and(|value| value <= u64::from(u8::MAX))
        {
            report.replace_expression_type(ExpressionType::new(initializer, primitives.byte_id));
        }
    }

    if let Some((unary_expressions, binary_expressions)) = executable_operators {
        let int_unary_expressions: Vec<_> = unary_expressions
            .iter()
            .filter(|expression| {
                !matches!(
                    report.expression_type(expression.operand),
                    Some(ty) if ty == primitives.float_id || ty == primitives.byte_id
                )
            })
            .cloned()
            .collect();
        let int_binary_expressions: Vec<_> = binary_expressions
            .iter()
            .filter(|expression| {
                !matches!(
                    (
                        report.expression_type(expression.left),
                        report.expression_type(expression.right),
                    ),
                    (Some(left), Some(right))
                        if left == primitives.float_id
                            || left == primitives.byte_id
                            || right == primitives.float_id
                            || right == primitives.byte_id
                )
            })
            .cloned()
            .collect();
        let operator_report = type_m0028_executable_int_operators(
            &int_unary_expressions,
            &int_binary_expressions,
            grouped_expressions,
            report.expression_types(),
            primitives.int_id,
        );
        merge_type_check_report(&mut report, operator_report);
        let primitive_report = type_m0035_primitive_operators(
            unary_expressions,
            binary_expressions,
            report.expression_types(),
            primitives.bool_id,
            primitives.float_id,
            primitives.byte_id,
            primitives.unit_id,
        );
        merge_type_check_report(&mut report, primitive_report);
    }

    for declaration in declarations {
        let Some(annotation_type) = report.declaration_signature(declaration.declaration) else {
            continue;
        };
        let Some(initializer) = declaration.initializer else {
            continue;
        };
        let Some(initializer_type) = report.expression_type(initializer) else {
            continue;
        };

        if annotation_type == primitives.byte_id
            && let Some(integer_literals) = integer_literals
            && let Some(integer) = integer_literals
                .iter()
                .find(|literal| literal.expression == initializer)
        {
            match integer.value {
                Some(value) if value <= u64::from(u8::MAX) => {
                    report.record_assignment_check(AssignmentCheck::new(
                        declaration.declaration,
                        annotation_type,
                        annotation_type,
                    ));
                }
                Some(_) | None => report.record_diagnostic(TypeCheckDiagnostic::static_integer(
                    TypeRuleDiagnostic::ByteLiteralOutOfRange,
                    initializer,
                )),
            }
            continue;
        }

        if assignment_compatible(annotation_type, initializer_type, type_arena) {
            report.record_assignment_check(AssignmentCheck::new(
                declaration.declaration,
                annotation_type,
                initializer_type,
            ));
        } else {
            report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                initializer,
                annotation_type,
                initializer_type,
            ));
        }
    }

    let assignment_report =
        type_assignment_statements(assignments, report.expression_types(), type_arena);
    merge_type_check_report(&mut report, assignment_report);

    let unsupported_report = type_unsupported_expressions(arena, executable_operators);
    merge_type_check_report(&mut report, unsupported_report);

    report
}

fn merge_type_check_report(target: &mut TypeCheckReport, source: TypeCheckReport) {
    for expression_type in source.expression_types() {
        target.record_expression_type(*expression_type);
    }
    for signature in source.declaration_signatures() {
        target.record_declaration_signature(*signature);
    }
    for assignment_check in source.assignment_checks() {
        target.record_assignment_check(*assignment_check);
    }
    for diagnostic in source.diagnostics() {
        target.record_diagnostic(diagnostic.clone());
    }
}

pub fn type_m0018_accepted_expressions(
    literals: &[ParsedLiteralExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    resolutions: &ResolutionTable,
    known_symbols: &[KnownSymbolType],
) -> (TypeArena, TypeCheckReport) {
    let (arena, _primitives, report) = type_m0018_accepted_expressions_with_primitives(
        literals,
        grouped_expressions,
        resolutions,
        known_symbols,
    );
    (arena, report)
}

fn type_m0018_accepted_expressions_with_primitives(
    literals: &[ParsedLiteralExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    resolutions: &ResolutionTable,
    known_symbols: &[KnownSymbolType],
) -> (TypeArena, PrimitiveTypeIds, TypeCheckReport) {
    let mut arena = TypeArena::new();
    let primitives = PrimitiveTypeIds::insert_into(&mut arena);
    let mut report = TypeCheckReport::new();

    record_m0018_accepted_expression_types(
        literals,
        grouped_expressions,
        resolutions,
        known_symbols,
        primitives,
        &mut report,
    );

    (arena, primitives, report)
}

fn record_m0018_accepted_expression_types(
    literals: &[ParsedLiteralExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    resolutions: &ResolutionTable,
    known_symbols: &[KnownSymbolType],
    primitives: PrimitiveTypeIds,
    report: &mut TypeCheckReport,
) {
    for literal in literals {
        report.record_expression_type(ExpressionType::new(
            literal.expression,
            primitives.type_for_literal(literal_kind_from_parser(literal.kind)),
        ));
    }

    for resolved in resolutions.resolved_names() {
        let Some(known) = known_symbols
            .iter()
            .find(|known| known.symbol() == resolved.symbol())
        else {
            report.record_diagnostic(TypeCheckDiagnostic::unresolved_type_rule(
                TypeRuleDiagnostic::MissingResolvedNameType,
                resolved.reference(),
            ));
            continue;
        };
        report.record_expression_type(ExpressionType::new(resolved.reference(), known.ty()));
    }

    loop {
        let mut added_group = false;
        for grouped in grouped_expressions {
            if report.expression_type(grouped.expression).is_some() {
                continue;
            }
            let Some(inner) = report.expression_type(grouped.inner) else {
                continue;
            };
            report.record_expression_type(ExpressionType::new(grouped.expression, inner));
            added_group = true;
        }
        if !added_group {
            break;
        }
    }
}

pub fn type_m0018_local_declaration_initializers(
    declarations: &[ParsedLocalDeclaration],
    type_name_references: &[ParsedTypeNameReference],
    literals: &[ParsedLiteralExpression],
    grouped_expressions: &[ParsedGroupedExpression],
    resolutions: &ResolutionTable,
    known_symbols: &[KnownSymbolType],
) -> (TypeArena, TypeCheckReport) {
    let (arena, primitives, expression_report) = type_m0018_accepted_expressions_with_primitives(
        literals,
        grouped_expressions,
        resolutions,
        known_symbols,
    );
    let mut report = TypeCheckReport::new();

    for expression_type in expression_report.expression_types() {
        report.record_expression_type(*expression_type);
    }

    for declaration in declarations {
        let Some(annotation_type) =
            primitive_annotation_type(declaration, type_name_references, primitives)
        else {
            report.record_diagnostic(TypeCheckDiagnostic::unresolved_type_rule(
                TypeRuleDiagnostic::MissingAnnotationType,
                declaration.declaration,
            ));
            continue;
        };
        report.record_declaration_signature(DeclarationSignature::new(
            declaration.declaration,
            annotation_type,
        ));

        let Some(initializer) = declaration.initializer else {
            continue;
        };
        let Some(initializer_type) = report.expression_type(initializer) else {
            continue;
        };

        if assignment_compatible(annotation_type, initializer_type, &arena) {
            report.record_assignment_check(AssignmentCheck::new(
                declaration.declaration,
                annotation_type,
                initializer_type,
            ));
        } else {
            report.record_diagnostic(TypeCheckDiagnostic::type_mismatch(
                initializer,
                annotation_type,
                initializer_type,
            ));
        }
    }

    (arena, report)
}

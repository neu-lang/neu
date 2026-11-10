use crate::{
    ast::AstNodeId,
    module::{ModuleName, PackageNamespace},
    symbol::SymbolId,
};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeId(usize);

impl TypeId {
    pub fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NominalTypeIdentity {
    module: ModuleName,
    package: PackageNamespace,
    declaration: AstNodeId,
    symbol: SymbolId,
}

impl NominalTypeIdentity {
    pub fn new(
        module: ModuleName,
        package: PackageNamespace,
        declaration: AstNodeId,
        symbol: SymbolId,
    ) -> Self {
        Self {
            module,
            package,
            declaration,
            symbol,
        }
    }

    pub fn module(&self) -> &ModuleName {
        &self.module
    }

    pub fn package(&self) -> &PackageNamespace {
        &self.package
    }

    pub fn declaration(&self) -> AstNodeId {
        self.declaration
    }

    pub fn symbol(&self) -> SymbolId {
        self.symbol
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GenericParameterType {
    declaration: AstNodeId,
    symbol: SymbolId,
}

impl GenericParameterType {
    pub fn new(declaration: AstNodeId, symbol: SymbolId) -> Self {
        Self {
            declaration,
            symbol,
        }
    }

    pub fn declaration(&self) -> AstNodeId {
        self.declaration
    }

    pub fn symbol(&self) -> SymbolId {
        self.symbol
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GenericTypeIdentity {
    declaration: NominalTypeIdentity,
    arguments: Vec<TypeId>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GenericSpecializationIdentity {
    declaration: AstNodeId,
    arguments: Vec<TypeId>,
}

impl GenericSpecializationIdentity {
    pub fn new(declaration: AstNodeId, arguments: Vec<TypeId>) -> Self {
        Self {
            declaration,
            arguments,
        }
    }

    pub fn declaration(&self) -> AstNodeId {
        self.declaration
    }

    pub fn arguments(&self) -> &[TypeId] {
        &self.arguments
    }

    pub fn mangle(&self, base: &str) -> String {
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.index().to_string())
            .collect::<Vec<_>>()
            .join("_");
        format!("{base}$g{}${arguments}", self.declaration.index())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpecializationRequest {
    New(usize),
    Existing(usize),
    Recursive,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GenericSpecializationRegistry {
    identities: Vec<GenericSpecializationIdentity>,
    active: Vec<GenericSpecializationIdentity>,
}

impl GenericSpecializationRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn request(&mut self, identity: GenericSpecializationIdentity) -> SpecializationRequest {
        if self.active.iter().any(|active| active == &identity) {
            return SpecializationRequest::Recursive;
        }
        if let Some(index) = self
            .identities
            .iter()
            .position(|existing| existing == &identity)
        {
            return SpecializationRequest::Existing(index);
        }
        let index = self.identities.len();
        self.identities.push(identity.clone());
        self.active.push(identity);
        SpecializationRequest::New(index)
    }

    pub fn finish(&mut self, identity: &GenericSpecializationIdentity) {
        if let Some(index) = self.active.iter().position(|active| active == identity) {
            self.active.remove(index);
        }
    }

    pub fn identities(&self) -> &[GenericSpecializationIdentity] {
        &self.identities
    }
}

impl GenericTypeIdentity {
    pub fn new(declaration: NominalTypeIdentity, arguments: Vec<TypeId>) -> Self {
        Self {
            declaration,
            arguments,
        }
    }

    pub fn declaration(&self) -> &NominalTypeIdentity {
        &self.declaration
    }

    pub fn arguments(&self) -> &[TypeId] {
        &self.arguments
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PrimitiveType {
    Bool,
    Int,
    String,
    Void,
    Null,
    Float,
    Byte,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NullableType {
    base: TypeId,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ArrayType {
    element: TypeId,
    length: u64,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DynamicArrayType {
    element: TypeId,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TaskType {
    result: TypeId,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ChannelType {
    element: TypeId,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ChannelResultType {
    element: TypeId,
}

impl TaskType {
    pub fn new(result: TypeId) -> Self {
        Self { result }
    }

    pub fn result(self) -> TypeId {
        self.result
    }
}

impl ChannelType {
    pub fn new(element: TypeId) -> Self {
        Self { element }
    }

    pub fn element(self) -> TypeId {
        self.element
    }
}

impl ChannelResultType {
    pub fn new(element: TypeId) -> Self {
        Self { element }
    }

    pub fn element(self) -> TypeId {
        self.element
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FunctionType {
    parameters: Vec<TypeId>,
    return_type: TypeId,
}

impl FunctionType {
    pub fn new(parameters: Vec<TypeId>, return_type: TypeId) -> Self {
        Self {
            parameters,
            return_type,
        }
    }

    pub fn parameters(&self) -> &[TypeId] {
        &self.parameters
    }

    pub fn return_type(&self) -> TypeId {
        self.return_type
    }
}

impl DynamicArrayType {
    pub fn new(element: TypeId) -> Self {
        Self { element }
    }
    pub fn element(self) -> TypeId {
        self.element
    }
}

impl ArrayType {
    pub fn new(element: TypeId, length: u64) -> Self {
        Self { element, length }
    }

    pub fn element(self) -> TypeId {
        self.element
    }

    pub fn length(self) -> u64 {
        self.length
    }
}

impl NullableType {
    pub fn new(base: TypeId) -> Self {
        Self { base }
    }

    pub fn base(self) -> TypeId {
        self.base
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TypeKind {
    Nominal(NominalTypeIdentity),
    GenericParameter(GenericParameterType),
    GenericInstance(GenericTypeIdentity),
    Function(FunctionType),
    Primitive(PrimitiveType),
    Nullable(NullableType),
    Array(ArrayType),
    DynamicArray(DynamicArrayType),
    Task(TaskType),
    Channel(ChannelType),
    ChannelResult(ChannelResultType),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsupportedTypeForm {
    VarianceAnnotation,
    WildcardOrStarProjection,
    ReceiverFunctionType,
    FunctionTypeParameterName,
    TypeAnnotationSyntax,
    TypeAlias,
    AssociatedType,
    HigherKindedType,
    DependentType,
    IntersectionType,
    UnionType,
    InferredPlaceholderType,
    LayoutType,
    EffectType,
    CoroutineSuspensionMarker,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeDiagnosticKind {
    UnsupportedTypeForm,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeDiagnostic {
    kind: TypeDiagnosticKind,
    form: Option<UnsupportedTypeForm>,
    node: AstNodeId,
}

impl TypeDiagnostic {
    pub fn unsupported_type_form(form: UnsupportedTypeForm, node: AstNodeId) -> Self {
        Self {
            kind: TypeDiagnosticKind::UnsupportedTypeForm,
            form: Some(form),
            node,
        }
    }

    pub fn kind(&self) -> TypeDiagnosticKind {
        self.kind
    }

    pub fn form(&self) -> Option<UnsupportedTypeForm> {
        self.form
    }

    pub fn node(&self) -> AstNodeId {
        self.node
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeRecord {
    id: TypeId,
    kind: TypeKind,
}

impl TypeRecord {
    pub fn nominal(identity: NominalTypeIdentity) -> Self {
        Self {
            id: TypeId::from_raw(usize::MAX),
            kind: TypeKind::Nominal(identity),
        }
    }

    pub fn generic_parameter(generic: GenericParameterType) -> Self {
        Self {
            id: TypeId::from_raw(usize::MAX),
            kind: TypeKind::GenericParameter(generic),
        }
    }

    pub fn generic_instance(generic: GenericTypeIdentity) -> Self {
        Self {
            id: TypeId::from_raw(usize::MAX),
            kind: TypeKind::GenericInstance(generic),
        }
    }

    pub fn function(function: FunctionType) -> Self {
        Self {
            id: TypeId::from_raw(usize::MAX),
            kind: TypeKind::Function(function),
        }
    }

    pub fn primitive(primitive: PrimitiveType) -> Self {
        Self {
            id: TypeId::from_raw(usize::MAX),
            kind: TypeKind::Primitive(primitive),
        }
    }

    pub fn nullable(nullable: NullableType) -> Self {
        Self {
            id: TypeId::from_raw(usize::MAX),
            kind: TypeKind::Nullable(nullable),
        }
    }

    pub fn array(array: ArrayType) -> Self {
        Self {
            id: TypeId::from_raw(usize::MAX),
            kind: TypeKind::Array(array),
        }
    }

    pub fn dynamic_array(array: DynamicArrayType) -> Self {
        Self {
            id: TypeId::from_raw(usize::MAX),
            kind: TypeKind::DynamicArray(array),
        }
    }

    pub fn task(task: TaskType) -> Self {
        Self {
            id: TypeId::from_raw(usize::MAX),
            kind: TypeKind::Task(task),
        }
    }

    pub fn channel(channel: ChannelType) -> Self {
        Self {
            id: TypeId::from_raw(usize::MAX),
            kind: TypeKind::Channel(channel),
        }
    }

    pub fn channel_result(result: ChannelResultType) -> Self {
        Self {
            id: TypeId::from_raw(usize::MAX),
            kind: TypeKind::ChannelResult(result),
        }
    }

    pub fn id(&self) -> TypeId {
        self.id
    }

    pub fn kind(&self) -> &TypeKind {
        &self.kind
    }

    fn with_id(mut self, id: TypeId) -> Self {
        self.id = id;
        self
    }
}

#[derive(Debug, Default)]
pub struct TypeArena {
    records: Vec<TypeRecord>,
    index: HashMap<TypeKind, TypeId>,
}

impl TypeArena {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, record: TypeRecord) -> TypeId {
        if let Some(id) = self.index.get(record.kind()).copied() {
            return id;
        }
        let id = TypeId::from_raw(self.records.len());
        self.index.insert(record.kind.clone(), id);
        self.records.push(record.with_id(id));
        id
    }

    pub fn array(&mut self, element: TypeId, length: u64) -> TypeId {
        self.insert(TypeRecord::array(ArrayType::new(element, length)))
    }

    pub fn nullable(&mut self, base: TypeId) -> TypeId {
        self.insert(TypeRecord::nullable(NullableType::new(base)))
    }

    pub fn generic_instance(&mut self, identity: GenericTypeIdentity) -> TypeId {
        self.insert(TypeRecord::generic_instance(identity))
    }

    pub fn function(&mut self, function: FunctionType) -> TypeId {
        self.insert(TypeRecord::function(function))
    }

    pub fn dynamic_array(&mut self, element: TypeId) -> TypeId {
        self.insert(TypeRecord::dynamic_array(DynamicArrayType::new(element)))
    }

    pub fn task(&mut self, result: TypeId) -> TypeId {
        self.insert(TypeRecord::task(TaskType::new(result)))
    }

    pub fn channel(&mut self, element: TypeId) -> TypeId {
        self.insert(TypeRecord::channel(ChannelType::new(element)))
    }

    pub fn channel_result(&mut self, element: TypeId) -> TypeId {
        self.insert(TypeRecord::channel_result(ChannelResultType::new(element)))
    }

    pub fn nominal(&mut self, identity: NominalTypeIdentity) -> TypeId {
        self.insert(TypeRecord::nominal(identity))
    }

    pub fn get(&self, id: TypeId) -> Option<&TypeRecord> {
        self.records.get(id.index())
    }

    pub fn records(&self) -> &[TypeRecord] {
        &self.records
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GenericSubstitution {
    mappings: Vec<(TypeId, TypeId)>,
}

impl GenericSubstitution {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, parameter: TypeId, replacement: TypeId) {
        if let Some((_, existing)) = self
            .mappings
            .iter_mut()
            .find(|(candidate, _)| *candidate == parameter)
        {
            *existing = replacement;
        } else {
            self.mappings.push((parameter, replacement));
        }
    }

    pub fn get(&self, parameter: TypeId) -> Option<TypeId> {
        self.mappings
            .iter()
            .find(|(candidate, _)| *candidate == parameter)
            .map(|(_, replacement)| *replacement)
    }

    pub fn apply(&self, ty: TypeId, arena: &mut TypeArena) -> TypeId {
        let Some(record) = arena.get(ty).cloned() else {
            return ty;
        };
        match record.kind() {
            TypeKind::GenericParameter(_) => self.get(ty).unwrap_or(ty),
            TypeKind::Nullable(nullable) => {
                let base = self.apply(nullable.base(), arena);
                arena.nullable(base)
            }
            TypeKind::Array(array) => {
                let element = self.apply(array.element(), arena);
                arena.array(element, array.length())
            }
            TypeKind::DynamicArray(array) => {
                let element = self.apply(array.element(), arena);
                arena.dynamic_array(element)
            }
            TypeKind::Task(task) => {
                let result = self.apply(task.result(), arena);
                arena.task(result)
            }
            TypeKind::Channel(channel) => {
                let element = self.apply(channel.element(), arena);
                arena.channel(element)
            }
            TypeKind::ChannelResult(result) => {
                let element = self.apply(result.element(), arena);
                arena.channel_result(element)
            }
            TypeKind::GenericInstance(instance) => {
                let arguments = instance
                    .arguments()
                    .iter()
                    .map(|argument| self.apply(*argument, arena))
                    .collect();
                arena.generic_instance(GenericTypeIdentity::new(
                    instance.declaration().clone(),
                    arguments,
                ))
            }
            TypeKind::Function(function) => {
                let parameters = function
                    .parameters()
                    .iter()
                    .map(|parameter| self.apply(*parameter, arena))
                    .collect();
                let return_type = self.apply(function.return_type(), arena);
                arena.function(FunctionType::new(parameters, return_type))
            }
            TypeKind::Nominal(_) | TypeKind::Primitive(_) => ty,
        }
    }
}

use crate::{
    ast::AstNodeId,
    module::{ModuleName, PackageNamespace},
    symbol::SymbolId,
};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveType {
    Bool,
    Int,
    String,
    Unit,
    Null,
    Float,
    Byte,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeKind {
    Nominal(NominalTypeIdentity),
    GenericParameter(GenericParameterType),
    GenericInstance(GenericTypeIdentity),
    Primitive(PrimitiveType),
    Nullable(NullableType),
    Array(ArrayType),
    DynamicArray(DynamicArrayType),
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
}

impl TypeArena {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, record: TypeRecord) -> TypeId {
        let id = TypeId::from_raw(self.records.len());
        self.records.push(record.with_id(id));
        id
    }

    pub fn array(&mut self, element: TypeId, length: u64) -> TypeId {
        let kind = TypeKind::Array(ArrayType::new(element, length));
        if let Some(record) = self.records.iter().find(|record| record.kind() == &kind) {
            return record.id();
        }
        self.insert(TypeRecord::array(ArrayType::new(element, length)))
    }

    pub fn nullable(&mut self, base: TypeId) -> TypeId {
        let kind = TypeKind::Nullable(NullableType::new(base));
        if let Some(record) = self.records.iter().find(|record| record.kind() == &kind) {
            return record.id();
        }
        self.insert(TypeRecord::nullable(NullableType::new(base)))
    }

    pub fn generic_instance(&mut self, identity: GenericTypeIdentity) -> TypeId {
        let kind = TypeKind::GenericInstance(identity.clone());
        if let Some(record) = self.records.iter().find(|record| record.kind() == &kind) {
            return record.id();
        }
        self.insert(TypeRecord::generic_instance(identity))
    }

    pub fn dynamic_array(&mut self, element: TypeId) -> TypeId {
        let kind = TypeKind::DynamicArray(DynamicArrayType::new(element));
        if let Some(record) = self.records.iter().find(|record| record.kind() == &kind) {
            return record.id();
        }
        self.insert(TypeRecord::dynamic_array(DynamicArrayType::new(element)))
    }

    pub fn nominal(&mut self, identity: NominalTypeIdentity) -> TypeId {
        let kind = TypeKind::Nominal(identity.clone());
        if let Some(record) = self.records.iter().find(|record| record.kind() == &kind) {
            return record.id();
        }
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
            TypeKind::Nominal(_) | TypeKind::Primitive(_) => ty,
        }
    }
}

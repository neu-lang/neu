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

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NullableType {
    base: TypeId,
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
    Nullable(NullableType),
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

    pub fn nullable(nullable: NullableType) -> Self {
        Self {
            id: TypeId::from_raw(usize::MAX),
            kind: TypeKind::Nullable(nullable),
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

    pub fn get(&self, id: TypeId) -> Option<&TypeRecord> {
        self.records.get(id.index())
    }

    pub fn records(&self) -> &[TypeRecord] {
        &self.records
    }
}

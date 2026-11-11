//! Type checking, grouped by area. Each submodule types one kind of construct.
//! The submodules are re-exported here, so paths such as `type_check::X` do not
//! change.

use crate::{
    ast::{AstArena, AstNodeId, AstNodeKind},
    module::{ModuleName, PackageNamespace},
    name_resolution::{LocalBinding, LocalBindingKind, ResolutionTable, ResolvedLocalBinding},
    ownership::{OwnershipCategory, classify_ownership_category},
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
    thread::{ThreadCapability, satisfies_thread_capability},
    types::{
        FunctionType, GenericParameterType, GenericSubstitution, GenericTypeIdentity,
        PrimitiveType, TypeArena, TypeId, TypeKind, TypeRecord,
    },
};
use std::collections::HashMap;

mod arrays;
mod calls;
mod classes;
mod concurrency;
mod diagnostics;
mod expressions;
mod flow;
mod generics;
mod intrinsics;
mod members;
mod operators;
mod returns;

pub use arrays::*;
pub use calls::*;
pub use classes::*;
pub use concurrency::*;
pub use diagnostics::*;
pub use expressions::*;
pub use flow::*;
pub use generics::*;
pub use intrinsics::*;
pub use members::*;
pub use operators::*;
pub use returns::*;

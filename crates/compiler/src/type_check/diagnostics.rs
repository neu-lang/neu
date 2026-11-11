//! Type-check diagnostic kinds and their rule names.

use super::*;

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
    ArrayIterationTargetInvalid,
    ArrayIterationStructuralMutation,
    ImmutableArrayMutation,
    StringIndexTypeMismatch,
    StringIndexOutOfBounds,
    StringOperationUnsupported,
    LambdaCaptureUnsupported,
    LambdaParameterTypeRequired,
    SpawnOutsideScope,
    AwaitOutsideSuspend,
    AwaitVoidResult,
    InvalidTaskOperation,
    InvalidChannelOperation,
    ChannelOpOutsideSuspend,
    InferredInitializerRequired,
    InferredTypeUnavailable,
    StaticFunctionInstanceAccess,
    StaticFunctionOverride,
    AbstractDeclarationInvalid,
    DuplicateField,
    FieldHiding,
    ImmutableFieldMutation,
    WhenSubjectInvalid,
    WhenPatternInvalid,
    WhenNonExhaustive,
    WhenDuplicatePattern,
    WhenArmTypeMismatch,
    WhenMissingArmValue,
}

impl PartialEq<AmbiguousTypeRule> for TypeRuleDiagnostic {
    fn eq(&self, other: &AmbiguousTypeRule) -> bool {
        matches!(self, Self::Ambiguous(rule) if rule == other)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypeCheckDiagnostic {
    pub(crate) kind: TypeCheckDiagnosticKind,
    pub(crate) rule: TypeRuleDiagnostic,
    pub(crate) node: AstNodeId,
    pub(crate) expected_type: Option<TypeId>,
    pub(crate) actual_type: Option<TypeId>,
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

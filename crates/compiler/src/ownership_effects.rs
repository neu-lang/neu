use crate::{
    ast::AstNodeId,
    borrow::BorrowKind,
    name_resolution::{LocalBinding, LocalBindingKind},
    parser::ParseOutput,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EffectKind {
    Read,
    Mutate,
    Consume,
    Store,
    ReturnOwned,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BindingState {
    Available,
    Consumed,
    MaybeConsumed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EffectEvent {
    Use {
        node: AstNodeId,
        binding: LocalBinding,
        effect: EffectKind,
        move_only: bool,
    },
    ConsumeAndRebind {
        node: AstNodeId,
        binding: LocalBinding,
    },
    Branch {
        node: AstNodeId,
        then_events: Vec<EffectEvent>,
        else_events: Vec<EffectEvent>,
    },
    Loop {
        node: AstNodeId,
        body: Vec<EffectEvent>,
    },
}

impl EffectEvent {
    pub fn use_value(node: AstNodeId, binding: LocalBinding, effect: EffectKind) -> Self {
        Self::Use {
            node,
            binding,
            effect,
            move_only: true,
        }
    }

    pub fn use_copyable(node: AstNodeId, binding: LocalBinding, effect: EffectKind) -> Self {
        Self::Use {
            node,
            binding,
            effect,
            move_only: false,
        }
    }

    pub fn consume_and_rebind(node: AstNodeId, binding: LocalBinding) -> Self {
        Self::ConsumeAndRebind { node, binding }
    }

    pub fn branch(
        node: AstNodeId,
        then_events: Vec<EffectEvent>,
        else_events: Vec<EffectEvent>,
    ) -> Self {
        Self::Branch {
            node,
            then_events,
            else_events,
        }
    }

    pub fn loop_body(node: AstNodeId, body: Vec<EffectEvent>) -> Self {
        Self::Loop { node, body }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OwnershipEffectDiagnosticKind {
    UseAfterConsumption,
    PossibleUseAfterConsumption,
    InvalidConsumingCall,
    NonRebindableConsumedValue,
    MissingEffectMetadata,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OwnershipEffectDiagnostic {
    kind: OwnershipEffectDiagnosticKind,
    node: AstNodeId,
    origin: Option<AstNodeId>,
}

impl OwnershipEffectDiagnostic {
    fn new(
        kind: OwnershipEffectDiagnosticKind,
        node: AstNodeId,
        origin: Option<AstNodeId>,
    ) -> Self {
        Self { kind, node, origin }
    }

    pub fn kind(self) -> OwnershipEffectDiagnosticKind {
        self.kind
    }

    pub fn node(self) -> AstNodeId {
        self.node
    }

    pub fn origin(self) -> Option<AstNodeId> {
        self.origin
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnershipEffectReport {
    states: Vec<(LocalBinding, BindingState)>,
    diagnostics: Vec<OwnershipEffectDiagnostic>,
}

impl OwnershipEffectReport {
    pub fn state(&self, index: usize) -> Option<BindingState> {
        self.states.get(index).map(|(_, state)| *state)
    }

    pub fn diagnostics(&self) -> &[OwnershipEffectDiagnostic] {
        &self.diagnostics
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParameterEffect {
    parameter: usize,
    effects: Vec<EffectKind>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectProjectionRegion {
    node: AstNodeId,
    binding: LocalBinding,
    projection: Vec<AstNodeId>,
    kind: BorrowKind,
}

impl EffectProjectionRegion {
    pub fn new(
        node: AstNodeId,
        binding: LocalBinding,
        projection: Vec<AstNodeId>,
        kind: BorrowKind,
    ) -> Self {
        Self {
            node,
            binding,
            projection,
            kind,
        }
    }

    pub fn node(&self) -> AstNodeId {
        self.node
    }

    pub fn binding(&self) -> &LocalBinding {
        &self.binding
    }

    pub fn projection(&self) -> &[AstNodeId] {
        &self.projection
    }

    pub fn kind(&self) -> BorrowKind {
        self.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnershipEffectContract {
    function: AstNodeId,
    parameters: Vec<ParameterEffect>,
    path_sensitive: bool,
    regions: Vec<EffectProjectionRegion>,
}

impl OwnershipEffectContract {
    pub fn new(
        function: AstNodeId,
        parameters: Vec<ParameterEffect>,
        path_sensitive: bool,
    ) -> Self {
        Self {
            function,
            parameters,
            path_sensitive,
            regions: Vec::new(),
        }
    }

    pub fn function(&self) -> AstNodeId {
        self.function
    }

    pub fn parameters(&self) -> &[ParameterEffect] {
        &self.parameters
    }

    pub fn is_path_sensitive(&self) -> bool {
        self.path_sensitive
    }

    pub fn with_regions(mut self, regions: Vec<EffectProjectionRegion>) -> Self {
        self.regions = regions;
        self
    }

    pub fn regions(&self) -> &[EffectProjectionRegion] {
        &self.regions
    }
}

impl ParameterEffect {
    pub fn parameter(&self) -> usize {
        self.parameter
    }

    pub fn effect(&self) -> EffectKind {
        self.effects
            .iter()
            .copied()
            .max()
            .unwrap_or(EffectKind::Read)
    }

    pub fn effects(&self) -> &[EffectKind] {
        &self.effects
    }

    pub fn contains(&self, effect: EffectKind) -> bool {
        self.effects.contains(&effect)
    }
}

pub fn infer_parameter_effects(
    parameter_count: usize,
    observations: &[(usize, EffectKind)],
) -> Vec<ParameterEffect> {
    let mut effects = vec![Vec::new(); parameter_count];
    for &(parameter, effect) in observations {
        let Some(current) = effects.get_mut(parameter) else {
            continue;
        };
        if !current.contains(&effect) {
            current.push(effect);
        }
    }
    effects
        .into_iter()
        .enumerate()
        .map(|(parameter, mut effects)| {
            if effects.is_empty() {
                effects.push(EffectKind::Read);
            }
            ParameterEffect { parameter, effects }
        })
        .collect()
}

pub fn infer_source_parameter_effects(
    parsed: &ParseOutput,
    function: AstNodeId,
) -> OwnershipEffectContract {
    let parameter_count = parsed
        .function_parameters
        .iter()
        .filter(|parameter| parameter.function == function)
        .count();
    let Some(function_declaration) = parsed
        .function_declarations
        .iter()
        .find(|declaration| declaration.declaration == function)
    else {
        return OwnershipEffectContract::new(
            function,
            infer_parameter_effects(parameter_count, &[]),
            true,
        );
    };
    let Some(body) = function_declaration
        .body
        .and_then(|body| parsed.arena.node(body))
    else {
        return OwnershipEffectContract::new(
            function,
            infer_parameter_effects(parameter_count, &[]),
            true,
        );
    };
    let parameters = parsed
        .function_parameters
        .iter()
        .filter(|parameter| parameter.function == function)
        .collect::<Vec<_>>();
    let mut observations = Vec::new();
    for (index, parameter) in parameters.iter().enumerate() {
        let references = parsed.name_references.iter().filter(|reference| {
            reference.name == parameter.name
                && body.span.start() <= reference.name_span.start()
                && reference.name_span.end() <= body.span.end()
        });
        let mut observed = false;
        for reference in references {
            observed = true;
            if parsed
                .assignment_statements
                .iter()
                .any(|assignment| assignment.target == reference.reference)
            {
                observations.push((index, EffectKind::Mutate));
            } else if parsed.return_statements.iter().any(|returned| {
                returned.function == function && returned.value == Some(reference.reference)
            }) {
                observations.push((index, EffectKind::ReturnOwned));
            } else {
                observations.push((index, EffectKind::Read));
            }
        }
        if !observed {
            observations.push((index, EffectKind::Read));
        }
    }
    OwnershipEffectContract::new(
        function,
        infer_parameter_effects(parameter_count, &observations),
        true,
    )
}

pub fn analyze_effect_events(
    initial_states: &[(LocalBinding, BindingState)],
    events: &[EffectEvent],
) -> OwnershipEffectReport {
    let mut report = OwnershipEffectReport {
        states: initial_states.to_vec(),
        diagnostics: Vec::new(),
    };
    analyze_events_in_place(&mut report, events);
    report
}

fn analyze_events_in_place(report: &mut OwnershipEffectReport, events: &[EffectEvent]) {
    for event in events {
        match event {
            EffectEvent::Use {
                node,
                binding,
                effect,
                move_only,
            } => apply_use(report, *node, binding, *effect, *move_only),
            EffectEvent::ConsumeAndRebind { node, binding } => {
                let Some(index) = binding_index(report, binding) else {
                    continue;
                };
                if binding.kind() != LocalBindingKind::Var {
                    report.diagnostics.push(OwnershipEffectDiagnostic::new(
                        OwnershipEffectDiagnosticKind::NonRebindableConsumedValue,
                        *node,
                        None,
                    ));
                    continue;
                }
                report.states[index].1 = BindingState::Available;
            }
            EffectEvent::Branch {
                then_events,
                else_events,
                ..
            } => {
                let mut then_report = report.clone();
                let mut else_report = report.clone();
                analyze_events_in_place(&mut then_report, then_events);
                analyze_events_in_place(&mut else_report, else_events);
                for (index, state) in report.states.iter_mut().enumerate() {
                    state.1 = join_state(then_report.states[index].1, else_report.states[index].1);
                }
                report.diagnostics.extend(then_report.diagnostics);
                report.diagnostics.extend(else_report.diagnostics);
            }
            EffectEvent::Loop { body, .. } => {
                let mut body_report = report.clone();
                analyze_events_in_place(&mut body_report, body);
                for (index, state) in report.states.iter_mut().enumerate() {
                    state.1 = join_state(state.1, body_report.states[index].1);
                }
                report.diagnostics.extend(body_report.diagnostics);
            }
        }
    }
}

fn apply_use(
    report: &mut OwnershipEffectReport,
    node: AstNodeId,
    binding: &LocalBinding,
    effect: EffectKind,
    move_only: bool,
) {
    let Some(index) = binding_index(report, binding) else {
        return;
    };
    match report.states[index].1 {
        BindingState::Consumed => report.diagnostics.push(OwnershipEffectDiagnostic::new(
            OwnershipEffectDiagnosticKind::UseAfterConsumption,
            node,
            None,
        )),
        BindingState::MaybeConsumed => report.diagnostics.push(OwnershipEffectDiagnostic::new(
            OwnershipEffectDiagnosticKind::PossibleUseAfterConsumption,
            node,
            None,
        )),
        BindingState::Available => {}
    }
    if move_only
        && matches!(
            effect,
            EffectKind::Mutate | EffectKind::Consume | EffectKind::Store
        )
    {
        report.states[index].1 = BindingState::Consumed;
    }
}

fn binding_index(report: &OwnershipEffectReport, binding: &LocalBinding) -> Option<usize> {
    report
        .states
        .iter()
        .position(|(candidate, _)| candidate == binding)
}

fn join_state(left: BindingState, right: BindingState) -> BindingState {
    match (left, right) {
        (BindingState::Available, BindingState::Available) => BindingState::Available,
        (BindingState::Consumed, BindingState::Consumed) => BindingState::Consumed,
        _ => BindingState::MaybeConsumed,
    }
}

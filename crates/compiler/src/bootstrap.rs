#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootstrapTrap {
    IntegerOverflow,
    DivisionByZero,
    NegativeExponent,
    InvalidShiftCount,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootstrapFailure {
    UnsupportedExitValue,
    RuntimeTrap(BootstrapTrap),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BootstrapOutcome {
    Exit(u8),
    Failure {
        status: u8,
        reason: BootstrapFailure,
    },
}

pub fn map_main_result(value: i64, failure_status: u8) -> BootstrapOutcome {
    match u8::try_from(value) {
        Ok(status) => BootstrapOutcome::Exit(status),
        Err(_) => BootstrapOutcome::Failure {
            status: failure_status,
            reason: BootstrapFailure::UnsupportedExitValue,
        },
    }
}

pub fn map_trap(trap: BootstrapTrap, failure_status: u8) -> BootstrapOutcome {
    BootstrapOutcome::Failure {
        status: failure_status,
        reason: BootstrapFailure::RuntimeTrap(trap),
    }
}

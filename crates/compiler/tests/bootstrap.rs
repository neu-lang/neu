use compiler::bootstrap::{
    BootstrapFailure, BootstrapOutcome, BootstrapTrap, map_main_result, map_trap,
};

#[test]
fn m0032_maps_bootstrap_exit_boundaries() {
    assert_eq!(map_main_result(0, 1), BootstrapOutcome::Exit(0));
    assert_eq!(map_main_result(255, 1), BootstrapOutcome::Exit(255));
    assert_eq!(
        map_main_result(-1, 7),
        BootstrapOutcome::Failure {
            status: 7,
            reason: BootstrapFailure::UnsupportedExitValue,
        }
    );
    assert_eq!(
        map_main_result(256, 7),
        BootstrapOutcome::Failure {
            status: 7,
            reason: BootstrapFailure::UnsupportedExitValue,
        }
    );
}

#[test]
fn m0032_preserves_bootstrap_trap_reasons() {
    for trap in [
        BootstrapTrap::IntegerOverflow,
        BootstrapTrap::DivisionByZero,
        BootstrapTrap::NegativeExponent,
        BootstrapTrap::InvalidShiftCount,
    ] {
        assert_eq!(
            map_trap(trap, 9),
            BootstrapOutcome::Failure {
                status: 9,
                reason: BootstrapFailure::RuntimeTrap(trap),
            }
        );
    }
}

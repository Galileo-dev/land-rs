mod object;
pub use object::{spawn_rocket, RocketBody, RocketConfig, RocketEngine, RocketRoot}; // Added RocketConfig

mod control;
use crate::prelude::*;
pub use control::*;

mod fuel;

mod drag;
pub use drag::DragForce;

mod legs;
pub use legs::{
    deploy_legs_system, engine_plume_system, rcs_effect_system, LandingLeg, RcsPod, FOLDED_ANGLE,
};

pub(crate) mod prelude {
    pub(crate) use super::{
        DragForce, EngineControl, EngineControlState, EngineSettings, FOLDED_ANGLE, LandingLeg,
        RcsPod, RocketBody, RocketConfig, RocketEngine, RocketRoot,
    };
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(control::RocketControlPlugin)
        .add_systems(Startup, spawn_rocket)
        .add_systems(Update, fuel::mass_depletion_system)
        // Force assembly: measure drag first, then assemble drag + thrust onto
        // the body. All in `Update`; Rapier steps afterwards (PostUpdate).
        .add_systems(
            Update,
            (drag::aerodynamic_drag_system, drag::assemble_forces_system).chain(),
        )
        // Visual systems.
        .add_systems(
            Update,
            (
                legs::deploy_legs_system,
                legs::rcs_effect_system,
                legs::engine_plume_system,
            ),
        );
}

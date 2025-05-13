mod object;
pub use object::{spawn_rocket, RocketBody, RocketConfig, RocketEngine, RocketRoot}; // Added RocketConfig

mod control;
use crate::prelude::*;
pub use control::*;

mod fuel;

pub(crate) mod prelude {
    pub(crate) use super::{
        EngineControl, EngineControlState, EngineSettings, RocketBody, RocketConfig, RocketEngine,
        RocketRoot,
    };
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(control::RocketControlPlugin)
        .add_systems(Startup, spawn_rocket)
        .add_systems(Update, fuel::mass_depletion_system);
}

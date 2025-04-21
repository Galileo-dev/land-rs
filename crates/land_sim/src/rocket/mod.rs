mod object;
pub use object::{spawn_rocket, RocketRoot};

mod control;
use crate::prelude::*;
pub use control::*;

pub(crate) mod prelude {
    pub(crate) use super::{
        EngineControl, EngineControlState, EngineSettings, RocketControlInput, RocketInputType,
        RocketRoot,
    };
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(control::RocketControlPlugin)
        .add_systems(Startup, spawn_rocket);
}

mod object;
pub use object::spawn_rocket;

mod control;
pub use control::RocketControl;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(control::RocketControlPlugin);
}

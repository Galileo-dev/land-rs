use crate::prelude::*;

mod double_click;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, double_click::mapper);
}

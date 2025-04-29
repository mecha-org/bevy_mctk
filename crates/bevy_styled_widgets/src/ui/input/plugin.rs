use super::{init, on_input_change, update_input_colors};
use bevy::prelude::*;

pub struct StyledInputPlugin;
impl Plugin for StyledInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, init)
            .add_systems(Update, update_input_colors)
            .add_observer(on_input_change);
    }
}

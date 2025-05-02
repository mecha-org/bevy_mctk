use super::systems::{init, on_toggle_button_changed, update_toggle_button_visuals};
use bevy::prelude::*;

pub struct StyledTogglePlugin;
impl Plugin for StyledTogglePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, init);
        app.add_observer(on_toggle_button_changed);
        app.add_systems(Update, update_toggle_button_visuals);
    }
}

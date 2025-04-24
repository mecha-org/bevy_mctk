use super::systems::{on_switch_changed, update_switch_colors};
use bevy::prelude::*;

pub struct StyledSwitchPlugin;
impl Plugin for StyledSwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_toggle_button_changed);
        app.add_systems(Update, update_toggle_button_visuals);
    }
}
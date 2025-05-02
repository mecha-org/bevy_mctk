use super::systems::{on_checkbox_changed, setup_checkbox_fonts, update_checkbox_visuals};
use bevy::prelude::*;

pub struct StyledCheckboxPlugin;
impl Plugin for StyledCheckboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_checkbox_changed);
        app.add_systems(Update, setup_checkbox_fonts);
        app.add_systems(Update, update_checkbox_visuals);
    }
}

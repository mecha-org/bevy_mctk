use super::{init, on_input_change, update_input_colors};
use crate::prelude::{update_button_bg_colors, update_text_color};
use bevy::prelude::*;

pub struct StyledInputPlugin;
impl Plugin for StyledInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, init);
        app.add_systems(Update, (update_button_bg_colors, update_text_color));
        app.add_systems(Update, update_input_colors);
        app.add_observer(on_input_change);
    }
}

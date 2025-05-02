use bevy::prelude::*;

use super::{change_slider_colors, on_thumb_changed, update_slider_thumb};

pub struct StyledSliderPlugin;
impl Plugin for StyledSliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_thumb_changed);
        app.add_systems(Update, update_slider_thumb);
        app.add_systems(Update, change_slider_colors);
    }
}

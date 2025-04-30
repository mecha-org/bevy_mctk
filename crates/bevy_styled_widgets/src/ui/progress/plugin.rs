use bevy::prelude::*;

use super::update_progress_colors;

pub struct StyledProgessPlugin;
impl Plugin for StyledProgessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_progress_colors);
    }
}

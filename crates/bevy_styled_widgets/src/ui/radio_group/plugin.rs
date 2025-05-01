use super::systems::{observe_radio_group_selection, update_radio_button_visuals};
use bevy::prelude::*;

pub struct StyledRadioGroupPlugin;
impl Plugin for StyledRadioGroupPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(observe_radio_group_selection);
        app.add_systems(Update, update_radio_button_visuals);
    }
}

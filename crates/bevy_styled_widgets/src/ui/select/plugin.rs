use bevy::prelude::*;

use super::{
    on_select_item_selection, on_select_triggered, open_select_popup, update_select_visuals,
};

pub struct StyledSelectTriggerPlugin;
impl Plugin for StyledSelectTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_select_triggered)
            .add_systems(Update, open_select_popup)
            .add_observer(on_select_item_selection)
            .add_systems(Update, update_select_visuals);
    }
}

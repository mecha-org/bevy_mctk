use bevy::app::{App, Plugin};
mod core_select;
mod core_switch;
mod interaction_states;
pub use core_select::{CoreSelectContent, CoreSelectItem, CoreSelectPlugin, CoreSelectTrigger};
pub use core_switch::{CoreSwitch, CoreSwitchPlugin};

pub use interaction_states::{DropdownOpen, IsSelected, SelectedItem};
pub struct AdditionalCoreWidgetsPlugin;

impl Plugin for AdditionalCoreWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CoreSwitchPlugin, CoreSelectPlugin));
    }
}

pub mod prelude {
    pub use crate::AdditionalCoreWidgetsPlugin;
}

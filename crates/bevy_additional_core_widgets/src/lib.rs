use bevy::app::{App, Plugin, Update};
mod core_switch;

pub use core_switch::{CoreSwitch, CoreSwitchPlugin};

pub struct AdditionalCoreWidgetsPlugin;

impl Plugin for AdditionalCoreWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CoreSwitchPlugin,));
    }
}

pub mod prelude {
    pub use crate::AdditionalCoreWidgetsPlugin;
}

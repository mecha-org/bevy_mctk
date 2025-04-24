mod themes;
mod ui;

use bevy::{
    app::{App, Plugin},
    input_focus::InputDispatchPlugin,
};
use bevy_additional_core_widgets::AdditionalCoreWidgetsPlugin;
use bevy_core_widgets::CoreWidgetsPlugin;

use ui::{button::StyledButtonPlugin, switch::StyledSwitchPlugin, text::StyledTextPlugin};

pub struct StyledWidgetsPligin;

impl Plugin for StyledWidgetsPligin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CoreWidgetsPlugin,
            InputDispatchPlugin,
            StyledButtonPlugin,
            StyledTextPlugin,
            AdditionalCoreWidgetsPlugin,
            StyledSwitchPlugin,
        ));
    }
}

pub mod prelude {
    pub use crate::StyledWidgetsPligin;
    pub use crate::themes::*;
    pub use crate::ui::button::*;
    pub use crate::ui::switch::*;
    pub use crate::ui::text::*;
    pub use crate::ui::toggle::*;
}

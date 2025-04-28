mod themes;
mod ui;

use bevy::{
    app::{App, Plugin},
    input_focus::InputDispatchPlugin,
};
use bevy_additional_core_widgets::AdditionalCoreWidgetsPlugin;
use bevy_core_widgets::CoreWidgetsPlugin;

use ui::{
    button::StyledButtonPlugin, input::StyledInputPlugin, slider::StyledSliderPlugin,
    switch::StyledSwitchPlugin, text::StyledTextPlugin,
};

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
            StyledInputPlugin,
            StyledSliderPlugin,
        ));
    }
}

pub mod prelude {
    pub use crate::StyledWidgetsPligin;
    pub use crate::themes::*;
    pub use crate::ui::button::*;
    pub use crate::ui::input::*;
    pub use crate::ui::slider::*;
    pub use crate::ui::switch::*;
    pub use crate::ui::text::*;
}

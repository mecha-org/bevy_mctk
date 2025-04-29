mod themes;
mod ui;

use bevy::{
    app::{App, Plugin},
    input_focus::InputDispatchPlugin,
};
use bevy_additional_core_widgets::AdditionalCoreWidgetsPlugin;
use bevy_asset_loader::prelude::*;
use bevy_core_widgets::CoreWidgetsPlugin;

use themes::fonts::FontAssets;
use ui::{
    button::StyledButtonPlugin, checkbox::StyledCheckboxPlugin, switch::StyledSwitchPlugin,
    text::StyledTextPlugin, toggle::StyledTogglePlugin,
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
            StyledTogglePlugin,
            StyledCheckboxPlugin,
        ));
        app.init_collection::<FontAssets>();
    }
}

pub mod prelude {
    pub use crate::StyledWidgetsPligin;
    pub use crate::themes::*;
    pub use crate::ui::button::*;
    pub use crate::ui::checkbox::*;
    pub use crate::ui::switch::*;
    pub use crate::ui::text::*;
    pub use crate::ui::toggle::*;
}

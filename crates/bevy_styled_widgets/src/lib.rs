mod icon;
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
    button::StyledButtonPlugin, radio_group::StyledRadioGroupPlugin, switch::StyledSwitchPlugin,
    text::StyledTextPlugin,
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
            StyledRadioGroupPlugin,
        ));
        app.init_collection::<FontAssets>();
    }
}

#[allow(ambiguous_glob_reexports)]
pub mod prelude {
    pub use crate::StyledWidgetsPligin;
    pub use crate::icon::*;
    pub use crate::themes::*;
    pub use crate::ui::button::*;
    pub use crate::ui::radio_group::*;
    pub use crate::ui::switch::*;
    pub use crate::ui::text::*;
}

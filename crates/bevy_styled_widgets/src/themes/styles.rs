use std::collections::HashMap;

use bevy::prelude::*;

use super::{
    ThemeModeConfigs,
    button::{ButtonSizeStyles, ButtonVariantStyles, button_sizes},
    panel::PanelStyle,
    text::TextStyle,
};

#[derive(Debug, Clone)]
pub struct ThemeStyles {
    pub buttons: ButtonVariantStyles,
    pub button_sizes: ButtonSizeStyles,
    pub text: TextStyle,
    pub panel: PanelStyle,
    pub icons: HashMap<String, String>,
}

impl ThemeStyles {
    pub fn from_colors(configs: ThemeModeConfigs) -> Self {
        Self {
            buttons: ButtonVariantStyles::from_colors(configs.colors.clone()),
            button_sizes: button_sizes(),
            text: TextStyle::from_colors(configs.colors.clone()),
            panel: PanelStyle::from_colors(configs.colors.clone()),
            icons: configs.icons.clone(),
        }
    }
}

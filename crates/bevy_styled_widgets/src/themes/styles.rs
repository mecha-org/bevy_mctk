use std::collections::HashMap;

use bevy::prelude::*;

use super::{
    ThemeModeConfigs,
    button::{ButtonSizeStyles, ButtonVariantStyles, button_sizes},
    panel::PanelStyle,
    radio::{RadioButtonSizeStyles, RadioButtonVariantStyles, radio_button_sizes},
    switch::{SwitchSizeStyles, SwitchVariantStyles, switch_sizes},
    text::TextStyle,
};

#[derive(Debug, Clone)]
pub struct ThemeStyles {
    pub buttons: ButtonVariantStyles,
    pub button_sizes: ButtonSizeStyles,
    pub text: TextStyle,
    pub panel: PanelStyle,
    pub icons: HashMap<String, String>,
    pub switches: SwitchVariantStyles,
    pub switch_sizes: SwitchSizeStyles,
    pub radio_buttons: RadioButtonVariantStyles,
    pub radio_button_sizes: RadioButtonSizeStyles,
}

impl ThemeStyles {
    pub fn from_colors(configs: ThemeModeConfigs) -> Self {
        Self {
            buttons: ButtonVariantStyles::from_colors(configs.colors.clone()),
            button_sizes: button_sizes(),
            text: TextStyle::from_colors(configs.colors.clone()),
            panel: PanelStyle::from_colors(configs.colors.clone()),
            switches: SwitchVariantStyles::from_colors(configs.colors.clone()),
            switch_sizes: switch_sizes(),
            icons: configs.icons.clone(),
            radio_buttons: RadioButtonVariantStyles::from_colors(configs.colors.clone()),
            radio_button_sizes: radio_button_sizes(),
        }
    }
}

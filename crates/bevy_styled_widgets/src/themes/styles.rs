use bevy::prelude::*;

use super::{
    ThemeColors,
    button::{ButtonSizeStyles, ButtonVariantStyles, button_sizes},
    panel::PanelStyle,
    switch::{SwitchSizeStyles, SwitchVariantStyles, switch_sizes},
    text::TextStyle,
};

#[derive(Debug, Clone)]
pub struct ThemeStyles {
    pub buttons: ButtonVariantStyles,
    pub button_sizes: ButtonSizeStyles,
    pub text: TextStyle,
    pub panel: PanelStyle,
    pub switches: SwitchVariantStyles,
    pub switch_sizes: SwitchSizeStyles,
}

impl ThemeStyles {
    pub fn from_colors(colors: ThemeColors) -> Self {
        Self {
            buttons: ButtonVariantStyles::from_colors(colors.clone()),
            button_sizes: button_sizes(),
            text: TextStyle::from_colors(colors.clone()),
            panel: PanelStyle::from_colors(colors.clone()),
            switches: SwitchVariantStyles::from_colors(colors.clone()),
            switch_sizes: switch_sizes(),
        }
    }
}

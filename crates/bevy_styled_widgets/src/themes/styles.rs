use std::collections::HashMap;

use bevy::prelude::*;

use super::{
    ThemeModeConfigs,
    button::{ButtonSizeStyles, ButtonVariantStyles, button_sizes},
    checkbox::{CheckboxSizeStyles, CheckboxVariantStyles, checkbox_sizes},
    input::InputStyle,
    panel::PanelStyle,
    progress::ProgressStyle,
    slider::SliderStyle,
    switch::{SwitchSizeStyles, SwitchVariantStyles, switch_sizes},
    text::TextStyle,
    toggle::{ToggleSizeStyles, ToggleVariantStyles, toggle_sizes},
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
    pub input: InputStyle,
    pub toggles: ToggleVariantStyles,
    pub toggle_sizes: ToggleSizeStyles,
    pub checkboxes: CheckboxVariantStyles,
    pub checkbox_sizes: CheckboxSizeStyles,
    pub progress: ProgressStyle,
    pub slider: SliderStyle,
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
            input: InputStyle::from_colors(configs.colors.clone()),
            toggles: ToggleVariantStyles::from_colors(configs.colors.clone()),
            toggle_sizes: toggle_sizes(),
            checkboxes: CheckboxVariantStyles::from_colors(configs.colors.clone()),
            checkbox_sizes: checkbox_sizes(),
            progress: ProgressStyle::from_colors(configs.colors.clone()),
            slider: SliderStyle::from_colors(configs.colors.clone()),
        }
    }
}

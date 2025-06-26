use bevy::prelude::*;

use super::ThemeColors;

#[derive(Debug, Clone)]
pub struct PopupStyle {
    pub background_color: Color,
}

impl PopupStyle {
    pub fn from_colors(colors: ThemeColors) -> Self {
        Self {
            background_color: colors.foreground.with_alpha(0.7),
        }
    }
}

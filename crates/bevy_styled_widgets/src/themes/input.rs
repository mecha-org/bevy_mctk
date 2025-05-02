use bevy::prelude::*;

use super::ThemeColors;

#[derive(Debug, Clone)]
pub struct InputStyle {
    pub text_color: Color,
    pub background_color: Color,
    pub border_color: Color,
}
impl InputStyle {
    pub fn from_colors(colors: ThemeColors) -> Self {
        Self {
            text_color: colors.foreground,
            background_color: colors.background,
            border_color: colors.border,
        }
    }
}

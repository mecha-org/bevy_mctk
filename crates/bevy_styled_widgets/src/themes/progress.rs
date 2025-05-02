use bevy::prelude::*;

use super::ThemeColors;

#[derive(Debug, Clone, Default)]
pub struct ProgressStyle {
    pub root_color: Color,
    pub indicator_color: Color,
}
impl ProgressStyle {
    pub fn from_colors(colors: ThemeColors) -> Self {
        Self {
            root_color: colors.primary_foreground,
            indicator_color: colors.primary,
        }
    }
}

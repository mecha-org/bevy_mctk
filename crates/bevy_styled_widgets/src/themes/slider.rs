use bevy::prelude::*;

use super::ThemeColors;

#[derive(Debug, Clone, Default)]
pub struct SliderStyle {
    pub track_color: Color,
    pub thumb_color: Color,
    pub filled_color: Color,
    pub icon_color: Color,
}
impl SliderStyle {
    pub fn from_colors(colors: ThemeColors) -> Self {
        Self {
            track_color: colors.primary.with_alpha(0.5),
            thumb_color: colors.primary,
            filled_color: colors.foreground.with_alpha(0.8),
            icon_color: colors.primary,
        }
    }
}

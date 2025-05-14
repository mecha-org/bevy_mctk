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

// Input size properties
#[derive(Debug, Clone)]
pub struct InputSizeProperties {
    pub min_width: f32,
    pub min_height: f32,
    pub font_size: f32,
    pub line_height: f32,
}

// Collection of size variants
#[derive(Debug, Clone)]
pub struct InputSizeStyles {
    pub xsmall: InputSizeProperties,
    pub small: InputSizeProperties,
    pub medium: InputSizeProperties,
    pub large: InputSizeProperties,
    pub xlarge: InputSizeProperties,
}

pub fn input_sizes() -> InputSizeStyles {
    InputSizeStyles {
        xsmall: InputSizeProperties {
            min_width: 60.0,
            min_height: 36.0,
            font_size: 12.0,
            line_height: 16.0,
        },
        small: InputSizeProperties {
            min_width: 80.0,
            min_height: 36.0,
            font_size: 14.0,
            line_height: 18.0,
        },
        medium: InputSizeProperties {
            min_width: 100.0, // 200.0,
            min_height: 36.0, // 40.0,
            font_size: 15.0,
            line_height: 20.0,
        },
        large: InputSizeProperties {
            min_width: 120.0,
            min_height: 56.0,
            font_size: 16.0,
            line_height: 22.0,
        },
        xlarge: InputSizeProperties {
            min_width: 144.0,
            min_height: 56.0,
            font_size: 18.0,
            line_height: 24.0,
        },
    }
}

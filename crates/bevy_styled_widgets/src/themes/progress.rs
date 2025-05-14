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

// Progress size properties
#[derive(Debug, Clone)]
pub struct ProgressSizeProperties {
    pub min_width: f32,
    pub min_height: f32,
}

// Collection of size variants
#[derive(Debug, Clone)]
pub struct ProgressSizeStyles {
    pub xsmall: ProgressSizeProperties,
    pub small: ProgressSizeProperties,
    pub medium: ProgressSizeProperties,
    pub large: ProgressSizeProperties,
    pub xlarge: ProgressSizeProperties,
}

pub fn progress_sizes() -> ProgressSizeStyles {
    ProgressSizeStyles {
        xsmall: ProgressSizeProperties {
            min_width: 56.0,
            min_height: 12.0,
        },
        small: ProgressSizeProperties {
            min_width: 80.0,
            min_height: 12.0,
        },
        medium: ProgressSizeProperties {
            min_width: 100.0,
            min_height: 12.0,
        },
        large: ProgressSizeProperties {
            min_width: 120.0,
            min_height: 12.0,
        },
        xlarge: ProgressSizeProperties {
            min_width: 144.0,
            min_height: 12.0,
        },
    }
}

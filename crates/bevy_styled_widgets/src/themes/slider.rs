use bevy::prelude::*;

use super::ThemeColors;

#[derive(Debug, Clone, Default)]
pub struct SliderStyle {
    pub track_color: Color,
    pub thumb_color: Color,
    pub disabled_knob_color: Color,
}
impl SliderStyle {
    pub fn from_colors(colors: ThemeColors) -> Self {
        Self {
            track_color: colors.primary.with_alpha(0.5),
            thumb_color: colors.primary,
            disabled_knob_color: colors.primary.with_alpha(0.5), // Placeholder for disabled knob color
                                                                 // disabled_knob_color: colors.background.with_alpha(alphas.disabled_knob_alpha),
        }
    }
}

// Slider size properties
#[derive(Debug, Clone)]
pub struct SliderSizeProperties {
    pub min_width: f32,
    pub min_height: f32,
}

// Collection of size variants
#[derive(Debug, Clone)]
pub struct SliderSizeStyles {
    pub xsmall: SliderSizeProperties,
    pub small: SliderSizeProperties,
    pub medium: SliderSizeProperties,
    pub large: SliderSizeProperties,
    pub xlarge: SliderSizeProperties,
}

pub fn slider_sizes() -> SliderSizeStyles {
    SliderSizeStyles {
        xsmall: SliderSizeProperties {
            min_width: 56.0,
            min_height: 12.0,
        },
        small: SliderSizeProperties {
            min_width: 80.0,
            min_height: 12.0,
        },
        medium: SliderSizeProperties {
            min_width: 100.0,
            min_height: 12.0,
        },
        large: SliderSizeProperties {
            min_width: 120.0,
            min_height: 12.0,
        },
        xlarge: SliderSizeProperties {
            min_width: 144.0,
            min_height: 12.0,
        },
    }
}

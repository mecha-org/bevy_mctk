use bevy::prelude::*;

use super::ThemeColors;

#[derive(Debug, Clone)]
pub struct RadioButtonStyle {
    // Colors
    pub outer_border: Color,
    pub disabled_outer_border: Color,
    pub hovered_outer_border: Color,
    pub hovered_inner_checked_background: Color,
    pub hovered_inner_unchecked_background: Color,
    pub inner_checked_background: Color,
    pub inner_unchecked_background: Color,
    pub disabled_inner_unchecked_background: Color,
    pub disabled_inner_checked_background: Color,
    pub caption_color: Color,
    // Transitions
    pub transition_duration: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct RadioButtonStyleAlphas {
    pub disabled_outer_alpha: f32,
    pub disabled_inner_checked_alpha: f32,
    pub hovered_alpha: f32,
}

#[derive(Debug, Clone)]
pub struct RadioButtonVariantStyles {
    pub default: RadioButtonStyle,
}

impl RadioButtonVariantStyles {
    pub fn from_colors(colors: ThemeColors) -> Self {
        let alphas = RadioButtonStyleAlphas {
            disabled_outer_alpha: 0.5,
            disabled_inner_checked_alpha: 0.5,
            hovered_alpha: 0.5,
        };

        Self {
            default: RadioButtonStyle {
                outer_border: colors.primary,
                disabled_outer_border: colors.primary.with_alpha(alphas.disabled_outer_alpha),
                hovered_outer_border: colors.primary.with_alpha(alphas.hovered_alpha),
                hovered_inner_checked_background: colors.primary.with_alpha(alphas.hovered_alpha),
                hovered_inner_unchecked_background: Color::NONE,
                inner_checked_background: colors.primary,
                inner_unchecked_background: Color::NONE,
                disabled_inner_unchecked_background: Color::NONE,
                disabled_inner_checked_background: colors
                    .primary
                    .with_alpha(alphas.disabled_inner_checked_alpha),
                caption_color: colors.foreground,
                transition_duration: 0.2,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct RadioButtonSizeProperties {
    pub outer_border_width: f32,
    pub outer_corner_radius: f32,
    pub outer_width: f32,
    pub outer_height: f32,
    pub inner_circle_width: f32,
    pub inner_circle_height: f32,
    pub inner_circle_left: f32,
    pub inner_circle_top: f32,
    pub inner_circle_corner_radius: f32,
    pub caption_font_size: f32,
}

// Collection of size variants for Radio button
#[derive(Debug, Clone)]
pub struct RadioButtonSizeStyles {
    pub xsmall: RadioButtonSizeProperties,
    pub small: RadioButtonSizeProperties,
    pub medium: RadioButtonSizeProperties,
    pub large: RadioButtonSizeProperties,
    pub xlarge: RadioButtonSizeProperties,
}

pub fn radio_button_sizes() -> RadioButtonSizeStyles {
    RadioButtonSizeStyles {
        xsmall: RadioButtonSizeProperties {
            outer_border_width: 1.0,
            outer_corner_radius: 50.0,
            outer_width: 16.0,
            outer_height: 16.0,
            inner_circle_width: 10.0,
            inner_circle_height: 10.0,
            inner_circle_left: 2.0,
            inner_circle_top: 2.0,
            inner_circle_corner_radius: 50.0,
            caption_font_size: 14.0,
        },
        small: RadioButtonSizeProperties {
            outer_border_width: 1.0,
            outer_corner_radius: 50.0,
            outer_width: 18.0,
            outer_height: 18.0,
            inner_circle_width: 12.0,
            inner_circle_height: 12.0,
            inner_circle_left: 2.0,
            inner_circle_top: 2.0,
            inner_circle_corner_radius: 50.0,
            caption_font_size: 14.0,
        },
        medium: RadioButtonSizeProperties {
            outer_border_width: 1.0,
            outer_corner_radius: 50.0,
            outer_width: 20.0,
            outer_height: 20.0,
            inner_circle_width: 14.0,
            inner_circle_height: 14.0,
            inner_circle_left: 2.0,
            inner_circle_top: 2.0,
            inner_circle_corner_radius: 50.0,
            caption_font_size: 14.0,
        },
        large: RadioButtonSizeProperties {
            outer_border_width: 1.0,
            outer_corner_radius: 50.0,
            outer_width: 28.0,
            outer_height: 28.0,
            inner_circle_width: 22.0,
            inner_circle_height: 22.0,
            inner_circle_left: 2.0,
            inner_circle_top: 2.0,
            inner_circle_corner_radius: 50.0,
            caption_font_size: 14.0,
        },
        xlarge: RadioButtonSizeProperties {
            outer_border_width: 1.0,
            outer_corner_radius: 50.0,
            outer_width: 38.0,
            outer_height: 38.0,
            inner_circle_width: 32.0,
            inner_circle_height: 32.0,
            inner_circle_left: 2.0,
            inner_circle_top: 2.0,
            inner_circle_corner_radius: 50.0,
            caption_font_size: 14.0,
        },
    }
}

use bevy::prelude::*;

use super::ThemeColors;

#[derive(Debug, Clone)]
pub struct SwitchStyle {
    // Colors
    pub on_background: Color,
    pub off_background: Color,
    pub hovered_background: Color,
    pub on_text_color: Color,
    pub off_text_color: Color,
    pub border_color: Color,
    pub knob_color: Color,
    pub disabled_knob_color: Color,
    pub disabled_on_background: Color,
    pub disabled_off_background: Color,

    // Dimensions
    pub border_width: f32,

    // Transitions
    pub transition_duration: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct SwitchStyleAlphas {
    pub disabled_knob_alpha: f32,
    pub disabled_on_alpha: f32,
    pub disabled_off_alpha: f32,
    pub hovered_alpha: f32,
}

#[derive(Debug, Clone)]
pub struct SwitchVariantStyles {
    pub rounded: SwitchStyle,
    pub rectangular: SwitchStyle,
}

impl SwitchVariantStyles {
    pub fn from_colors(colors: ThemeColors) -> Self {
        let alphas = SwitchStyleAlphas {
            disabled_knob_alpha: 10.0,
            disabled_on_alpha: 0.5,
            disabled_off_alpha: 0.5,
            hovered_alpha: 0.5,
        };

        Self {
            rounded: SwitchStyle {
                on_background: colors.primary,
                off_background: colors.input,
                knob_color: colors.background,
                disabled_knob_color: colors.background.with_alpha(alphas.disabled_knob_alpha),
                disabled_on_background: colors.primary.with_alpha(alphas.disabled_on_alpha),
                disabled_off_background: colors.input.with_alpha(alphas.disabled_off_alpha),
                hovered_background: colors.primary.with_alpha(alphas.hovered_alpha),
                on_text_color: colors.primary_foreground,
                off_text_color: colors.foreground,
                border_color: Color::NONE,
                border_width: 0.0,
                transition_duration: 0.2,
            },
            rectangular: SwitchStyle {
                on_background: colors.primary,
                off_background: colors.input,
                knob_color: colors.background,
                disabled_knob_color: colors.background.with_alpha(alphas.disabled_knob_alpha),
                disabled_on_background: colors.primary.with_alpha(alphas.disabled_on_alpha),
                disabled_off_background: colors.input.with_alpha(alphas.disabled_off_alpha),
                hovered_background: colors.primary.with_alpha(alphas.hovered_alpha),
                on_text_color: colors.primary_foreground,
                off_text_color: colors.foreground,
                border_color: Color::NONE,
                border_width: 0.0,
                transition_duration: 0.2,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct SwitchSizeProperties {
    pub track_border_width: f32,
    pub track_corner_radius: f32,
    pub track_width: f32,
    pub track_height: f32,
    pub knob_width: f32,
    pub knob_height: f32,
    pub knob_offset_x_on: f32,
    pub knob_offset_x: f32,
    pub knob_corner_radius: f32,
    pub label_font_size: f32,
    pub label_offset_on: f32,
    pub label_offset: f32,
}

// Collection of size variants for Switch
#[derive(Debug, Clone)]
pub struct SwitchSizeStyles {
    pub xsmall: SwitchSizeProperties,
    pub small: SwitchSizeProperties,
    pub medium: SwitchSizeProperties,
    pub large: SwitchSizeProperties,
    pub xlarge: SwitchSizeProperties,
}

pub fn switch_sizes() -> SwitchSizeStyles {
    SwitchSizeStyles {
        xsmall: SwitchSizeProperties {
            track_border_width: 0.0,
            track_corner_radius: 6.0,
            track_width: 24.0,
            track_height: 12.0,
            knob_width: 10.0,
            knob_height: 10.0,
            knob_corner_radius: 5.0,
            knob_offset_x: 1.0,
            knob_offset_x_on: 13.0,
            label_font_size: 6.0,
            label_offset_on: 1.0,
            label_offset: 12.0,
        },
        small: SwitchSizeProperties {
            track_border_width: 0.0,
            track_corner_radius: 8.0,
            track_width: 32.0,
            track_height: 16.0,
            knob_width: 12.0,
            knob_height: 12.0,
            knob_corner_radius: 6.0,
            knob_offset_x: 2.0,
            knob_offset_x_on: 18.0,
            label_font_size: 8.0,
            label_offset_on: 2.0,
            label_offset: 17.0,
        },
        medium: SwitchSizeProperties {
            track_border_width: 0.0,
            track_corner_radius: 10.0,
            track_width: 40.0,
            track_height: 20.0,
            knob_width: 16.0,
            knob_height: 16.0,
            knob_corner_radius: 8.0,
            knob_offset_x: 2.0,
            knob_offset_x_on: 22.0,
            label_font_size: 9.0,
            label_offset_on: 3.0,
            label_offset: 22.0,
        },
        large: SwitchSizeProperties {
            track_border_width: 0.0,
            track_corner_radius: 13.0,
            track_width: 48.0,
            track_height: 24.0,
            knob_width: 20.0,
            knob_height: 20.0,
            knob_corner_radius: 10.0,
            knob_offset_x: 3.0,
            knob_offset_x_on: 26.0,
            label_font_size: 10.0,
            label_offset_on: 3.0,
            label_offset: 26.0,
        },
        xlarge: SwitchSizeProperties {
            track_border_width: 0.0,
            track_corner_radius: 16.0,
            track_width: 56.0,
            track_height: 28.0,
            knob_width: 24.0,
            knob_height: 24.0,
            knob_corner_radius: 12.0,
            knob_offset_x: 3.0,
            knob_offset_x_on: 30.0,
            label_font_size: 12.0,
            label_offset_on: 3.0,
            label_offset: 30.0,
        },
    }
}

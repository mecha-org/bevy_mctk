use bevy::prelude::*;

use super::ThemeColors;

#[derive(Debug, Clone)]
pub struct ToggleStyle {
    // Colors
    pub on_background: Color,
    pub off_background: Color,
    pub hovered_background: Color,
    pub text_color: Color,
    pub border_color: Color,
    // Dimensions
    pub border_width: f32,
    // Transitions
    pub transition_duration: f32,
}

#[derive(Debug, Clone)]
pub struct ToggleVariantStyles {
    pub rounded: ToggleStyle,
    pub rectangular_with_text: ToggleStyle,
}

impl ToggleVariantStyles {
    pub fn from_colors(colors: ThemeColors) -> Self {
        Self {
            rounded: ToggleStyle {
                on_background: colors.primary,
                off_background: colors.input,
                knob_color: colors.background,
                hovered_background: colors.primary.with_alpha(0.5),
                text_color: colors.primary_foreground,
                border_color: Color::NONE,
                border_width: 0.0,
                transition_duration: 0.2,
            },
            rectangular_with_text: ToggleStyle {
                on_background: colors.primary,
                off_background: colors.input,
                knob_color: colors.background,
                hovered_background: colors.primary.with_alpha(0.5),
                text_color: colors.primary_foreground,
                border_color: Color::NONE,
                border_width: 0.0,
                transition_duration: 0.2,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToggleSizeProperties {
    pub padding_horizontal: f32,
    pub padding_vertical: f32,
    pub font_size: f32,
    pub icon_size: f32,
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

// Collection of size variants for Toggle
#[derive(Debug, Clone)]
pub struct ToggleSizeStyles {
    pub xsmall: ToggleSizeProperties,
    pub small: ToggleSizeProperties,
    pub medium: ToggleSizeProperties,
    pub large: ToggleSizeProperties,
    pub xlarge: ToggleSizeProperties,
}

pub fn Toggle_sizes() -> ToggleSizeStyles {
    ToggleSizeStyles {
        xsmall: ToggleSizeProperties {
            padding_horizontal: 4.0,
            padding_vertical: 2.0,
            font_size: 10.0,
            icon_size: 10.0,
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
        small: ToggleSizeProperties {
            padding_horizontal: 6.0,
            padding_vertical: 3.0,
            font_size: 12.0,
            icon_size: 12.0,
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
        medium: ToggleSizeProperties {
            padding_horizontal: 8.0,
            padding_vertical: 4.0,
            font_size: 14.0,
            icon_size: 14.0,
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
        large: ToggleSizeProperties {
            padding_horizontal: 10.0,
            padding_vertical: 5.0,
            font_size: 16.0,
            icon_size: 16.0,
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
        xlarge: ToggleSizeProperties {
            padding_horizontal: 12.0,
            padding_vertical: 6.0,
            font_size: 18.0,
            icon_size: 18.0,
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

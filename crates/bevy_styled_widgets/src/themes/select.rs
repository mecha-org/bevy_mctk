use bevy::prelude::*;

use super::ThemeColors;

#[derive(Debug, Clone)]
pub struct SelectButtonStyles {
    // Colors
    // trigger
    pub button_background: Color,
    pub button_text_color: Color,
    pub button_border_color: Color,

    // items
    pub popover_background: Color,
    pub popover_border_color: Color,

    pub hovered_item_background: Color,
    pub active_item_background: Color,

    pub active_border_color: Color,

    pub disabled_background: Color,
    pub disabled_text_color: Color,
    pub disabled_border_color: Color,
}

impl SelectButtonStyles {
    pub fn from_colors(colors: ThemeColors) -> Self {
        Self {
            button_background: colors.primary,
            button_text_color: colors.primary_foreground,
            button_border_color: colors.border,

            popover_background: colors.primary,
            popover_border_color: colors.border,

            hovered_item_background: colors.primary.with_alpha(0.9),
            active_item_background: colors.primary.with_alpha(0.7),

            active_border_color: colors.border,

            disabled_background: colors.secondary.with_alpha(0.5),
            disabled_text_color: colors.secondary.with_alpha(0.5),
            disabled_border_color: Color::NONE,
        }
    }
}

// Button size properties
#[derive(Debug, Clone)]
pub struct SelectButtonSizeProperties {
    pub font_size: f32,
    pub icon_size: f32,
    pub min_width: f32,
    pub min_height: f32,
    pub border_width: f32,
    pub border_radius: f32,
    pub padding_horizontal: f32,
    pub padding_vertical: f32,
}

// Collection of size variants
#[derive(Debug, Clone)]
pub struct SelectButtonSizeStyles {
    pub xsmall: SelectButtonSizeProperties,
    pub small: SelectButtonSizeProperties,
    pub medium: SelectButtonSizeProperties,
    pub large: SelectButtonSizeProperties,
    pub xlarge: SelectButtonSizeProperties,
}

pub fn select_button_sizes() -> SelectButtonSizeStyles {
    SelectButtonSizeStyles {
        xsmall: SelectButtonSizeProperties {
            padding_horizontal: 6.0,
            padding_vertical: 2.0,
            font_size: 12.0,
            icon_size: 12.0,
            min_width: 56.0,
            min_height: 28.0,
            border_width: 1.0,
            border_radius: 4.0,
        },
        small: SelectButtonSizeProperties {
            padding_horizontal: 12.0,
            padding_vertical: 4.0,
            font_size: 14.0,
            icon_size: 14.0,
            min_width: 80.0,
            min_height: 32.0,
            border_width: 1.0,
            border_radius: 5.0,
        },
        medium: SelectButtonSizeProperties {
            padding_horizontal: 16.0,
            padding_vertical: 8.0,
            font_size: 15.0,
            icon_size: 16.0,
            min_width: 100.0,
            min_height: 36.0,
            border_width: 1.5,
            border_radius: 6.0,
        },
        large: SelectButtonSizeProperties {
            padding_horizontal: 24.0,
            padding_vertical: 8.0,
            font_size: 16.0,
            icon_size: 18.0,
            min_width: 120.0,
            min_height: 40.0,
            border_width: 2.0,
            border_radius: 6.0,
        },
        xlarge: SelectButtonSizeProperties {
            padding_horizontal: 28.0,
            padding_vertical: 10.0,
            font_size: 18.0,
            icon_size: 20.0,
            min_width: 144.0,
            min_height: 52.0,
            border_width: 2.0,
            border_radius: 8.0,
        },
    }
}

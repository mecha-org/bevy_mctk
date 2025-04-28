use bevy::prelude::*;

use super::ThemeColors;

#[derive(Debug, Clone)]
pub struct ToggleStyle {
    // Colors
    pub active_background: Color,
    pub active_text_color: Color,
    pub inactive_background: Color,
    pub inactive_text_color: Color,
    pub hovered_active_background: Color,
    pub hovered_text_color: Color,
    pub disabled_inactive_background: Color,
    pub disabled_active_background: Color,
    pub disabled_text_color: Color,
    pub border_color: Color,
    // Transitions
    pub transition_duration: f32,
}

#[derive(Debug, Clone)]
pub struct ToggleVariantStyles {
    pub default: ToggleStyle,
    pub outline: ToggleStyle,
    pub with_text: ToggleStyle,
}

impl ToggleVariantStyles {
    pub fn from_colors(colors: ThemeColors) -> Self {
        Self {
            default: ToggleStyle {
                active_background: colors.accent,
                active_text_color: colors.accent_foreground,
                inactive_background: Color::NONE,
                inactive_text_color: colors.accent_foreground,
                hovered_active_background: colors.muted,
                hovered_text_color: colors.muted_foreground,
                disabled_inactive_background: Color::NONE,
                disabled_active_background: colors.accent,
                disabled_text_color: colors.muted_foreground,
                border_color: Color::NONE,
                transition_duration: 0.2,
            },
            outline: ToggleStyle {
                active_background: colors.accent,
                active_text_color: colors.accent_foreground,
                inactive_background: Color::NONE,
                inactive_text_color: colors.accent_foreground,
                hovered_active_background: colors.muted,
                hovered_text_color: colors.muted_foreground,
                disabled_inactive_background: Color::NONE,
                disabled_active_background: colors.accent,
                disabled_text_color: colors.muted_foreground,
                border_color: colors.border,
                transition_duration: 0.2,
            },
            with_text: ToggleStyle {
                active_background: colors.accent,
                active_text_color: colors.accent_foreground,
                inactive_background: Color::NONE,
                inactive_text_color: colors.accent_foreground,
                hovered_active_background: colors.muted,
                hovered_text_color: colors.muted_foreground,
                disabled_inactive_background: Color::NONE,
                disabled_active_background: colors.accent,
                disabled_text_color: colors.muted_foreground,
                border_color: Color::NONE,
                transition_duration: 0.2,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToggleSizeProperties {
    pub icon_size: f32,
    pub border_width: f32,
    pub corner_radius: f32,
    pub width: f32,
    pub height: f32,
    pub label_font_size: f32,
    pub padding_horizontal: f32,
    pub padding_vertical: f32,
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

pub fn toggle_sizes() -> ToggleSizeStyles {
    ToggleSizeStyles {
        xsmall: ToggleSizeProperties {
            icon_size: 10.0,
            border_width: 1.0,
            corner_radius: 6.0,
            width: 24.0,
            height: 24.0,
            label_font_size: 10.0,
            padding_horizontal: 8.0,
            padding_vertical: 4.0,
        },
        small: ToggleSizeProperties {
            icon_size: 12.0,
            border_width: 1.0,
            corner_radius: 7.0,
            width: 32.0,
            height: 32.0,
            label_font_size: 12.0,
            padding_horizontal: 10.0,
            padding_vertical: 6.0,
        },
        medium: ToggleSizeProperties {
            icon_size: 14.0,
            border_width: 1.0,
            corner_radius: 9.0,
            width: 40.0,
            height: 40.0,
            label_font_size: 14.0,
            padding_horizontal: 14.0,
            padding_vertical: 8.0,
        },
        large: ToggleSizeProperties {
            icon_size: 16.0,
            border_width: 1.0,
            corner_radius: 10.0,
            width: 48.0,
            height: 48.0,
            label_font_size: 16.0,
            padding_horizontal: 16.0,
            padding_vertical: 8.0,
        },
        xlarge: ToggleSizeProperties {
            icon_size: 18.0,
            border_width: 1.0,
            corner_radius: 12.0,
            width: 56.0,
            height: 56.0,
            label_font_size: 18.0,
            padding_horizontal: 18.0,
            padding_vertical: 8.0,
        },
    }
}

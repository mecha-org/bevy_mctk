use bevy::prelude::*;

use super::ThemeColors;

#[derive(Debug, Clone)]
pub struct CheckboxStyle {
    // Colors
    pub checked_background: Color,
    pub text_color: Color,
    pub unchecked_background: Color,
    pub hovered_background: Color,
    pub disabled_unchecked_background: Color,
    pub disabled_checked_background: Color,
    pub disabled_text_color: Color,
    pub border_color: Color,
    pub disabled_border_color: Color,
    pub caption_color: Color,
    pub description_color: Color,
    // Transitions
    pub transition_duration: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct CheckboxStyleAlphas {
    pub disabled_checked_alpha: f32,
    pub disabled_border_alpha: f32,
    pub disabled_text_alpha: f32,
    pub hovered_alpha: f32,
}

#[derive(Debug, Clone)]
pub struct CheckboxVariantStyles {
    pub default: CheckboxStyle,
    pub with_text: CheckboxStyle,
}

impl CheckboxVariantStyles {
    pub fn from_colors(colors: ThemeColors) -> Self {
        let alphas = CheckboxStyleAlphas {
            disabled_checked_alpha: 0.5,
            disabled_border_alpha: 0.5,
            disabled_text_alpha: 0.5,
            hovered_alpha: 0.5,
        };

        Self {
            default: CheckboxStyle {
                checked_background: colors.primary,
                text_color: colors.primary_foreground,
                unchecked_background: Color::NONE,
                hovered_background: colors.primary.with_alpha(alphas.hovered_alpha),
                disabled_unchecked_background: Color::NONE,
                disabled_checked_background: colors
                    .primary
                    .with_alpha(alphas.disabled_checked_alpha),
                disabled_text_color: colors
                    .primary_foreground
                    .with_alpha(alphas.disabled_text_alpha),
                border_color: colors.primary,
                disabled_border_color: colors.primary.with_alpha(alphas.disabled_border_alpha),
                caption_color: colors.foreground,
                description_color: colors.muted_foreground,
                transition_duration: 0.2,
            },
            with_text: CheckboxStyle {
                checked_background: colors.primary,
                text_color: colors.primary_foreground,
                unchecked_background: Color::NONE,
                hovered_background: colors.primary.with_alpha(alphas.hovered_alpha),
                disabled_unchecked_background: Color::NONE,
                disabled_checked_background: colors
                    .primary
                    .with_alpha(alphas.disabled_checked_alpha),
                disabled_text_color: colors
                    .primary_foreground
                    .with_alpha(alphas.disabled_text_alpha),
                border_color: colors.primary,
                disabled_border_color: colors.primary.with_alpha(alphas.disabled_border_alpha),
                caption_color: colors.foreground,
                description_color: colors.muted_foreground,
                transition_duration: 0.2,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckboxSizeProperties {
    pub border_width: f32,
    pub corner_radius: f32,
    pub width: f32,
    pub height: f32,
    pub check_mark_font_size: f32,
    pub padding_horizontal: f32,
    pub padding_vertical: f32,
    pub caption_font_size: f32,
    pub description_font_size: f32,
}

// Collection of size variants for Checkbox
#[derive(Debug, Clone)]
pub struct CheckboxSizeStyles {
    pub xsmall: CheckboxSizeProperties,
    pub small: CheckboxSizeProperties,
    pub medium: CheckboxSizeProperties,
    pub large: CheckboxSizeProperties,
    pub xlarge: CheckboxSizeProperties,
}

pub fn checkbox_sizes() -> CheckboxSizeStyles {
    CheckboxSizeStyles {
        xsmall: CheckboxSizeProperties {
            border_width: 1.0,
            corner_radius: 3.0,
            width: 16.0,
            height: 16.0,
            check_mark_font_size: 10.0,
            padding_horizontal: 6.0,
            padding_vertical: 2.0,
            caption_font_size: 14.0,
            description_font_size: 12.0,
        },
        small: CheckboxSizeProperties {
            border_width: 1.0,
            corner_radius: 4.0,
            width: 18.0,
            height: 18.0,
            check_mark_font_size: 12.0,
            padding_horizontal: 8.0,
            padding_vertical: 4.0,
            caption_font_size: 14.0,
            description_font_size: 12.0,
        },
        medium: CheckboxSizeProperties {
            border_width: 1.0,
            corner_radius: 5.0,
            width: 22.0,
            height: 22.0,
            check_mark_font_size: 14.0,
            padding_horizontal: 10.0,
            padding_vertical: 6.0,
            caption_font_size: 14.0,
            description_font_size: 12.0,
        },
        large: CheckboxSizeProperties {
            border_width: 1.0,
            corner_radius: 7.0,
            width: 30.0,
            height: 30.0,
            check_mark_font_size: 20.0,
            padding_horizontal: 12.0,
            padding_vertical: 8.0,
            caption_font_size: 14.0,
            description_font_size: 12.0,
        },
        xlarge: CheckboxSizeProperties {
            border_width: 1.0,
            corner_radius: 9.0,
            width: 40.0,
            height: 40.0,
            check_mark_font_size: 30.0,
            padding_horizontal: 16.0,
            padding_vertical: 8.0,
            caption_font_size: 14.0,
            description_font_size: 12.0,
        },
    }
}

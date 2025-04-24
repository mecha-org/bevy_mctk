use bevy::{
    ecs::system::SystemId, input_focus::tab_navigation::TabIndex, prelude::*,
    window::SystemCursorIcon, winit::cursor::CursorIcon,
};
use bevy_core_widgets::{Checked, hover::Hovering};

use bevy_additional_core_widgets::CoreSwitch;

use super::{
    SwitchSize,
    components::{AccessibleName, StyledSwitch, SwitchVariant},
};
use crate::themes::ThemeManager;

pub struct StyledToggleButtonBuilder {
    pub active: bool,
    pub label: String,
    pub on_change: Option<fn(bool)>,
    pub icon: Option<UiImage>,
    pub variant: ToggleVariant,
    pub theme: ButtonTheme,
}

impl StyledToggleButtonBuilder {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            active: false,
            on_change: None,
            icon: None,
            variant: ToggleVariant::RectangularWithText,
            theme: ButtonTheme::default(),
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn icon(mut self, icon: UiImage) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn on_change(mut self, callback: fn(bool)) -> Self {
        self.on_change = Some(callback);
        self
    }

    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn theme(mut self, theme: ButtonTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn build(self) -> impl Bundle {
        let background = if self.active {
            self.theme.active_background
        } else {
            self.theme.inactive_background
        };

        let text_color = if self.active {
            self.theme.active_text_color
        } else {
            self.theme.inactive_text_color
        };

        (
            ButtonBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                background_color: background.into(),
                ..default()
            },
            TextBundle::from_section(
                self.label.clone(),
                TextStyle {
                    font_size: 16.0,
                    color: text_color,
                    ..default()
                },
            ),
            StyledToggleButton {
                active: self.active,
                on_change: self.on_change,
                label: self.label.clone(),
                variant: self.variant,
            },
            AccessibleName(self.label),
            TabIndex(0),
        )
    }
}

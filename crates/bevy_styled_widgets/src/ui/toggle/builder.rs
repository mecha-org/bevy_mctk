use bevy::{
    ecs::system::SystemId, input_focus::tab_navigation::TabIndex, prelude::*,
    window::SystemCursorIcon, winit::cursor::CursorIcon,
};
use bevy_core_widgets::{Checked, hover::Hovering};

use bevy_additional_core_widgets::CoreSwitch;

use super::{
    ToggleSize,
    components::{AccessibleName, StyledToggle, ToggleVariant},
};
use crate::themes::ThemeManager;

#[derive(Component, Default)]
pub struct RootComponent;

#[derive(Default)]
pub struct ToggleBuilder {
    pub active: bool,
    pub label: Option<String>,
    pub on_change: Option<SystemId<In<(Entity, bool)>>>,
    pub icon: Option<String>,
    pub variant: ToggleVariant,
    pub disabled: bool,
    pub size: Option<ToggleSize>,
}

impl ToggleBuilder {
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn icon<S: Into<String>>(mut self, icon: S) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn on_change(mut self, system_id: SystemId<In<(Entity, bool)>>) -> Self {
        self.on_change = Some(system_id);
        self
    }

    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn build(self) -> impl Bundle {
        let theme_manager = ThemeManager::default();

        let toggle_size_styles = theme_manager.styles.toggle_sizes.clone();
        //Update size styles
        let toggle_size_style = match self.size.unwrap_or_default() {
            ToggleSize::XSmall => toggle_size_styles.xsmall,
            ToggleSize::Small => toggle_size_styles.small,
            ToggleSize::Medium => toggle_size_styles.medium,
            ToggleSize::Large => toggle_size_styles.large,
            ToggleSize::XLarge => toggle_size_styles.xlarge,
        };

        let style = match self.variant {
            ToggleVariant::Default => &theme_manager.styles.toggles.default,
            ToggleVariant::Outline => &theme_manager.styles.toggles.outline,
            ToggleVariant::WithText => &theme_manager.styles.toggles.with_text,
        };

        let is_active = self.active;
        let is_disabled = self.disabled;

        let background = if is_disabled {
            if is_active {
                style.disabled_active_background
            } else {
                style.disabled_inactive_background
            }
        } else {
            if is_active {
                style.active_background
            } else {
                style.inactive_background
            }
        };

        let text_color = if is_disabled {
            if is_active {
                style.disabled_text_color
            } else {
                style.disabled_text_color
            }
        } else {
            if is_active {
                style.active_text_color
            } else {
                style.inactive_text_color
            }
        };

        let text = self.label.clone().unwrap_or_else(|| "T".to_string());
        let cursor_icon = if self.disabled {
            SystemCursorIcon::NotAllowed
        } else {
            SystemCursorIcon::Pointer
        };

        let child_nodes = Children::spawn((Spawn((
            Node {
                width: Val::Auto,
                height: Val::Px(toggle_size_style.height),
                padding: UiRect::axes(
                    Val::Px(toggle_size_style.padding_horizontal),
                    Val::Px(toggle_size_style.padding_vertical),
                ),
                border: UiRect::all(Val::Px(if self.variant == ToggleVariant::Outline {
                    toggle_size_style.border_width
                } else {
                    0.0
                })),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(style.border_color),
            BorderRadius::all(Val::Px(toggle_size_style.corner_radius)),
            BackgroundColor(background),
            Children::spawn((Spawn((
                Node {
                    position_type: PositionType::Relative,
                    left: Val::Px(0.0),
                    ..default()
                },
                Text::new(text),
                TextFont {
                    font_size: toggle_size_style.label_font_size,
                    ..default()
                },
                TextColor(text_color),
            )),)),
        )),));

        (
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            Name::new("Switch"),
            Hovering::default(),
            CursorIcon::System(cursor_icon),
            StyledToggle {
                active: self.active,
                on_change: self.on_change,
                label: self.label.clone(),
                variant: self.variant,
                disabled: self.disabled,
                size: self.size,
                icon: self.icon.clone(),
            },
            CoreSwitch {
                on_change: self.on_change,
            },
            Checked(self.active),
            RootComponent,
            AccessibleName(self.label.clone().unwrap_or_else(|| "Switch".to_string())),
            TabIndex(0),
            child_nodes,
        )
    }
}

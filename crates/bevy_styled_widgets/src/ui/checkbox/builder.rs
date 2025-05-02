use bevy::{
    ecs::system::SystemId, input_focus::tab_navigation::TabIndex, prelude::*,
    window::SystemCursorIcon, winit::cursor::CursorIcon,
};
use bevy_core_widgets::{Checked, CoreCheckbox, InteractionDisabled, hover::Hovering};

use super::{
    CheckboxSize,
    components::{AccessibleName, CheckboxVariant, StyledCheckbox},
};
use crate::themes::ThemeManager;

#[derive(Component, Default)]
pub struct RootComponent;

#[derive(Component)]
pub struct CheckboxCheckmarkText;

#[derive(Default)]
pub struct CheckboxBuilder {
    pub checked: bool,
    pub check_mark: Option<String>,
    pub on_change: Option<SystemId<In<bool>>>,
    pub variant: CheckboxVariant,
    pub disabled: bool,
    pub size: Option<CheckboxSize>,
    pub caption: Option<String>,
    pub description: Option<String>,
}

impl CheckboxBuilder {
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn check_mark<S: Into<String>>(mut self, check_mark: S) -> Self {
        self.check_mark = Some(check_mark.into());
        self
    }

    pub fn on_change(mut self, system_id: SystemId<In<bool>>) -> Self {
        self.on_change = Some(system_id);
        self
    }

    pub fn variant(mut self, variant: CheckboxVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn size(mut self, size: CheckboxSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn build(self) -> impl Bundle {
        let theme_manager = ThemeManager::default();

        let checkbox_size_styles = theme_manager.styles.checkbox_sizes.clone();
        //Update size styles
        let checkbox_size_style = match self.size.unwrap_or_default() {
            CheckboxSize::XSmall => checkbox_size_styles.xsmall,
            CheckboxSize::Small => checkbox_size_styles.small,
            CheckboxSize::Medium => checkbox_size_styles.medium,
            CheckboxSize::Large => checkbox_size_styles.large,
            CheckboxSize::XLarge => checkbox_size_styles.xlarge,
        };

        let style = match self.variant {
            CheckboxVariant::Default => &theme_manager.styles.checkboxes.default,
            CheckboxVariant::WithText => &theme_manager.styles.checkboxes.with_text,
        };

        let is_checked = self.checked;
        let is_disabled = self.disabled;

        let background = if is_disabled {
            if is_checked {
                style.disabled_checked_background
            } else {
                style.disabled_unchecked_background
            }
        } else {
            if is_checked {
                style.checked_background
            } else {
                style.unchecked_background
            }
        };

        let border_color = if is_disabled {
            style.disabled_border_color
        } else {
            style.border_color
        };

        let text_color = if is_disabled {
            style.disabled_text_color
        } else {
            style.text_color
        };

        let text = self.check_mark.clone().unwrap_or_else(|| "✔".to_string());
        let caption = self.caption.clone().unwrap_or_else(|| "".to_string());
        let description = self.description.clone().unwrap_or_else(|| "".to_string());

        let cursor_icon = if self.disabled {
            SystemCursorIcon::NotAllowed
        } else {
            SystemCursorIcon::Pointer
        };

        let child_nodes = Children::spawn((
            // Checkbox box (left)
            Spawn((
                Node {
                    width: Val::Px(checkbox_size_style.width),
                    height: Val::Px(checkbox_size_style.height),
                    border: UiRect::all(Val::Px(checkbox_size_style.border_width)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(border_color),
                BorderRadius::all(Val::Px(checkbox_size_style.corner_radius)),
                BackgroundColor(background),
                Children::spawn((Spawn((
                    Text::new(if is_checked {
                        text.clone()
                    } else {
                        "".to_string()
                    }),
                    TextFont {
                        font_size: checkbox_size_style.check_mark_font_size,
                        ..default()
                    },
                    TextColor(text_color),
                    CheckboxCheckmarkText,
                )),)),
            )),
            // Caption + description (right)
            Spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::left(Val::Px(12.0)),
                    ..default()
                },
                Children::spawn((
                    // Caption (on same row as checkbox)
                    Spawn((
                        Text::new(caption),
                        TextFont {
                            font_size: checkbox_size_style.caption_font_size,
                            ..default()
                        },
                        TextColor(style.caption_color),
                    )),
                    // Description (under caption)
                    Spawn((
                        Text::new(description),
                        TextFont {
                            font_size: checkbox_size_style.description_font_size,
                            ..default()
                        },
                        TextColor(style.description_color),
                    )),
                )),
            )),
        ));

        (
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            Name::new("Checkbox"),
            Hovering::default(),
            CursorIcon::System(cursor_icon),
            StyledCheckbox {
                checked: self.checked,
                on_change: self.on_change,
                check_mark: self.check_mark.clone(),
                variant: self.variant,
                disabled: self.disabled,
                size: self.size,
            },
            CoreCheckbox {
                on_change: self.on_change,
            },
            Checked(self.checked),
            RootComponent,
            AccessibleName(
                self.check_mark
                    .clone()
                    .unwrap_or_else(|| "Checkbox".to_string()),
            ),
            TabIndex(0),
            child_nodes,
        )
    }
}

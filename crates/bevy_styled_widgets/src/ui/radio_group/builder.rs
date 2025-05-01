use bevy::{
    ecs::system::SystemId, input_focus::tab_navigation::TabIndex, prelude::*,
    window::SystemCursorIcon, winit::cursor::CursorIcon,
};
use bevy_core_widgets::{Checked, CoreRadio, CoreRadioGroup, hover::Hovering};

use super::{
    RadioButtonSize,
    components::{AccessibleName, RadioButtonVariant, StyledRadioButton, StyledRadioGroup},
};
use crate::themes::ThemeManager;

#[derive(Component, Default)]
pub struct RootComponent;

#[derive(Component, Debug, Clone)]
pub struct RadioValue(pub String);

#[derive(Default, Clone)]
pub struct RadioButtonBuilder {
    pub checked: bool,
    pub on_change: Option<SystemId<In<bool>>>,
    pub variant: RadioButtonVariant,
    pub disabled: bool,
    pub size: Option<RadioButtonSize>,
    pub caption: Option<String>,
    pub value: String,
}

impl RadioButtonBuilder {
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn on_change(mut self, system_id: SystemId<In<bool>>) -> Self {
        self.on_change = Some(system_id);
        self
    }

    pub fn variant(mut self, variant: RadioButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn size(mut self, size: RadioButtonSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn value<S: Into<String>>(mut self, value: S) -> Self {
        self.value = value.into();
        self
    }

    pub fn build(self) -> impl Bundle {
        let theme_manager = ThemeManager::default();

        let radio_size_styles = theme_manager.styles.radio_button_sizes.clone();
        //Update size styles
        let radio_size_style = match self.size.unwrap_or_default() {
            RadioButtonSize::XSmall => radio_size_styles.xsmall,
            RadioButtonSize::Small => radio_size_styles.small,
            RadioButtonSize::Medium => radio_size_styles.medium,
            RadioButtonSize::Large => radio_size_styles.large,
            RadioButtonSize::XLarge => radio_size_styles.xlarge,
        };

        let style = match self.variant {
            RadioButtonVariant::Default => &theme_manager.styles.radio_buttons.default,
        };

        let is_checked = self.checked;
        let is_disabled = self.disabled;

        let background_color = if is_disabled {
            if is_checked {
                style.disabled_inner_checked_background
            } else {
                style.disabled_inner_unchecked_background
            }
        } else {
            if is_checked {
                style.inner_checked_background
            } else {
                style.inner_unchecked_background
            }
        };

        let border_color = if is_disabled {
            style.disabled_outer_border
        } else {
            style.outer_border
        };

        let caption = self.caption.clone().unwrap_or_else(|| "".to_string());

        let cursor_icon = if self.disabled {
            SystemCursorIcon::NotAllowed
        } else {
            SystemCursorIcon::Pointer
        };

        let child_nodes = Children::spawn((
            Spawn((
                // Radio outer
                Node {
                    display: Display::Flex,
                    width: Val::Px(radio_size_style.outer_width),
                    height: Val::Px(radio_size_style.outer_height),
                    border: UiRect::all(Val::Px(radio_size_style.outer_border_width)),
                    ..default()
                },
                BorderColor(border_color), 
                BorderRadius::all(Val::Percent(radio_size_style.outer_corner_radius)),
                children![
                    // Radio inner
                    (
                        Node {
                            display: Display::Flex,
                            width: Val::Px(radio_size_style.inner_circle_width),
                            height: Val::Px(radio_size_style.inner_circle_height),
                            position_type: PositionType::Absolute,
                            left: Val::Px(radio_size_style.inner_circle_left),
                            top: Val::Px(radio_size_style.inner_circle_top),
                            ..default()
                        },
                        BackgroundColor(background_color),
                        BorderRadius::all(Val::Percent(
                            radio_size_style.inner_circle_corner_radius
                        )),
                    ),
                ],
            )),
            Spawn((
                Text::new(caption.clone()),
                TextFont {
                    font_size: radio_size_style.caption_font_size,
                    ..default()
                },
                TextColor(style.caption_color),
            )),
        ));

        (
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                border: UiRect::all(Val::Px(1.0)),
                column_gap: Val::Px(4.0),
                ..default()
            },
            Name::new("Radio"),
            Hovering::default(),
            CursorIcon::System(cursor_icon),
            StyledRadioButton {
                checked: self.checked,
                on_change: self.on_change,
                variant: self.variant,
                disabled: self.disabled,
                size: self.size,
                caption: self.caption,
                value: self.value.clone(),
            },
            CoreRadio,
            Checked(self.checked),
            RadioValue(self.value.clone()),
            RootComponent,
            AccessibleName(caption.clone()),
            TabIndex(0),
            child_nodes,
        )
    }
}

#[derive(Default, Clone)]
pub struct RadioGroupBuilder {
    pub on_change: Option<SystemId<In<Entity>>>,
    pub children: Vec<RadioButtonBuilder>,
}

impl RadioGroupBuilder {
    pub fn on_change(mut self, system_id: SystemId<In<Entity>>) -> Self {
        self.on_change = Some(system_id);
        self
    }

    pub fn children(mut self, radios: impl IntoIterator<Item = RadioButtonBuilder>) -> Self {
        self.children.extend(radios);
        self
    }

    pub fn build(self) -> (impl Bundle, Vec<impl Bundle>) {
        let child_bundles = self
            .children
            .into_iter()
            .map(|builder| builder.build())
            .collect::<Vec<_>>();

        let parent_bundle = (
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                align_content: AlignContent::Start,
                ..default()
            },
            CoreRadioGroup {
                on_change: self.on_change,
            },
            TabIndex(0),
        );

        (parent_bundle, child_bundles)
    }
}

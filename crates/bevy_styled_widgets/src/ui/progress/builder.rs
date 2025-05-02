use bevy::{
    color::palettes::css::{GREY, LIGHT_GRAY},
    input_focus::tab_navigation::TabIndex,
    prelude::*,
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};
use bevy_core_widgets::hover::Hovering;

use super::{StyledProgress, components::AccessibleName};

#[derive(Component)]
pub struct ProgressRoot;

#[derive(Component)]
pub struct ProgressIndicator;

#[derive(Default)]
pub struct ProgressBuilder {
    value: f32,
    root_color: Option<Color>,
    indicator_color: Option<Color>,
}

impl ProgressBuilder {
    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn root_color(mut self, color: Color) -> Self {
        self.root_color = Some(color);
        self
    }

    pub fn indicator_color(mut self, color: Color) -> Self {
        self.indicator_color = Some(color);
        self
    }

    pub fn build(self) -> impl Bundle {
        let max = if self.value.clone() > 10.0 {
            100.
        } else {
            10.0
        };
        let progress_value = (self.value / max) * 100.0;

        (
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Stretch,
                align_items: AlignItems::Stretch,
                justify_items: JustifyItems::Center,
                height: Val::Px(12.0),
                width: Val::Percent(100.0),
                ..default()
            },
            Name::new("Progress"),
            AccessibleName("Progress".to_string()),
            Hovering::default(),
            CursorIcon::System(SystemCursorIcon::Default),
            StyledProgress {
                value: self.value.clone(),
                root_color: self.root_color.clone(),
                indicator_color: self.indicator_color.clone(),
            },
            TabIndex(0),
            Children::spawn((
                // Progress background root
                Spawn((
                    Node {
                        height: Val::Px(6.0),
                        ..default()
                    },
                    BackgroundColor(self.indicator_color.unwrap_or(GREY.into())),
                    BorderRadius::all(Val::Px(3.0)),
                    ProgressRoot,
                )),
                // Progress indicator
                Spawn((
                    Node {
                        display: Display::Flex,
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        height: Val::Px(6.0),
                        width: Val::Percent(progress_value), // Indicator width based on value
                        ..default()
                    },
                    BackgroundColor(self.indicator_color.unwrap_or(LIGHT_GRAY.into())),
                    BorderRadius::all(Val::Px(3.0)),
                    ProgressIndicator,
                )),
            )),
        )
    }
}

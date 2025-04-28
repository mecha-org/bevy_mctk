use bevy::{
    ecs::system::SystemId, input_focus::tab_navigation::TabIndex, prelude::*,
    window::SystemCursorIcon, winit::cursor::CursorIcon,
};
use bevy_core_widgets::{CoreSlider, hover::Hovering};

use crate::themes::ThemeManager;

use super::{StyledSlider, components::AccessibleName};

#[derive(Component)]
pub struct Track;

#[derive(Component)]
pub struct Thumb;

#[derive(Default)]
pub struct SliderBuilder {
    min: f32,
    max: f32,
    value: f32,
    on_change: Option<SystemId<In<f32>>>,
    track_color: Option<Color>,
    thumb_color: Option<Color>,
    hovered_thumb_color: Option<Color>,
}

impl SliderBuilder {
    pub fn on_change(mut self, system_id: SystemId<In<f32>>) -> Self {
        self.on_change = Some(system_id);
        self
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = Some(color);
        self
    }

    pub fn thumb_color(mut self, color: Color) -> Self {
        self.thumb_color = Some(color);
        self
    }

    pub fn hovered_thumb_color(mut self, color: Color) -> Self {
        self.hovered_thumb_color = Some(color);
        self
    }

    pub fn build(self) -> impl Bundle {
        let theme_manager = ThemeManager::default();
        let slider_styles = theme_manager.styles.slider.clone();
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
            Name::new("Slider"),
            AccessibleName("Slider".to_string()),
            Hovering::default(),
            CursorIcon::System(SystemCursorIcon::Pointer),
            StyledSlider {
                min: self.min.clone(),
                max: self.max.clone(),
                value: self.value.clone(),
                on_change: self.on_change,
                track_color: self.track_color,
                thumb_color: self.thumb_color,
                hovered_thumb_color: self.hovered_thumb_color,
            },
            CoreSlider {
                min: self.min,
                max: self.max,
                value: self.value,
                on_change: self.on_change,
                thumb_size: 12.0,
                ..default()
            },
            TabIndex(0),
            Children::spawn((
                // Slider background rail
                Spawn((
                    Node {
                        height: Val::Px(6.0),
                        ..default()
                    },
                    Track,
                    BackgroundColor(self.track_color.unwrap_or(slider_styles.track_color)),
                    BorderRadius::all(Val::Px(3.0)),
                )),
                // Invisible track to allow absolute placement of thumb entity. This is narrower than
                // the actual slider, which allows us to position the thumb entity using simple
                // percentages, without having to measure the actual width of the slider thumb.
                Spawn((
                    Node {
                        display: Display::Flex,
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.0),
                        right: Val::Px(12.0), // Track is short by 12px to accommodate the thumb
                        top: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        ..default()
                    },
                    children![(
                        // Thumb
                        Node {
                            display: Display::Flex,
                            width: Val::Px(12.0),
                            height: Val::Px(12.0),
                            position_type: PositionType::Absolute,
                            left: Val::Percent(50.0), // This will be updated by the slider's value
                            ..default()
                        },
                        Thumb,
                        BorderRadius::all(Val::Px(6.0)),
                        BackgroundColor(self.thumb_color.unwrap_or(slider_styles.thumb_color)),
                    )],
                )),
            )),
        )
    }
}

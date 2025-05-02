use bevy::{
    color::palettes::css::{LIGHT_GRAY, ORANGE},
    ecs::system::SystemId,
    input_focus::tab_navigation::TabIndex,
    prelude::*,
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};
use bevy_cosmic_edit::{
    CosmicBackgroundColor, CosmicEditBuffer, CosmicFontSystem, CosmicTextAlign, HorizontalAlign,
    HoverCursor, MaxLines, SelectedTextColor, VerticalAlign,
    cosmic_text::{
        self,
        Attrs,
        Family,
        Metrics, // Color
    },
    placeholder::Placeholder,
    prelude::TextEdit,
};

use crate::themes::ThemeManager;

use super::components::{AccessibleName, InputVariant, StyledInput};

#[derive(Default)]
pub struct InputBuilder<'a> {
    variant: InputVariant,
    width: Option<Val>,
    height: Option<Val>,
    value: Option<String>,
    placeholder: Option<&'static str>,
    on_change: Option<SystemId<In<(Entity, String)>>>,
    text_color: Option<Color>,
    placeholder_color: Option<Color>,
    background_color: Option<Color>,
    border_color: Option<Color>,
    border_width: Option<UiRect>,
    border_radius: Option<BorderRadius>,
    font_size: Option<f32>,
    line_height: Option<f32>,
    font_system: Option<&'a mut CosmicFontSystem>,
    disabled: bool,
}

impl<'a> InputBuilder<'a> {
    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn on_change(mut self, system_id: SystemId<In<(Entity, String)>>) -> Self {
        self.on_change = Some(system_id);
        self
    }

    pub fn value<S: Into<String>>(mut self, value: S) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn placeholder(mut self, placeholder: &'static str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn placeholder_color(mut self, color: Color) -> Self {
        self.placeholder_color = Some(color);
        self
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn border_width(mut self, width: UiRect) -> Self {
        self.border_width = Some(width);
        self
    }

    pub fn border_radius(mut self, radius: BorderRadius) -> Self {
        self.border_radius = Some(radius);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: Val) -> Self {
        self.height = Some(height);
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }

    pub fn line_height(mut self, size: f32) -> Self {
        self.line_height = Some(size);
        self
    }

    pub fn font_system(mut self, font_system: &'a mut CosmicFontSystem) -> Self {
        self.font_system = Some(font_system);
        self
    }

    pub fn build(self) -> impl Bundle {
        let theme_manager = ThemeManager::default();
        let text_input_style = theme_manager.styles.input.clone();

        (
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                ..default()
            },
            Name::new("TextInput"),
            StyledInput {
                variant: self.variant,
                on_change: self.on_change,
                background_color: self.background_color,
                border_color: self.border_color,
                border_width: self.border_width,
                border_radius: self.border_radius,
                text_color: self.text_color,
                disabled: self.disabled,
                value: self.value.clone(),
                placeholder: self.placeholder,
                font_size: self.font_size,
                line_height: self.line_height,
                width: self.width.unwrap_or(Val::Px(200.0)),
                height: self.height.unwrap_or(Val::Px(40.0)),
                placeholder_color: self.placeholder_color,
            },
            AccessibleName(
                self.value
                    .clone()
                    .unwrap_or_else(|| "text input".to_string()),
            ),
            TabIndex(0),
            children![get_text_input(
                self.font_system,
                self.value.clone().unwrap_or("".to_string()),
                self.placeholder,
                self.text_color.unwrap_or(text_input_style.text_color),
                self.border_color.unwrap_or(text_input_style.border_color),
                self.background_color
                    .unwrap_or(text_input_style.background_color),
                self.placeholder_color.unwrap_or(LIGHT_GRAY.into()),
                // self.background_color.unwrap_or(Color::WHITE),
                self.width,
                self.height
            )],
        )
    }
}

pub fn get_text_input(
    mut font_system: Option<&mut CosmicFontSystem>,
    input_text_value: String,
    placeholder: Option<&'static str>,
    text_color: Color,
    placeholder_color: Color,
    border_color: Color,
    background_color: Color,
    width: Option<Val>,
    height: Option<Val>,
) -> impl Bundle {
    let mut attrs = Attrs::new();
    attrs = attrs.family(Family::Name("Victor Mono"));
    attrs = attrs.color(cosmic_text::Color(text_color.to_linear().as_u32())); // convert bevy color to cosmic color

    let cosmic_text_buffer =
        CosmicEditBuffer::new(font_system.as_mut().unwrap(), Metrics::new(20., 20.))
            .with_rich_text(
                font_system.as_mut().unwrap(),
                vec![(input_text_value.clone().as_str(), attrs)],
                attrs,
            );

    (
        (
            CosmicBackgroundColor(background_color),
            CosmicTextAlign {
                horizontal: Some(HorizontalAlign::Left),
                vertical: VerticalAlign::Center,
            },
            HoverCursor(CursorIcon::System(SystemCursorIcon::Pointer)),
            SelectedTextColor(ORANGE.into()),
        ),
        TextEdit,
        cosmic_text_buffer,
        MaxLines(1),
        Placeholder::new(
            placeholder.unwrap_or(""),
            attrs.color(cosmic_text::Color(placeholder_color.to_linear().as_u32())),
        ),
        BorderColor(border_color),
        BorderRadius::all(Val::Px(6.)),
        ImageNode::default().with_mode(bevy::ui::widget::NodeImageMode::Stretch), // FOR BORDER
        Node {
            border: UiRect::all(Val::Px(2.)), // border width
            width: width.unwrap_or(Val::Px(200.0)),
            height: height.unwrap_or(Val::Px(40.0)),
            ..default()
        },
    )
}

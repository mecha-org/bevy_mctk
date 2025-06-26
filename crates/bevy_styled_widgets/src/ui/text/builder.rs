use super::{StyledText, TextVariant};
use bevy::{prelude::*, text::FontWeight};

#[derive(Default)]
pub struct TextBuilder {
    variant: TextVariant,
    content: String,
    color: Option<Color>,
    font: Option<Handle<Font>>,
    font_size: Option<f32>,
    font_weight: Option<FontWeight>,
    alignment: Option<JustifyText>,
    line_height: Option<f32>,
    max_width: Option<f32>,
    selectable: bool,
}

impl TextBuilder {
    pub fn variant(mut self, variant: TextVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn content<S: Into<String>>(mut self, content: S) -> Self {
        self.content = content.into();
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = Some(font);
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }

    pub fn font_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = Some(weight);
        self
    }

    pub fn alignment(mut self, alignment: JustifyText) -> Self {
        self.alignment = Some(alignment);
        self
    }

    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(line_height);
        self
    }

    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }

    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    pub fn build(self) -> impl Bundle {
        let default_text_font = TextFont::default();
        (
            Text::new(self.content.clone()),
            StyledText {
                variant: self.variant,
                color: self.color,
                font: self.font.clone(),
                font_size: self.font_size,
                font_weight: self.font_weight,
                alignment: self.alignment,
                line_height: self.line_height,
                max_width: self.max_width,
                selectable: self.selectable,
                content: self.content.clone(),
            },
            Name::new(format!("Text: {}", self.content)),
            TextFont {
                font: self.font.unwrap_or(default_text_font.font),
                font_size: self.font_size.unwrap_or(default_text_font.font_size),
                ..Default::default()
            },
        )
    }
}

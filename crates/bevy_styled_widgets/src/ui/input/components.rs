use bevy::a11y::AccessibilityNode;
use bevy::ecs::{component::HookContext, system::SystemId, world::DeferredWorld};
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
pub struct StyledInput {
    #[reflect(ignore)]
    pub variant: InputVariant,
    #[reflect(ignore)]
    pub width: Val,
    pub height: Val,
    pub value: Option<String>,
    pub placeholder: Option<&'static str>,
    #[reflect(ignore)]
    pub on_change: Option<SystemId<In<(Entity, String)>>>,
    pub text_color: Option<Color>,
    pub placeholder_color: Option<Color>,
    pub background_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<UiRect>,
    pub border_radius: Option<BorderRadius>,
    pub font_size: Option<f32>,
    pub line_height: Option<f32>,
    pub disabled: bool,
}

impl StyledInput {
    pub fn builder<'a>() -> super::builder::InputBuilder<'a> {
        super::builder::InputBuilder::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum InputVariant {
    Text,
    Password,
    Email,
}

impl Default for InputVariant {
    fn default() -> Self {
        InputVariant::Text
    }
}

#[derive(Component, Default)]
#[component(immutable, on_add = on_set_label, on_replace = on_set_label)]
pub struct AccessibleName(pub String);

fn on_set_label(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    let name = entt.get::<AccessibleName>().unwrap().0.clone();
    if let Some(mut accessibility) = entt.get_mut::<AccessibilityNode>() {
        accessibility.set_label(name.as_str());
    }
}

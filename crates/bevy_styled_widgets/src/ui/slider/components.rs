use bevy::a11y::AccessibilityNode;
use bevy::ecs::{component::HookContext, system::SystemId, world::DeferredWorld};
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
pub struct StyledSlider {
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub track_color: Option<Color>,
    pub thumb_color: Option<Color>,
    pub hovered_thumb_color: Option<Color>,
    #[reflect(ignore)]
    pub on_change: Option<SystemId<In<f32>>>,
    pub size: Option<SliderSize>,
    pub disabled: bool,
}

impl StyledSlider {
    pub fn builder() -> super::builder::SliderBuilder {
        super::builder::SliderBuilder::default()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
pub enum SliderSize {
    XSmall,
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
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

use bevy::a11y::AccessibilityNode;
use bevy::ecs::{component::HookContext, world::DeferredWorld};
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
pub struct StyledProgress {
    pub value: f32,
    pub root_color: Option<Color>,
    pub indicator_color: Option<Color>,
    pub size: Option<ProgressSize>,
}

impl StyledProgress {
    pub fn builder() -> super::builder::ProgressBuilder {
        super::builder::ProgressBuilder::default()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
pub enum ProgressSize {
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

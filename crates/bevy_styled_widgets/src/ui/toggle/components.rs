use bevy::a11y::AccessibilityNode;
use bevy::ecs::{component::HookContext, system::SystemId, world::DeferredWorld};
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
pub struct StyledToggle {
    pub active: bool,
    #[reflect(ignore)]
    pub on_change: Option<SystemId<In<(Entity, bool)>>>,
    pub label: Option<String>,
    #[reflect(ignore)]
    pub variant: ToggleVariant,
    #[reflect(ignore)]
    pub size: Option<ToggleSize>,
    pub disabled: bool,
    pub icon: Option<String>,
}

impl StyledToggle {
    pub fn builder() -> super::builder::ToggleBuilder {
        super::builder::ToggleBuilder::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum ToggleVariant {
    Default,
    Outline,
    WithText,
}

impl Default for ToggleVariant {
    fn default() -> Self {
        ToggleVariant::Default
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum ToggleSize {
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

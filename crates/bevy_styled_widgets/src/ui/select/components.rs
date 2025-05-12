use super::builder::SelectItemBuilder;
use bevy::a11y::AccessibilityNode;
use bevy::ecs::{component::HookContext, system::SystemId, world::DeferredWorld};
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
pub struct StyledSelectItem {
    pub selected: bool,
    #[reflect(ignore)]
    pub on_change: Option<SystemId<In<bool>>>,
    pub disabled: bool,
    pub key: Option<String>,
    pub value: String,
}

impl StyledSelectItem {
    pub fn builder() -> super::builder::SelectItemBuilder {
        super::builder::SelectItemBuilder::default()
    }
}

#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
pub struct StyledSelect {
    #[reflect(ignore)]
    pub options: Vec<SelectItemBuilder>,
    #[reflect(ignore)]
    pub on_click: Option<SystemId<In<bool>>>,
    #[reflect(ignore)]
    pub on_change: Option<SystemId<In<Entity>>>,
    pub selected_value: Option<String>,
    pub disabled: bool,
}

impl StyledSelect {
    pub fn builder() -> super::builder::SelectBuilder {
        super::builder::SelectBuilder::default()
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

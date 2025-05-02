use bevy::a11y::AccessibilityNode;
use bevy::ecs::{component::HookContext, system::SystemId, world::DeferredWorld};
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
pub struct StyledCheckbox {
    pub checked: bool,
    #[reflect(ignore)]
    pub on_change: Option<SystemId<In<bool>>>,
    pub check_mark: Option<String>,
    #[reflect(ignore)]
    pub variant: CheckboxVariant,
    #[reflect(ignore)]
    pub size: Option<CheckboxSize>,
    pub disabled: bool,
}

impl StyledCheckbox {
    pub fn builder() -> super::builder::CheckboxBuilder {
        super::builder::CheckboxBuilder::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum CheckboxVariant {
    Default,
    WithText,
}

impl Default for CheckboxVariant {
    fn default() -> Self {
        CheckboxVariant::Default
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum CheckboxSize {
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

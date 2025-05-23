use super::builder::RadioButtonBuilder;
use bevy::a11y::AccessibilityNode;
use bevy::ecs::{component::HookContext, system::SystemId, world::DeferredWorld};
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
pub struct StyledRadioButton {
    pub checked: bool,
    #[reflect(ignore)]
    pub on_change: Option<SystemId<In<bool>>>,
    #[reflect(ignore)]
    pub variant: RadioButtonVariant,
    #[reflect(ignore)]
    pub size: Option<RadioButtonSize>,
    pub disabled: bool,
    pub caption: Option<String>,
    pub value: String,
}

impl StyledRadioButton {
    pub fn builder() -> super::builder::RadioButtonBuilder {
        super::builder::RadioButtonBuilder::default()
    }
}

#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
pub struct StyledRadioGroup {
    #[reflect(ignore)]
    pub buttons: Vec<RadioButtonBuilder>,
    #[reflect(ignore)]
    pub on_change: Option<SystemId<In<Entity>>>,
}

impl StyledRadioGroup {
    pub fn builder() -> super::builder::RadioGroupBuilder {
        super::builder::RadioGroupBuilder::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum RadioButtonVariant {
    Default,
}

impl Default for RadioButtonVariant {
    fn default() -> Self {
        RadioButtonVariant::Default
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum RadioButtonSize {
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

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadioButtonDirection {
    #[default]
    Vertical,
    Horizontal,
}
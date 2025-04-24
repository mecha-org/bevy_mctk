use bevy::a11y::AccessibilityNode;
use bevy::ecs::{component::HookContext, system::SystemId, world::DeferredWorld};
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
pub struct StyledSwitch {
    #[reflect(ignore)]
    pub variant: SwitchVariant,
    pub state: bool,
    #[reflect(ignore)]
    pub on_change: Option<SystemId<In<(Entity, bool)>>>,
    pub on_color: Option<Color>,
    pub off_color: Option<Color>,
    pub knob_color: Option<Color>,
    pub border_color: Option<Color>,
    pub hover_color: Option<Color>,
    pub on_text_color: Option<Color>,
    pub off_text_color: Option<Color>,
    pub on_label: Option<String>,
    pub off_label: Option<String>,
    #[reflect(ignore)]
    pub size: Option<SwitchSize>,
    pub disabled: bool,
}

impl StyledSwitch {
    pub fn builder() -> super::builder::SwitchBuilder {
        super::builder::SwitchBuilder::default()
    }
}

/// Toggle switch variants styled like common UI systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum SwitchVariant {
    Rounded,             // Pill-shaped, soft rounded toggle
    Rectangular, // Sharp corners, rectangular switch with text
}

impl Default for SwitchVariant {
    fn default() -> Self {
        SwitchVariant::Rounded
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum SwitchSize {
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

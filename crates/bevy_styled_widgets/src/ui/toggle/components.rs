use bevy::a11y::AccessibilityNode;
use bevy::ecs::{component::HookContext, system::SystemId, world::DeferredWorld};
use bevy::prelude::*;

#[derive(Component)]
pub struct StyledToggleButton {
    pub active: bool,
    pub on_change: Option<fn(bool)>,
    pub label: String,
    pub variant: ToggleVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToggleVariant {
    Rounded,
    RectangularWithText,
}

impl Default for ToggleVariant {
    fn default() -> Self {
        ToggleVariant::RectangularWithText
    }
}

#[derive(Debug, Clone)]
pub struct ButtonTheme {
    pub active_background: Color,
    pub inactive_background: Color,
    pub active_text_color: Color,
    pub inactive_text_color: Color,
}

impl Default for ButtonTheme {
    fn default() -> Self {
        Self {
            active_background: Color::rgb(0.2, 0.7, 0.3),
            inactive_background: Color::rgb(0.3, 0.3, 0.3),
            active_text_color: Color::WHITE,
            inactive_text_color: Color::GRAY,
        }
    }
}

#[derive(Component, Default)]
#[component(immutable, on_add = on_set_label, on_replace = on_set_label)]
pub struct AccessibleName(pub String);

fn on_set_label(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    let name = entt.get::<AccessibleName>().unwrap().0.clone();
    if let Some(mut accessibility) = entt.get_mut::<AccessibilityNode>() {
        accessibility.set_label(&name);
    }
}

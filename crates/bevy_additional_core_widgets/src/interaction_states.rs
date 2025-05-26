use bevy::{
    a11y::AccessibilityNode,
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::Component,
};

/// Component that indicates whether the select widget has a drodown.
#[derive(Component, Default, Debug)]
#[component(immutable, on_add = update_expanded_a11y, on_replace = update_expanded_a11y)]
pub struct DropdownOpen(pub bool);

pub fn update_expanded_a11y(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    let is_open = entt.get::<DropdownOpen>().unwrap().0;

    if let Some(mut accessibility_node) = entt.get_mut::<AccessibilityNode>() {
        accessibility_node.set_expanded(is_open);
        accessibility_node.set_has_popup(accesskit::HasPopup::Listbox); // Set to Listbox
    } else {
        eprintln!("Error in update_expanded_a11y()");
    }
}

/// Component that indicates whether the item is selected
#[derive(Component, Default, Debug)]
#[component(on_add = update_is_selected_a11y, on_replace = update_is_selected_a11y)]
pub struct IsSelected(pub bool);

pub fn update_is_selected_a11y(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    let is_selected = entt.get::<IsSelected>().unwrap().0;

    if let Some(mut accessibility_node) = entt.get_mut::<AccessibilityNode>() {
        accessibility_node.set_selected(is_selected);
    } else {
        eprintln!("Error in update_is_selected_a11y()");
    }
}

/// Component that indicates the selected item

#[derive(Component, Debug)]
#[component(on_add = update_selected_item_a11y, on_replace = update_selected_item_a11y)]
pub struct SelectedItem {
    pub label: String,
    pub value: String,
}

pub fn update_selected_item_a11y(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);

    let (label, value) = {
        let selected_item = entt.get::<SelectedItem>().unwrap();
        (selected_item.label.clone(), selected_item.value.clone())
    };

    if let Some(mut accessibility_node) = entt.get_mut::<AccessibilityNode>() {
        accessibility_node.set_label(label);
        accessibility_node.set_value(value);
    } else {
        eprintln!("Error in update_selected_item_a11y()");
    }
}

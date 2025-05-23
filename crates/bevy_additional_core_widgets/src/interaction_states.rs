use bevy::{
    a11y::AccessibilityNode,
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::Component,
};
/// Component that indicates whether the select widget has a popup.
#[derive(Component, Default, Debug)]
#[component(immutable, on_add = on_add_has_popup, on_replace = on_add_has_popup)]
pub struct SelectHasPopup(pub bool);

// Hook to set the a11y "HasPopup" state when the select widget is added or updated.
pub fn on_add_has_popup(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);
    let has_popup = entt.get::<SelectHasPopup>().unwrap().0;

    if let Some(mut accessibility) = entt.get_mut::<AccessibilityNode>() {
        if has_popup == true {
            accessibility.set_has_popup(accesskit::HasPopup::Listbox); // Set to Listbox
        } else {
            accessibility.clear_has_popup(); // Clear the HasPopup property
        }
    } else {
        eprintln!("Error in on_add_has_popup()");
    }
}

/// Component that indicates the state of a ListBoxOption.
#[derive(Component, Default, Debug)]
#[component(on_add = on_add_listbox_option)]
pub struct ListBoxOptionState {
    pub label: String,
    pub is_selected: bool,
}

// Hook to set the a11y properties for a ListBoxOption when added or updated.
fn on_add_listbox_option(mut world: DeferredWorld, context: HookContext) {
    let mut entt = world.entity_mut(context.entity);

    let (label, is_selected) = {
        let state = entt.get::<ListBoxOptionState>().unwrap();
        (state.label.clone(), state.is_selected)
    };

    if let Some(mut accessibility) = entt.get_mut::<AccessibilityNode>() {
        accessibility.set_label(&*label);

        // Set the selected state
        if is_selected {
            accessibility.set_selected(true);
        } else {
            accessibility.clear_selected();
        }
    } else {
        eprintln!("Error in on_add_has_popup()");
    }
}

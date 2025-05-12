use bevy::{color::palettes::css::LIGHT_GRAY, prelude::*};
use bevy_additional_core_widgets::{
    CoreSelectContent, CoreSelectItem, CoreSelectTrigger, ListBoxOptionState, SelectHasPopup,
};
use bevy_core_widgets::{Checked, InteractionDisabled, ValueChange, hover::Hovering};

use super::{SelectContent, StyledSelect, StyledSelectItem, builder::SelectedValue};

#[allow(clippy::type_complexity)]
pub fn on_select_triggered(
    mut trigger: Trigger<ValueChange<SelectHasPopup>>,
    query: Query<&StyledSelect>,
    mut commands: Commands,
) {
    trigger.propagate(false);

    let clicked = &trigger.event().0;
    let entity = trigger.target();

    commands.entity(entity).insert(SelectHasPopup(clicked.0));

    if let Ok(styled_select) = query.get(entity) {
        if styled_select.disabled {
            commands.entity(entity).insert(InteractionDisabled);
        }
        if let Some(system_id) = styled_select.on_click {
            // Defer the callback system using commands
            commands.run_system_with(system_id, clicked.0);
        }
    }
}

pub fn open_select_popup(
    mut content_query: Query<&mut Node, With<SelectContent>>,
    has_popup_query: Query<&SelectHasPopup, Changed<SelectHasPopup>>,
) {
    for SelectHasPopup(is_open) in has_popup_query.iter() {
        if *is_open {
            for mut content in content_query.iter_mut() {
                content.display = Display::Flex;
            }
        } else {
            for mut content in content_query.iter_mut() {
                content.display = Display::None;
            }
        }
    }
}

pub fn on_select_item_selection(
    mut trigger: Trigger<ValueChange<Entity>>,
    q_select_content: Query<&Children, With<CoreSelectContent>>,
    q_select_item: Query<(&ChildOf, &SelectedValue), With<CoreSelectItem>>,
    q_select_trigger: Query<(&StyledSelect, &Children), With<CoreSelectTrigger>>,
    mut q_text: Query<&mut Text>,
    mut q_name: Query<&mut Name>,
    mut commands: Commands,
) {
    trigger.propagate(false);

    let target = trigger.target();

    // Ensure the trigger target is CoreSelectContent
    if !q_select_content.contains(target) {
        return;
    }

    let selected_entity = trigger.event().0;

    // Get the selected item's value and the parent content it belongs to
    let (child_of, selected_value) = match q_select_item.get(selected_entity) {
        Ok(res) => res,
        Err(_) => return,
    };

    let group_children = match q_select_content.get(child_of.parent()) {
        Ok(children) => children,
        Err(_) => return,
    };

    // Deselect all others in the same CoreSelectContent group
    for child in group_children.iter() {
        if let Ok((_, value)) = q_select_item.get(child) {
            commands
                .entity(child)
                .insert(Checked(value.0 == selected_value.0));
        }
    }

    // Update the trigger text
    for (_styled_select, trigger_children) in q_select_trigger.iter() {
        for child in trigger_children.iter() {
            if let Ok(mut text) = q_text.get_mut(child) {
                text.0 = selected_value.0.clone();
            }

            if let Ok(mut name) = q_name.get_mut(child) {
                name.set(selected_value.0.clone());
            }
        }
    }

    // Close dropdown
    commands.entity(target).insert(SelectHasPopup(false));
}

#[allow(clippy::type_complexity)]
pub fn update_select_visuals(
    mut query_set: ParamSet<(
        Query<
            (
                &Hovering,
                &SelectHasPopup,
                &mut BackgroundColor,
                Has<InteractionDisabled>,
            ),
            (With<CoreSelectTrigger>, With<StyledSelect>),
        >,
        Query<
            (
                &Hovering,
                &mut BackgroundColor,
                &ListBoxOptionState,
                Has<InteractionDisabled>,
                &StyledSelectItem,
                &Checked,
            ),
            With<CoreSelectItem>,
        >,
    )>,
) {
    // Query 0: Trigger
    for (hovering, has_popup, mut bg_color, is_disabled) in query_set.p0().iter_mut() {
        if is_disabled {
            *bg_color = BackgroundColor(LIGHT_GRAY.into());
        } else if has_popup.0 {
            *bg_color = BackgroundColor(bevy::color::palettes::css::LIGHT_GRAY.into());
        } else if has_popup.0 || hovering.0 {
            *bg_color = BackgroundColor(bevy::color::palettes::css::LIGHT_GRAY.into());
        } else {
            *bg_color = BackgroundColor(bevy::color::palettes::css::GRAY.into());
        }
    }

    // Query 1: Items
    for (hovering, mut bg_color, option_state, is_disabled, item, Checked(checked)) in
        query_set.p1().iter_mut()
    {
        if item.disabled || is_disabled {
            *bg_color = BackgroundColor(LIGHT_GRAY.into());
        } else if hovering.0 {
            *bg_color = BackgroundColor(bevy::color::palettes::css::LIGHT_GRAY.into());
        } else if item.selected || option_state.is_selected || *checked {
            *bg_color = BackgroundColor(bevy::color::palettes::css::DARK_GREY.into());
        } else {
            *bg_color = BackgroundColor(bevy::color::palettes::css::GRAY.into());
        }
    }
}

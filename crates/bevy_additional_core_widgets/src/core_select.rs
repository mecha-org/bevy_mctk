use accesskit::Role;
use bevy::{
    a11y::AccessibilityNode,
    ecs::system::SystemId,
    input_focus::{InputFocus, InputFocusVisible},
    prelude::*,
};

use bevy_core_widgets::{ButtonClicked, Checked, InteractionDisabled, ValueChange};

use crate::{ListBoxOptionState, interaction_states::SelectHasPopup};

#[derive(Component, Debug)]
#[require(AccessibilityNode(accesskit::Node::new(Role::ListBox)), SelectHasPopup)]
pub struct CoreSelectTrigger {
    pub on_click: Option<SystemId<In<bool>>>,
}

pub fn select_on_pointer_click(
    mut trigger: Trigger<Pointer<Click>>,
    q_state: Query<(
        &CoreSelectTrigger,
        &SelectHasPopup,
        Has<InteractionDisabled>,
    )>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    mut commands: Commands,
) {
    if let Ok((select_trigger, SelectHasPopup(clicked), disabled)) = q_state.get(trigger.target()) {
        let select_id = trigger.target();
        focus.0 = Some(select_id);
        focus_visible.0 = false;
        trigger.propagate(false);

        if !disabled {
            let is_open = clicked;
            let new_clicked = !is_open;

            if let Some(on_click) = select_trigger.on_click {
                commands.run_system_with(on_click, new_clicked);
            } else {
                commands.trigger_targets(ValueChange(SelectHasPopup(new_clicked)), select_id);
            }
        }
    }
}

#[derive(Component, Debug)]
#[require(AccessibilityNode(accesskit::Node::new(Role::ListBox)))]
pub struct CoreSelectContent {
    pub on_change: Option<SystemId<In<Entity>>>,
}

fn select_root_on_button_click(
    mut trigger: Trigger<ButtonClicked>,
    q_group: Query<(&CoreSelectContent, &Children)>,
    q_select_item: Query<(&Checked, &ChildOf, Has<InteractionDisabled>), With<CoreSelectItem>>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    mut commands: Commands,
) {
    let select_id = trigger.target();

    // Find the select item button that was clicked.
    let Ok((_, child_of, _)) = q_select_item.get(select_id) else {
        return;
    };

    // Find the parent CoreSelectContent of the clicked select item button.
    let group_id = child_of.parent();
    let Ok((CoreSelectContent { on_change }, group_children)) = q_group.get(group_id) else {
        warn!("select item button clicked without a valid CoreSelectContent parent");
        return;
    };

    // Set focus to group and hide focus ring
    focus.0 = Some(group_id);
    focus_visible.0 = false;

    // Get all the select root children.
    let select_children = group_children
        .iter()
        .filter_map(|child_id| match q_select_item.get(child_id) {
            Ok((checked, _, false)) => Some((child_id, checked.0)),
            Ok((_, _, true)) => None,
            Err(_) => None,
        })
        .collect::<Vec<_>>();

    if select_children.is_empty() {
        return; // No enabled select item buttons in the group
    }

    trigger.propagate(false);
    let current_select_item = select_children
        .iter()
        .find(|(_, checked)| *checked)
        .map(|(id, _)| *id);

    if current_select_item == Some(select_id) {
        // If they clicked the currently checked radio button, do nothing
        return;
    }

    // Trigger the on_change event for the newly checked radio button
    if let Some(on_change) = on_change {
        commands.run_system_with(*on_change, select_id);
    } else {
        commands.trigger_targets(ValueChange(select_id), group_id);
    }
}

#[derive(Component, Debug)]
#[require(AccessibilityNode(accesskit::Node::new(Role::ListBoxOption)), Checked)]
pub struct CoreSelectItem;

fn select_item_on_pointer_click(
    mut trigger: Trigger<Pointer<Click>>,
    q_state: Query<(&Checked, Has<InteractionDisabled>), With<CoreSelectItem>>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    mut commands: Commands,
) {
    if let Ok((checked, disabled)) = q_state.get(trigger.target()) {
        let checkbox_id = trigger.target();
        focus.0 = Some(checkbox_id);
        focus_visible.0 = false;
        trigger.propagate(false);
        if checked.0 || disabled {
            // If the radio is already checked, or disabled, we do nothing.
            return;
        }
        commands.trigger_targets(ButtonClicked, trigger.target());
    }
}

// -----

pub struct CoreSelectPlugin;

impl Plugin for CoreSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(select_on_pointer_click)
            .add_observer(select_root_on_button_click)
            .add_observer(select_item_on_pointer_click);
    }
}

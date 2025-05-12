use accesskit::Role;
use bevy::{
    a11y::AccessibilityNode,
    ecs::system::SystemId,
    input::{ButtonState, keyboard::KeyboardInput},
    input_focus::{FocusedInput, InputFocus, InputFocusVisible},
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

fn select_content_on_button_click(
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
        // If they clicked the currently checked item, do nothing
        return;
    }

    // Trigger the on_change event for the newly checked item
    if let Some(on_change) = on_change {
        commands.run_system_with(*on_change, select_id);
    } else {
        commands.trigger_targets(ValueChange(select_id), group_id);
    }
}

fn select_content_on_key_input(
    mut trigger: Trigger<FocusedInput<KeyboardInput>>,
    q_group: Query<(&CoreSelectContent, &Children)>,
    q_select_item: Query<(&Checked, &ChildOf, Has<InteractionDisabled>), With<CoreSelectItem>>,
    mut commands: Commands,
) {
    if let Ok((CoreSelectContent { on_change }, group_children)) = q_group.get(trigger.target()) {
        let event = &trigger.event().input;
        if event.state == ButtonState::Pressed
            && !event.repeat
            && matches!(
                event.key_code,
                KeyCode::ArrowUp
                    | KeyCode::ArrowDown
                    | KeyCode::ArrowLeft
                    | KeyCode::ArrowRight
                    | KeyCode::Home
                    | KeyCode::End
            )
        {
            let key_code = event.key_code;
            trigger.propagate(false);

            let select_children = group_children
                .iter()
                .filter_map(|child_id| match q_select_item.get(child_id) {
                    Ok((checked, _, false)) => Some((child_id, checked.0)),
                    Ok((_, _, true)) => None,
                    Err(_) => None,
                })
                .collect::<Vec<_>>();

            if select_children.is_empty() {
                return; // No select items in the group
            }
            let current_index = select_children
                .iter()
                .position(|(_, checked)| *checked)
                .unwrap_or(usize::MAX); // Default to invalid index if none are checked

            let next_index = match key_code {
                KeyCode::ArrowUp | KeyCode::ArrowLeft => {
                    // Navigate to the previous select item in the group
                    if current_index == 0 {
                        // If we're at the first one, wrap around to the last
                        select_children.len() - 1
                    } else {
                        // Move to the previous one
                        current_index - 1
                    }
                }
                KeyCode::ArrowDown | KeyCode::ArrowRight => {
                    // Navigate to the next select item in the group
                    if current_index >= select_children.len() - 1 {
                        // If we're at the last one, wrap around to the first
                        0
                    } else {
                        // Move to the next one
                        current_index + 1
                    }
                }
                KeyCode::Home => {
                    // Navigate to the first select item in the group
                    0
                }
                KeyCode::End => {
                    // Navigate to the last select item in the group
                    select_children.len() - 1
                }
                _ => {
                    return;
                }
            };

            if current_index == next_index {
                // If the next index is the same as the current, do nothing
                return;
            }

            let (next_id, _) = select_children[next_index];

            // Trigger the on_change event for the newly checked select item
            if let Some(on_change) = on_change {
                commands.run_system_with(*on_change, next_id);
            } else {
                commands.trigger_targets(ValueChange(next_id), trigger.target());
            }
        }
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
            .add_observer(select_content_on_button_click)
            .add_observer(select_content_on_key_input)
            .add_observer(select_item_on_pointer_click);
    }
}

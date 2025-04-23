use accesskit::Role;
use bevy::{
    a11y::AccessibilityNode,
    ecs::system::SystemId,
    input::{ButtonState, keyboard::KeyboardInput},
    input_focus::{FocusedInput, InputFocus, InputFocusVisible},
    prelude::*,
};

use bevy_core_widgets::{Checked, InteractionDisabled, ValueChange};

#[derive(Component, Debug)]
#[require(AccessibilityNode(accesskit::Node::new(Role::Switch)), Checked)]
pub struct CoreSwitch {
    pub on_change: Option<SystemId<In<(Entity, bool)>>>,
}

fn switch_on_key_input(
    mut trigger: Trigger<FocusedInput<KeyboardInput>>,
    q_state: Query<(&CoreSwitch, &Checked, Has<InteractionDisabled>)>,
    mut commands: Commands,
) {
    if let Ok((switch, checked, disabled)) = q_state.get(trigger.target()) {
        let event = &trigger.event().input;
        if !disabled
            && event.state == ButtonState::Pressed
            && !event.repeat
            && (event.key_code == KeyCode::Enter || event.key_code == KeyCode::Space)
        {
            trigger.propagate(false);
            let is_on = checked.0;
            let new_checked = !is_on;
            let entity = trigger.target();

            if let Some(on_change) = switch.on_change {
                commands.run_system_with(on_change, (entity, new_checked));
            } else {
                commands.trigger_targets(ValueChange(new_checked), entity);
            }
        }
    }
}

fn switch_on_pointer_click(
    mut trigger: Trigger<Pointer<Click>>,
    q_state: Query<(&CoreSwitch, &Checked, Has<InteractionDisabled>)>,
    mut focus: ResMut<InputFocus>,
    mut focus_visible: ResMut<InputFocusVisible>,
    mut commands: Commands,
) {
    if let Ok((switch, checked, disabled)) = q_state.get(trigger.target()) {
        let entity = trigger.target();
        focus.0 = Some(entity);
        focus_visible.0 = false;
        trigger.propagate(false);

        if !disabled {
            let is_on = checked.0;
            let new_checked = !is_on;

            if let Some(on_change) = switch.on_change {
                commands.run_system_with(on_change, (entity, new_checked));
            } else {
                commands.trigger_targets(ValueChange(new_checked), entity);
            }
        }
    }
}

pub struct CoreSwitchPlugin;

impl Plugin for CoreSwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(switch_on_key_input)
            .add_observer(switch_on_pointer_click);
    }
}

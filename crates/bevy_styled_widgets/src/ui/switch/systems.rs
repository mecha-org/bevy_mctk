use bevy::{prelude::*, ui};
use bevy_core_widgets::{Checked, InteractionDisabled, ValueChange, hover::Hovering};

use crate::themes::ThemeManager;

use super::{
    SwitchSize,
    builder::RootComponent,
    components::{StyledSwitch, SwitchVariant},
};

#[allow(clippy::type_complexity)]
pub fn update_switch_colors(
    theme_manager: Res<ThemeManager>,
    mut query: Query<
        (
            &StyledSwitch,
            &Hovering,
            &Checked,
            Has<InteractionDisabled>,
            &Children,
        ),
        (With<RootComponent>,),
    >,
    mut q_border_color: Query<
        (&mut BorderColor, &mut BackgroundColor, &mut Children),
        Without<RootComponent>,
    >,
    mut q_knob: Query<
        (&mut BackgroundColor, &mut Node),
        (Without<Children>, Without<RootComponent>, Without<Text>),
    >,
    mut q_text: Query<(&mut Text, &mut Node, &mut TextColor)>,
) {
    for (switch, Hovering(is_hovering), Checked(checked), is_disabled, children) in query.iter_mut()
    {
        let switch_styles = theme_manager.styles.switches.clone();
        let switch_size_styles = theme_manager.styles.switch_sizes.clone();

        // Select switch style based on variant
        let switch_style = match switch.variant {
            SwitchVariant::Rounded => switch_styles.rounded,
            SwitchVariant::Rectangular => switch_styles.rectangular,
        };

        //Update size styles
        let switch_size_style = match switch.size.unwrap_or_default() {
            SwitchSize::XSmall => switch_size_styles.xsmall,
            SwitchSize::Small => switch_size_styles.small,
            SwitchSize::Medium => switch_size_styles.medium,
            SwitchSize::Large => switch_size_styles.large,
            SwitchSize::XLarge => switch_size_styles.xlarge,
        };

        let Some(track_id) = children.first() else {
            continue;
        };

        let Ok((mut border_color, mut bg_color, track_children)) =
            q_border_color.get_mut(*track_id)
        else {
            continue;
        };

        match (is_disabled, *checked, is_hovering) {
            (true, true, _) => {
                bg_color.0 = switch_style.disabled_on_background;
                border_color.0 = switch_style.border_color;
            }
            (true, false, _) => {
                bg_color.0 = switch_style.disabled_off_background;
                border_color.0 = switch_style.border_color;
            }
            (_, true, true) => {
                bg_color.0 = switch_style.hovered_background;
                border_color.0 = switch_style.border_color;
            }
            (_, false, true) => {
                bg_color.0 = switch_style.hovered_background;
                border_color.0 = switch_style.border_color;
            }
            (_, true, false) => {
                bg_color.0 = switch_style.on_background;
                border_color.0 = switch_style.border_color;
            }
            (_, false, false) => {
                bg_color.0 = switch_style.off_background;
                border_color.0 = switch_style.border_color;
            }
        }

        let Some(knob_id) = track_children.first() else {
            continue;
        };
        let Ok((mut knob_bg, mut node)) = q_knob.get_mut(*knob_id) else {
            continue;
        };

        if switch.disabled {
            knob_bg.0 = switch_style.disabled_knob_color;
            continue; // Skip updating visuals or responding to events
        }

        if *checked {
            knob_bg.0 = switch_style.knob_color;
            node.left = ui::Val::Px(switch_size_style.knob_offset_x_on)
        } else {
            knob_bg.0 = switch_style.knob_color;
            node.left = ui::Val::Px(switch_size_style.knob_offset_x)
        };

        if switch.variant == SwitchVariant::Rectangular {
            // label
            let label = if *checked {
                switch.on_label.clone().unwrap_or("ON".into())
            } else {
                switch.off_label.clone().unwrap_or("OFF".into())
            };

            let label_id = track_children[1];
            if let Ok((mut text, mut node, mut text_color)) = q_text.get_mut(label_id) {
                **text = if *checked {
                    label.to_string()
                } else {
                    label.to_string()
                };

                node.left = if *checked {
                    Val::Px(switch_size_style.label_offset_on)
                } else {
                    Val::Px(switch_size_style.label_offset)
                };

                text_color.0 = if *checked {
                    switch_style.on_text_color
                } else {
                    switch_style.off_text_color
                };
            }
        }
    }
}

pub fn on_switch_changed(
    mut trigger: Trigger<ValueChange<bool>>,
    query: Query<&StyledSwitch>,
    mut commands: Commands,
) {
    trigger.propagate(false);

    let checked = trigger.event().0;
    let entity = trigger.target();

    commands.entity(entity).insert(Checked(checked));

    if let Ok(styled_switch) = query.get(entity) {
        if styled_switch.disabled {
            commands.entity(entity).insert(InteractionDisabled);
        }
        if let Some(system_id) = styled_switch.on_change {
            // Defer the callback system using commands
            commands.run_system_with(system_id, (entity, checked));
        }
    }
}

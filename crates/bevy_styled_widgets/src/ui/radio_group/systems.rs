use bevy::prelude::*;
use bevy_core_widgets::{Checked, InteractionDisabled, ValueChange, hover::Hovering};

use super::{
    RadioButtonSize,
    builder::{RadioValue, RootComponent},
    components::{RadioButtonVariant, StyledRadioButton},
};
use crate::themes::ThemeManager;
use bevy_core_widgets::{CoreRadio, CoreRadioGroup};

#[allow(clippy::type_complexity)]
pub fn update_radio_button_visuals(
    theme_manager: Res<ThemeManager>,
    mut query: Query<
        (
            &mut Node,
            &StyledRadioButton,
            &Hovering,
            &Checked,
            Has<InteractionDisabled>,
            &Children,
        ),
        (With<RootComponent>,),
    >,
    mut q_border_color: Query<(&mut BorderColor, &mut Children), Without<RootComponent>>,
    mut q_inner: Query<&mut BackgroundColor>,
    mut q_caption_text: Query<&mut TextColor>,
) {
    for (
        mut radio_button_node,
        radio_button,
        Hovering(is_hovering),
        Checked(checked),
        is_disabled,
        children,
    ) in query.iter_mut()
    {
        let radio_button_styles = theme_manager.styles.radio_buttons.clone();

        let radio_button_style = match radio_button.variant {
            RadioButtonVariant::Default => radio_button_styles.default,
        };

        let Some(radio_button_id) = children.first() else {
            continue;
        };
        let Some(caption_id) = children.get(1) else {
            continue;
        };

        // Outer ring node
        let Ok((mut border_color, main_children)) = q_border_color.get_mut(*radio_button_id) else {
            continue;
        };

        // Inner circle
        let inner_component_id = main_children[0];
        if let Ok(mut bg_color) = q_inner.get_mut(inner_component_id) {
            let (new_bg, new_border_color) = match (*checked, is_disabled, is_hovering) {
                (true, true, _) => (
                    radio_button_style.disabled_inner_checked_background,
                    radio_button_style.disabled_outer_border,
                ),
                (false, true, _) => (
                    radio_button_style.disabled_inner_unchecked_background,
                    radio_button_style.disabled_outer_border,
                ),
                (true, false, true) => (
                    radio_button_style.hovered_inner_checked_background,
                    radio_button_style.hovered_outer_border,
                ),
                (false, false, true) => (
                    radio_button_style.hovered_inner_unchecked_background,
                    radio_button_style.hovered_outer_border,
                ),
                (true, false, false) => (
                    radio_button_style.inner_checked_background,
                    radio_button_style.outer_border,
                ),
                (false, false, false) => (
                    radio_button_style.inner_unchecked_background,
                    radio_button_style.outer_border,
                ),
            };

            bg_color.0 = new_bg;
            border_color.0 = new_border_color;
        }

        // Caption node
        if let Ok(mut caption_color) = q_caption_text.get_mut(*caption_id) {
            caption_color.0 = radio_button_style.caption_color;
        }
    }
}

pub fn observe_radio_group_selection(
    mut trigger: Trigger<ValueChange<Entity>>,
    q_radio_group: Query<&Children, With<CoreRadioGroup>>,
    q_radio: Query<(&ChildOf, &RadioValue), With<CoreRadio>>,
    mut commands: Commands,
) {
    trigger.propagate(false);
    if q_radio_group.contains(trigger.target()) {
        let selected_entity = trigger.event().0;
        let (child_of, radio_value) = q_radio.get(selected_entity).unwrap();
        let group_children = q_radio_group.get(child_of.parent()).unwrap();
        for radio_child in group_children.iter() {
            if let Ok((_, value)) = q_radio.get(radio_child) {
                commands
                    .entity(radio_child)
                    .insert(Checked(value.0 == radio_value.0));
            }
        }
    }
}
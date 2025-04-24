use bevy::prelude::*;
use bevy_core_widgets::{Checked, InteractionDisabled, ValueChange, hover::Hovering};

use crate::themes::ThemeManager;
use super::components::{StyledToggleButton, ToggleVariant};

pub fn update_toggle_button_visuals(
    theme_manager: Res<ThemeManager>,
    mut query: Query<(
        Entity,
        &StyledToggleButton,
        &Hovering,
        Has<InteractionDisabled>,
        &mut BackgroundColor,
        &mut Text,
        &mut TextColor,
    )>,
) {
    for (entity, toggle, Hovering(is_hovering), is_disabled, mut bg_color, mut text, mut text_color) in query.iter_mut() {
        let theme = &theme_manager.styles.toggle_buttons;

        let (new_bg, new_text_color) = match (toggle.active, is_disabled, is_hovering) {
            (true, true, _) => (theme.disabled_active_background, theme.disabled_text_color),
            (false, true, _) => (theme.disabled_inactive_background, theme.disabled_text_color),
            (true, false, true) => (theme.hovered_active_background, theme.active_text_color),
            (false, false, true) => (theme.hovered_inactive_background, theme.inactive_text_color),
            (true, false, false) => (theme.active_background, theme.active_text_color),
            (false, false, false) => (theme.inactive_background, theme.inactive_text_color),
        };

        bg_color.0 = new_bg;
        text.sections[0].value = toggle.label.clone();
        text_color.0 = new_text_color;
    }
}

pub fn on_toggle_button_changed(
    mut trigger: Trigger<ValueChange<bool>>,
    query: Query<&StyledToggleButton>,
    mut commands: Commands,
) {
    info!("in toggle button observer");

    let checked = trigger.event().0;
    let entity = trigger.target();

    if let Ok(toggle) = query.get(entity) {
        if let Some(callback) = toggle.on_change {
            if !commands.get_entity(entity).unwrap().contains::<InteractionDisabled>() {
                callback(checked);
            }
        }

        if !commands.get_entity(entity).unwrap().contains::<InteractionDisabled>() {
            commands.entity(entity).insert(Checked(checked));
        }
    }

    // Always stop propagation
    trigger.propagate(false);
}

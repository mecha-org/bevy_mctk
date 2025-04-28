use bevy::prelude::*;
use bevy_core_widgets::{InteractionDisabled, ValueChange};
use bevy_cosmic_edit::{
    CosmicBackgroundColor, CosmicEditBuffer, CosmicFontSystem, FocusedWidget,
    cosmic_text::{self, Attrs, AttrsOwned},
    input::CosmicTextChanged,
    prelude::focus_on_click,
};

use crate::themes::ThemeManager;

use super::StyledInput;

pub fn update_input_colors(
    theme_manager: Res<ThemeManager>,
    mut q_text_inputs: Query<(&StyledInput, &Children)>,
    mut q_children: Query<(
        &mut CosmicBackgroundColor,
        &mut BorderColor,
        &mut CosmicEditBuffer,
    )>,
    font_system: Option<ResMut<CosmicFontSystem>>,
) {
    let Some(mut font_system) = font_system else {
        return;
    };
    for (_styled_input, children) in q_text_inputs.iter_mut() {
        let text_input_style = theme_manager.styles.input.clone();

        for child in children.iter() {
            if let Ok((mut bg_color, mut border_color, mut buffer)) = q_children.get_mut(child) {
                bg_color.0 = text_input_style.background_color;
                border_color.0 = text_input_style.border_color;

                // Update text color
                let mut attrs = Attrs::new();
                attrs = attrs.color(cosmic_text::Color(
                    text_input_style.text_color.to_linear().as_u32(),
                ));

                let default_attrs = AttrsOwned::new(Attrs::new());
                let text_from_buffer = buffer.get_text_spans(default_attrs);
                let text_value = text_from_buffer
                    .iter()
                    .flat_map(|line| line.iter().map(|(text, _)| text.as_str()))
                    .collect::<String>();

                buffer.set_text(&mut font_system, &text_value, attrs);
            }
        }
    }
}

pub fn on_input_change(
    mut trigger: Trigger<CosmicTextChanged>,
    mut events: EventReader<CosmicTextChanged>,
    query: Query<&StyledInput>,
    mut commands: Commands,
) {
    for event in events.read() {
        let CosmicTextChanged((entity, text_value)) = event;
        info!("Entity {:?} changed text: {:?}", entity.clone(), text_value);
    }
    // Prevent further propagation of this event
    trigger.propagate(false);

    // Extract event data and target entity from the trigger
    let (entity, text_value) = &trigger.event().0;

    info!("Entity {:?} changed text: {:?}", entity, text_value);

    if let Ok(styled_input) = query.get(*entity) {
        if styled_input.disabled {
            commands.entity(*entity).insert(InteractionDisabled);
        }
        if let Some(system_id) = styled_input.on_change {
            // Defer the callback system using commands
            commands.run_system_with(system_id, (*entity, text_value.clone()));
        }
    }
}

pub fn init(mut commands: Commands, mut query: Query<(Entity, &StyledInput)>) {
    for (entity, text_input) in query.iter_mut() {
        if let Some(_text_input) = text_input.value.clone() {
            commands.entity(entity).observe(focus_on_click);
            commands.insert_resource(FocusedWidget(Some(entity)));
        }
    }
}

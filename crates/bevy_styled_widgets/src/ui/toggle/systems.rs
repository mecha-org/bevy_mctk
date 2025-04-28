use bevy::prelude::*;
use bevy_core_widgets::{Checked, InteractionDisabled, ValueChange, hover::Hovering};

use super::{
    ToggleSize,
    builder::RootComponent,
    components::{StyledToggle, ToggleVariant},
};
use crate::themes::ThemeManager;

#[allow(clippy::type_complexity)]
pub fn update_toggle_button_visuals(
    theme_manager: Res<ThemeManager>,
    mut query: Query<
        (
            &mut Node,
            &StyledToggle,
            &Hovering,
            &Checked,
            Has<InteractionDisabled>,
            &Children,
        ),
        (With<RootComponent>,),
    >,
    mut q_background_color: Query<(&mut BackgroundColor, &mut Children), Without<RootComponent>>,
    mut q_text: Query<(&mut Text, &mut TextColor)>,
) {
    for (mut toggle_node, toggle, Hovering(is_hovering), Checked(checked), is_disabled, children) in
        query.iter_mut()
    {
        let toggle_styles = theme_manager.styles.toggles.clone();
        let toggle_size_styles = theme_manager.styles.toggle_sizes.clone();

        // Select switch style based on variant
        let toggle_style = match toggle.variant {
            ToggleVariant::Default => toggle_styles.default,
            ToggleVariant::Outline => toggle_styles.outline,
            ToggleVariant::WithText => toggle_styles.with_text,
        };

        let Some(toggle_id) = children.first() else {
            continue;
        };

        let Ok((mut bg_color, main_children)) = q_background_color.get_mut(*toggle_id) else {
            continue;
        };

        let label_id = main_children[0];
        if let Ok((mut text, mut text_color)) = q_text.get_mut(label_id) {
            **text = toggle.label.clone().unwrap_or("T".into());

            let (new_bg, new_text_color) = match (*checked, is_disabled, is_hovering) {
                (true, true, _) => (
                    toggle_style.disabled_active_background,
                    toggle_style.disabled_text_color,
                ),
                (false, true, _) => (
                    toggle_style.disabled_inactive_background,
                    toggle_style.disabled_text_color,
                ),
                (true, false, true) => (
                    toggle_style.hovered_active_background,
                    toggle_style.active_text_color,
                ),
                (false, false, true) => (
                    toggle_style.hovered_active_background,
                    toggle_style.hovered_text_color,
                ),
                (true, false, false) => (
                    toggle_style.active_background,
                    toggle_style.active_text_color,
                ),
                (false, false, false) => (
                    toggle_style.inactive_background,
                    toggle_style.active_text_color,
                ),
            };

            //Update size styles
            let toggle_size_style = match toggle.size.unwrap_or_default() {
                ToggleSize::XSmall => toggle_size_styles.xsmall,
                ToggleSize::Small => toggle_size_styles.small,
                ToggleSize::Medium => toggle_size_styles.medium,
                ToggleSize::Large => toggle_size_styles.large,
                ToggleSize::XLarge => toggle_size_styles.xlarge,
            };
            toggle_node.padding = UiRect::axes(
                Val::Px(toggle_size_style.padding_horizontal),
                Val::Px(toggle_size_style.padding_vertical),
            );
            if toggle.variant == ToggleVariant::Outline {
                toggle_node.border = UiRect::all(Val::Px(toggle_size_style.border_width));
            }
            bg_color.0 = new_bg;
            text_color.0 = new_text_color;
        }
    }
}

pub fn on_toggle_button_changed(
    mut trigger: Trigger<ValueChange<bool>>,
    query: Query<&StyledToggle>,
    mut commands: Commands,
) {
    trigger.propagate(false);

    let checked = trigger.event().0;
    let entity = trigger.target();

    commands.entity(entity).insert(Checked(checked));

    if let Ok(styled_toggle) = query.get(entity) {
        if styled_toggle.disabled {
            commands.entity(entity).insert(InteractionDisabled);
        }
        if let Some(system_id) = styled_toggle.on_change {
            // Defer the callback system using commands
            commands.run_system_with(system_id, (entity, checked));
        }
    }
}

pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &StyledToggle)>,
) {
    for (entity, toggle) in query.iter_mut() {
        let mut command = commands.entity(entity);

        // if icon is present, add it as a child
        if let Some(icon) = toggle.icon.clone() {
            command.with_children(|parent| {
                parent.spawn((
                    ImageNode::new(asset_server.load(&icon)),
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        ..default()
                    },
                ));
            });
        }
    }
}

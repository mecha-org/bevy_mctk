use bevy::prelude::*;
use bevy_core_widgets::{Checked, InteractionDisabled, ValueChange, hover::Hovering};

use super::{
    CheckboxCheckmarkText, CheckboxSize,
    builder::RootComponent,
    components::{CheckboxVariant, StyledCheckbox},
};
use crate::themes::ThemeManager;

#[allow(clippy::type_complexity)]
pub fn update_checkbox_visuals(
    theme_manager: Res<ThemeManager>,
    mut query: Query<
        (
            &mut Node,
            &StyledCheckbox,
            &Hovering,
            &Checked,
            Has<InteractionDisabled>,
            &Children,
        ),
        (With<RootComponent>,),
    >,
    mut q_background_color: Query<
        (&mut BackgroundColor, &mut BorderColor, &mut Children),
        Without<RootComponent>,
    >,
    mut q_text: Query<(&mut Text, &mut TextColor)>,
) {
    for (
        mut checkbox_node,
        checkbox,
        Hovering(is_hovering),
        Checked(checked),
        is_disabled,
        children,
    ) in query.iter_mut()
    {
        let checkbox_styles = theme_manager.styles.checkboxes.clone();
        let checkbox_size_styles = theme_manager.styles.checkbox_sizes.clone();

        // Select switch style based on variant
        let checkbox_style = match checkbox.variant {
            CheckboxVariant::Default => checkbox_styles.default,
            CheckboxVariant::WithText => checkbox_styles.with_text,
        };

        let Some(checkbox_id) = children.first() else {
            continue;
        };

        let Ok((mut bg_color, mut border_color, main_children)) =
            q_background_color.get_mut(*checkbox_id)
        else {
            continue;
        };

        let check_mark_id = main_children[0];
        if let Ok((mut text, mut text_color)) = q_text.get_mut(check_mark_id) {
            if *checked {
                **text = checkbox.check_mark.clone().unwrap_or("✔".into());
            } else {
                **text = "".into();
            }

            let (new_bg, new_text_color, new_border_color) =
                match (*checked, is_disabled, is_hovering) {
                    (true, true, _) => (
                        checkbox_style.disabled_checked_background,
                        checkbox_style.disabled_text_color,
                        checkbox_style.disabled_border_color,
                    ),
                    (false, true, _) => (
                        checkbox_style.disabled_unchecked_background,
                        checkbox_style.disabled_text_color,
                        checkbox_style.disabled_border_color,
                    ),
                    (true, false, true) => (
                        checkbox_style.hovered_background,
                        checkbox_style.text_color,
                        checkbox_style.border_color,
                    ),
                    (false, false, true) => (
                        checkbox_style.hovered_background,
                        checkbox_style.text_color,
                        checkbox_style.border_color,
                    ),
                    (true, false, false) => (
                        checkbox_style.checked_background,
                        checkbox_style.text_color,
                        checkbox_style.border_color,
                    ),
                    (false, false, false) => (
                        checkbox_style.unchecked_background,
                        checkbox_style.text_color,
                        checkbox_style.border_color,
                    ),
                };

            //Update size styles
            let checkbox_size_style = match checkbox.size.unwrap_or_default() {
                CheckboxSize::XSmall => checkbox_size_styles.xsmall,
                CheckboxSize::Small => checkbox_size_styles.small,
                CheckboxSize::Medium => checkbox_size_styles.medium,
                CheckboxSize::Large => checkbox_size_styles.large,
                CheckboxSize::XLarge => checkbox_size_styles.xlarge,
            };
            checkbox_node.padding = UiRect::axes(
                Val::Px(checkbox_size_style.padding_horizontal),
                Val::Px(checkbox_size_style.padding_vertical),
            );

            bg_color.0 = new_bg;
            text_color.0 = new_text_color;
            border_color.0 = new_border_color
        }

        if children.len() >= 2 {
            let caption_container_id = children[1];
            if let Ok((_, _, caption_desc_children)) =
                q_background_color.get_mut(caption_container_id)
            {
                if caption_desc_children.len() >= 2 {
                    // Caption
                    if let Ok((mut caption_text, mut caption_color)) =
                        q_text.get_mut(caption_desc_children[0])
                    {
                        caption_color.0 = checkbox_style.caption_color;
                    }

                    // Description
                    if let Ok((mut desc_text, mut desc_color)) =
                        q_text.get_mut(caption_desc_children[1])
                    {
                        desc_color.0 = checkbox_style.description_color;
                    }
                }
            }
        }
    }
}

pub fn on_checkbox_changed(
    mut trigger: Trigger<ValueChange<bool>>,
    query: Query<&StyledCheckbox>,
    mut commands: Commands,
) {
    trigger.propagate(false);

    let checked = trigger.event().0;
    let entity = trigger.target();

    commands.entity(entity).insert(Checked(checked));

    if let Ok(styled_checkbox) = query.get(entity) {
        if styled_checkbox.disabled {
            commands.entity(entity).insert(InteractionDisabled);
        }
        if let Some(system_id) = styled_checkbox.on_change {
            // Defer the callback system using commands
            commands.run_system_with(system_id, checked);
        }
    }
}

pub fn setup_checkbox_fonts(
    asset_server: Res<AssetServer>,
    mut query: Query<&mut TextFont, With<CheckboxCheckmarkText>>,
) {
    let font = asset_server.load("fonts/DejaVuSans.ttf");

    for mut text_font in &mut query {
        text_font.font = font.clone();
    }
}

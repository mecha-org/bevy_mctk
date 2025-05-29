use bevy::prelude::*;
use bevy_core_widgets::{
    CoreSlider, InteractionDisabled, SliderDragState, ValueChange, hover::Hovering,
};

use crate::themes::ThemeManager;

use super::{SliderSize, StyledSlider, Thumb, Track};

pub fn on_thumb_changed(
    mut trigger: Trigger<ValueChange<f32>>,
    mut q_slider: Query<&mut CoreSlider>,
    query: Query<&StyledSlider>,
    mut commands: Commands,
) {
    trigger.propagate(false);
    let entity = trigger.target();
    let value = trigger.event().0;

    if let Ok(mut slider) = q_slider.get_mut(trigger.target()) {
        slider.set_value(trigger.event().0);
    }

    if let Ok(styled_slider) = query.get(entity) {
        if styled_slider.disabled {
            commands.entity(entity).insert(InteractionDisabled);
        }
        if let Some(system_id) = styled_slider.on_change {
            // Defer the callback system using commands
            commands.run_system_with(system_id, value);
        }
    }
}

// Update the button's background color.
#[allow(clippy::type_complexity)]
pub fn update_slider_thumb(
    theme_manager: Res<ThemeManager>,
    mut q_slider: Query<
        (
            &CoreSlider,
            &SliderDragState,
            &Hovering,
            Has<InteractionDisabled>,
            &Children,
        ),
        (
            With<StyledSlider>,
            Or<(Added<StyledSlider>, Changed<Hovering>, Changed<CoreSlider>)>,
        ),
    >,
    mut q_track: Query<&mut Children, Without<StyledSlider>>,
    mut q_thumb: Query<
        (&mut BackgroundColor, &mut Node),
        (Without<StyledSlider>, Without<Children>),
    >,
) {
    // Get styles from theme manager
    let slider_styles = theme_manager.styles.slider.clone();

    for (slider_state, drag_state, Hovering(is_hovering), is_disabled, children) in
        q_slider.iter_mut()
    {
        let color: Color = if is_disabled {
            slider_styles.disabled_knob_color
        } else if *is_hovering || drag_state.dragging {
            // If hovering, use a lighter color
            slider_styles.thumb_color.with_alpha(0.9)
        } else {
            // Default color for the slider
            slider_styles.thumb_color
        };

        let Some(track_id) = children.last() else {
            warn!("Slider does not have a track entity.");
            continue;
        };

        let Ok(track_children) = q_track.get_mut(*track_id) else {
            continue;
        };

        let Some(mark_id) = track_children.first() else {
            warn!("Slider does not have a thumb entity.");
            continue;
        };

        let Ok((mut thumb_bg, mut node)) = q_thumb.get_mut(*mark_id) else {
            warn!("Slider thumb lacking a background color or node.");
            continue;
        };

        if thumb_bg.0 != color {
            thumb_bg.0 = color;
        }

        let thumb_position = Val::Percent(slider_state.thumb_position() * 100.0);
        if node.left != thumb_position {
            node.left = thumb_position;
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn change_slider_visuals(
    theme_manager: Res<ThemeManager>,
    children: Query<&mut Children>,
    mut q_node: Query<(&mut Node, &StyledSlider), With<StyledSlider>>,
    mut query: Query<(Entity, &StyledSlider)>,
    mut q_track: Query<&mut BackgroundColor, (With<Track>, Without<Thumb>)>,
    mut q_thumb: Query<&mut BackgroundColor, (With<Thumb>, Without<Track>)>,
) {
    for (mut node, slider) in q_node.iter_mut() {
        let slider_size_styles = theme_manager.styles.slider_sizes.clone();

        let slider_size_style = match slider.size.unwrap_or_default() {
            SliderSize::XSmall => slider_size_styles.xsmall,
            SliderSize::Small => slider_size_styles.small,
            SliderSize::Medium => slider_size_styles.medium,
            SliderSize::Large => slider_size_styles.large,
            SliderSize::XLarge => slider_size_styles.xlarge,
        };
        node.width = Val::Px(slider_size_style.min_width);
        node.height = Val::Px(slider_size_style.min_height);
    }

    for (progress_entity_id, slider) in query.iter_mut() {
        let progress_style = &theme_manager.styles.slider;

        if let Ok(children) = children.get(progress_entity_id) {
            for child in children.iter() {
                if let Ok(mut bg) = q_track.get_mut(child) {
                    bg.0 = if slider.track_color.is_none() {
                        progress_style.track_color
                    } else {
                        slider.track_color.unwrap()
                    };
                }

                if let Ok(mut bg) = q_thumb.get_mut(child) {
                    bg.0 = if slider.thumb_color.is_none() {
                        progress_style.thumb_color
                    } else {
                        slider.thumb_color.unwrap()
                    };
                }
            }
        }
    }
}

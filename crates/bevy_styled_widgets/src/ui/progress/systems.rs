use bevy::prelude::*;

use crate::themes::ThemeManager;

use super::{ProgressIndicator, ProgressRoot, StyledProgress};

#[allow(clippy::type_complexity)]
pub fn update_progress_colors(
    theme_manager: Res<ThemeManager>,
    children: Query<&mut Children>,
    mut query: Query<(Entity, &StyledProgress)>,
    mut q_root: Query<&mut BackgroundColor, (With<ProgressRoot>, Without<ProgressIndicator>)>,
    mut q_indicators: Query<&mut BackgroundColor, (With<ProgressIndicator>, Without<ProgressRoot>)>,
) {
    for (progress_entity_id, progress) in query.iter_mut() {
        let progress_style = &theme_manager.styles.progress;

        if let Ok(children) = children.get(progress_entity_id) {
            for child in children.iter() {
                if let Ok(mut bg) = q_root.get_mut(child) {
                    bg.0 = if progress.root_color.is_none() {
                        progress_style.root_color
                    } else {
                        progress.root_color.unwrap()
                    };
                }

                if let Ok(mut bg) = q_indicators.get_mut(child) {
                    bg.0 = if progress.indicator_color.is_none() {
                        progress_style.indicator_color
                    } else {
                        progress.indicator_color.unwrap()
                    };
                }
            }
        }
    }
}

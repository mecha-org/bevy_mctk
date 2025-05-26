use bevy::prelude::*;
use bevy_additional_core_widgets::{
    CoreSelectContent, CoreSelectItem, CoreSelectTrigger, DropdownOpen, IsSelected,
};
use bevy_core_widgets::{InteractionDisabled, ValueChange, hover::Hovering};

use crate::themes::ThemeManager;

use super::{
    SelectButtonSize, SelectWidget, StyledSelect, StyledSelectItem, builder::SelectedValue,
};

pub fn on_select_triggered(
    mut trigger: Trigger<ValueChange<DropdownOpen>>,
    mut commands: Commands,
    all_triggers: Query<(Entity, &StyledSelect), With<CoreSelectTrigger>>,
) {
    trigger.propagate(false);

    let clicked = &trigger.event().0;

    let clicked_entity = trigger.target();

    // update the clicked one
    for (entity, styled_select) in &all_triggers {
        if styled_select.disabled {
            commands.entity(entity).insert(InteractionDisabled);
            continue;
        }

        if entity == clicked_entity {
            commands.entity(entity).insert(DropdownOpen(clicked.0));
            if let Some(system_id) = styled_select.on_click {
                commands.run_system_with(system_id, clicked.0);
            }
        } else {
            commands.entity(entity).insert(DropdownOpen(false));
        }
    }
}

pub fn open_select_content(
    query_open_widgets: Query<(Entity, &DropdownOpen), Changed<DropdownOpen>>,
    root_query: Query<(&Children, Entity), With<SelectWidget>>,
    mut dropdown_query: Query<(Entity, &mut Node), With<CoreSelectContent>>,
) {
    for (widget_entity, DropdownOpen(is_open)) in &query_open_widgets {
        // Find the root SelectWidget this DropdownOpen belongs to
        for (children, _) in &root_query {
            if children.contains(&widget_entity) {
                // Update its dropdown container
                for &child in children {
                    if let Ok((_, mut node)) = dropdown_query.get_mut(child) {
                        node.display = if *is_open {
                            Display::Flex
                        } else {
                            Display::None
                        };
                    }
                }
            }
        }
    }
}

pub fn on_select_item_selection(
    mut trigger: Trigger<ValueChange<Entity>>,
    q_select_widget: Query<(&Children, Entity), With<SelectWidget>>,
    q_select_content: Query<&Children, With<CoreSelectContent>>,
    q_select_item: Query<(&ChildOf, &SelectedValue, &StyledSelectItem), With<CoreSelectItem>>,
    q_select_trigger: Query<&Children, With<CoreSelectTrigger>>,

    mut q_text: Query<&mut Text>,
    mut q_name: Query<&mut Name>,
    mut commands: Commands,
) {
    trigger.propagate(false);

    let target = trigger.target(); // the CoreSelectContent entity

    // Only proceed if the trigger target is a valid CoreSelectContent
    let group_children = match q_select_content.get(target) {
        Ok(children) => children,
        Err(_) => return,
    };

    let selected_entity = trigger.event().0;

    // Get the selected item's value and its parent (CoreSelectContent)
    let (child_of, selected_value, styled_select_item) = match q_select_item.get(selected_entity) {
        Ok(res) => res,
        Err(_) => return,
    };

    // 1. Find the root SelectWidget this content belongs to
    let mut widget_entity = None;
    for (children, root) in &q_select_widget {
        if children.contains(&target) {
            widget_entity = Some((root, children));
            break;
        }
    }

    let (widget_entity, widget_children) = match widget_entity {
        Some(val) => val,
        None => return,
    };

    // 2. Deselect all other CoreSelectItems in the same content group & set selected
    for child in group_children.iter() {
        if let Ok((_, value, styled_select_item)) = q_select_item.get(child) {
            if !styled_select_item.disabled {
                commands
                    .entity(child)
                    .insert(IsSelected(value.0 == selected_value.0));
            }
        }
    }

    // 3. Update the text and name of the trigger that belongs to this specific widget
    for child in widget_children.iter() {
        if let Ok(trigger_children) = q_select_trigger.get(child) {
            for grandchild in trigger_children.iter() {
                if let Ok(mut text) = q_text.get_mut(grandchild) {
                    text.0 = selected_value.0.clone();
                }
                if let Ok(mut name) = q_name.get_mut(grandchild) {
                    name.set(selected_value.0.clone());
                }
            }
        }
    }

    // 4. Close the dropdown (just for this widget)
    commands.entity(target).insert(DropdownOpen(false));
}

#[allow(clippy::type_complexity)]
pub fn update_select_visuals(
    theme_manager: Res<ThemeManager>,
    mut query_set: ParamSet<(
        Query<
            (
                Entity,
                &mut Node,
                &Hovering,
                &DropdownOpen,
                &mut StyledSelect,
                &mut BackgroundColor,
                &mut BorderColor,
                &mut BorderRadius,
                Has<InteractionDisabled>,
            ),
            (With<StyledSelect>, With<CoreSelectTrigger>),
        >,
        Query<
            (
                Entity,
                &Hovering,
                &mut BackgroundColor,
                Has<InteractionDisabled>,
                &StyledSelectItem,
                &IsSelected,
                &ChildOf,
            ),
            With<CoreSelectItem>,
        >,
        Query<
            (
                Entity,
                &mut Node,
                &mut BackgroundColor,
                &mut BorderColor,
                &mut BorderRadius,
                &StyledSelect,
            ),
            (With<CoreSelectContent>, Without<CoreSelectTrigger>),
        >,
    )>,
) {
    let select_styles = theme_manager.styles.select_styles.clone();

    // Store active roots from the triggers.
    let mut active_roots = Vec::new();
    for (trigger_entity, _, Hovering(is_hovering), DropdownOpen(is_open), _, _, _, _, _) in
        query_set.p0().iter()
    {
        if *is_open || *is_hovering {
            active_roots.push(trigger_entity);
        }
    }

    // === Part 2: Update trigger visuals for active roots only ===
    for (
        trigger_entity,
        mut node,
        Hovering(is_hovering),
        DropdownOpen(is_open),
        select_button,
        mut bg_color,
        mut border_color,
        mut border_radius,
        is_disabled,
    ) in query_set.p0().iter_mut()
    {
        if !active_roots.contains(&trigger_entity) {
            continue;
        }
        let select_size_styles = theme_manager.styles.select_sizes.clone();
        let select_size_style = match select_button.size.unwrap_or_default() {
            SelectButtonSize::XSmall => select_size_styles.xsmall,
            SelectButtonSize::Small => select_size_styles.small,
            SelectButtonSize::Medium => select_size_styles.medium,
            SelectButtonSize::Large => select_size_styles.large,
            SelectButtonSize::XLarge => select_size_styles.xlarge,
        };

        node.border = UiRect::all(Val::Px(select_size_style.border_width));
        border_radius.top_left = Val::Px(select_size_style.border_radius);
        border_radius.top_right = Val::Px(select_size_style.border_radius);
        border_radius.bottom_left = Val::Px(select_size_style.border_radius);
        border_radius.bottom_right = Val::Px(select_size_style.border_radius);

        if is_disabled {
            *bg_color = BackgroundColor(select_styles.disabled_background);
            *border_color = BorderColor(select_styles.disabled_border_color);
        } else if *is_open || *is_hovering {
            *bg_color = BackgroundColor(select_styles.background);
            *border_color = BorderColor(select_styles.active_border_color);
        } else {
            *bg_color = BackgroundColor(select_styles.background);
            *border_color = BorderColor(select_styles.active_border_color);
        }
    }

    // === Part 3: Update CoreSelectItem visuals, including hovered state ===
    for (
        _item_entity,
        hovering,
        mut bg_color,
        is_disabled,
        item,
        IsSelected(is_selected),
        child_of,
    ) in query_set.p1().iter_mut()
    {
        if item.disabled || is_disabled {
            *bg_color = BackgroundColor(select_styles.disabled_background);
        } else if hovering.0 {
            *bg_color = BackgroundColor(select_styles.hovered_item_background);
        } else if *is_selected {
            *bg_color = BackgroundColor(select_styles.active_item_background);
        } else {
            *bg_color = BackgroundColor(select_styles.popover_background);
        }
    }

    // === Part 4: Update CoreSelectContent visuals ===
    for (entity, mut node, mut bg_color, mut border_color, mut border_radius, select) in
        query_set.p2().iter_mut()
    {
        let select_size_styles = theme_manager.styles.select_sizes.clone();
        let select_size_style = match select.size.unwrap_or_default() {
            SelectButtonSize::XSmall => select_size_styles.xsmall,
            SelectButtonSize::Small => select_size_styles.small,
            SelectButtonSize::Medium => select_size_styles.medium,
            SelectButtonSize::Large => select_size_styles.large,
            SelectButtonSize::XLarge => select_size_styles.xlarge,
        };

        // Update the background based on a new field (for example, content_background)
        *bg_color = BackgroundColor(select_styles.popover_background);

        node.border = UiRect::all(Val::Px(select_size_style.border_width));
        *border_color = BorderColor(select_styles.popover_border_color);

        border_radius.top_left = Val::Px(select_size_style.border_radius);
        border_radius.top_right = Val::Px(select_size_style.border_radius);
        border_radius.bottom_left = Val::Px(select_size_style.border_radius);
        border_radius.bottom_right = Val::Px(select_size_style.border_radius);
    }
}

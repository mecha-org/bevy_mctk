use bevy::{
    a11y::AccessibilityNode, color::palettes::css::GRAY, ecs::system::SystemId,
    input_focus::tab_navigation::TabIndex, prelude::*, window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};
use bevy_additional_core_widgets::{
    CoreSelectContent, CoreSelectItem, CoreSelectTrigger, ListBoxOptionState, SelectHasPopup,
};
use bevy_core_widgets::{Checked, hover::Hovering};

use super::{
    StyledSelect,
    components::{AccessibleName, StyledSelectItem},
};
use accesskit::{Node as Accessible, Role};

#[derive(Component, Default)]
pub struct SelectContent;

#[derive(Component, Debug, Clone)]
pub struct SelectedValue(pub String);

#[derive(Default, Clone)]
pub struct SelectItemBuilder {
    pub selected: bool,
    pub on_change: Option<SystemId<In<bool>>>,
    pub disabled: bool,
    pub key: Option<String>,
    pub value: String,
}

impl SelectItemBuilder {
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_change(mut self, system_id: SystemId<In<bool>>) -> Self {
        self.on_change = Some(system_id);
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn key<S: Into<String>>(mut self, key: S) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn value<S: Into<String>>(mut self, value: S) -> Self {
        self.value = value.into();
        self
    }

    pub fn build(self) -> impl Bundle {
        let is_selected = self.selected;
        let is_disabled = self.disabled;
        let width = 144.0;
        let height = 52.0;

        let key = self.key.clone().unwrap_or_else(|| "".to_string());
        // select content- dropdown
        let child_nodes = Children::spawn((
            Spawn((
                Node {
                    display: Display::Block,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    min_width: Val::Px(width),
                    min_height: Val::Px(height),
                    ..default()
                },
                Name::new(self.key.clone().unwrap_or("".to_string())),
                Text::new(self.value.clone()),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
            )),
            //
        ));

        (
            Node {
                display: Display::None, // Initially hidden
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Auto,
                padding: UiRect::axes(Val::Px(0.0), Val::Px(4.0)),
                ..default()
            },
            GlobalZIndex(99),
            SelectContent,
            CoreSelectItem,
            Name::new("Select Item"),
            AccessibilityNode(Accessible::new(Role::ListBoxOption)),
            Hovering::default(),
            CursorIcon::System(SystemCursorIcon::Pointer),
            ListBoxOptionState {
                label: self.value.clone(),
                is_selected: false,
            },
            StyledSelectItem {
                selected: self.selected,
                on_change: self.on_change,
                disabled: self.disabled,
                key: self.key,
                value: self.value.clone(),
            },
            SelectedValue(self.value.clone()),
            Checked(is_selected),
            AccessibleName(key.clone()),
            TabIndex(0),
            child_nodes,
        )
    }
}

#[derive(Component, Default)]
pub struct SelectRoot;

#[derive(Component, Default)]
pub struct SelectTrigger;

#[derive(Default, Clone)]
pub struct SelectBuilder {
    pub on_click: Option<SystemId<In<bool>>>,
    pub on_change: Option<SystemId<In<Entity>>>,
    pub children: Vec<SelectItemBuilder>,
    pub selected_value: Option<String>,
    pub disabled: bool,
}

impl SelectBuilder {
    pub fn on_change(mut self, system_id: SystemId<In<Entity>>) -> Self {
        self.on_change = Some(system_id);
        self
    }

    pub fn on_click(mut self, system_id: SystemId<In<bool>>) -> Self {
        self.on_click = Some(system_id);
        self
    }

    pub fn children(mut self, options: impl IntoIterator<Item = SelectItemBuilder>) -> Self {
        self.children.extend(options);
        self
    }

    pub fn selected_value<S: Into<String>>(mut self, selected_value: S) -> Self {
        self.selected_value = Some(selected_value.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    // pub fn build(self) -> (impl Bundle, Vec<impl Bundle>) {
    pub fn build(self) -> (impl Bundle, impl Bundle, impl Bundle, Vec<impl Bundle>) {
        let button_width = 144.0;
        let button_height = 52.0;

        let parent_bundle = (
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            SelectRoot,
            AccessibilityNode(Accessible::new(Role::ListBox)),
        );

        let select_trigger_bundle = Children::spawn((Spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                min_width: Val::Px(button_width),
                min_height: Val::Px(button_height),
                padding: UiRect::axes(Val::Px(28.), Val::Px(10.)),
                ..default()
            },
            StyledSelect {
                options: self.children.clone(),
                on_click: self.on_click,
                selected_value: self.selected_value.clone(),
                disabled: self.disabled,
                on_change: self.on_change,
            },
            BackgroundColor(GRAY.into()),
            Name::new(self.selected_value.clone().unwrap_or("Select".to_string())), // Name::new("Select"),
            Hovering::default(),
            CursorIcon::System(SystemCursorIcon::Pointer),
            SelectHasPopup(false),
            SelectTrigger,
            CoreSelectTrigger {
                on_click: self.on_click,
            },
            AccessibilityNode(Accessible::new(Role::Button)),
            TabIndex(0),
            BorderRadius::default(),
            BorderColor::default(),
            Text::new(self.selected_value.clone().unwrap_or("Select".to_string())),
            TextFont {
                font_size: 14.,
                ..Default::default()
            },
        )),));

        let select_content_bundle = (
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column, // Dropdown items in a column
                justify_content: JustifyContent::FlexStart, // Align children to the start
                align_items: AlignItems::FlexStart,    // Align items to the start
                align_content: AlignContent::FlexStart, // Align content to the start
                width: Val::Px(button_width),          // Match the width of the trigger button
                // max_height: Val::Px(200.0),
                ..default()
            },
            CoreSelectContent {
                on_change: self.on_change,
            },
        );

        let child_bundles = self
            .children
            .into_iter()
            .map(|builder| builder.build())
            .collect::<Vec<_>>();

        (
            parent_bundle,
            select_trigger_bundle,
            select_content_bundle,
            child_bundles,
        )
    }
}

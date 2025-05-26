use bevy::{
    a11y::AccessibilityNode, ecs::system::SystemId, input_focus::tab_navigation::TabIndex,
    prelude::*, window::SystemCursorIcon, winit::cursor::CursorIcon,
};
use bevy_additional_core_widgets::{
    CoreSelectContent, CoreSelectItem, CoreSelectTrigger, DropdownOpen, IsSelected, SelectedItem,
};
use bevy_core_widgets::hover::Hovering;

use crate::themes::ThemeManager;

use super::{
    SelectButtonSize, StyledSelect,
    components::{AccessibleName, StyledSelectItem},
};
use accesskit::{Node as Accessible, Role};

#[derive(Component, Default)]
pub struct SelectWidget; // marker

#[derive(Component)]
pub struct StyledSelectText; // marker

#[derive(Component)]
pub struct StyledSelectOptionText; // marker

#[derive(Default, Clone)]
pub struct SelectBuilder {
    on_click: Option<SystemId<In<bool>>>,
    on_change: Option<SystemId<In<Entity>>>,
    children: Vec<SelectItemBuilder>,
    selected_value: Option<String>,
    disabled: bool,
    size: Option<SelectButtonSize>,
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

    pub fn size(mut self, size: SelectButtonSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn build(self) -> (impl Bundle, impl Bundle, impl Bundle, Vec<impl Bundle>) {
        let theme_manager = ThemeManager::default();
        let select_size_styles = theme_manager.styles.select_sizes.clone();
        let select_styles = theme_manager.styles.select_styles.clone();

        // Update size styles
        let select_size_style = match self.size.unwrap_or_default() {
            SelectButtonSize::XSmall => select_size_styles.xsmall,
            SelectButtonSize::Small => select_size_styles.small,
            SelectButtonSize::Medium => select_size_styles.medium,
            SelectButtonSize::Large => select_size_styles.large,
            SelectButtonSize::XLarge => select_size_styles.xlarge,
        };

        let button_width = select_size_style.min_width;
        let button_height = select_size_style.min_height;

        let font_size = select_size_style.font_size;

        // Root: SelectWidget
        let root = (
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            SelectWidget,
            AccessibilityNode(Accessible::new(Role::ComboBox)),
            DropdownOpen(false),
            AccessibleName(self.selected_value.clone().unwrap_or("Select".to_string())),
        );

        let trigger = Children::spawn((Spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                min_width: Val::Px(button_width),
                min_height: Val::Px(button_height),
                padding: UiRect::axes(Val::Px(4.), Val::Px(4.)),
                ..default()
            },
            StyledSelect {
                options: self.children.clone(),
                on_click: self.on_click,
                selected_value: self.selected_value.clone(),
                disabled: self.disabled,
                on_change: self.on_change,
                size: self.size,
            },
            BackgroundColor(select_styles.background.into()),
            Name::new(self.selected_value.clone().unwrap_or("Select".to_string())), // Name::new("Select"),
            Hovering::default(),
            CursorIcon::System(SystemCursorIcon::Pointer),
            AccessibilityNode(Accessible::new(Role::Button)),
            TabIndex(0),
            BorderRadius {
                top_left: Val::Px(select_size_style.border_radius),
                top_right: Val::Px(select_size_style.border_radius),
                bottom_left: Val::Px(select_size_style.border_radius),
                bottom_right: Val::Px(select_size_style.border_radius),
            },
            SelectedValue(self.selected_value.clone().unwrap_or("Select".to_string())),
            CoreSelectTrigger {
                on_click: self.on_click,
            },
            BorderColor::default(),
            Children::spawn(Spawn((
                Text::new(self.selected_value.clone().unwrap_or("Select".to_string())),
                TextFont {
                    font_size: font_size,
                    ..Default::default()
                },
                StyledSelectText,
            ))),
        )),));

        // Dropdown container
        let dropdown = (
            Node {
                display: Display::None,
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(2.0)),
                width: Val::Px(button_width),
                height: Val::Auto,
                ..default()
            },
            BackgroundColor(select_styles.popover_background.into()),
            BorderRadius {
                top_left: Val::Px(select_size_style.border_radius),
                top_right: Val::Px(select_size_style.border_radius),
                bottom_left: Val::Px(select_size_style.border_radius),
                bottom_right: Val::Px(select_size_style.border_radius),
            },
            CoreSelectContent {
                on_change: self.on_change,
            },
            AccessibilityNode(Accessible::new(Role::ListBox)),
        );

        let child_bundles = self
            .children
            .into_iter()
            .map(|builder| builder.size(self.size.unwrap_or_default()).build())
            .collect::<Vec<_>>();

        (root, trigger, dropdown, child_bundles)
    }
}

#[derive(Component, Debug, Clone)]
pub struct SelectedValue(pub String);

#[derive(Default, Clone)]
pub struct SelectItemBuilder {
    pub selected: bool,
    pub on_change: Option<SystemId<In<bool>>>,
    pub disabled: bool,
    pub label: Option<String>,
    pub value: String,
    pub size: Option<SelectButtonSize>,
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

    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn value<S: Into<String>>(mut self, value: S) -> Self {
        self.value = value.into();
        self
    }

    pub fn size(mut self, size: SelectButtonSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn build(self) -> impl Bundle {
        let theme_manager = ThemeManager::default();
        let select_size_styles = theme_manager.styles.select_sizes.clone();
        // Update size styles
        let select_size_style = match self.size.unwrap_or_default() {
            SelectButtonSize::XSmall => select_size_styles.xsmall,
            SelectButtonSize::Small => select_size_styles.small,
            SelectButtonSize::Medium => select_size_styles.medium,
            SelectButtonSize::Large => select_size_styles.large,
            SelectButtonSize::XLarge => select_size_styles.xlarge,
        };
        let height = select_size_style.min_height;
        let font_size = select_size_style.font_size;

        // select content- dropdown
        let child_nodes = Children::spawn((
            Spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                    width: Val::Percent(100.0),
                    height: Val::Px(height),
                    ..default()
                },
                Name::new(self.label.clone().unwrap_or(self.value.to_string())),
                Children::spawn(Spawn((
                    Text::new(self.value.clone()),
                    TextFont {
                        font_size: font_size,
                        ..Default::default()
                    },
                    StyledSelectOptionText,
                ))),
            )),
            //
        ));

        (
            Node {
                display: Display::Flex, // Initially hidden
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Stretch,
                width: Val::Percent(100.0),
                height: Val::Auto,
                padding: UiRect::axes(Val::Px(0.0), Val::Px(4.0)),
                ..default()
            },
            GlobalZIndex(99), // to ensure it appears above other UI elements
            CoreSelectItem,
            Name::new("Select Item"),
            Hovering::default(),
            CursorIcon::System(SystemCursorIcon::Pointer),
            StyledSelectItem {
                selected: self.selected,
                on_change: self.on_change,
                disabled: self.disabled,
                label: self.label.clone(),
                value: self.value.clone(),
            },
            SelectedValue(self.value.clone()),
            SelectedItem {
                label: self.label.clone().unwrap_or(self.value.clone()),
                value: self.value.clone(),
            },
            IsSelected(self.selected),
            AccessibilityNode(Accessible::new(Role::ListBoxOption)),
            TabIndex(0),
            child_nodes,
        )
    }
}

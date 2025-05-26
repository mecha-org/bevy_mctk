use bevy::{input_focus::tab_navigation::TabGroup, prelude::*, winit::WinitSettings};
use bevy_additional_core_widgets::IsSelected;
use bevy_styled_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StyledWidgetsPlugin))
        .insert_resource(ThemeManager::default())
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup_view_root)
        .add_systems(Update, update_root_background)
        .run();
}

#[derive(Component)]
struct ThemeToggleButton;

#[derive(Component)]
struct RootWindow;

fn update_root_background(
    theme_manager: Res<ThemeManager>,
    mut query: Query<&mut BackgroundColor, With<RootWindow>>,
) {
    for mut bg_color in query.iter_mut() {
        let theme_styles = theme_manager.styles.clone();
        let color = theme_styles.panel.background_color;
        bg_color.0 = color;
    }
}

fn toggle_mode(mut theme_manager: ResMut<ThemeManager>) {
    let current_mode = theme_manager.current_mode;
    let new_mode = match current_mode {
        ThemeMode::Light => ThemeMode::Dark,
        ThemeMode::Dark => ThemeMode::Light,
    };
    theme_manager.set_theme_mode(new_mode);
}

fn set_theme(id: ThemeId) -> impl FnMut(ResMut<ThemeManager>) + Clone {
    move |mut theme_manager: ResMut<ThemeManager>| {
        theme_manager.set_theme(id.clone());
    }
}

fn run_on_select_changed(
    In(selected_entity): In<Entity>,
    q_select_content: Query<&Children>,
    select_query: Query<(&ChildOf, &SelectedValue)>,
    mut commands: Commands,
) {
    if let Ok((child_of, selected_value)) = select_query.get(selected_entity) {
        let group_children = q_select_content.get(child_of.parent()).unwrap();
        for select_item_child in group_children.iter() {
            if let Ok((_, value)) = select_query.get(select_item_child) {
                commands
                    .entity(select_item_child)
                    .insert(IsSelected(value.0 == selected_value.0));
            }
        }
    }
}

fn setup_view_root(mut commands: Commands) {
    commands.spawn(Camera2d);

    let on_toggle_theme_mode = commands.register_system(toggle_mode);

    // Example theme change handlers (register your real handlers)
    let on_default_theme = commands.register_system(set_theme(ThemeId("default".into())));
    let on_red_theme = commands.register_system(set_theme(ThemeId("red".into())));
    let on_rose_theme = commands.register_system(set_theme(ThemeId("rose".into())));
    let on_orange_theme = commands.register_system(set_theme(ThemeId("orange".into())));
    let on_green_theme = commands.register_system(set_theme(ThemeId("green".into())));
    let on_blue_theme = commands.register_system(set_theme(ThemeId("blue".into())));
    let on_yellow_theme = commands.register_system(set_theme(ThemeId("yellow".into())));
    let on_violet_theme = commands.register_system(set_theme(ThemeId("violet".into())));

    let select_on_change_system_id = commands.register_system(run_on_select_changed);

    let options_l = vec![
        StyledSelectItem::builder().value("Option 1".to_string()),
        StyledSelectItem::builder().value("Option 2".to_string()),
        StyledSelectItem::builder().value("Option 3".to_string()),
    ];

    let options = vec![
        StyledSelectItem::builder()
            .label("Juice".to_string())
            .value("Juice".to_string()),
        StyledSelectItem::builder()
            .label("Tea".to_string())
            .value("Tea".to_string()),
        StyledSelectItem::builder()
            .label("Coffee".to_string())
            .value("Coffee".to_string()),
    ];

    let (parent_bundle, select_trigger_bundle, select_content_bundle, child_bundles) =
        StyledSelect::builder()
            .children(options.clone())
            // .on_change(select_on_change_system_id)
            .build();

    let (parent_bundle_l, select_trigger_bundle_l, select_content_bundle_l, child_bundles_l) =
        StyledSelect::builder()
            .children(options_l.clone())
            .size(SelectButtonSize::Large)
            .build();

    commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                top: Val::Px(0.),
                right: Val::Px(0.),
                bottom: Val::Px(0.),
                padding: UiRect::all(Val::Px(3.)),
                row_gap: Val::Px(18.),
                ..Default::default()
            },
            RootWindow,
            TabGroup::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(6.0),
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(0.0)),
                    ..default()
                },
                Children::spawn((
                    Spawn(
                        StyledButton::builder()
                            .text("Default")
                            .variant(ButtonVariant::Ghost)
                            .on_click(on_default_theme)
                            .build(),
                    ),
                    Spawn(
                        StyledButton::builder()
                            .text("Red")
                            .variant(ButtonVariant::Ghost)
                            .on_click(on_red_theme)
                            .build(),
                    ),
                    Spawn(
                        StyledButton::builder()
                            .text("Rose")
                            .variant(ButtonVariant::Ghost)
                            .on_click(on_rose_theme)
                            .build(),
                    ),
                    Spawn(
                        StyledButton::builder()
                            .text("Orange")
                            .variant(ButtonVariant::Ghost)
                            .on_click(on_orange_theme)
                            .build(),
                    ),
                    Spawn(
                        StyledButton::builder()
                            .text("Green")
                            .variant(ButtonVariant::Ghost)
                            .on_click(on_green_theme)
                            .build(),
                    ),
                    Spawn(
                        StyledButton::builder()
                            .text("Blue")
                            .variant(ButtonVariant::Ghost)
                            .on_click(on_blue_theme)
                            .build(),
                    ),
                    Spawn(
                        StyledButton::builder()
                            .text("Yellow")
                            .variant(ButtonVariant::Ghost)
                            .on_click(on_yellow_theme)
                            .build(),
                    ),
                    Spawn(
                        StyledButton::builder()
                            .text("Violet")
                            .variant(ButtonVariant::Ghost)
                            .on_click(on_violet_theme)
                            .build(),
                    ),
                )),
            ));

            parent.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(6.0),
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(0.0)),
                    ..default()
                },
                Children::spawn(Spawn((
                    StyledButton::builder()
                        .icon("theme_mode_toggle")
                        .on_click(on_toggle_theme_mode)
                        .variant(ButtonVariant::Secondary)
                        .build(),
                    ThemeToggleButton,
                ))),
            ));

            parent.spawn(
                StyledText::builder()
                    .content("Select")
                    .font_size(24.0)
                    .build(),
            );

            parent
                .spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Start,
                    align_content: AlignContent::Start,
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(0.0)),
                    width: Val::Px(45.),
                    height: Val::Px(60.),
                    ..default()
                },))
                .insert(parent_bundle)
                .insert(select_trigger_bundle)
                .with_children(|parent| {
                    parent
                        .spawn(select_content_bundle)
                        .with_children(|content| {
                            for child in child_bundles {
                                content.spawn(child);
                            }
                        });
                });

            parent.spawn(
                StyledText::builder()
                    .content("Sizes")
                    .font_size(24.0)
                    .build(),
            );

            parent
                .spawn((Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Start,
                    align_content: AlignContent::Start,
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(0.0)),
                    width: Val::Px(45.),
                    height: Val::Px(60.),
                    ..default()
                },))
                .insert(parent_bundle_l)
                .insert(select_trigger_bundle_l)
                .with_children(|parent| {
                    parent
                        .spawn(select_content_bundle_l)
                        .with_children(|content| {
                            for child in child_bundles_l {
                                content.spawn(child);
                            }
                        });
                });
        });
}

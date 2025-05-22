use bevy::{input_focus::tab_navigation::TabGroup, prelude::*, winit::WinitSettings};
use bevy_core_widgets::{Checked, InteractionDisabled};
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

fn set_theme(id: ThemeId) -> impl FnMut(ResMut<ThemeManager>) + Clone {
    move |mut theme_manager: ResMut<ThemeManager>| {
        theme_manager.set_theme(id.clone());
    }
}

fn run_on_radio_selected(
    In(selected_entity): In<Entity>,
    q_radio_group: Query<&Children>,
    query: Query<(&ChildOf, &RadioValue)>,
    mut theme_manager: ResMut<ThemeManager>,
    mut commands: Commands,
) {
    if let Ok((child_of, radio_value)) = query.get(selected_entity) {
        match radio_value.0.as_str() {
            "dark" => theme_manager.set_theme_mode(ThemeMode::Dark),
            "light" => theme_manager.set_theme_mode(ThemeMode::Light),
            &_ => warn!("Unhandled radio value: {}", radio_value.0.to_string()),
        }

        let group_children = q_radio_group.get(child_of.parent()).unwrap();
        for radio_child in group_children.iter() {
            if let Ok((_, value)) = query.get(radio_child) {
                commands
                    .entity(radio_child)
                    .insert(Checked(value.0 == radio_value.0));
            }
        }
    }
}

fn setup_view_root(mut commands: Commands, theme: Res<ThemeManager>) {
    commands.spawn(Camera2d);

    let radio_on_change_system_id = commands.register_system(run_on_radio_selected);

    // Example theme change handlers (register your real handlers)
    let on_default_theme = commands.register_system(set_theme(ThemeId("default".into())));
    let on_red_theme = commands.register_system(set_theme(ThemeId("red".into())));
    let on_rose_theme = commands.register_system(set_theme(ThemeId("rose".into())));
    let on_orange_theme = commands.register_system(set_theme(ThemeId("orange".into())));
    let on_green_theme = commands.register_system(set_theme(ThemeId("green".into())));
    let on_blue_theme = commands.register_system(set_theme(ThemeId("blue".into())));
    let on_yellow_theme = commands.register_system(set_theme(ThemeId("yellow".into())));
    let on_violet_theme = commands.register_system(set_theme(ThemeId("violet".into())));

    let (group_bundle, children) = StyledRadioGroup::builder()
        .on_change(radio_on_change_system_id)
        .children([
            StyledRadioButton::builder()
                .caption("Light mode")
                .value("light")
                .checked(theme.current_mode == ThemeMode::Light),
            StyledRadioButton::builder()
                .caption("Dark mode")
                .value("dark")
                .checked(theme.current_mode == ThemeMode::Dark),
        ])
        .direction(RadioButtonDirection::Vertical)
        .build();

    commands
        .spawn((
            RootWindow,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                top: Val::Px(0.),
                right: Val::Px(0.),
                bottom: Val::Px(0.),
                padding: UiRect::all(Val::Px(3.)),
                row_gap: Val::Px(6.),
                ..Default::default()
            },
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

            parent.spawn(
                StyledText::builder()
                    .content("Radio Group")
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
                    row_gap: Val::Px(6.0),
                    ..default()
                },))
                .insert(group_bundle)
                .with_children(|group_parent| {
                    for child in children {
                        group_parent.spawn(child);
                    }
                });

            // Sizes section
            parent.spawn(
                StyledText::builder()
                    .content("Sizes")
                    .font_size(24.0)
                    .build(),
            );

            parent.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(0.0)),
                    column_gap: Val::Px(6.0),
                    ..default()
                },
                Children::spawn((
                    Spawn(
                        StyledText::builder()
                            .content("XSmall")
                            .font_size(14.0)
                            .build(),
                    ),
                    Spawn(
                        StyledRadioButton::builder()
                            .size(RadioButtonSize::XSmall)
                            .variant(RadioButtonVariant::Default)
                            .checked(true)
                            .build(),
                    ),
                    Spawn(
                        StyledText::builder()
                            .content("Small")
                            .font_size(14.0)
                            .build(),
                    ),
                    Spawn(
                        StyledRadioButton::builder()
                            .size(RadioButtonSize::Small)
                            .variant(RadioButtonVariant::Default)
                            .checked(true)
                            .build(),
                    ),
                    Spawn(
                        StyledText::builder()
                            .content("Medium")
                            .font_size(14.0)
                            .build(),
                    ),
                    Spawn(
                        StyledRadioButton::builder()
                            .size(RadioButtonSize::Medium)
                            .variant(RadioButtonVariant::Default)
                            .checked(true)
                            .build(),
                    ),
                    Spawn(
                        StyledText::builder()
                            .content("Large")
                            .font_size(14.0)
                            .build(),
                    ),
                    Spawn(
                        StyledRadioButton::builder()
                            .size(RadioButtonSize::Large)
                            .variant(RadioButtonVariant::Default)
                            .checked(true)
                            .build(),
                    ),
                    Spawn(
                        StyledText::builder()
                            .content("XLarge")
                            .font_size(14.0)
                            .build(),
                    ),
                    Spawn(
                        StyledRadioButton::builder()
                            .size(RadioButtonSize::XLarge)
                            .variant(RadioButtonVariant::Default)
                            .checked(true)
                            .build(),
                    ),
                )),
            ));

            // Disabled section
            parent.spawn(
                StyledText::builder()
                    .content("Disabled")
                    .font_size(24.0)
                    .build(),
            );

            parent.spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(0.0)),
                    column_gap: Val::Px(6.0),
                    ..default()
                },
                Children::spawn((Spawn((
                    StyledRadioButton::builder()
                        .variant(RadioButtonVariant::Default)
                        .checked(true)
                        .disabled()
                        .build(),
                    InteractionDisabled,
                )),)),
            ));
        });
}

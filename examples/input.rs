use bevy::{input_focus::tab_navigation::TabGroup, prelude::*, winit::WinitSettings};
use bevy_cosmic_edit::{CosmicEditPlugin, CosmicFontConfig, CosmicFontSystem};
use bevy_styled_widgets::prelude::*;

fn main() {
    let font_bytes: &[u8] = include_bytes!("../assets/fonts/SpaceGrotesk-Medium.ttf");
    let font_config = CosmicFontConfig {
        fonts_dir_path: None,
        font_bytes: Some(vec![font_bytes]),
        load_system_fonts: true,
    };

    App::new()
        .add_plugins((
            DefaultPlugins,
            StyledWidgetsPligin,
            CosmicEditPlugin { font_config },
        ))
        .insert_resource(ThemeManager::default())
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup_view_root)
        .add_systems(Update, update_root_background)
        .add_systems(
            Update,
            update_theme_toggle_button.run_if(resource_exists_and_changed::<ThemeManager>),
        )
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

fn set_theme(theme: Theme) -> impl FnMut(ResMut<ThemeManager>) + Clone {
    move |mut theme_manager: ResMut<ThemeManager>| {
        let id = ThemeId(theme.into());
        theme_manager.set_theme(id);
    }
}

fn update_theme_toggle_button(
    theme_manager: Res<ThemeManager>,
    mut query: Query<&mut StyledButton, With<ThemeToggleButton>>,
) {
    println!("update_theme_toggle_button()");
    for mut button in query.iter_mut() {
        let icon = match theme_manager.current_mode {
            ThemeMode::Light => "dark.png",
            ThemeMode::Dark => "light.png",
        };
        button.icon = Some(icon.to_string());
    }
}

fn setup_view_root(mut commands: Commands, mut font_system: ResMut<CosmicFontSystem>) {
    commands.spawn(Camera2d);

    let on_toogle_theme_mode = commands.register_system(toggle_mode);

    // Example theme change handlers (register your real handlers)
    let on_default_theme = commands.register_system(set_theme(Theme::Default));
    let on_red_theme = commands.register_system(set_theme(Theme::Red));
    let on_rose_theme = commands.register_system(set_theme(Theme::Rose));
    let on_orange_theme = commands.register_system(set_theme(Theme::Orange));
    let on_green_theme = commands.register_system(set_theme(Theme::Green));
    let on_blue_theme = commands.register_system(set_theme(Theme::Blue));
    let on_yellow_theme = commands.register_system(set_theme(Theme::Yellow));
    let on_violet_theme = commands.register_system(set_theme(Theme::Violet));

    let handle_text_input = commands.register_system(get_text_input);

    // TODO: make this working
    fn get_text_input(In((entity, value)): In<(Entity, String)>) {
        println!("handle_text_input entity {:?} which is {:?}", entity, value);
    }

    commands.spawn((
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
        Children::spawn((
            // Theme selection row
            Spawn((
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
            )),
            // Light / Dark toggle row
            Spawn((
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
                    Spawn((
                        StyledButton::builder()
                            .icon("dark.png")
                            .on_click(on_toogle_theme_mode)
                            .variant(ButtonVariant::Secondary)
                            .build(),
                        ThemeToggleButton,
                    )),
                    // Spawn(
                    //     StyledButton::builder()
                    //         .icon("dark.png")
                    //         .on_click(on_dark_click)
                    //         .variant(ButtonVariant::Secondary)
                    //         .build(),
                    // ),
                )),
            )),
            // Text Input
            Spawn(
                StyledText::builder()
                    .content("Input")
                    .font_size(24.0)
                    .build(),
            ),
            Spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(0.0)),
                    ..default()
                },
                Children::spawn((Spawn(
                    StyledInput::builder()
                        .value("Mecha..")
                        .width(Val::Px(200.))
                        .height(Val::Px(40.))
                        .placeholder("Enter name")
                        .variant(InputVariant::Text)
                        .font_system(&mut *font_system)
                        .on_change(handle_text_input)
                        .build(),
                ),)),
            )), // types
        )),
    ));

    // commands.add_observer(focus_on_click);
}

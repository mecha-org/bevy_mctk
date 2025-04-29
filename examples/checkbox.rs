use bevy::{input_focus::tab_navigation::TabGroup, prelude::*, winit::WinitSettings};
use bevy_core_widgets::{Checked, InteractionDisabled};
use bevy_styled_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StyledWidgetsPligin))
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
pub struct ThemeToggleButton;

#[derive(Component)]
pub struct ThemeToggleCheckbox;

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

fn update_theme_toggle_button(
    theme_manager: Res<ThemeManager>,
    mut query: Query<&mut StyledToggle, With<ThemeToggleButton>>,
) {
    for mut toggle in query.iter_mut() {
        let icon = match theme_manager.current_mode {
            ThemeMode::Light => "dark.png",
            ThemeMode::Dark => "light.png",
        };
        toggle.icon = Some(icon.to_string());
    }
}

fn update_light_dark_theme(
    In(checked): In<bool>,
    mut query: Query<(&mut StyledCheckbox, Entity), With<ThemeToggleCheckbox>>,
    mut commands: Commands,
    mut theme_manager: ResMut<ThemeManager>,
) {
    for (mut styled_checkbox, entity) in &mut query {
        if styled_checkbox.disabled {
            commands.entity(entity).insert(InteractionDisabled);
            return;
        }

        commands.entity(entity).insert(Checked(checked));
        let new_mode = match theme_manager.current_mode {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
        theme_manager.set_theme_mode(new_mode);
    }
}

fn setup_view_root(mut commands: Commands, theme: Res<ThemeManager>) {
    commands.spawn(Camera2d);

    let on_toogle_theme_mode = commands.register_system(update_light_dark_theme);

    // Example theme change handlers (register your real handlers)
    let on_default_theme = commands.register_system(set_theme(ThemeId("default".into())));
    let on_red_theme = commands.register_system(set_theme(ThemeId("red".into())));
    let on_rose_theme = commands.register_system(set_theme(ThemeId("rose".into())));
    let on_orange_theme = commands.register_system(set_theme(ThemeId("orange".into())));
    let on_green_theme = commands.register_system(set_theme(ThemeId("green".into())));
    let on_blue_theme = commands.register_system(set_theme(ThemeId("blue".into())));
    let on_yellow_theme = commands.register_system(set_theme(ThemeId("yellow".into())));
    let on_violet_theme = commands.register_system(set_theme(ThemeId("violet".into())));

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
            row_gap: Val::Px(6.),
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
                    column_gap: Val::Px(4.0),
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(0.0)),
                    ..default()
                },
                Children::spawn((Spawn((
                    StyledCheckbox::builder()
                        .variant(CheckboxVariant::Default)
                        .on_change(on_toogle_theme_mode)
                        .caption("Dark")
                        .description("Enable dark mode")
                        .build(),
                    ThemeToggleCheckbox,
                )),)),
            )),
            // Checkbox section
            Spawn(
                StyledText::builder()
                    .content("Checkbox")
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
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(0.0)),
                    column_gap: Val::Px(6.0),
                    ..default()
                },
                Children::spawn((
                    Spawn(
                        StyledText::builder()
                            .content("Checked")
                            .font_size(14.0)
                            .build(),
                    ),
                    Spawn(
                        StyledCheckbox::builder()
                            .variant(CheckboxVariant::Default)
                            .checked(true)
                            .build(),
                    ),
                    Spawn(
                        StyledText::builder()
                            .content("Inactive")
                            .font_size(14.0)
                            .build(),
                    ),
                    Spawn(
                        StyledCheckbox::builder()
                            .variant(CheckboxVariant::WithText)
                            .build(),
                    ),
                )),
            )),
            Spawn(
                StyledText::builder()
                    .content("Sizes")
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
                        StyledCheckbox::builder()
                            .size(CheckboxSize::XSmall)
                            .variant(CheckboxVariant::Default)
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
                        StyledCheckbox::builder()
                            .size(CheckboxSize::Small)
                            .variant(CheckboxVariant::Default)
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
                        StyledCheckbox::builder()
                            .size(CheckboxSize::Medium)
                            .variant(CheckboxVariant::Default)
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
                        StyledCheckbox::builder()
                            .size(CheckboxSize::Large)
                            .variant(CheckboxVariant::Default)
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
                        StyledCheckbox::builder()
                            .size(CheckboxSize::XLarge)
                            .variant(CheckboxVariant::Default)
                            .checked(true)
                            .build(),
                    ),
                )),
            )),
            Spawn(
                StyledText::builder()
                    .content("Disabled")
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
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(0.0)),
                    column_gap: Val::Px(6.0),
                    ..default()
                },
                Children::spawn((Spawn((
                    StyledCheckbox::builder()
                        .variant(CheckboxVariant::Default)
                        .checked(true)
                        .disabled()
                        .build(),
                    InteractionDisabled,
                )),)),
            )),
        )),
    ));
}

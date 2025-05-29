use bevy::{input_focus::tab_navigation::TabGroup, prelude::*, winit::WinitSettings};
use bevy_core_widgets::{CoreSlider, InteractionDisabled};
use bevy_styled_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StyledWidgetsPligin))
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

#[derive(Component)]
pub struct XLSlider;

fn xl_slider_on_change(
    In(value): In<f32>,
    mut query: Query<(&mut StyledSlider, Entity, &mut CoreSlider), With<XLSlider>>,
    mut commands: Commands,
) {
    for (styled_slider, entity, mut core_slider) in &mut query {
        if styled_slider.disabled {
            commands.entity(entity).insert(InteractionDisabled);
            return;
        } else {
            core_slider.set_value(value);
        }
    }
}

fn setup_view_root(mut commands: Commands) {
    commands.spawn(Camera2d);
    let xl_slider_on_change_system = commands.register_system(xl_slider_on_change);

    let on_toogle_theme_mode = commands.register_system(toggle_mode);

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
                Children::spawn((Spawn((
                    StyledButton::builder()
                        .icon("theme_mode_toggle")
                        .on_click(on_toogle_theme_mode)
                        .variant(ButtonVariant::Secondary)
                        .build(),
                    ThemeToggleButton,
                )),)),
            )),
            // Slider
            Spawn(
                StyledText::builder()
                    .content("Slider")
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
                Children::spawn((Spawn(
                    StyledSlider::builder()
                        .max(100.)
                        .min(0.)
                        .value(50.)
                        .size(SliderSize::Medium)
                        .build(),
                ),)),
            )), // types
            Spawn(
                StyledText::builder()
                    .content("Sizes")
                    .font_size(24.0)
                    .build(),
            ),
            Spawn((
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Stretch,
                    align_items: AlignItems::Stretch,
                    align_content: AlignContent::Stretch,
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(12.0)),
                    column_gap: Val::Px(10.0),
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
                        StyledSlider::builder()
                            .max(100.)
                            .min(0.)
                            .value(50.)
                            .size(SliderSize::XSmall)
                            .build(),
                    ),
                    Spawn(
                        StyledText::builder()
                            .content("Small")
                            .font_size(14.0)
                            .build(),
                    ),
                    Spawn(
                        StyledSlider::builder()
                            .max(100.)
                            .min(0.)
                            .value(50.)
                            .size(SliderSize::Small)
                            .build(),
                    ),
                    Spawn(
                        StyledText::builder()
                            .content("Medium")
                            .font_size(14.0)
                            .build(),
                    ),
                    Spawn(
                        StyledSlider::builder()
                            .max(100.)
                            .min(0.)
                            .value(50.)
                            .size(SliderSize::Medium)
                            .build(),
                    ),
                    Spawn(
                        StyledText::builder()
                            .content("Large")
                            .font_size(14.0)
                            .build(),
                    ),
                    Spawn(
                        StyledSlider::builder()
                            .max(100.)
                            .min(0.)
                            .value(50.)
                            .size(SliderSize::Large)
                            .build(),
                    ),
                    Spawn(
                        StyledText::builder()
                            .content("XLarge")
                            .font_size(14.0)
                            .build(),
                    ),
                    Spawn((
                        StyledSlider::builder()
                            .max(100.)
                            .min(0.)
                            .value(50.)
                            .size(SliderSize::XLarge)
                            .on_change(xl_slider_on_change_system)
                            .build(),
                        XLSlider,
                    )),
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
                    StyledSlider::builder()
                        .max(100.)
                        .min(0.)
                        .value(50.)
                        .size(SliderSize::Large)
                        .disabled(true)
                        .build(),
                    InteractionDisabled,
                )),)),
            )),
        )),
    ));
}

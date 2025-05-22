use bevy::{color::palettes::css, prelude::*, winit::WinitSettings};
use bevy_asset_loader::prelude::*;
use bevy_styled_widgets::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    // #[asset(path = "examples/images/parachute.png")]
    #[asset(key = "images.parachute")]
    parachute: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
#[allow(dead_code)]
pub struct FontAssets {
    //Primary fonts
    #[asset(key = "fonts.primary.300")]
    primary_300: Handle<Font>,

    #[asset(key = "fonts.primary.400")]
    primary_400: Handle<Font>,

    #[asset(key = "fonts.primary.500")]
    primary_500: Handle<Font>,

    #[asset(key = "fonts.primary.600")]
    primary_600: Handle<Font>,

    #[asset(key = "fonts.primary.700")]
    primary_700: Handle<Font>,

    //secondary assets
    #[asset(key = "fonts.secondary.300")]
    secondary_300: Handle<Font>,

    #[asset(key = "fonts.secondary.400")]
    secondary_400: Handle<Font>,

    #[asset(key = "fonts.secondary.500")]
    secondary_500: Handle<Font>,

    #[asset(key = "fonts.secondary.600")]
    secondary_600: Handle<Font>,

    #[asset(key = "fonts.secondary.700")]
    secondary_700: Handle<Font>,

    #[asset(key = "fonts.icons")]
    font_icons: Handle<Font>,
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum AssetsLoadingState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Icon {
    Moon = 0xe900,
    Bulb = 0xe901,
}

impl Icon {
    /// Converts the icon to its UTF-8 string representation.
    pub fn to_string(&self) -> String {
        let code_point = *self as u32;

        match std::char::from_u32(code_point) {
            Some(c) => String::from(c),
            None => String::new(),
        }
    }
}

impl Into<String> for Icon {
    fn into(self) -> String {
        self.to_string()
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, StyledWidgetsPlugin))
        .insert_resource(ThemeManager::default())
        .insert_resource(WinitSettings::desktop_app())
        .init_state::<AssetsLoadingState>()
        .add_loading_state(
            LoadingState::new(AssetsLoadingState::Loading)
                .continue_to_state(AssetsLoadingState::Loaded)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("examples/settings.ron")
                .load_collection::<ImageAssets>()
                .load_collection::<FontAssets>(),
        )
        .add_systems(OnEnter(AssetsLoadingState::Loaded), setup_view_root)
        .run();
}

fn setup_view_root(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    font_assets: Res<FontAssets>,
) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(3.)),
                row_gap: Val::Px(18.),
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            BackgroundColor(Color::Srgba(css::BLACK)),
        ))
        .with_children(|parent| {
            //Primary text font
            parent.spawn((
                Text::new("Primary font text"),
                TextFont {
                    font: font_assets.primary_700.clone(),
                    font_size: 24.,
                    ..Default::default()
                },
            ));

            //Secondary text font
            parent.spawn((
                Text::new("Secondary font text"),
                TextFont {
                    font: font_assets.secondary_700.clone(),
                    font_size: 24.,
                    ..Default::default()
                },
            ));

            //Icon font
            parent.spawn((
                Text::new(Icon::Bulb),
                TextFont {
                    font: font_assets.font_icons.clone(),
                    font_size: 24.,
                    ..Default::default()
                },
            ));

            parent.spawn((
                ImageNode {
                    image: image_assets.parachute.clone(),
                    ..Default::default()
                },
                Node {
                    width: Val::Px(300.),
                    ..Default::default()
                },
            ));
        });
}

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/SpaceGrotesk-Medium.ttf")]
    pub space_grotesk_medium: Handle<Font>,

    #[asset(path = "fonts/font-icons.ttf")]
    pub font_icons: Handle<Font>,
}

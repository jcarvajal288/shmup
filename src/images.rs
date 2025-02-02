use bevy::prelude::*;

#[derive(Resource)]
pub struct Images {
    pub dark_background: Handle<Image>,
}

impl Default for Images {
    fn default() -> Self {
        Self {
            dark_background: Handle::default(),
        }
    }
}

pub fn load_images(mut images: ResMut<Images>, asset_server: Res<AssetServer>) {
    images.dark_background = asset_server.load("images/dark-background.png");
}
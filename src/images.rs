use bevy::prelude::*;

#[derive(Resource)]
pub struct Images {
    pub dark_background: Handle<Image>,
    pub frame: Handle<Image>,
}

impl Default for Images {
    fn default() -> Self {
        Self {
            dark_background: Handle::default(),
            frame: Handle::default(),
        }
    }
}

pub fn load_images(mut images: ResMut<Images>, asset_server: Res<AssetServer>) {
    images.dark_background = asset_server.load("images/dark-background.png");
    images.frame = asset_server.load("images/frame.png");
}
use bevy::prelude::*;

#[derive(Resource)]
#[derive(Default)]
pub struct Images {
    pub dark_background: Handle<Image>,
    pub frame: Handle<Image>,
    pub bullets: Handle<Image>,
    pub sidebar: Handle<Image>,
    pub remilia: Handle<Image>,
    pub fairies: Handle<Image>,
    pub player_accessories: Handle<Image>,
    pub rumia: Handle<Image>,
}


pub fn load_images(mut images: ResMut<Images>, asset_server: Res<AssetServer>) {
    images.dark_background = asset_server.load("images/dark-background.png");
    images.frame = asset_server.load("images/frame.png");
    images.bullets = asset_server.load("images/bullets.png");
    images.sidebar = asset_server.load("images/sidebar.png");
    images.remilia = asset_server.load("images/remilia.png");
    images.fairies = asset_server.load("images/fairies1.png");
    images.player_accessories = asset_server.load("images/player-accessories.png");
    images.rumia = asset_server.load("images/rumia.png");
}
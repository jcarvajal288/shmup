mod menu;
mod game;
mod sprites;
mod player;

use bevy::prelude::*;
use crate::game::game_plugin;
use crate::menu::menu_plugin;
use crate::sprites::{load_sprites, Sprites};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    MENU,
    GAME,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .add_systems(Startup, (setup, load_sprites).chain())
        .add_plugins((menu_plugin, game_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.insert_resource(Sprites::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

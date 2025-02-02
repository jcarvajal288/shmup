mod menu;
mod game;
mod sprites;
mod player;
mod images;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use crate::game::game_plugin;
use crate::images::{load_images, Images};
use crate::menu::menu_plugin;
use crate::sprites::{load_sprites, Sprites};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    MENU,
    GAME,
}

const DEFAULT_RESOLUTION: Vec2 = Vec2::new(800., 600.);
const DEFAULT_SCALING_FACTOR: f32 = 1.0;

const SCALING_MODIFIER: f32 = 2.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(
                            DEFAULT_RESOLUTION.x * SCALING_MODIFIER,
                            DEFAULT_RESOLUTION.y * SCALING_MODIFIER
                        )
                        .with_scale_factor_override(DEFAULT_SCALING_FACTOR * SCALING_MODIFIER),
                    ..default()
                }),
                ..default()
            })
        )
        .init_state::<GameState>()
        .add_systems(Startup, (setup, load_images, load_sprites).chain())
        .add_plugins((menu_plugin, game_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.insert_resource(Sprites::default());
    commands.insert_resource(Images::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

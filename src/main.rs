mod menu;
mod game;
mod sprites;
mod player;
mod images;
mod level1;
mod bullet;
mod enemy;
mod movement_patterns;
mod bullet_patterns;
mod player_stats;
mod game_over_menu;

use crate::game::game_plugin;
use crate::images::{load_images, Images};
use crate::menu::menu_plugin;
use crate::player_stats::PlayerStats;
use crate::sprites::{load_sprites, Sprites};
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::game_over_menu::game_over_menu_plugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    PlayingGame,
    GameOver,
}

const DEFAULT_RESOLUTION: Vec2 = Vec2::new(800., 600.);
const SCALING_FACTOR: f32 = 2.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(
                            DEFAULT_RESOLUTION.x * SCALING_FACTOR,
                            DEFAULT_RESOLUTION.y * SCALING_FACTOR
                        )
                        .with_scale_factor_override(SCALING_FACTOR),
                    ..default()
                }),
                ..default()
            })
        )
        .init_state::<GameState>()
        .add_systems(Startup, (setup, load_images, load_sprites).chain())
        .add_plugins((menu_plugin, game_plugin, game_over_menu_plugin))
        //.add_plugins(WorldInspectorPlugin::new())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.insert_resource(Sprites::default());
    commands.insert_resource(Images::default());
    commands.insert_resource(PlayerStats::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

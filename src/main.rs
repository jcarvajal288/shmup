mod game;
mod player;
mod level1;
mod bullet;
mod enemy;
mod movement_patterns;
mod bullet_patterns;
mod player_stats;
mod menus;
mod resources;
mod bosses;
mod sprites;
mod testbed;

use crate::game::{game_plugin, GameObject};
use crate::menus::game_over_menu::game_over_menu_plugin;
use crate::menus::main_menu::main_menu_plugin;
use crate::player_stats::PlayerStats;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use resources::images::{load_images, Images};
use resources::sprites::{load_sprites, Sprites};
use crate::bosses::rumia::spell1::reset_spell1;
use crate::level1::reset_level1;
use crate::menus::pause_menu::pause_menu_plugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    MainMenu,
    StartingGame,
    PlayingGame,
    GameOver,
    Paused,
    Resetting,
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
        .add_systems(OnEnter(GameState::MainMenu), despawn_screen::<GameObject>)
        .add_systems(OnEnter(GameState::Resetting), (
            despawn_screen::<GameObject>,
            reset_level1,
            reset_spell1,
            reset_game
        ).chain())
        // TODO: look at one-shot systems in bevy-cheatbook to refactor
        .add_systems(Startup, (setup, load_images, load_sprites).chain())
        .add_plugins((
            main_menu_plugin,
            game_plugin,
            game_over_menu_plugin,
            pause_menu_plugin,
        ))
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

fn reset_game(
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::StartingGame);
}
use crate::bullet::move_bullets;
use crate::enemy::{spawn_enemy, update_enemies, Enemy, EnemySpawner};
use crate::images::Images;
use crate::level1::level1_setup;
use crate::player::{check_bullet_player_collision, move_player, spawn_player, switch_player_sprite};
use crate::sprites::{animate_sprite, Sprites};
use crate::GameState;
use bevy::prelude::*;

pub const FRAME_BORDER_LEFT: f32 = 32. - 400. + 15.;
pub const FRAME_BORDER_TOP: f32 = 300. - 15. - 19.;
pub const FRAME_BORDER_RIGHT: f32 = 480. - 400. + 17.;
pub const FRAME_BORDER_BOTTOM: f32 = 300. - 560. + 2.;

#[derive(Component)]
pub struct SpawnTimer(pub Timer);

pub fn game_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::GAME), (game_setup, level1_setup))
        .add_systems(Update, (
            animate_sprite,
            move_player,
            move_bullets,
            switch_player_sprite,
            check_bullet_player_collision,
            spawn_on_delay,
            update_enemies,
        ));

}

fn game_setup(mut commands: Commands, sprites: Res<Sprites>, images: Res<Images>) {
    draw_background(&mut commands, &images);
    draw_ui_frame(&mut commands, &images);
    spawn_player(commands, sprites);
}

fn draw_background(commands: &mut Commands, images: &Res<Images>) {
    commands.spawn((
        Sprite {
            image: images.dark_background.clone(),
            ..Default::default()
        },
        Transform::from_xyz(200.0, 200.0, 0.0),
    ));
}

fn draw_ui_frame(commands: &mut Commands, images: &Res<Images>) {
    commands.spawn((
        Sprite {
            image: images.frame.clone(),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

fn spawn_on_delay(
    mut commands: Commands,
    sprites: Res<Sprites>,
    time: Res<Time>,
    mut spawns: Query<(&mut EnemySpawner, &mut SpawnTimer)>,
) {
    for (mut enemy_spawner, mut timer) in &mut spawns {
        if timer.0.tick(time.delta()).just_finished() {
            spawn_enemy(&mut commands, &sprites, &mut enemy_spawner);
        }
    }
}
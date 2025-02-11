use std::ptr::replace;
use crate::bullet::{move_bullets, Bullet};
use crate::enemy::{spawn_enemy, update_enemies, Enemy, EnemySpawner};
use crate::images::Images;
use crate::level1::level1_setup;
use crate::player::{check_bullet_player_collision, move_player, spawn_player, switch_player_sprite};
use crate::sprites::{animate_sprite, Sprites};
use crate::{GameState, SCALING_FACTOR};
use bevy::prelude::*;
use crate::player_stats::PlayerStats;

pub const FRAME_BORDER_LEFT: f32 = 32. - 400. + 15.;
pub const FRAME_BORDER_TOP: f32 = 300. - 15. - 19.;
pub const FRAME_BORDER_RIGHT: f32 = 480. - 400. + 17.;
pub const FRAME_BORDER_BOTTOM: f32 = 300. - 560. + 2.;

#[derive(Component)]
pub struct SpawnTimer(pub Timer);

#[derive(Component)]
pub struct PlayerRespawnTimer(pub Timer);

pub fn game_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::GAME), (game_setup, level1_setup))
        .add_systems(Update, (
            animate_sprite,
            move_player,
            respawn_player,
            move_bullets,
            switch_player_sprite,
            check_bullet_player_collision,
            spawn_enemies,
            update_enemies,
        ));

}

fn game_setup(mut commands: Commands, sprites: Res<Sprites>, images: Res<Images>, player_stats: Res<PlayerStats>) {
    draw_background(&mut commands, &images);
    draw_ui_frame(&mut commands, &images, &player_stats);
    spawn_player(&mut commands, &sprites);
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

fn draw_ui_frame(commands: &mut Commands, images: &Res<Images>, player_stats: &Res<PlayerStats>) {
    let player_spell_rect = Rect::new(307.0, 130.0, 343.0, 162.0);
    let player_spell_translation = Vec3::new(162.0, 150.0, 1.1);

    commands.spawn(( // frame background
        Sprite {
            image: images.frame.clone(),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
    commands.spawn(( // Player and Spell text
        Sprite {
            image: images.sidebar.clone(),
            rect: Option::from(player_spell_rect),
            ..Default::default()
        },
        Transform::from_translation(player_spell_translation)
            .with_scale(Vec3::splat(1.5)),
    ));

    let life_counter_left_bound = player_spell_translation.x + player_spell_rect.max.x - player_spell_rect.min.x + 8.0;
    for i in 0..player_stats.lives {
        commands.spawn((
             Sprite {
                 image: images.sidebar.clone(),
                 rect: Option::from(Rect::new(368.0, 98.0, 383.0, 113.0)),
                 ..Default::default()
             },
             Transform::from_xyz(life_counter_left_bound + (i as f32 * 22.0), 163.0, 1.1)
                 .with_scale(Vec3::splat(1.5)),
        ));
    }
}

fn spawn_enemies(
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
fn respawn_player(
    mut commands: Commands,
    sprites: Res<Sprites>,
    time: Res<Time>,
    mut timer_query: Query<&mut PlayerRespawnTimer>,
) {
    for mut timer in &mut timer_query {
        if timer.0.tick(time.delta()).just_finished() {
            spawn_player(&mut commands, &sprites);
        }
    }
}

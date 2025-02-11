use crate::movement_patterns::{BoxedMovementPattern, MovementPattern};
use crate::sprites::Sprites;
use bevy::prelude::*;
use crate::bullet_patterns::BoxedBulletPattern;
use crate::bullet_patterns::bullet_stream::BulletStream;
use crate::enemy::EnemyType::BlueFairy;
use crate::game::SpawnTimer;
use crate::images::Images;
use crate::movement_patterns::move_straight::MoveStraight;
use crate::player::Player;

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
}

#[derive(Clone)]
pub enum EnemyType {
    BlueFairy,
}

#[derive(Component)]
pub struct EnemySpawner {
    pub enemy_type: EnemyType,
    pub starting_position: Vec2,
    pub movement_pattern: BoxedMovementPattern,
    pub bullet_pattern: BoxedBulletPattern,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            enemy_type: BlueFairy,
            starting_position: Vec2::default(),
            movement_pattern: BoxedMovementPattern(Box::new(MoveStraight::default())),
            bullet_pattern: BoxedBulletPattern(Box::new(BulletStream::default())),
        }
    }
}

pub fn spawn_enemy(commands: &mut Commands, sprites: &Res<Sprites>, spawner: &mut EnemySpawner) {
    let enemy_spawner = std::mem::take(spawner);
    commands.spawn((
        Enemy {
            enemy_type: enemy_spawner.enemy_type.clone(),
        },
        Transform::from_xyz(enemy_spawner.starting_position.x, enemy_spawner.starting_position.y, 0.6),
        sprites.blue_fairy.sprite.clone(),
        sprites.blue_fairy.animation_indices.clone(),
        sprites.blue_fairy.animation_timer.clone(),
        enemy_spawner.movement_pattern,
        enemy_spawner.bullet_pattern,
    ));
}

pub fn update_enemies(
    time: Res<Time>,
    mut commands: Commands,
    images: Res<Images>,
    mut enemy_query: Query<(&Enemy, &mut Transform, &mut BoxedMovementPattern, &mut BoxedBulletPattern)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    for (_enemy, mut transform, mut movement_pattern, mut bullet_pattern) in enemy_query.iter_mut() {
        movement_pattern.0.do_move(&mut *transform, &time);
        for player_transform in player_query.iter() {
            bullet_pattern.0.fire(&mut commands, &images, *transform, &time, player_transform);
        }
    }
}

pub fn spawn_enemies(
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
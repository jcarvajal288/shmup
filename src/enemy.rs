use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use crate::movement_patterns::{BoxedMovementPattern, MovementPattern};
use crate::sprites::{AnimatedSprite, Sprites};
use bevy::prelude::*;
use crate::bullet_patterns::BoxedBulletPattern;
use crate::bullet_patterns::bullet_stream::BulletStream;
use crate::enemy::EnemyType::BlueFairy;
use crate::game::SpawnTimer;
use crate::movement_patterns::move_straight::MoveStraight;
use crate::player::{Player, PlayerShot};

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub hit_points: i32,
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
        Name::new("Enemy"),
        Enemy {
            enemy_type: enemy_spawner.enemy_type.clone(),
            hit_points: 5,
        },
        Transform::from_xyz(enemy_spawner.starting_position.x, enemy_spawner.starting_position.y, 0.6),
        sprites.blue_fairy.clone(),
        sprites.blue_fairy.sprite.clone(),
        enemy_spawner.movement_pattern,
        enemy_spawner.bullet_pattern,
    ));
}

pub fn update_enemies(
    time: Res<Time>,
    mut commands: Commands,
    sprites: ResMut<Sprites>,
    mut enemy_query: Query<(&Enemy, &mut Transform, &mut BoxedMovementPattern, &mut BoxedBulletPattern)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    for (_enemy, mut transform, mut movement_pattern, mut bullet_pattern) in enemy_query.iter_mut() {
        movement_pattern.0.do_move(&mut *transform, &time);
        for player_transform in player_query.iter() {
            bullet_pattern.0.fire(&mut commands, &sprites, *transform, &time, player_transform);
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

pub fn check_shot_enemy_collision(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Enemy, &AnimatedSprite, &Transform)>,
    shot_query: Query<(&PlayerShot, &Transform, &Sprite, Entity)>,
) {
    for (mut enemy, enemy_sprite, enemy_transform) in enemy_query.iter_mut() {
        for (shot, shot_transform, shot_sprite, shot_entity) in shot_query.iter() {
            let enemy_hit_circle = BoundingCircle::new(
                enemy_transform.translation.truncate(),
                enemy_sprite.sprite_size as f32 / 2.0
            );
            let shot_hit_box = Aabb2d::new(
                shot_transform.translation.truncate(),
                Vec2::from(shot_sprite.rect.unwrap().half_size()),
            );
            if enemy_hit_circle.intersects(&shot_hit_box) {
                enemy.hit_points -= shot.damage;
                commands.entity(shot_entity).despawn();
            }
        }
    }
}

pub fn check_for_enemy_death(mut commands: Commands, enemy_query: Query<(&Enemy, Entity)>) {
    for (enemy, entity) in enemy_query.iter() {
        if enemy.hit_points <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
use std::f32::consts::PI;
use std::ops::Deref;
use crate::movement_patterns::{BoxedMovementPattern, MovementPattern};
use crate::sprites::Sprites;
use bevy::prelude::*;
use crate::bullet_patterns::BoxedBulletPattern;
use crate::images::Images;
use crate::movement_patterns::move_straight::MoveStraight;

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
}

pub enum EnemyType {
    BlueFairy,
}

pub struct EnemySpawner {
    pub enemy_type: EnemyType,
    pub starting_position: Vec2,
    pub movement_pattern: BoxedMovementPattern,
    pub bullet_pattern: BoxedBulletPattern,
}

pub fn spawn_enemy(commands: &mut Commands, sprites: &Res<Sprites>, spawner: EnemySpawner) {
    commands.spawn((
        Enemy {
            enemy_type: spawner.enemy_type,
        },
        Transform::from_xyz(spawner.starting_position.x, spawner.starting_position.y, 0.6),
        sprites.blue_fairy.sprite.clone(),
        sprites.blue_fairy.animation_indices.clone(),
        sprites.blue_fairy.animation_timer.clone(),
        spawner.movement_pattern,
        spawner.bullet_pattern,
    ));
}

pub fn update_enemies(
    time: Res<Time>,
    mut commands: Commands,
    images: Res<Images>,
    mut enemy_query: Query<(&Enemy, &mut Transform, &mut BoxedMovementPattern, &mut BoxedBulletPattern)>,
) {
    for (_enemy, mut transform, mut movement_pattern, mut bullet_pattern) in enemy_query.iter_mut() {
        movement_pattern.0.do_move(&mut *transform, &time);

        bullet_pattern.0.fire(&mut commands, &images, *transform, &time);
    }
}
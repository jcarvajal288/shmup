use crate::movement_patterns::{BoxedMovementPattern, MovementPattern};
use crate::sprites::Sprites;
use bevy::prelude::*;

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
}

pub fn spawn_enemy(commands: &mut Commands, sprites: &Res<Sprites>, spawner: EnemySpawner) {
    commands.spawn((
        Enemy {
            enemy_type: spawner.enemy_type,
        },
        Transform::from_xyz(spawner.starting_position.x, spawner.starting_position.y, 0.6),
        spawner.movement_pattern,
        sprites.blue_fairy.sprite.clone(),
        sprites.blue_fairy.animation_indices.clone(),
        sprites.blue_fairy.animation_timer.clone(),
    ));
}

pub fn move_enemies(
    time: Res<Time>,
    mut enemy_query: Query<(&Enemy, &mut Transform, &mut BoxedMovementPattern)>,
) {
    for (_enemy, mut transform, mut movement_pattern) in enemy_query.iter_mut() {
        movement_pattern.0.do_move(&mut *transform, &time);
    }
}
use bevy::prelude::*;
use crate::movement_pattern::MovementPattern;
use crate::sprites::Sprites;

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub movement_pattern: Box<dyn MovementPattern>,
}

pub enum EnemyType {
    BlueFairy,
}

pub struct EnemySpawner {
    pub enemy_type: EnemyType,
    pub starting_position: Vec2,
    pub movement_pattern: Box<dyn MovementPattern>,
}

pub fn spawn_enemy(commands: &mut Commands, sprites: &Res<Sprites>, spawner: EnemySpawner) {
    commands.spawn((
        Enemy {
            enemy_type: spawner.enemy_type,
            movement_pattern: spawner.movement_pattern,
        },
        Transform::from_xyz(spawner.starting_position.x, spawner.starting_position.y, 0.6),
        sprites.blue_fairy.sprite.clone(),
        sprites.blue_fairy.animation_indices.clone(),
        sprites.blue_fairy.animation_timer.clone(),
    ));
}

pub fn move_enemies(
    time: Res<Time>,
    mut enemy_query: Query<(&Enemy, &mut Transform)>,
) {
    for (mut enemy, mut transform) in enemy_query.iter_mut() {
        enemy.movement_pattern.do_move(&mut *transform, &time);
    }
}
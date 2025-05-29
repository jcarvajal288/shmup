use crate::enemy::EnemyType::*;
use crate::game::{GameObject, SpawnTimer};
use crate::movement_patterns::MovementPatterns::StraightLine;
use crate::movement_patterns::{run_movement_pattern, MovementPatterns};
use crate::player::PlayerShot;
use crate::resources::sprites::{AnimatedSprite, Sprites};
use crate::sprites;
use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub hit_points: i32,
}

#[derive(Clone)]
pub enum EnemyType {
    BlueFairy,
    Rumia,
}

#[derive(Component)]
pub struct EnemySpawner {
    pub name: &'static str,
    pub enemy_type: EnemyType,
    pub starting_position: Vec2,
    pub movement_pattern: MovementPatterns,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            name: "Enemy",
            enemy_type: BlueFairy,
            starting_position: Vec2::default(),
            movement_pattern: StraightLine(Rot2::degrees(0.0), 0.0),
        }
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnemySystemSet;

pub fn spawn_enemy(commands: &mut Commands, sprites: &Res<Sprites>, spawner: &mut EnemySpawner) {
    let enemy_spawner = std::mem::take(spawner);
    let animated_sprite = sprites::get_sprite_for_enemy_type(sprites, &enemy_spawner.enemy_type);
    commands.spawn((
        Name::new(enemy_spawner.name),
        Enemy {
            enemy_type: enemy_spawner.enemy_type.clone(),
            hit_points: 5,
        },
        Transform::from_xyz(enemy_spawner.starting_position.x, enemy_spawner.starting_position.y, 0.6),
        animated_sprite.clone(),
        animated_sprite.sprite.clone(),
        animated_sprite.animation_indices.clone(),
        animated_sprite.animation_timer.clone(),
        enemy_spawner.movement_pattern,
        GameObject,
    ));
}

pub fn move_enemies(
    time: Res<Time>,
    mut enemy_query: Query<(&Enemy, &MovementPatterns, &mut Transform)>,
) {
    for (_enemy, movement_pattern, mut transform) in enemy_query.iter_mut() {
        run_movement_pattern(movement_pattern, &mut transform, &time);
    }
}

// pub fn update_enemies(
//     mut commands: Commands,
//     sprites: Res<Sprites>,
//     time: Res<Time>,
//     mut enemy_query: Query<(
//         &Enemy,
//         &mut Transform,
//         &mut BoxedMovementPattern,
//         &mut BoxedBulletPattern,
//         &mut BoxedBulletMovementPattern,
//         &mut ShotSchedule,
//     )>,
//     player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
// ) {
//     for (_enemy, mut transform, mut movement_pattern, mut bullet_pattern, mut bullet_movement_pattern, mut shot_schedule) in enemy_query.iter_mut() {
//         movement_pattern.0.do_move(&mut transform, &time);
//         for player_transform in player_query.iter() {
//             if shot_schedule.delay_timer.tick(time.delta()).finished() && shot_schedule.repeat_timer.tick(time.delta()).just_finished() {
//                 bullet_pattern.0.fire(
//                     &mut commands,
//                     &sprites,
//                     *transform,
//                     &time,
//                     player_transform,
//                     &mut bullet_movement_pattern,
//                 );
//                 if shot_schedule.times > 1 {
//                     shot_schedule.repeat_timer.reset();
//                     shot_schedule.times -= 1;
//                 }
//             }
//         }
//     }
// }

pub fn spawn_enemies(
    mut commands: Commands,
    sprites: Res<Sprites>,
    time: Res<Time>,
    mut spawns: Query<(&mut EnemySpawner, &mut SpawnTimer, Entity)>,
) {
    for (mut enemy_spawner, mut timer, entity) in &mut spawns {
        if timer.0.tick(time.delta()).just_finished() {
            println!("spawn_enemy");
            spawn_enemy(&mut commands, &sprites, &mut enemy_spawner);
            commands.entity(entity).despawn_recursive();
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
            // TODO: turn this into a box to account for different x and y
            let enemy_hit_circle = BoundingCircle::new(
                enemy_transform.translation.truncate(),
                enemy_sprite.sprite_size.x as f32 / 2.0
            );
            let shot_hit_box = Aabb2d::new(
                shot_transform.translation.truncate(),
                shot_sprite.rect.unwrap().half_size(),
            );
            if enemy_hit_circle.intersects(&shot_hit_box) {
                enemy.hit_points -= shot.damage;
                commands.entity(shot_entity).try_despawn();
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
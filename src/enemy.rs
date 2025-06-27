use std::f32::consts::PI;
use crate::bullet_patterns::BulletPattern::SingleShotPattern;
use crate::bullet_patterns::{BulletPattern, Target};
use crate::enemy::EnemyType::*;
use crate::game::{is_moving_laterally, GameObject, SpawnTimer, UP_DOWN_MOVEMENT_BRACKET};
use crate::movement_patterns::MovementPatterns::StraightLinePattern;
use crate::movement_patterns::{get_lateral_movement, run_movement_pattern, MovementPatterns};
use crate::player::PlayerShot;
use crate::resources::sprites::{use_side_indices, use_straight_indices, AnimatedSprite, AnimationIndices, Sprites};
use crate::resources;
use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;
use crate::bullet_patterns::single_shot::SingleShot;
use crate::bullet_patterns::shot_schedule::ShotSchedule;
use crate::movement_patterns::straight_line::StraightLine;

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub hit_points: i32,
}

#[derive(Clone)]
pub enum EnemyType {
    BlueFairy,
    BigFairy,
    Rumia,
}

#[derive(Component)]
pub struct EnemySpawner {
    pub name: &'static str,
    pub enemy_type: EnemyType,
    pub hit_points: i32,
    pub starting_position: Vec2,
    pub movement_pattern: MovementPatterns,
    pub bullet_pattern: BulletPattern,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            name: "Enemy",
            enemy_type: BlueFairy,
            hit_points: 5,
            starting_position: Vec2::default(),
            movement_pattern: StraightLinePattern(StraightLine::default()),
            bullet_pattern: SingleShotPattern(SingleShot::default(), Target::Player, ShotSchedule::default())
        }
    }
}

#[derive(Event)]
pub struct EnemyDeathEvent {
    pub enemy_type: EnemyType,
    pub position: Vec3,
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnemySystemSet;

pub fn spawn_enemy(commands: &mut Commands, sprites: &Res<Sprites>, spawner: &mut EnemySpawner) {
    let enemy_spawner = std::mem::take(spawner);
    let animated_sprite = resources::sprites::get_sprite_for_enemy_type(sprites, &enemy_spawner.enemy_type);
    commands.spawn((
        Name::new(enemy_spawner.name),
        Enemy {
            enemy_type: enemy_spawner.enemy_type.clone(),
            hit_points: enemy_spawner.hit_points,
        },
        Transform::from_xyz(enemy_spawner.starting_position.x, enemy_spawner.starting_position.y, 0.6),
        animated_sprite.clone(),
        animated_sprite.sprite.clone(),
        animated_sprite.animation_indices.clone(),
        animated_sprite.animation_timer.clone(),
        enemy_spawner.movement_pattern,
        enemy_spawner.bullet_pattern,
        GameObject,
    ));
}

pub fn move_enemies(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Sprite, &mut MovementPatterns, &mut Transform, &mut AnimationIndices), With<Enemy>>,
) {
    for (mut sprite, mut movement_pattern, mut transform, mut animation_indices) in enemy_query.iter_mut() {
        run_movement_pattern(&mut movement_pattern, &mut transform, &time, false);
        let lateral_movement = get_lateral_movement(&*movement_pattern);
        if is_moving_laterally(movement_pattern, lateral_movement) {
            use_side_indices(&mut animation_indices);
        } else {
            use_straight_indices(&mut animation_indices);
        }
        sprite.flip_x = f32::abs(lateral_movement) >= UP_DOWN_MOVEMENT_BRACKET.end;
    }
}


pub fn spawn_enemies(
    mut commands: Commands,
    sprites: Res<Sprites>,
    time: Res<Time>,
    mut spawns: Query<(&mut EnemySpawner, &mut SpawnTimer, Entity)>,
) {
    for (mut enemy_spawner, mut timer, entity) in &mut spawns {
        if timer.0.tick(time.delta()).just_finished() {
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
        // TODO: turn this into a box to account for different x and y
        let enemy_hit_circle = BoundingCircle::new(
            enemy_transform.translation.truncate(),
            enemy_sprite.sprite_size.x as f32 / 2.0
        );
        for (shot, shot_transform, shot_sprite, shot_entity) in shot_query.iter() {
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

pub fn check_for_enemy_death(
    mut commands: Commands,
    enemy_query: Query<(&Enemy, &Transform, Entity)>,
    mut enemy_death_events: EventWriter<EnemyDeathEvent>,
) {
    for (enemy, transform, entity) in enemy_query.iter() {
        if enemy.hit_points <= 0 {
            enemy_death_events.send(EnemyDeathEvent {
                enemy_type: enemy.enemy_type.clone(),
                position: transform.translation,
            });
            commands.entity(entity).despawn_recursive();
        }
    }
}

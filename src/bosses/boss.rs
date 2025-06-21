use std::f32::consts::PI;
use std::ops::{Div, Range};
use std::time::Duration;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use crate::enemy::EnemyType;
use crate::game::{GameObject, SpawnTimer};
use crate::movement_patterns::{get_lateral_movement, is_finished, run_movement_pattern, DontMove, MovementPatterns};
use crate::resources::sprites::{set_next_animation, AnimatedSprite, AnimationIndices, Sprites};
use crate::resources::sprites::get_sprite_for_enemy_type;
use bevy::prelude::*;
use crate::bosses::boss_health_bar::{listen_for_boss_damage, scale_boss_health_bar, BossDamageEvent};
use crate::movement_patterns::decelerate::create_move_to_pattern;
use crate::movement_patterns::MovementPatterns::{DontMovePattern};
use crate::player::PlayerShot;
use crate::spawns::{SPAWN_CENTER, SPAWN_TOP};

const UP_DOWN_MOVEMENT_BRACKET: Range<f32> = 5.0 * PI / 12.0 .. 7.0 * PI / 12.0;


#[derive(Component)]
pub struct Boss;

#[derive(Component)]
pub struct BossSpawner {
    pub name: &'static str,
    pub enemy_type: EnemyType,
    pub starting_position: Vec2,
    pub movement_pattern: MovementPatterns,
}

impl Default for BossSpawner {
    fn default() -> Self {
        Self {
            name: "Boss",
            enemy_type: EnemyType::Rumia,
            starting_position: Vec2::ZERO,
            movement_pattern: DontMovePattern(DontMove::default()),
        }
    }
}

pub fn spawn_boss(commands: &mut Commands, sprites: &Res<Sprites>, boss_spawner: &mut BossSpawner) {
    let spawner = std::mem::take(boss_spawner);
    let animated_sprite = get_sprite_for_enemy_type(sprites, &spawner.enemy_type);
    commands.spawn((
        Name::new(spawner.name),
        Boss,
        Transform::from_xyz(spawner.starting_position.x, spawner.starting_position.y, 0.6),
        animated_sprite.clone(),
        animated_sprite.sprite.clone(),
        animated_sprite.animation_indices.clone(),
        animated_sprite.animation_timer.clone(),
        spawner.movement_pattern,
        GameObject,
    ));
}

pub fn update_bosses(
    time: Res<Time>,
    mut boss_query: Query<(&Boss, &mut Transform, &mut MovementPatterns, &mut Sprite, &mut AnimationIndices)>,
) {
    for (_boss, mut transform, mut movement_pattern, mut sprite, mut indices) in boss_query.iter_mut() {
        run_movement_pattern(&mut *movement_pattern, &mut *transform, &time, false);
        let lateral_movement = get_lateral_movement(&*movement_pattern);
        if is_moving_laterally(movement_pattern, lateral_movement) {
            set_next_animation(&mut indices, 5, 5);
        } else {
            set_next_animation(&mut indices, 4, 4);
        }
        sprite.flip_x = lateral_movement >= PI / 2.0;
    }
}

fn is_moving_laterally(movement_pattern: Mut<MovementPatterns>, lateral_movement: f32) -> bool {
    !UP_DOWN_MOVEMENT_BRACKET.contains(&lateral_movement.abs()) && !is_finished(&*movement_pattern)
}

pub fn spawn_bosses(
    mut commands: Commands,
    sprites: Res<Sprites>,
    time: Res<Time>,
    mut spawns: Query<(&mut BossSpawner, &mut SpawnTimer, Entity)>,
) {
    for (mut boss_spawner, mut timer, entity) in &mut spawns {
        if timer.0.tick(time.delta()).just_finished() {
            spawn_boss(&mut commands, &sprites, &mut boss_spawner);
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn check_boss_being_shot(
    mut commands: Commands,
    boss_query: Query<(&Transform, &AnimatedSprite), With<Boss>>,
    mut boss_damage_event_writer: EventWriter<BossDamageEvent>,
    shot_query: Query<(&PlayerShot, &Transform, &Sprite, Entity)>,
) {
    for (boss_transform, boss_sprite) in boss_query.iter() {
        let boss_hit_box = Aabb2d::new(
            boss_transform.translation.truncate(),
            boss_sprite.sprite_size.as_vec2().div(2.0),
        );
        for (shot, shot_transform, shot_sprite, shot_entity) in shot_query.iter() {
            let shot_hit_box = Aabb2d::new(
                shot_transform.translation.truncate(),
                shot_sprite.rect.unwrap().half_size(),
            );
            if boss_hit_box.intersects(&shot_hit_box) {
                boss_damage_event_writer.send(BossDamageEvent(shot.damage));
                commands.entity(shot_entity).try_despawn();
            }
        }
    }
}
use crate::enemy::EnemyType;
use crate::game::{GameObject, SpawnTimer};
use crate::movement_patterns::move_straight::MoveStraight;
use crate::movement_patterns::BoxedMovementPattern;
use crate::resources::sprites::{set_next_animation, AnimationIndices, Sprites};
use crate::sprites::get_sprite_for_enemy_type;
use bevy::prelude::*;

#[derive(Component)]
pub struct Boss;

#[derive(Component)]
pub struct BossSpawner {
    pub name: &'static str,
    pub enemy_type: EnemyType,
    pub starting_position: Vec2,
    pub movement_pattern: BoxedMovementPattern,
}

impl Default for BossSpawner {
    fn default() -> Self {
        Self {
            name: "Boss",
            enemy_type: EnemyType::Rumia,
            starting_position: Vec2::ZERO,
            movement_pattern: BoxedMovementPattern(Box::new(MoveStraight::default())),
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
    mut boss_query: Query<(&Boss, &mut Transform, &mut BoxedMovementPattern, &mut Sprite, &mut AnimationIndices)>,
) {
    for (_boss, mut transform, mut movement_pattern, mut sprite, mut indices) in boss_query.iter_mut() {
        movement_pattern.0.do_move(&mut transform, &time);
        let lateral_movement = movement_pattern.0.lateral_movement();
        if !(-1.0..1.0).contains(&lateral_movement) {
            set_next_animation(&mut indices, 5, 5);
        } else {
            set_next_animation(&mut indices, 4, 4);
        }
        sprite.flip_x = lateral_movement < 0.0;
    }
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

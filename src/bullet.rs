use std::time::Duration;
use crate::game::{is_in_playfield, GameObject, SpawnTimer};
use crate::movement_patterns::MovementPatterns::DontMovePattern;
use crate::movement_patterns::{run_movement_pattern, DontMove, MovementPatterns};
use crate::resources::sprites::Sprites;
use bevy::prelude::*;
use crate::bullet_patterns::{fire_bullet_pattern, BulletPatterns};
use crate::player::Player;

#[derive(Component)]
pub struct Bullet {
    pub bullet_type: BulletType,
}

pub struct BulletProps {
    pub hit_circle_radius: f32,
}

#[derive(Clone, Copy)]
pub enum BulletType {
    WhiteArrow,
    BlueRimmedCircle,
    RedRimmedCircle,
    SmallRedCircle,
    SmallYellowCircle,
    SmallGreenCircle,
    SmallPurpleCircle,
    SmallBlueCircle,
}

#[derive(Component)]
pub struct BulletSpawner {
    pub bullet_type: BulletType,
    pub position: Vec2,
    pub movement_pattern: MovementPatterns,
}

#[derive(Event)]
pub struct BulletSpawnEvent {
    pub bullet_type: BulletType,
    pub position: Vec2,
    pub movement_pattern: MovementPatterns,
}

impl Default for BulletSpawnEvent {
    fn default() -> Self {
        Self {
            bullet_type: BulletType::WhiteArrow,
            position: Default::default(),
            movement_pattern: DontMovePattern(DontMove::default()),
        }
    }
}

pub fn read_bullet_spawn_events(
    sprites: Res<Sprites>,
    mut commands: Commands,
    mut bullet_spawn_events: EventReader<BulletSpawnEvent>,
) {
    for event in bullet_spawn_events.read() {
        let spawner = BulletSpawner {
            bullet_type: event.bullet_type,
            position: event.position,
            movement_pattern: event.movement_pattern.clone(),
        };
        if is_in_playfield(event.position) {
            spawn_bullet(&sprites, &mut commands, &spawner);
        }
    }
}

pub fn spawn_delayed_bullets(
    sprites: Res<Sprites>,
    mut commands: Commands,
    time: Res<Time>,
    mut bullet_spawner_query: Query<(&BulletSpawner, &mut SpawnTimer, Entity)>,
) {
    for (bullet_spawner, mut timer, entity) in &mut bullet_spawner_query {
        if timer.0.tick(time.delta()).just_finished() {
            spawn_bullet(&sprites, &mut commands, bullet_spawner);
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_bullet(sprites: &Res<Sprites>, commands: &mut Commands, spawner: &BulletSpawner) {
    commands.spawn((
        Name::new("Bullet"),
        sprite_for_bullet_type(&spawner.bullet_type, &sprites),
        Transform::from_xyz(spawner.position.x, spawner.position.y, 0.7),
        Bullet {
            bullet_type: spawner.bullet_type,
        },
        spawner.movement_pattern.clone(),
        GameObject,
    ));
}

pub fn props_for_bullet_type(_bullet_type: &BulletType) -> BulletProps {
    BulletProps {
        hit_circle_radius: 1.0,
    }
}

pub fn move_bullets(
    time: Res<Time>,
    mut bullet_query: Query<(&Bullet, &mut Transform, &mut MovementPatterns)>,
) {
    for (_bullet, mut transform, mut movement_pattern) in bullet_query.iter_mut() {
        run_movement_pattern(&mut movement_pattern, &mut transform, &time, true)
    }
}

fn sprite_for_bullet_type(bullet_type: &BulletType, sprites: &Sprites) -> Sprite {
    match bullet_type {
        BulletType::WhiteArrow => sprites.bullet_white_arrow.clone(),
        BulletType::BlueRimmedCircle => sprites.bullet_blue_rimmed_circle.clone(),
        BulletType::RedRimmedCircle => sprites.bullet_red_rimmed_circle.clone(),
        BulletType::SmallRedCircle => sprites.bullet_small_red_circle.clone(),
        BulletType::SmallYellowCircle => sprites.bullet_small_yellow_circle.clone(),
        BulletType::SmallGreenCircle => sprites.bullet_small_green_circle.clone(),
        BulletType::SmallPurpleCircle => sprites.bullet_small_purple_circle.clone(),
        BulletType::SmallBlueCircle => sprites.bullet_small_blue_circle.clone(),
    }
}

pub fn fire_bullet_patterns(
    time: Res<Time>,
    player_transform_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut BulletPatterns, &mut Transform), Without<Player>>,
    mut bullet_spawn_events: EventWriter<BulletSpawnEvent>,
) {
    for (player_transform) in player_transform_query.iter() {
        for (mut bullet_pattern, transform) in enemy_query.iter_mut() {
            fire_bullet_pattern(&mut bullet_pattern, &time, &transform, &player_transform, &mut bullet_spawn_events);
        }
    }
}
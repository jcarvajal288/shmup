use crate::enemy::Enemy;
use crate::game::GameObject;
use crate::movement_patterns::MovementPatterns::{StraightAtPlayer, StraightLine};
use crate::movement_patterns::{run_movement_pattern, BoxedBulletMovementPattern, MovementPatterns};
use crate::player::Player;
use crate::resources::sprites::Sprites;
use bevy::prelude::*;

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
    SmallRedCircle,
    SmallYellowCircle,
    SmallGreenCircle,
    SmallPurpleCircle,
    SmallBlueCircle,
}

#[derive(Component, Clone, Default)]
pub struct ShotSchedule {
    pub delay_timer: Timer,
    pub repeat_timer: Timer,
    pub times: usize,
}

#[derive(Component)]
pub struct BulletSpawner {
    pub bullet_type: BulletType,
    pub position: Vec2,
    pub movement_pattern: MovementPatterns,
}

pub fn spawn_bullet(commands: &mut Commands, sprites: &Res<Sprites>, bullet_spawner: BulletSpawner) {
    commands.spawn((
        Name::new("Bullet"),
        sprite_for_bullet_type(&bullet_spawner.bullet_type, sprites),
        Transform::from_xyz(bullet_spawner.position.x, bullet_spawner.position.y, 0.7),
        Bullet {
            bullet_type: bullet_spawner.bullet_type,
        },
        bullet_spawner.movement_pattern,
        GameObject,
    ));
}

pub fn spawn_bullets(
    sprites: Res<Sprites>,
    mut commands: Commands,
    mut query: Query<(Entity, &BulletSpawner)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    for (entity, bullet_spawner) in query.iter_mut() {
        for player_transform in player_query.iter() {
            let movement_pattern = match bullet_spawner.movement_pattern {
                StraightAtPlayer(speed) => {
                    let diff = player_transform.translation.truncate() - bullet_spawner.position;
                    let angle = diff.y.atan2(diff.x);
                    StraightLine(Rot2::radians(angle), speed, true)
                }
                _ => bullet_spawner.movement_pattern.clone(),
            };

            commands.spawn((
                Name::new("Bullet"),
                sprite_for_bullet_type(&bullet_spawner.bullet_type, &sprites),
                Transform::from_xyz(bullet_spawner.position.x, bullet_spawner.position.y, 0.7),
                Bullet {
                    bullet_type: bullet_spawner.bullet_type,
                },
                movement_pattern,
                GameObject,
            ));
            commands.entity(entity).despawn();
        }
    }
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
        run_movement_pattern(&movement_pattern, &mut transform, &time)
    }
}

fn sprite_for_bullet_type(bullet_type: &BulletType, sprites: &Sprites) -> Sprite {
    match bullet_type {
        BulletType::WhiteArrow => sprites.bullet_white_arrow.clone(),
        BulletType::BlueRimmedCircle => sprites.bullet_blue_rimmed_circle.clone(),
        BulletType::SmallRedCircle => sprites.bullet_small_red_circle.clone(),
        BulletType::SmallYellowCircle => sprites.bullet_small_yellow_circle.clone(),
        BulletType::SmallGreenCircle => sprites.bullet_small_green_circle.clone(),
        BulletType::SmallPurpleCircle => sprites.bullet_small_purple_circle.clone(),
        BulletType::SmallBlueCircle => sprites.bullet_small_blue_circle.clone(),
    }
}
pub mod bullet_stream;
pub mod circle_spawn;

use std::time::Duration;
use bevy::math::Rot2;
use crate::bullet_patterns::BulletPatternTarget::Player;
use crate::movement_patterns::{BoxedBulletMovementPattern, MovementPatterns};
use crate::resources::sprites::Sprites;
use bevy::prelude::{Commands, Component, Res, Time, Transform};
use bevy::time::Timer;
use dyn_clone::DynClone;
use crate::bullet::{spawn_bullet, BulletSpawner, BulletType};
use crate::bullet_patterns::BulletPatterns::ShootAtPlayer;
use crate::movement_patterns::move_straight::MoveStraight;
use crate::movement_patterns::MovementPatterns::StraightAtPlayer;

#[derive(Component)]
pub enum BulletPatterns {
    ShootAtPlayer(BulletType, f32, Timer), // bullet type, speed, shot interval
}

pub fn fire_bullet_pattern(
    commands: &mut Commands,
    bullet_pattern: &mut BulletPatterns,
    time: &Res<Time>,
    transform: &Transform
) {
    match bullet_pattern {
        ShootAtPlayer(bullet_type, speed, ref mut shot_timer) => {
            if shot_timer.tick(time.delta()).just_finished() {
                commands.spawn((
                    BulletSpawner {
                        bullet_type: *bullet_type,
                        position: transform.translation.truncate(),
                        movement_pattern: StraightAtPlayer(*speed)
                    }
                ));
                shot_timer.reset();
            }
        }
    }
}

pub trait BulletPattern: DynClone {
    fn fire(
        &mut self,
        commands: &mut Commands,
        sprites: &Res<Sprites>,
        transform: Transform,
        time: &Res<Time>,
        player_transform: &Transform,
        movement_pattern: &mut BoxedBulletMovementPattern
    );
}

dyn_clone::clone_trait_object!(BulletPattern);

#[derive(Component, Clone)]
pub struct BoxedBulletPattern(pub Box<dyn BulletPattern + Send + Sync>);

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum BulletPatternTarget {
    Player,
    Down,
}

#[derive(Clone, Copy)]
pub struct BulletPatternAngle {
    pub target: BulletPatternTarget,
    pub spread: f32,
    pub offset: f32,
}

fn get_target_transform(target: &BulletPatternTarget, starting_transform: &Transform, player_transform: &Transform) -> Transform {
    if *target == Player {
        Transform::from_translation(player_transform.translation - starting_transform.translation)
    } else {
        Transform::from_translation(*starting_transform.down())
    }
}


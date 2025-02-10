use crate::bullet::BulletType::*;
use crate::bullet_patterns::bullet_stream::BulletStream;
use crate::bullet_patterns::BulletPatternTarget::*;
use crate::bullet_patterns::{BoxedBulletPattern, BulletPatternAngle};
use crate::enemy::EnemyType::*;
use crate::enemy::EnemySpawner;
use crate::game::SpawnTimer;
use crate::movement_patterns::move_straight::MoveStraight;
use crate::movement_patterns::BoxedMovementPattern;
use crate::sprites::Sprites;
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn level1_setup(mut commands: Commands) {

    commands.spawn((
        EnemySpawner {
            enemy_type: BlueFairy,
            starting_position: Vec2::new(-248.0, 150.0),
            movement_pattern: BoxedMovementPattern(Box::new(MoveStraight {
                angle: 0.0,
                speed: 30.0,
                acceleration: 0.0,
                ..default()
            })),
            bullet_pattern: BoxedBulletPattern(Box::new(BulletStream {
                bullet_type: WhiteArrow,
                bullets_per_wave: 24,
                waves_per_iteration: 3,
                num_iterations: 99,
                angle: BulletPatternAngle {
                    target: Player,
                    spread: PI * 2.0,
                },
                speed: 20.0,
                acceleration: 0.3,
                startup_timer: Timer::from_seconds(1.0, TimerMode::Once),
                wave_timer: Timer::from_seconds(0.3, TimerMode::Once),
                iteration_timer: Timer::from_seconds(0.5, TimerMode::Once),
                ..default()
            })),
        },
        SpawnTimer(Timer::from_seconds(1.0, TimerMode::Once)),
    ));

    commands.spawn((
        EnemySpawner {
            enemy_type: BlueFairy,
            starting_position: Vec2::new(0.0, 150.0),
            movement_pattern: BoxedMovementPattern(Box::new(MoveStraight {
                angle: PI,
                speed: 30.0,
                acceleration: 0.0,
                ..default()
            })),
            bullet_pattern: BoxedBulletPattern(Box::new(BulletStream {
                bullet_type: WhiteArrow,
                bullets_per_wave: 24,
                waves_per_iteration: 3,
                num_iterations: 99,
                angle: BulletPatternAngle {
                    target: Player,
                    spread: PI * 2.0,
                },
                speed: 20.0,
                acceleration: 0.3,
                startup_timer: Timer::from_seconds(1.0, TimerMode::Once),
                wave_timer: Timer::from_seconds(0.3, TimerMode::Once),
                iteration_timer: Timer::from_seconds(0.5, TimerMode::Once),
                ..default()
            })),
        },
        SpawnTimer(Timer::from_seconds(5.0, TimerMode::Once)),
   ));
}
use crate::bullet::BulletType::*;
use crate::bullet_patterns::bullet_stream::BulletStream;
use crate::bullet_patterns::BulletPatternTarget::*;
use crate::bullet_patterns::{BoxedBulletPattern, BulletPatternAngle};
use crate::enemy::EnemySpawner;
use crate::enemy::EnemyType::*;
use crate::game::SpawnTimer;
use crate::movement_patterns::move_straight::MoveStraight;
use crate::movement_patterns::BoxedMovementPattern;
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn level1_setup(mut commands: Commands) {

    let bullet_stream = BulletStream {
        bullet_type: WhiteArrow,
        bullets_per_wave: 1,
        waves_per_iteration: 1,
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
    };

    for i in 0..5 {
        let initial_delay = 0.0;
        let iter_delay = 1.0;
        let full_delay = initial_delay + (iter_delay * i as f32);
        commands.spawn((
            Name::new("EnemySpawner"),
            EnemySpawner {
                enemy_type: BlueFairy,
                starting_position: Vec2::new(-400.0, 150.0),
                movement_pattern: BoxedMovementPattern(Box::new(MoveStraight {
                    angle: 0.0,
                    speed: 40.0,
                    ..default()
                })),
                bullet_pattern: BoxedBulletPattern(Box::new(bullet_stream.clone())),
            },
            SpawnTimer(Timer::from_seconds(full_delay, TimerMode::Once)),
        ));
    }
}
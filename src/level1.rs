use std::f32::consts::PI;
use crate::bullet::BulletType::*;
use crate::enemy::EnemyType::*;
use crate::enemy::{spawn_enemy, EnemySpawner};
use crate::movement_patterns::{BoxedMovementPattern};
use crate::sprites::Sprites;
use bevy::prelude::*;
use crate::bullet_patterns::BoxedBulletPattern;
use crate::bullet_patterns::bullet_stream::BulletStream;
use crate::movement_patterns::move_straight::MoveStraight;

pub fn level1_system(mut commands: Commands, sprites: Res<Sprites>) {

    spawn_enemy(&mut commands, &sprites, EnemySpawner {
        enemy_type: BlueFairy,
        starting_position: Vec2::new(-128.0, 150.0),
        movement_pattern: BoxedMovementPattern(Box::new(MoveStraight::default())),
        bullet_pattern: BoxedBulletPattern(Box::new(BulletStream {
            bullet_type: WhiteArrow,
            num_bullets: 1,
            num_iterations: 10,
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            movement_pattern: BoxedMovementPattern(Box::new(MoveStraight {
                angle: PI,
                speed: 10.0,
                acceleration: 0.0,
                face_travel_direction: true,
            })),
        })),
    });

}
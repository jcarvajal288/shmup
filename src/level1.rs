use crate::bullet::BulletType::*;
use crate::enemy::EnemyType::*;
use crate::enemy::{spawn_enemy, EnemySpawner};
use crate::images::Images;
use crate::movement_patterns::{BoxedMovementPattern};
use crate::sprites::Sprites;
use bevy::prelude::*;
use crate::movement_patterns::move_straight::MoveStraight;

pub fn level1_system(mut commands: Commands, sprites: Res<Sprites>, images: Res<Images>) {

    spawn_enemy(&mut commands, &sprites, EnemySpawner {
        enemy_type: BlueFairy,
        starting_position: Vec2::new(-128.0, 150.0),
        movement_pattern: BoxedMovementPattern(Box::new(MoveStraight::default())),
    });

    // spawn_bullet(&mut commands, &images, BulletSpawner {
    //     bullet_type: WhiteArrow,
    //     position: Vec2::new(-128.0, 0.0),
    //     angle: -std::f32::consts::PI,
    //     speed: 200.0,
    // });
}
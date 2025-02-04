use crate::bullet::{spawn_bullet, BulletSpawner, BulletType};
use crate::images::Images;
use crate::sprites::Sprites;
use bevy::prelude::*;

pub fn level1_system(mut commands: Commands, sprites: Res<Sprites>, images: Res<Images>) {
    commands.spawn((
        Transform::from_xyz(-128.0, 150.0, 0.6),
        sprites.blue_fairy.sprite.clone(),
        sprites.blue_fairy.animation_indices.clone(),
        sprites.blue_fairy.animation_timer.clone(),
    ));
    spawn_bullet(&mut commands, &images, BulletSpawner {
        bullet_type: BulletType::WhiteArrow,
        position: Vec2::new(-128.0, 0.0),
        angle: -std::f32::consts::PI,
        speed: 200.0,
    });
}
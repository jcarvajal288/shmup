use bevy::math::Vec2;
use bevy::prelude::{Commands, Quat, Rect, Res, Sprite, TextureSlice, TextureSlicer, Transform};
use crate::images::Images;
use crate::sprites::Sprites;

pub fn level1_system(mut commands: Commands, sprites: Res<Sprites>, images: Res<Images>) {
    commands.spawn((
        Transform::from_xyz(-128.0, 150.0, 0.6),
        sprites.blue_fairy.sprite.clone(),
        sprites.blue_fairy.animation_indices.clone(),
        sprites.blue_fairy.animation_timer.clone(),
    ));
    commands.spawn((
        Sprite {
            image: images.bullets.clone(),
            rect: Option::from(Rect {
                min: Vec2::new(0.0, 16.0),
                max: Vec2::new(16.0, 32.0),
            }),
            ..Default::default()
        },
        Transform::from_xyz(-128.0, 0.0, 0.6)
            .with_rotation(Quat::from_rotation_z(-std::f32::consts::PI)),
    ));
}
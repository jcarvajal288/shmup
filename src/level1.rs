use bevy::prelude::{Commands, Res, Transform};
use crate::sprites::Sprites;

pub fn level1_system(mut commands: Commands, sprites: Res<Sprites>) {
    commands.spawn((
        Transform::from_xyz(-128.0, 150.0, 0.6),
        sprites.blue_fairy.sprite.clone(),
        sprites.blue_fairy.animation_indices.clone(),
        sprites.blue_fairy.animation_timer.clone(),
    ));
}
use crate::sprites::Sprites;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Sprite)]
pub struct Player {
    pub position: Vec2,
}

pub fn spawn_player(mut commands: Commands, sprites: Res<Sprites>) {
    commands.spawn((
        Player {
            position: Vec2::new(0.0, 0.0),
            ..default()
        },
        sprites.remilia.sprite.clone(),
        sprites.remilia.animation_indices.clone(),
        sprites.remilia.animation_timer.clone(),
    ));
}
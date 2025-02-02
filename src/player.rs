use crate::sprites::{AnimationIndices, Sprites};
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Sprite)]
pub struct Player {
    pub movement_speed: f32,
}

pub fn spawn_player(mut commands: Commands, sprites: Res<Sprites>) {
    commands.spawn((
        Player {
            movement_speed: 100.0,
            ..default()
        },
        Transform::from_xyz(0.0, -150.0, 1.0),
        sprites.remilia.sprite.clone(),
        sprites.remilia.animation_indices.clone(),
        sprites.remilia.animation_timer.clone(),
    ));
}

pub fn move_player(
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (player, mut transform) in &mut player_query {
        if keyboard.pressed(KeyCode::ArrowUp) {
            transform.translation.y += player.movement_speed * time.delta_secs();
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= player.movement_speed * time.delta_secs();
        }
        if keyboard.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= player.movement_speed * time.delta_secs();
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            transform.translation.x += player.movement_speed * time.delta_secs();
        }
    }
}

pub fn switch_player_sprite(
    mut player_query: Query<(&Player, &mut Sprite, &mut AnimationIndices)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (_player, mut sprite, mut animation_indices) in &mut player_query {
        if keyboard.pressed(KeyCode::ArrowLeft) {
            animation_indices.first = 4;
            animation_indices.last = 7;
            sprite.flip_x = false;
        } else if keyboard.pressed(KeyCode::ArrowRight) {
            animation_indices.first = 4;
            animation_indices.last = 7;
            sprite.flip_x = true;
        } else {
            animation_indices.first = 0;
            animation_indices.last = 3;
            sprite.flip_x = false;
        }
    }
}
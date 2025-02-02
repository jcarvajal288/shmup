use crate::player::{move_player, spawn_player, switch_player_sprite};
use crate::sprites::{animate_sprite, Sprites};
use crate::GameState;
use bevy::prelude::*;
use crate::images::Images;

pub fn game_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::GAME), game_setup)
        .add_systems(Update, (animate_sprite, move_player, switch_player_sprite))
    ;

}

fn game_setup(mut commands: Commands, sprites: Res<Sprites>, images: Res<Images>) {
    draw_background(&mut commands, images);
    spawn_player(commands, sprites);
}

fn draw_background(commands: &mut Commands, images: Res<Images>) {
    commands.spawn((
        Sprite {
            image: images.dark_background.clone(),
            ..Default::default()
        },
        Transform::from_xyz(200.0, 200.0, 0.0),
    ));
}
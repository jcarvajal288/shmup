use crate::player::{move_player, spawn_player, switch_player_sprite};
use crate::sprites::{animate_sprite, Sprites};
use crate::GameState;
use bevy::prelude::*;
use crate::images::Images;
use crate::level1::level1_system;

pub const FRAME_BORDER_LEFT: f32 = 32. - 400. + 15.;
pub const FRAME_BORDER_TOP: f32 = 300. - 15. - 19.;
pub const FRAME_BORDER_RIGHT: f32 = 480. - 400. + 17.;
pub const FRAME_BORDER_BOTTOM: f32 = 300. - 560. + 2.;

pub fn game_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::GAME), (game_setup, level1_system))
        .add_systems(Update, (animate_sprite, move_player, switch_player_sprite))
    ;

}

fn game_setup(mut commands: Commands, sprites: Res<Sprites>, images: Res<Images>) {
    draw_background(&mut commands, &images);
    draw_frame(&mut commands, &images);
    spawn_player(commands, sprites);
}

fn draw_background(commands: &mut Commands, images: &Res<Images>) {
    commands.spawn((
        Sprite {
            image: images.dark_background.clone(),
            ..Default::default()
        },
        Transform::from_xyz(200.0, 200.0, 0.0),
    ));
}

fn draw_frame(commands: &mut Commands, images: &Res<Images>) {
    commands.spawn((
        Sprite {
            image: images.frame.clone(),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}
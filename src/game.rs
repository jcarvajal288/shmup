use crate::player::{move_player, spawn_player, switch_player_sprite};
use crate::sprites::{animate_sprite, Sprites};
use crate::GameState;
use bevy::prelude::*;

pub fn game_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::GAME), game_setup)
        .add_systems(Update, (animate_sprite, move_player, switch_player_sprite))
    ;

}

fn game_setup(commands: Commands, sprites: Res<Sprites>) {
    spawn_player(commands, sprites);
}
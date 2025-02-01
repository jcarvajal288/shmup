use bevy::prelude::*;
use crate::GameState;
use crate::player::spawn_player;
use crate::sprites::{animate_sprite, load_sprites, Sprites};

pub fn game_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::GAME), game_setup)
        .add_systems(Update, animate_sprite)
    ;

}

fn game_setup(commands: Commands, sprites: Res<Sprites>) {
    spawn_player(commands, sprites);
}
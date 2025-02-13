use crate::bullet::move_bullets;
use crate::enemy::{spawn_enemies, update_enemies};
use crate::level1::level1_setup;
use crate::player::{check_bullet_player_collision, fire_shot, move_player, move_shot, respawn_player, spawn_player, switch_player_sprite, PlayerDeathEvent};
use crate::player_stats::{initialize_player_stats, listen_for_player_death};
use crate::sprites::{animate_sprite, Sprites};
use crate::GameState;
use bevy::prelude::*;

pub const FRAME_BORDER_LEFT: f32 = -353.0;
pub const FRAME_BORDER_TOP: f32 = 266.0;
pub const FRAME_BORDER_RIGHT: f32 = 97.0;
pub const FRAME_BORDER_BOTTOM: f32 = -258.0;

#[derive(Component)]
pub struct SpawnTimer(pub Timer);

#[derive(Component)]
pub struct PlayerRespawnTimer(pub Timer);

pub fn game_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::GAME), (
            game_setup,
            level1_setup,
            initialize_player_stats,
        ))
        .add_systems(Update, (
            animate_sprite,
            move_player,
            respawn_player,
            move_bullets,
            switch_player_sprite,
            check_bullet_player_collision,
            spawn_enemies,
            update_enemies,
            listen_for_player_death,
            fire_shot,
            move_shot,
        ))
        .add_event::<PlayerDeathEvent>()
    ;

}

fn game_setup(mut commands: Commands, sprites: ResMut<Sprites>) {
    draw_background(&mut commands, &sprites);
    draw_ui_frame(&mut commands, &sprites);
    spawn_player(&mut commands, &sprites);
}

fn draw_background(commands: &mut Commands, sprites: &ResMut<Sprites>) {
    commands.spawn((
        sprites.dark_background.clone(),
        Transform::from_xyz(200.0, 200.0, 0.0),
    ));
}

fn draw_ui_frame(commands: &mut Commands, sprites: &ResMut<Sprites>) {
    commands.spawn((
        sprites.frame.clone(),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
    commands.spawn((
        sprites.player_spell_text.clone(),
        Transform::from_xyz(162.0, 150.0, 1.1)
            .with_scale(Vec3::splat(1.5)),
    ));

}


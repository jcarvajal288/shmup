use crate::player::{PlayerContinueEvent, PlayerDeathEvent};
use crate::resources::sprites::Sprites;
use crate::GameState;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, Entity, EventReader, Name, NextState, ResMut, Resource, Transform};
use crate::game::GameObject;

#[derive(Component)]
struct PlayerLifeCounter;

#[derive(Resource)]
pub struct PlayerStats {
    pub starting_life_count: usize,
    pub lives: Vec<Entity>,
}

const STARTING_LIFE_COUNT: usize = 2;

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            starting_life_count: STARTING_LIFE_COUNT,
            lives: Default::default(),
        }
    }
}

pub fn initialize_player_stats(
    commands: Commands,
    mut player_stats: ResMut<PlayerStats>,
    sprites: ResMut<Sprites>,
) {
    reset_player_lives(commands, &mut player_stats, sprites);
}

fn reset_player_lives(mut commands: Commands, player_stats: &mut ResMut<PlayerStats>, sprites: ResMut<Sprites>) {
    let lives_left_bound = 206.0;
    player_stats.lives.clear();
    for i in 0..player_stats.starting_life_count {
        player_stats.lives.push(commands.spawn((
            Name::new("PlayerLifeCounter"),
            sprites.life_counter.clone(),
            Transform::from_xyz(lives_left_bound + (i as f32 * 22.0), 163.0, 1.1)
                .with_scale(Vec3::splat(1.5)),
            PlayerLifeCounter,
            GameObject
        )).id());
    }
}

pub fn listen_for_player_death(
    mut commands: Commands,
    mut player_stats: ResMut<PlayerStats>,
    mut player_death_event_reader: EventReader<PlayerDeathEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if !player_death_event_reader.is_empty() {
        match player_stats.lives.pop() {
            Some(life_counter) => commands.entity(life_counter).despawn(),
            None => {
                game_state.set(GameState::GameOver);
            },
        }
        player_death_event_reader.clear();
    }
}

pub fn listen_for_player_continue(
    commands: Commands,
    mut player_stats: ResMut<PlayerStats>,
    mut player_continue_event_reader: EventReader<PlayerContinueEvent>,
    sprites: ResMut<Sprites>,
) {
    if !player_continue_event_reader.is_empty() {
        reset_player_lives(commands, &mut player_stats, sprites);
        player_continue_event_reader.clear();
    }
}

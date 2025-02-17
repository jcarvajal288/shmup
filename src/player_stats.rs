use crate::player::PlayerDeathEvent;
use crate::resources::sprites::Sprites;
use crate::GameState;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, Entity, EventReader, NextState, ResMut, Resource, Transform};

#[derive(Component)]
struct PlayerLifeCounter;

#[derive(Resource)]
pub struct PlayerStats {
    pub starting_life_count: usize,
    pub lives: Vec<Entity>,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            starting_life_count: 2,
            lives: Default::default(),
        }
    }
}

pub fn initialize_player_stats(
    mut commands: Commands,
    mut player_stats: ResMut<PlayerStats>,
    sprites: ResMut<Sprites>,
) {
    let lives_left_bound = 206.0;
    for i in 0..player_stats.starting_life_count {
        player_stats.lives.push(commands.spawn((
            sprites.life_counter.clone(),
            Transform::from_xyz(lives_left_bound + (i as f32 * 22.0), 163.0, 1.1)
                .with_scale(Vec3::splat(1.5)),
            PlayerLifeCounter
        )).id());
    }
}

pub fn listen_for_player_death(
    mut commands: Commands,
    mut player_stats: ResMut<PlayerStats>,
    mut player_death_event_reader: EventReader<PlayerDeathEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for _event in player_death_event_reader.read() {
        match player_stats.lives.pop() {
            Some(life_counter) => commands.entity(life_counter).despawn(),
            None => game_state.set(GameState::GameOver),
        }
    }
}

use bevy::math::{Rect, Vec3};
use bevy::prelude::{Commands, Component, Entity, EventReader, Res, ResMut, Resource, Sprite, Transform};
use crate::images::Images;
use crate::player::PlayerDeathEvent;

#[derive(Component)]
struct PlayerLife;

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
    images: Res<Images>,
) {
    let lives_left_bound = 206.0;
    for i in 0..player_stats.starting_life_count {
        player_stats.lives.push(commands.spawn((
            Sprite {
                image: images.sidebar.clone(),
                rect: Option::from(Rect::new(368.0, 98.0, 383.0, 113.0)),
                ..Default::default()
            },
            Transform::from_xyz(lives_left_bound + (i as f32 * 22.0), 163.0, 1.1)
                .with_scale(Vec3::splat(1.5)),
            PlayerLife
        )).id());
    }
}

pub fn listen_for_player_death(
    mut commands: Commands,
    mut player_stats: ResMut<PlayerStats>,
    mut player_death_event_reader: EventReader<PlayerDeathEvent>
) {
    for _event in player_death_event_reader.read() {
        match player_stats.lives.pop() {
            Some(entity) => commands.entity(entity).despawn(),
            None => (),
        }
    }
}

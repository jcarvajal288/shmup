pub mod spell1;

use crate::bosses::boss::{Boss, BossSpawner};
use crate::bosses::rumia::spell1::{spell1_plugin, Spell1State};
use crate::enemy::EnemyType::Rumia;
use crate::game::{GameObject, SpawnTimer, FRAME_BORDER_TOP, SPAWN_CENTER, SPAWN_TOP};
use crate::level1::FirstLevelState;
use crate::movement_patterns::move_to::create_move_to_pattern;
use crate::movement_patterns::{is_finished, MovementPatterns};
use bevy::prelude::*;
use std::time::Duration;
use crate::movement_patterns::sine_wave::create_sine_wave_pattern;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum RumiaState {
    #[default]
    Inactive,
    Spell1,
}

pub fn rumia_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(FirstLevelState::Rumia), rumia_setup)
        .add_systems(Update, rumia_orchestrator)
        .add_systems(OnEnter(RumiaState::Inactive), rumia_cleanup)
        .add_plugins(spell1_plugin)
        .init_state::<RumiaState>()
    ;
}

pub fn rumia_setup(
    mut commands: Commands,
) {
    let start = Vec2::new(SPAWN_CENTER, SPAWN_TOP);
    let destination = Vec2::new(SPAWN_CENTER + 150.0, FRAME_BORDER_TOP - 100.0);
    commands.spawn((
        Name::new("RumiaSpawner"),
        BossSpawner {
            name: "Rumia",
            enemy_type: Rumia,
            starting_position: start,
            movement_pattern: create_move_to_pattern(start, destination, Duration::from_millis(1500)),
        },
        SpawnTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        GameObject,
    ));
}

pub fn rumia_orchestrator(
    boss_query: Query<(&Boss, &MovementPatterns)>,
    rumia_state: Res<State<RumiaState>>,
    mut rumia_next_state: ResMut<NextState<RumiaState>>,
) {
    for (_boss, movement_pattern) in boss_query.iter() {
        if *rumia_state.get() == RumiaState::Inactive && is_finished(movement_pattern) {
            rumia_next_state.set(RumiaState::Spell1);
        }
    }
}

fn rumia_cleanup(
    mut state: ResMut<NextState<Spell1State>>,
) {
    state.set(Spell1State::Inactive);
}
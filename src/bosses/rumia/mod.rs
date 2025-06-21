pub mod spell1;
pub mod spell2;

use crate::bosses::boss::{Boss, BossSpawner};
use crate::bosses::boss_health_bar::{despawn_boss_health_bar, BossHealthBar};
use crate::bosses::rumia::spell1::{spell1_plugin, Spell1State};
use crate::bosses::rumia::spell2::spell2_plugin;
use crate::enemy::EnemyType::Rumia;
use crate::game::{GameObject, SpawnTimer, FRAME_BORDER_TOP};
use crate::level1::FirstLevelState;
use crate::movement_patterns::decelerate::create_move_to_pattern;
use crate::movement_patterns::{is_finished, MovementPatterns};
use crate::spawns::{SPAWN_CENTER, SPAWN_TOP};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum RumiaState {
    #[default]
    Inactive,
    Spell1,
    Spell2,
}

pub fn rumia_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(FirstLevelState::Rumia), rumia_setup)
        .add_systems(Update, rumia_orchestrator.run_if(in_state(FirstLevelState::Rumia)))
        .add_systems(OnEnter(RumiaState::Inactive), (rumia_cleanup, despawn_boss_health_bar))
        .add_plugins(spell1_plugin)
        .add_plugins(spell2_plugin)
        .init_state::<RumiaState>()
    ;
}

pub fn rumia_setup(
    mut commands: Commands,
) {
    let start = Vec2::new(SPAWN_CENTER, SPAWN_TOP);
    commands.spawn((
        Name::new("RumiaSpawner"),
        BossSpawner {
            name: "Rumia",
            enemy_type: Rumia,
            starting_position: start,
            ..default()
        },
        SpawnTimer(Timer::from_seconds(0.0, TimerMode::Once)),
        GameObject,
    ));
    commands.spawn((
        BossHealthBar {
            current: 100,
            maximum: 100,
        },
        GameObject
    ));
}

pub fn rumia_orchestrator(
    mut commands: Commands,
    boss_query: Query<(&Boss, &MovementPatterns)>,
    rumia_state: Res<State<RumiaState>>,
    mut rumia_next_state: ResMut<NextState<RumiaState>>,
    health_bar_query: Query<(&BossHealthBar, Entity)>,
) {
    for (_boss, movement_pattern) in boss_query.iter() {
        if *rumia_state.get() == RumiaState::Inactive && is_finished(movement_pattern) {
            rumia_next_state.set(RumiaState::Spell1);
        }
    }
    for (health_bar, entity) in health_bar_query.iter() {
        if health_bar.current <= 0 {
            match *rumia_state.get() {
                RumiaState::Spell1 => {
                    rumia_next_state.set(RumiaState::Spell2)
                },
                _ => {}
            }
            commands.entity(entity).try_despawn();
        }
    }
}

pub fn rumia_cleanup(
    mut state: ResMut<NextState<Spell1State>>,
) {
    state.set(Spell1State::Inactive);
}
pub mod spell1;
pub mod spell2;

use crate::bosses::boss::{Boss, BossSpawner};
use crate::bosses::boss_health_bar::{despawn_boss_health_bar, BossHealthBar};
use crate::bosses::rumia::spell1::{spell1_plugin, Spell1State};
use crate::bosses::rumia::spell2::{spell2_plugin, Spell2State};
use crate::enemy::EnemyType::Rumia;
use crate::game::{GameObject, SpawnTimer};
use crate::level1::FirstLevelState;
use crate::movement_patterns::{is_finished, MovementPatterns};
use crate::spawns::{SPAWN_CENTER, SPAWN_TOP};
use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum RumiaState {
    #[default]
    Inactive,
    Spell1,
    Spell2,
    Complete,
}

pub fn rumia_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(FirstLevelState::Rumia), rumia_setup)
        .add_systems(Update, rumia_orchestrator.run_if(in_state(FirstLevelState::Rumia)))
        .add_systems(OnEnter(RumiaState::Complete), transition_out_of_fight)
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
                    rumia_next_state.set(RumiaState::Spell2);
                    commands.entity(entity).try_despawn();
                },
                RumiaState::Spell2 => {
                    rumia_next_state.set(RumiaState::Complete);
                    commands.entity(entity).try_despawn();
                }
                _ => {}
            }
        }
    }
}

fn transition_out_of_fight(
    mut first_level_next_state: ResMut<NextState<FirstLevelState>>,
    mut rumia_next_state: ResMut<NextState<RumiaState>>,
) {
    first_level_next_state.set(FirstLevelState::PostRumia);
    rumia_next_state.set(RumiaState::Inactive);
}

pub fn rumia_cleanup(
    mut commands: Commands,
    boss_query: Query<Entity, With<Boss>>,
    health_bar_query: Query<Entity, With<BossHealthBar>>,
    mut spell1_state: ResMut<NextState<Spell1State>>,
    mut spell2_state: ResMut<NextState<Spell2State>>,
) {
    for boss_entity in boss_query.iter() {
        commands.entity(boss_entity).try_despawn();
    }
    for health_bar_entity in health_bar_query.iter() {
        commands.entity(health_bar_entity).try_despawn();
    }
    spell1_state.set(Spell1State::Inactive);
    spell2_state.set(Spell2State::Inactive);
}
use crate::bosses::rumia::{rumia_plugin, RumiaState};
use crate::bullet::BulletType::*;
use crate::bullet::ShotSchedule;
use crate::bullet_patterns::circle_spawn::CircleSpawn;
use crate::bullet_patterns::BulletPatternTarget::*;
use crate::bullet_patterns::{BoxedBulletPattern, BulletPatternAngle};
use crate::enemy::EnemyType::*;
use crate::enemy::{Enemy, EnemySpawner};
use crate::game::{GameObject, LevelState, SpawnTimer, SPAWN_CENTER, SPAWN_TOP};
use crate::movement_patterns::move_direction::{build_move_direction, MoveDirectionBuilder};
use crate::movement_patterns::sine_wave::MoveSineWave;
use crate::movement_patterns::{BoxedBulletMovementPattern, BoxedMovementPattern, MovementPatterns};
use crate::GameState;
use bevy::prelude::*;
use std::f32::consts::PI;
use crate::movement_patterns::MovementPatterns::StraightLine;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum FirstLevelState {
    #[default]
    Inactive,
    PreRumia,
    Rumia,
}

pub fn level1_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(LevelState::Level1), level1_setup)
        .add_systems(Update, listen_for_rumia_entrance
            .run_if(in_state(FirstLevelState::PreRumia)))
        .add_systems(OnEnter(FirstLevelState::Inactive), first_level_cleanup)
        .add_plugins(rumia_plugin)
        .init_state::<FirstLevelState>()
    ;
}

fn level1_setup(
    mut commands: Commands,
    mut next_state: ResMut<NextState<FirstLevelState>>,
) {
        let starting_position = Vec2::new(SPAWN_CENTER, SPAWN_TOP - 50.0);
        commands.spawn((
            Name::new("EnemySpawner"),
            EnemySpawner {
                name: "Blue Fairy",
                enemy_type: BlueFairy,
                starting_position,
                movement_pattern: StraightLine(Rot2::degrees(270.0), 20.0)
            },
            SpawnTimer(Timer::from_seconds(0.1, TimerMode::Once)),
            GameObject,
        ));
    next_state.set(FirstLevelState::PreRumia);
}

fn listen_for_rumia_entrance(
    spawns: Query<&EnemySpawner>,
    enemies: Query<&Enemy>,
    game_state: Res<State<GameState>>,
    first_level_state: Res<State<FirstLevelState>>,
    mut next_first_level_state: ResMut<NextState<FirstLevelState>>,
) {
    if *game_state.get() == GameState::PlayingGame
        && *first_level_state.get() == FirstLevelState::PreRumia
        && spawns.is_empty()
        && enemies.is_empty()
    {
        next_first_level_state.set(FirstLevelState::Rumia);
    }
}

fn first_level_cleanup(
    mut state: ResMut<NextState<RumiaState>>,
) {
    state.set(RumiaState::Inactive);
}
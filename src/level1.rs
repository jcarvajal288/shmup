use crate::bosses::rumia::{rumia_plugin, RumiaState};
use crate::bullet::BulletType::*;
use crate::bullet_patterns::shoot_at_player::shoot_at_player_pattern;
use crate::bullet_patterns::BulletPatternTarget::*;
use crate::bullet_patterns::ENDLESS;
use crate::enemy::EnemyType::*;
use crate::enemy::{Enemy, EnemySpawner};
use crate::game::{GameObject, LevelState, SpawnTimer, SPAWN_CENTER, SPAWN_TOP};
use crate::movement_patterns::MovementPatterns::SineWave;
use crate::GameState;
use bevy::prelude::*;

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
                //movement_pattern: create_decelerate_pattern(Rot2::degrees(270.0), 200.0, 20.0, Duration::from_secs(2)),
                // movement_pattern: StraightLine(Rot2::degrees(270.0), 20.0),
                movement_pattern: SineWave(150.0, 100.0, 25.0, starting_position),
                bullet_pattern: shoot_at_player_pattern(WhiteArrow, 200.0, 0.5, ENDLESS),
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
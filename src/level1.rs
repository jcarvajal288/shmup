use crate::bosses::rumia::{rumia_plugin, RumiaState};
use crate::bullet::BulletType::*;
use crate::bullet_patterns::shot_schedule::ShotSchedule;
use crate::bullet_patterns::starburst::Starburst;
use crate::bullet_patterns::BulletPatterns::StarburstPattern;
use crate::bullet_patterns::{Target, ENDLESS};
use crate::enemy::EnemyType::*;
use crate::enemy::{Enemy, EnemySpawner};
use crate::game::{GameObject, LevelState, FRAME_BORDER_LEFT, FRAME_BORDER_RIGHT};
use crate::movement_patterns::straight_line::create_straight_line_pattern;
use crate::spawns::{horizontal_line, SpawnTimeTracker, SPAWN_CENTER, SPAWN_OUTSIDE_LEFT, SPAWN_LEFTMOST, SPAWN_TOP};
use crate::GameState;
use bevy::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;
use crate::bullet_patterns::single_shot::single_shot_at_player;
use crate::movement_patterns::decelerate::create_decelerate_pattern;

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
    let mut spawn_delay = SpawnTimeTracker::default();

    for _ in 0..5 {
        commands.spawn((
            Name::new("EnemySpawner"),
            EnemySpawner {
                name: "Blue Fairy",
                enemy_type: BlueFairy,
                starting_position: Vec2::new(SPAWN_OUTSIDE_LEFT, SPAWN_TOP),
                movement_pattern: create_straight_line_pattern(Rot2::degrees(315.0), 100.0),
                bullet_pattern: single_shot_at_player(WhiteArrow, 200.0, 0.5, ENDLESS),
            },
            spawn_delay.create_timer_and_increment(0.4),
            GameObject,
        ));
    };

    spawn_delay.increment(2.0);

    let spawn_line = horizontal_line(FRAME_BORDER_LEFT, SPAWN_CENTER, SPAWN_TOP, 5);
    for starting_position in spawn_line {
        commands.spawn((
            Name::new("EnemySpawner"),
            EnemySpawner {
                name: "Blue Fairy",
                enemy_type: BlueFairy,
                starting_position,
                movement_pattern: create_decelerate_pattern(Rot2::degrees(270.0), 400.0, 20.0, Duration::from_secs(2)),
                bullet_pattern: single_shot_at_player(WhiteArrow, 200.0, 0.5, ENDLESS),
            },
            spawn_delay.create_timer_and_increment(0.2),
            GameObject,
        ));
    }

    spawn_delay.increment(2.0);
    let mut spawn_delay_2 = spawn_delay.clone();

    for _ in 0..3 {
        commands.spawn((
            Name::new("EnemySpawner"),
            EnemySpawner {
                name: "Blue Fairy",
                enemy_type: BlueFairy,
                starting_position: Vec2::new(FRAME_BORDER_RIGHT - 10.0, SPAWN_TOP),
                movement_pattern: create_straight_line_pattern(Rot2::degrees(270.0), 100.0),
                bullet_pattern: StarburstPattern(
                    Starburst {
                        bullets: vec![BlueRimmedCircle],
                        num_lines: 6,
                        speed_range: (200.0, 400.0),
                        spread: PI,
                        ..default()
                    },
                    Target::Angle(Rot2::degrees(-90.0)),
                    ShotSchedule {
                        delay: Timer::from_seconds(0.5, TimerMode::Once),
                        interval: Timer::from_seconds(1.0, TimerMode::Once),
                        repetitions: ENDLESS,
                    }
                ),
            },
            spawn_delay.create_timer_and_increment(0.4),
            GameObject,
        ));
    }

    for _ in 0..3 {
        commands.spawn((
            Name::new("EnemySpawner"),
            EnemySpawner {
                name: "Blue Fairy",
                enemy_type: BlueFairy,
                starting_position: Vec2::new(SPAWN_LEFTMOST, SPAWN_TOP),
                movement_pattern: create_straight_line_pattern(Rot2::degrees(270.0), 100.0),
                bullet_pattern: StarburstPattern(
                    Starburst {
                        bullets: vec![BlueRimmedCircle],
                        num_lines: 6,
                        speed_range: (200.0, 400.0),
                        spread: PI,
                        ..default()
                    },
                    Target::Angle(Rot2::degrees(90.0)),
                    ShotSchedule {
                        delay: Timer::from_seconds(0.5, TimerMode::Once),
                        interval: Timer::from_seconds(1.0, TimerMode::Once),
                        repetitions: ENDLESS,
                    }
                ),
            },
            spawn_delay_2.create_timer_and_increment(0.4),
            GameObject,
        ));
    }

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
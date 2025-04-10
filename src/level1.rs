use crate::bosses::rumia::rumia_plugin;
use crate::bullet::BulletType::*;
use crate::bullet::{ShotSchedule};
use crate::bullet_patterns::circle_spawn::CircleSpawn;
use crate::bullet_patterns::BulletPatternTarget::*;
use crate::bullet_patterns::{BoxedBulletPattern, BulletPatternAngle};
use crate::enemy::EnemyType::*;
use crate::enemy::{Enemy, EnemySpawner};
use crate::game::{GameObject, LevelState, SpawnTimer, SPAWN_LEFT, SPAWN_RIGHT};
use crate::movement_patterns::move_direction::{build_move_direction, MoveDirectionBuilder};
use crate::movement_patterns::move_straight::MoveStraight;
use crate::movement_patterns::{BoxedBulletMovementPattern, BoxedMovementPattern};
use bevy::prelude::*;
use std::f32::consts::PI;

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
        .add_plugins(rumia_plugin)
        .init_state::<FirstLevelState>()
    ;
}

pub fn reset_level1(
    mut state: ResMut<NextState<FirstLevelState>>,
) {
    state.set(FirstLevelState::Inactive);
    println!("FirstLevelState set to Inactive");
}

fn level1_setup(mut commands: Commands, mut next_state: ResMut<NextState<FirstLevelState>>) {

    println!("Level 1 setup");
    let bullet_stream = CircleSpawn {
        bullet_type: WhiteArrow,
        bullets_in_circle: 1,
        bullets_in_lines: 1,
        angle: BulletPatternAngle {
            target: Player,
            spread: PI,
            offset: 0.0,
        },
        spawn_circle_radius: 0.0,
    };

    for i in 0..1 {
        let initial_delay = 0.0;
        let iter_delay = 1.0;
        let full_delay = initial_delay + (iter_delay * i as f32);
        commands.spawn((
            Name::new("EnemySpawner"),
            EnemySpawner {
                name: "Blue Fairy",
                enemy_type: BlueFairy,
                starting_position: Vec2::new(SPAWN_LEFT, 150.0),
                movement_pattern: BoxedMovementPattern(Box::new(MoveStraight {
                    angle: 0.0,
                    speed: 40.0,
                    ..default()
                })),
                bullet_pattern: BoxedBulletPattern(Box::new(bullet_stream.clone())),
                bullet_movement_pattern: BoxedBulletMovementPattern(Box::new(build_move_direction(MoveDirectionBuilder {
                    direction: Rot2::degrees(-90.0),
                    starting_velocity: 300.0,
                    final_velocity: 300.0,
                    time_to_decelerate: Default::default(),
                }))),
                shot_schedule: ShotSchedule {
                    delay_timer: Timer::from_seconds(1.0, TimerMode::Once),
                    repeat_timer: Timer:: from_seconds(0.5, TimerMode::Once),
                    times: 5,
                }
            },
            SpawnTimer(Timer::from_seconds(full_delay, TimerMode::Once)),
            GameObject,
        ));
    }

    for i in 0..1 {
        let initial_delay = 5.0;
        let iter_delay = 1.0;
        let full_delay = initial_delay + (iter_delay * i as f32);
        commands.spawn((
            Name::new("EnemySpawner"),
            EnemySpawner {
                name: "Blue Fairy",
                enemy_type: BlueFairy,
                starting_position: Vec2::new(SPAWN_RIGHT, 150.0),
                movement_pattern: BoxedMovementPattern(Box::new(MoveStraight {
                    angle: -PI,
                    speed: 40.0,
                    ..default()
                })),
                bullet_pattern: BoxedBulletPattern(Box::new(bullet_stream.clone())),
                bullet_movement_pattern: BoxedBulletMovementPattern(Box::new(build_move_direction(MoveDirectionBuilder {
                    direction: Rot2::degrees(-90.0),
                    starting_velocity: 300.0,
                    final_velocity: 300.0,
                    time_to_decelerate: Default::default(),
                }))),
                shot_schedule: ShotSchedule {
                    delay_timer: Timer::from_seconds(1.0, TimerMode::Once),
                    repeat_timer: Timer:: from_seconds(0.5, TimerMode::Once),
                    times: 5,
                }
            },
            SpawnTimer(Timer::from_seconds(full_delay, TimerMode::Once)),
            GameObject,
        ));
    }
    next_state.set(FirstLevelState::PreRumia);
    println!("FirstLevelState set to PreRumia");
}

fn listen_for_rumia_entrance(
    spawns: Query<&EnemySpawner>,
    enemies: Query<&Enemy>,
    state: Res<State<FirstLevelState>>,
    mut next_state: ResMut<NextState<FirstLevelState>>,
) {
    if *state.get() == FirstLevelState::PreRumia && spawns.is_empty() && enemies.is_empty() {
        next_state.set(FirstLevelState::Rumia);
        println!("FirstLevelState set to Rumia");
    }
}
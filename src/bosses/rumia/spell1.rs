use crate::bosses::boss::Boss;
use crate::player::Player;
use crate::bosses::rumia::RumiaState;
use crate::bullet::{spawn_bullet, BulletSpawnEvent, BulletType};
use crate::bullet_patterns::circle_spawn::CircleSpawn;
use crate::bullet_patterns::BulletPatternTarget::*;
use crate::bullet_patterns::{BoxedBulletPattern, BulletPatternAngle, BulletPatternTarget};
use crate::game::{GameObject, LevelState, SpawnTimer, SPAWN_CENTER, SPAWN_TOP};
use crate::movement_patterns::move_away::{build_move_away, MoveAwayBuilder};
use crate::movement_patterns::move_distance_away::{build_move_distance_away, MoveDistanceAwayBuilder};
use crate::movement_patterns::move_to::{build_move_to, MoveToBuilder};
use crate::movement_patterns::{BoxedBulletMovementPattern, BoxedMovementPattern};
use crate::resources::sprites::{set_one_off_animation, AnimationIndices};
use bevy::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;
use crate::enemy::Enemy;
use crate::movement_patterns::MovementPatterns::StraightLine;
use crate::player;

#[derive(Component)]
struct SpellTimer(Timer);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum Spell1State {
    #[default]
    Inactive,
    Phase1,
    MoveToPhase2,
    Phase2,
}

pub fn spell1_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(LevelState::None), reset_spell1)
        .add_systems(OnEnter(RumiaState::Spell1), phase1_setup)
        .add_systems(Update, phase1_countdown
            .run_if(in_state(Spell1State::Phase1)))
        .add_systems(OnEnter(Spell1State::MoveToPhase2), move_to_phase2_setup)
        .add_systems(Update, wait_for_move_to_phase2
            .run_if(in_state(Spell1State::MoveToPhase2)))
        .add_systems(OnEnter(Spell1State::Phase2), phase2_setup)
        .add_systems(Update, phase2_update
            .run_if(in_state(Spell1State::Phase2)))
        .init_state::<Spell1State>()
    ;
}

pub fn reset_spell1(
    mut state: ResMut<NextState<Spell1State>>,
) {
    state.set(Spell1State::Inactive);
}

fn phase1_setup(
    mut commands: Commands,
    mut rumia_query: Query<(&Boss, &Transform, &mut AnimationIndices)>,
    player_transform_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut bullet_spawn_events: EventWriter<BulletSpawnEvent>,
    mut state: ResMut<NextState<Spell1State>>,
) {
    let player_transform: Transform = *player_transform_query.get_single().expect("Error: could not find player transform.");
    for (_boss, boss_transform, mut animation_indices) in rumia_query.iter_mut() {
        set_one_off_animation(&mut animation_indices, 0, 3);
        let num_lines = 16;
        let bullet_line = vec!(
            BulletType::BlueRimmedCircle,
            BulletType::BlueRimmedCircle,
            BulletType::BlueRimmedCircle,
            BulletType::BlueRimmedCircle,
            BulletType::BlueRimmedCircle,
        );
        let speeds = vec!(200.0, 180.0, 160.0, 140.0, 120.0);
        let target = Transform::from_translation(player_transform.translation - boss_transform.translation);
        fire_starburst(&mut bullet_spawn_events, boss_transform, num_lines, bullet_line, speeds, target);

    }
    commands.spawn((
        Name::new("Spell Timer 1"),
        SpellTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        GameObject,
    ));
    state.set(Spell1State::Phase1);
}

fn fire_starburst(bullet_spawn_events: &mut EventWriter<BulletSpawnEvent>, boss_transform: &Transform, num_lines: usize, bullet_line: Vec<BulletType>, speeds: Vec<f32>, target: Transform) {
    let firing_angle = target.translation.y.atan2(target.translation.x);
    let step_size = (2.0 * PI) / num_lines as f32;
    let angles = (0..num_lines).map(|i: usize| {
        firing_angle - (2.0 * PI / 2.0) + (i as f32 * step_size)
    }).collect::<Vec<f32>>();
    for (bullet_type, speed) in bullet_line.iter()
        .zip(speeds.iter())
        .map(|(bullet_type, speed)| (bullet_type, speed))
    {
        for angle in &angles {
            bullet_spawn_events.send(BulletSpawnEvent {
                bullet_type: *bullet_type,
                position: boss_transform.translation.truncate(),
                movement_pattern: StraightLine(Rot2::radians(*angle), *speed),
            });
        }
    }
}

fn phase1_countdown(
    time: Res<Time>,
    mut timer_query: Query<&mut SpellTimer>,
    mut next_state: ResMut<NextState<Spell1State>>,
) {
    for mut timer in timer_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            next_state.set(Spell1State::MoveToPhase2);
        };
    }
}

fn move_to_phase2_setup(
    mut rumia_query: Query<(&Boss, &Transform, &mut BoxedMovementPattern)>,
) {
    for (_boss, transform, mut boxed_movement_pattern) in rumia_query.iter_mut() {
        let start = transform.translation.xy();
        let destination = Vec2::new(SPAWN_CENTER, SPAWN_TOP - 100.0);
        let time = 1.5;
        boxed_movement_pattern.0 = Box::new(build_move_to(MoveToBuilder {
            start,
            destination,
            time,
        }))
    }
}

fn wait_for_move_to_phase2(
    rumia_query: Query<(&Boss, &BoxedMovementPattern)>,
    mut next_state: ResMut<NextState<Spell1State>>,
) {
    for (_boss, boxed_movement_pattern) in rumia_query.iter() {
        if boxed_movement_pattern.0.is_finished() {
            next_state.set(Spell1State::Phase2);
        }
    }
}

fn phase2_setup(
    mut commands: Commands,
    mut rumia_query: Query<(&Boss, &Transform, &mut AnimationIndices)>,
) {
    for (_boss, transform, mut animation_indices) in rumia_query.iter_mut() {
        set_one_off_animation(&mut animation_indices, 0, 3);
        let waves = [
            (BulletType::SmallRedCircle, 0.0),
            (BulletType::SmallYellowCircle, 1.0),
            (BulletType::SmallGreenCircle, 2.0),
            (BulletType::SmallPurpleCircle, 3.0),
            (BulletType::SmallBlueCircle, 4.0)
        ];
        for (bullet_type, index) in waves.iter() {
            commands.spawn((
                Name::new("spell2"),
                BoxedBulletPattern(Box::new(CircleSpawn {
                    bullet_type: *bullet_type,
                    bullets_in_circle: 64,
                    angle: BulletPatternAngle {
                        target: Down,
                        spread: 2.0 * PI,
                        offset: 0.0 + PI / 3.0 * index,
                    },
                    spawn_circle_radius: 30.0,
                })),
                BoxedBulletMovementPattern(Box::new(build_move_distance_away(MoveDistanceAwayBuilder {
                    name: "phase2_part1",
                    repulsion_point: transform.translation,
                    duration: Duration::from_millis(500),
                    distance: 75.0,
                }))),
                *transform,
                SpawnTimer(Timer::from_seconds(0.2 * index, TimerMode::Once)),
                GameObject,
            ));
        }
    }
}

fn phase2_update(
    mut rumia_query: Query<(&Boss, &Transform)>,
    mut query: Query<&mut BoxedBulletMovementPattern>,
) {
    for mut movement_pattern in query.iter_mut()
        .filter(|m| m.0.name() == "phase2_part1") {

        for (_boss, transform) in rumia_query.iter_mut() {
            if movement_pattern.0.is_finished() {
                let new_movement_pattern = Box::new(build_move_away(MoveAwayBuilder {
                    repulsion_point: transform.translation,
                    starting_velocity: 0.0,
                    final_velocity: 350.0,
                    time_to_final_velocity: Duration::from_secs(1),
                }));
                let _old_movement_pattern = std::mem::replace(&mut movement_pattern.0, new_movement_pattern);
            }
        }
    }
}
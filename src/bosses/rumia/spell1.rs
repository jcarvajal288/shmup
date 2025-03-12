use crate::bosses::boss::Boss;
use crate::bosses::rumia::RumiaState;
use crate::bullet::BulletType;
use crate::bullet_patterns::circle_spawn::CircleSpawn;
use crate::bullet_patterns::BulletPatternTarget::*;
use crate::bullet_patterns::{BoxedBulletPattern, BulletPatternAngle, BulletPatternTarget};
use crate::enemy::Enemy;
use crate::game::{SpawnTimer, SPAWN_CENTER, SPAWN_TOP};
use crate::movement_patterns::move_away::{build_move_away, MoveAwayBuilder};
use crate::movement_patterns::move_to::{build_move_to, MoveToBuilder};
use crate::movement_patterns::BoxedMovementPattern;
use crate::player::Player;
use crate::resources::sprites::{set_one_off_animation, AnimationIndices, Sprites};
use bevy::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;

#[derive(Component)]
struct SpellTimer(Timer);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum Spell1State {
    #[default]
    Phase1,
    MoveToPhase2,
    Phase2,
}

pub fn spell1_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(RumiaState::Spell1), phase1_setup)
        .add_systems(Update, phase1_countdown
            .run_if(in_state(Spell1State::Phase1)))
        .add_systems(OnEnter(Spell1State::MoveToPhase2), move_to_phase2_setup)
        .add_systems(Update, wait_for_move_to_phase2
            .run_if(in_state(Spell1State::MoveToPhase2)))
        .add_systems(OnEnter(Spell1State::Phase2), phase2_setup)
        .add_systems(Update, (update_spellcard)
            .run_if(in_state(RumiaState::Spell1)))
        .init_state::<Spell1State>()
    ;
}

fn phase1_setup(
    mut commands: Commands,
    mut rumia_query: Query<(&Boss, &Transform, &mut AnimationIndices)>,
) {
    for (_boss, transform, mut animation_indices) in rumia_query.iter_mut() {
        set_one_off_animation(&mut *animation_indices, 0, 3);
        for i in 0..6 {
            commands.spawn((
                Name::new("spell1"),
                BoxedBulletPattern(Box::new(CircleSpawn {
                    bullet_type: BulletType::BlueRimmedCircle,
                    bullets_in_circle: 16,
                    bullets_in_lines: 1,
                    angle: BulletPatternAngle {
                        target: BulletPatternTarget::Player,
                        spread: 2.0 * PI,
                        offset: 0.0,
                    },
                    spawn_circle_radius: 10.0,
                })),
                BoxedMovementPattern(Box::new(build_move_away(MoveAwayBuilder {
                    repulsion_point: transform.translation,
                    starting_velocity: 100.0,
                    final_velocity: 300.0,
                    time_to_final_velocity: Duration::from_secs(1),
                }))),
                transform.clone(),
                SpawnTimer(Timer::from_seconds(0.0 + 0.1 * i as f32, TimerMode::Once)),
            ));
        }
    }
    commands.spawn((
        Name::new("Spell Timer 1"),
        SpellTimer(Timer::from_seconds(2.0, TimerMode::Once)),
    ));
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
    mut rumia_query: Query<(&Boss, &BoxedMovementPattern)>,
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
    mut rumia_query: Query<(&Boss, &Transform)>,
) {
    for (_boss, transform) in rumia_query.iter_mut() {
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
                    bullet_type: bullet_type.clone(),
                    bullets_in_circle: 64,
                    bullets_in_lines: 1,
                    angle: BulletPatternAngle {
                        target: Down,
                        spread: 2.0 * PI,
                        offset: 0.0 + (2.0 * PI / 64.0 * index),
                    },
                    spawn_circle_radius: 50.0,
                })),
                BoxedMovementPattern(Box::new(build_move_away(MoveAwayBuilder {
                    repulsion_point: transform.translation,
                    starting_velocity: 200.0,
                    final_velocity: 20.0,
                    time_to_final_velocity: Duration::from_secs(5),
                }))),
                transform.clone(),
                SpawnTimer(Timer::from_seconds(0.2 * index, TimerMode::Once)),
            ));
        }
    }
}

fn update_spellcard(
    time: Res<Time>,
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut bullet_pattern_query: Query<(&mut BoxedBulletPattern, &mut BoxedMovementPattern, &Transform, &mut SpawnTimer)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    for (mut bullet_pattern, mut movement_pattern, transform, mut timer) in bullet_pattern_query.iter_mut() {
        if timer.0.tick(time.delta()).finished() {
            for player_transform in player_query.iter() {
                bullet_pattern.0.fire(&mut commands, &sprites, *transform, &time, player_transform, &mut movement_pattern);
            }
        }
    }
}
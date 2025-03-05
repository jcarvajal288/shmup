use std::f32::consts::PI;
use bevy::prelude::*;
use crate::bosses::boss::Boss;
use crate::bosses::rumia::RumiaState;
use crate::bullet::BulletType;
use crate::bullet_patterns::{BoxedBulletPattern, BulletPatternAngle};
use crate::bullet_patterns::bullet_stream::BulletStream;
use crate::bullet_patterns::BulletPatternTarget::*;
use crate::bullet_patterns::circle_spawn::CircleSpawn;
use crate::enemy::Enemy;
use crate::game::{SPAWN_CENTER, SPAWN_LEFT, SPAWN_TOP};
use crate::movement_patterns::BoxedMovementPattern;
use crate::movement_patterns::move_to::{build_move_to, MoveToBuilder};
use crate::resources::sprites::{set_one_off_animation, AnimationIndices, Sprites};

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
        commands.spawn((
            Name::new("spell1"),
            BoxedBulletPattern(Box::new(BulletStream {
                bullet_type: BulletType::BlueRimmedCircle,
                bullets_per_wave: 16,
                waves_per_iteration: 7,
                num_iterations: 1,
                angle: BulletPatternAngle {
                    target: Player,
                    spread: PI * 2.0,
                    offset: 0.0,
                },
                speed: 200.0,
                acceleration: -1.0,
                startup_timer: Default::default(),
                wave_timer: Timer::from_seconds(0.05, TimerMode::Once),
                iteration_timer: Default::default(),
                waves_left: 0,
                iterations_left: 0,
            })),
            transform.clone(),
        ));
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
                Name::new("spell1"),
                BoxedBulletPattern(Box::new(CircleSpawn {
                    bullet_type: bullet_type.clone(),
                    bullets_in_circle: 64,
                    bullets_in_lines: 1,
                    angle: BulletPatternAngle {
                        target: Down,
                        spread: 2.0 * PI,
                        offset: 0.0,
                    },
                    spawn_circle_radius: 100.0,
                    starting_speed: 0.0,
                    final_speed: 0.0,
                    time_to_final_speed: 0.0,
                })),
                transform.clone(),
            ));
        }
    }
}

fn update_spellcard(
    time: Res<Time>,
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut bullet_pattern_query: Query<(&mut BoxedBulletPattern, &Transform)>,
    player_query: Query<&Transform, (With<crate::player::Player>, Without<Enemy>)>,
) {
    for (mut bullet_pattern, transform) in bullet_pattern_query.iter_mut() {
        for player_transform in player_query.iter() {
            bullet_pattern.0.fire(&mut commands, &sprites, *transform, &time, player_transform);
        }
    }
}
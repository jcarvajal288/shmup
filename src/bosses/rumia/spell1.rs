use crate::bosses::boss::Boss;
use crate::bosses::rumia::RumiaState;
use crate::bullet::BulletType;
use crate::bullet::BulletType::BlueRimmedCircle;
use crate::bullet_patterns::shot_schedule::ShotSchedule;
use crate::bullet_patterns::starburst::Starburst;
use crate::bullet_patterns::BulletPatterns::StarburstPattern;
use crate::enemy::Enemy;
use crate::game::{GameObject, LevelState, SPAWN_CENTER, SPAWN_TOP};
use crate::movement_patterns::decelerate::create_move_to_pattern;
use crate::movement_patterns::{is_finished, MovementPatterns};
use crate::player::Player;
use crate::resources::sprites::{set_one_off_animation, AnimationIndices};
use bevy::prelude::*;
use std::time::Duration;

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
    mut state: ResMut<NextState<Spell1State>>,
) {
    let player_transform: Transform = *player_transform_query.get_single().expect("Error: could not find player transform.");
    for (_boss, boss_transform, mut animation_indices) in rumia_query.iter_mut() {
        set_one_off_animation(&mut animation_indices, 0, 3);
        commands.spawn((
            StarburstPattern(
                Starburst {
                    bullets: vec![BlueRimmedCircle; 5],
                    num_lines: 16,
                    speed_range: (120.0, 200.0),
                    target: player_transform.translation.truncate(),
                    ..default()
                },
                ShotSchedule::default()
            ),
            Transform::from_translation(boss_transform.translation),
        ));
    }
    commands.spawn((
        Name::new("Spell Timer 1"),
        SpellTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        GameObject,
    ));
    state.set(Spell1State::Phase1);
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
    mut rumia_query: Query<(&Boss, &Transform, &mut MovementPatterns)>,
) {
    for (_boss, transform, mut movement_pattern) in rumia_query.iter_mut() {
        let start = transform.translation.xy();
        let destination = Vec2::new(SPAWN_CENTER, SPAWN_TOP - 100.0);
        *movement_pattern = create_move_to_pattern(start, destination, Duration::from_millis(1500));
    }
}

fn wait_for_move_to_phase2(
    rumia_query: Query<(&Boss, &MovementPatterns)>,
    mut next_state: ResMut<NextState<Spell1State>>,
) {
    for (_boss, movement_pattern) in rumia_query.iter() {
        if is_finished(movement_pattern) {
            next_state.set(Spell1State::Phase2);
        }
    }
}

fn phase2_setup(
    mut commands: Commands,
    mut rumia_query: Query<(&Boss, &Transform, &mut AnimationIndices)>,
) {
    for (_boss, boss_transform, mut animation_indices) in rumia_query.iter_mut() {
        set_one_off_animation(&mut animation_indices, 0, 3);
        let waves = [
            (BulletType::SmallRedCircle, 0.0),
            (BulletType::SmallYellowCircle, 1.0),
            (BulletType::SmallGreenCircle, 2.0),
            (BulletType::SmallPurpleCircle, 3.0),
            (BulletType::SmallBlueCircle, 4.0)
        ];
        for wave in waves {
            commands.spawn((
                StarburstPattern(
                    Starburst {
                        bullets: vec![wave.0],
                        num_lines: 64,
                        speed_range: (120.0, 200.0),
                        offset: wave.1,
                        target: boss_transform.translation.with_y(boss_transform.translation.y - 1.0).truncate(),
                    },
                    ShotSchedule {
                        delay: Timer::from_seconds(0.2 * wave.1, TimerMode::Once),
                        ..default()
                    }
                ),
                Transform::from_translation(boss_transform.translation),
            ));
        }
    }
}

fn phase2_update(
    mut rumia_query: Query<(&Boss, &Transform)>,
    mut query: Query<&mut MovementPatterns>,
) {
    // for mut movement_pattern in query.iter_mut()
    //     .filter(|m| m.name() == "phase2_part1") {
    //
    //     for (_boss, transform) in rumia_query.iter_mut() {
    //         if movement_pattern.is_finished() {
    //             let new_movement_pattern = Box::new(build_move_away(MoveAwayBuilder {
    //                 repulsion_point: transform.translation,
    //                 starting_velocity: 0.0,
    //                 final_velocity: 350.0,
    //                 time_to_final_velocity: Duration::from_secs(1),
    //             }));
    //             let _old_movement_pattern = std::mem::replace(&mut movement_pattern.0, new_movement_pattern);
    //         }
    //     }
    // }
}
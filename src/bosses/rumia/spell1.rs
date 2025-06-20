use crate::bosses::boss::{check_boss_being_shot, Boss};
use crate::bosses::rumia::RumiaState;
use crate::bullet::BulletType;
use crate::bullet::BulletType::{BlueRimmedCircle, RedRimmedCircle};
use crate::bullet_patterns::shot_schedule::ShotSchedule;
use crate::bullet_patterns::starburst::Starburst;
use crate::bullet_patterns::BulletPatterns::StarburstPattern;
use crate::enemy::Enemy;
use crate::game::{angle_to_transform, GameObject, LevelState, FRAME_BORDER_TOP};
use crate::movement_patterns::decelerate::create_move_to_pattern;
use crate::movement_patterns::{is_finished, MovementPatterns};
use crate::player::Player;
use crate::resources::sprites::{set_one_off_animation, AnimationIndices};
use crate::spawns::{SPAWN_CENTER, SPAWN_TOP};
use bevy::prelude::*;
use std::time::Duration;
use crate::bosses::boss_health_bar::{despawn_boss_health_bar, listen_for_boss_damage, scale_boss_health_bar, spawn_boss_health_bar};

#[derive(Component)]
struct SpellTimer(Timer);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum Spell1State {
    #[default]
    Inactive,
    Phase1,
    MoveToPhase2,
    Phase2,
    MoveToPhase3,
    Phase3,
    MoveToPhase1,
}

pub fn spell1_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(RumiaState::Spell1), (enter_spell1, spawn_boss_health_bar))
        .add_systems(Update, (check_boss_being_shot, listen_for_boss_damage, scale_boss_health_bar))
        .add_systems(OnEnter(Spell1State::Phase1), phase1_setup)
        .add_systems(Update, phase1_countdown
            .run_if(in_state(Spell1State::Phase1)))

        .add_systems(OnEnter(Spell1State::MoveToPhase2), move_to_phase2_setup)
        .add_systems(Update, wait_for_move_to_phase2
            .run_if(in_state(Spell1State::MoveToPhase2)))

        .add_systems(OnEnter(Spell1State::Phase2), phase2_setup)
        .add_systems(Update, phase2_countdown
            .run_if(in_state(Spell1State::Phase2)))

        .add_systems(OnEnter(Spell1State::MoveToPhase3), move_to_phase3_setup)
        .add_systems(Update, wait_for_move_to_phase3
            .run_if(in_state(Spell1State::MoveToPhase3)))

        .add_systems(OnEnter(Spell1State::Phase3), phase3_setup)
        .add_systems(Update, phase3_countdown
            .run_if(in_state(Spell1State::Phase3)))

        .add_systems(OnEnter(Spell1State::MoveToPhase1), move_to_phase1_setup)
        .add_systems(Update, wait_for_move_to_phase1
            .run_if(in_state(Spell1State::MoveToPhase1)))
        .add_systems(OnEnter(Spell1State::Inactive), despawn_boss_health_bar)
        .add_systems(OnExit(LevelState::Level1), reset_spell1)
        .add_systems(OnExit(RumiaState::Spell1), reset_spell1)
        .init_state::<Spell1State>()
    ;
}

pub fn reset_spell1(
    mut state: ResMut<NextState<Spell1State>>,
) {
    state.set(Spell1State::Inactive);
}

fn enter_spell1(
    mut state: ResMut<NextState<Spell1State>>,
) {
    state.set(Spell1State::Phase1);
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
        let angle = angle_to_transform(*boss_transform, player_transform);
        commands.spawn((
            StarburstPattern(
                Starburst {
                    bullets: vec![BlueRimmedCircle; 5],
                    num_lines: 16,
                    speed_range: (120.0, 200.0),
                    angle,
                    ..default()
                },
                ShotSchedule::default()
            ),
            Transform::from_translation(boss_transform.translation),
        ));
    }
    commands.spawn((
        Name::new("Rumia Phase 1 Timer"),
        SpellTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        GameObject,
    ));
    state.set(Spell1State::Phase1);
}

fn phase1_countdown(
    mut commands: Commands,
    mut timer_query: Query<(&mut SpellTimer, &Name, Entity)>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<Spell1State>>,
) {
    for (mut timer, name, entity) in timer_query.iter_mut() {
        if name.as_str() == "Rumia Phase 1 Timer" && timer.0.tick(time.delta()).just_finished() {
            next_state.set(Spell1State::MoveToPhase2);
            commands.entity(entity).despawn();
        }
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
                        ..default()
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

    commands.spawn((
        Name::new("Rumia Phase 2 Timer"),
        SpellTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        GameObject,
    ));
}

fn phase2_countdown(
    mut commands: Commands,
    mut timer_query: Query<(&mut SpellTimer, &Name, Entity)>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<Spell1State>>,
) {
    for (mut timer, name, entity) in timer_query.iter_mut() {
        if name.as_str() == "Rumia Phase 2 Timer" && timer.0.tick(time.delta()).just_finished() {
            next_state.set(Spell1State::MoveToPhase3);
            commands.entity(entity).despawn();
        }
    }
}

fn move_to_phase3_setup(
    mut rumia_query: Query<(&Boss, &Transform, &mut MovementPatterns)>,
) {
    for (_boss, transform, mut movement_pattern) in rumia_query.iter_mut() {
        let start = transform.translation.xy();
        let destination = Vec2::new(SPAWN_CENTER - 150.0, FRAME_BORDER_TOP - 100.0);
        *movement_pattern = create_move_to_pattern(start, destination, Duration::from_millis(1500));
    }
}

fn wait_for_move_to_phase3(
    rumia_query: Query<(&Boss, &MovementPatterns)>,
    mut next_state: ResMut<NextState<Spell1State>>,
) {
    for (_boss, movement_pattern) in rumia_query.iter() {
        if is_finished(movement_pattern) {
            next_state.set(Spell1State::Phase3);
        }
    }
}

fn phase3_setup(
    mut commands: Commands,
    mut rumia_query: Query<(&Boss, &Transform, &mut AnimationIndices)>,
    player_transform_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform: Transform = *player_transform_query.get_single().expect("Error: could not find player transform.");
    for (_boss, boss_transform, mut animation_indices) in rumia_query.iter_mut() {
        let angle = angle_to_transform(*boss_transform, player_transform);
        set_one_off_animation(&mut animation_indices, 0, 3);
        commands.spawn((
            StarburstPattern(
                Starburst {
                    bullets: vec![RedRimmedCircle; 5],
                    num_lines: 16,
                    speed_range: (120.0, 200.0),
                    angle,
                    ..default()
                },
                ShotSchedule::default()
            ),
            Transform::from_translation(boss_transform.translation),
        ));
    }
    commands.spawn((
        Name::new("Rumia Phase 3 Timer"),
        SpellTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        GameObject,
    ));
}

fn phase3_countdown(
    mut commands: Commands,
    mut timer_query: Query<(&mut SpellTimer, &Name, Entity)>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<Spell1State>>,
) {
    for (mut timer, name, entity) in timer_query.iter_mut() {
        if name.as_str() == "Rumia Phase 3 Timer" && timer.0.tick(time.delta()).just_finished() {
            next_state.set(Spell1State::MoveToPhase1);
            commands.entity(entity).despawn();
        }
    }
}

fn move_to_phase1_setup(
    mut rumia_query: Query<(&Boss, &Transform, &mut MovementPatterns)>,
) {
    for (_boss, transform, mut movement_pattern) in rumia_query.iter_mut() {
        let start = transform.translation.xy();
        let destination = Vec2::new(SPAWN_CENTER + 150.0, FRAME_BORDER_TOP - 100.0);
        *movement_pattern = create_move_to_pattern(start, destination, Duration::from_millis(1500));
    }
}

fn wait_for_move_to_phase1(
    rumia_query: Query<(&Boss, &MovementPatterns)>,
    mut next_state: ResMut<NextState<Spell1State>>,
) {
    for (_boss, movement_pattern) in rumia_query.iter() {
        if is_finished(movement_pattern) {
            next_state.set(Spell1State::Phase1);
        }
    }
}

use crate::bosses::boss::{check_boss_being_shot, Boss};
use crate::bosses::boss_health_bar::{listen_for_boss_damage, scale_boss_health_bar, spawn_boss_health_bar, BossHealthBar};
use crate::bosses::rumia::RumiaState;
use crate::bullet::BulletType::{BlueRimmedCircle, RedRimmedCircle};
use crate::bullet_patterns::shot_schedule::create_shot_schedule;
use crate::bullet_patterns::shotgun::Shotgun;
use crate::bullet_patterns::starburst::Starburst;
use crate::bullet_patterns::BulletPattern::{ShotgunPattern, StarburstPattern};
use crate::bullet_patterns::{Target, ENDLESS};
use crate::game::{GameObject, LevelState};
use crate::movement_patterns::decelerate::create_move_to_pattern;
use crate::movement_patterns::{is_finished, MovementPatterns};
use crate::resources::sprites::{set_one_off_animation, AnimationIndices};
use crate::spawns::{SPAWN_CENTER, SPAWN_TOP};
use bevy::app::App;
use bevy::math::{Vec2, Vec3Swizzles};
use bevy::prelude::{default, in_state, AppExtStates, Commands, Component, Entity, IntoSystemConfigs, NextState, OnEnter, OnExit, Query, ResMut, States, Transform, Update, With};
use std::f32::consts::PI;
use std::time::Duration;
use bevy::core::Name;
use crate::bullet::Bullet;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum Spell2State {
    #[default]
    Inactive,
    MoveToPhase1,
    Phase1,
}

#[derive(Component)]
struct RumiaSpell2Object;

pub fn spell2_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(RumiaState::Spell2), enter_spell2)
        .add_systems(Update, (check_boss_being_shot, listen_for_boss_damage, scale_boss_health_bar)
            .run_if(in_state(RumiaState::Spell2)))
        .add_systems(Update, wait_for_move_to_phase1
            .run_if(in_state(Spell2State::MoveToPhase1)))

        .add_systems(OnEnter(Spell2State::Phase1), (phase1_setup, spawn_boss_health_bar))

        .add_systems(OnExit(LevelState::Level1), reset_spell2)
        .add_systems(OnExit(RumiaState::Spell2), reset_spell2)
        .init_state::<Spell2State>()
    ;
}

pub fn reset_spell2(
    mut commands: Commands,
    mut next_state: ResMut<NextState<Spell2State>>,
    mut spell_query: Query<Entity, With<RumiaSpell2Object>>,
    mut bullet_query: Query<Entity, With<Bullet>>
) {
    next_state.set(Spell2State::Inactive);
    for entity in spell_query.iter() {
        commands.entity(entity).try_despawn();
    }
    for entity in bullet_query.iter() {
        commands.entity(entity).try_despawn();
    }
}

fn enter_spell2(
    mut rumia_query: Query<(&Boss, &Transform, &mut MovementPatterns)>,
    mut next_state: ResMut<NextState<Spell2State>>,
) {
    for (_boss, transform, mut movement_pattern) in rumia_query.iter_mut() {
        let start = transform.translation.xy();
        let destination = Vec2::new(SPAWN_CENTER, SPAWN_TOP - 150.0);
        *movement_pattern = create_move_to_pattern(start, destination, Duration::from_millis(1500));
    }
    next_state.set(Spell2State::MoveToPhase1);
}
fn wait_for_move_to_phase1(
    rumia_query: Query<(&Boss, &MovementPatterns)>,
    mut next_state: ResMut<NextState<Spell2State>>,
) {
    for (_boss, movement_pattern) in rumia_query.iter() {
        if is_finished(movement_pattern) {
            next_state.set(Spell2State::Phase1);
        }
    }
}

fn phase1_setup(
    mut commands: Commands,
    mut rumia_query: Query<(&Boss, &Transform, &mut AnimationIndices)>,
) {
    for (_boss, boss_transform, mut animation_indices) in rumia_query.iter_mut() {
        set_one_off_animation(&mut animation_indices, 0, 3);
        commands.spawn((
            Name::new("Phase 2 Shotgun Pattern"),
            ShotgunPattern(
                Shotgun {
                    bullets: vec![RedRimmedCircle; 25],
                    spread: PI / 8.0,
                    speed_range: (200.0, 300.0),
                },
                Target::Player,
                create_shot_schedule(1.5, 1.5, ENDLESS),
            ),
            Transform::from_translation(boss_transform.translation),
            GameObject,
            RumiaSpell2Object,
        ));
        commands.spawn((
            Name::new("Phase 2 Starburst Pattern"),
            StarburstPattern(
                Starburst {
                    bullets: vec![BlueRimmedCircle; 5],
                    num_lines: 48,
                    speed_range: (50.0, 250.0),
                    ..default()
                },
                Target::Down,
                create_shot_schedule(0.0, 5.0, ENDLESS),
            ),
            Transform::from_translation(boss_transform.translation),
            GameObject,
            RumiaSpell2Object,
        ));
        commands.spawn((
            Name::new("Rumia Spell 2 Health Bar"),
            BossHealthBar {
                current: 100,
                maximum: 100,
            },
            GameObject,
            RumiaSpell2Object,
        ));
    }
}

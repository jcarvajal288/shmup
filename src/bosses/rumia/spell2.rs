use crate::bosses::boss::Boss;
use crate::bosses::boss_health_bar::spawn_boss_health_bar;
use crate::bosses::rumia::spell1::Spell1State;
use crate::bosses::rumia::RumiaState;
use crate::enemy::Enemy;
use crate::game::{angle_to_transform, LevelState};
use crate::movement_patterns::decelerate::create_move_to_pattern;
use crate::movement_patterns::{is_finished, MovementPatterns};
use crate::player::Player;
use crate::resources::sprites::{set_one_off_animation, AnimationIndices};
use crate::spawns::{SPAWN_CENTER, SPAWN_TOP};
use bevy::app::App;
use bevy::math::{Vec2, Vec3Swizzles};
use bevy::prelude::{default, in_state, AppExtStates, Commands, IntoSystemConfigs, NextState, OnEnter, OnExit, Query, ResMut, States, Transform, Update, With, Without};
use std::time::Duration;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum Spell2State {
    #[default]
    Inactive,
    MoveToPhase1,
    Phase1,
}

pub fn spell2_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(RumiaState::Spell2), enter_spell2)
        .add_systems(Update, wait_for_move_to_phase1
            .run_if(in_state(Spell2State::MoveToPhase1)))

        .add_systems(OnEnter(Spell2State::Phase1), (phase1_setup, spawn_boss_health_bar))

        .add_systems(OnExit(RumiaState::Spell2), reset_spell2)
        .add_systems(OnExit(LevelState::Level1), reset_spell2)
        .init_state::<Spell2State>()
    ;
}

pub fn reset_spell2(
    mut next_state: ResMut<NextState<Spell2State>>,
) {
    next_state.set(Spell2State::Inactive);
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
    player_transform_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut state: ResMut<NextState<Spell1State>>,
) {
    let player_transform: Transform = *player_transform_query.get_single().expect("Error: could not find player transform.");
    for (_boss, boss_transform, mut animation_indices) in rumia_query.iter_mut() {
        set_one_off_animation(&mut animation_indices, 0, 3);
        let angle = angle_to_transform(*boss_transform, player_transform);
        commands.spawn((
            Transform::from_translation(boss_transform.translation),
        ));
    }
}

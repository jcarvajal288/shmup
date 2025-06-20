use crate::bosses::boss::Boss;
use crate::bosses::boss_health_bar::spawn_boss_health_bar;
use crate::bosses::rumia::RumiaState;
use crate::movement_patterns::decelerate::create_move_to_pattern;
use crate::movement_patterns::{is_finished, MovementPatterns};
use crate::spawns::{SPAWN_CENTER, SPAWN_TOP};
use bevy::app::App;
use bevy::math::{Vec2, Vec3Swizzles};
use bevy::prelude::{in_state, AppExtStates, IntoSystemConfigs, NextState, OnEnter, OnExit, Query, ResMut, States, Transform, Update};
use std::time::Duration;
use crate::game::LevelState;

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

        .add_systems(OnEnter(Spell2State::Phase1), spawn_boss_health_bar)
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

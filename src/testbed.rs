use bevy::prelude::*;
use crate::bosses::boss::Boss;
use crate::bosses::boss_health_bar::spawn_boss_health_bar;
use crate::bosses::rumia::{rumia_cleanup, rumia_setup, RumiaState};
use crate::bosses::rumia::spell2::spell2_plugin;
use crate::game::LevelState;
use crate::movement_patterns::{is_finished, MovementPatterns};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum TestBedState {
    #[default]
    Setup,
    Execute,
}

pub fn testbed_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(LevelState::TestBed), (
            rumia_setup,
            spawn_boss_health_bar,
            testbed_setup,
        ).chain())
        .add_systems(OnEnter(TestBedState::Execute), testbed_execute)
        .add_systems(OnExit(LevelState::TestBed), (testbed_cleanup, rumia_cleanup))
        .init_state::<TestBedState>()
    ;
}

fn testbed_setup(
    mut state: ResMut<NextState<TestBedState>>,
) {
    state.set(TestBedState::Execute);
}

fn testbed_execute(
    mut next_state: ResMut<NextState<RumiaState>>,
) {
    next_state.set(RumiaState::Spell2);
}

fn testbed_cleanup(
    mut state: ResMut<NextState<RumiaState>>,
) {
    state.set(RumiaState::Inactive);
}

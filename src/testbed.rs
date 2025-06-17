use bevy::prelude::*;
use crate::bosses::boss_health_bar::spawn_boss_health_bar;
use crate::bosses::rumia::{rumia_setup, RumiaState};
use crate::game::LevelState;

pub fn testbed_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(LevelState::TestBed), (rumia_setup, spawn_boss_health_bar))
        .add_systems(OnExit(LevelState::TestBed), testbed_cleanup)
    ;
}

fn testbed_cleanup(
    mut state: ResMut<NextState<RumiaState>>,
) {
    state.set(RumiaState::Inactive);
}

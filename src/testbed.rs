use bevy::prelude::*;
use crate::bosses::rumia::{rumia_setup, RumiaState};
use crate::game::LevelState;

pub fn testbed_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(LevelState::TestBed), rumia_setup)
        .add_systems(OnExit(LevelState::TestBed), testbed_cleanup)
    ;
}

fn testbed_cleanup(
    mut state: ResMut<NextState<RumiaState>>,
) {
    state.set(RumiaState::Inactive);
    println!("RumiaState set to Inactive");
}

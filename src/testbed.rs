use bevy::prelude::*;
use crate::bosses::rumia::rumia_setup;
use crate::game::LevelState;

pub fn testbed_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(LevelState::TestBed), rumia_setup)
    ;
}
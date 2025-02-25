use bevy::prelude::*;
use crate::bosses::rumia::RumiaState;

pub fn spell1_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(RumiaState::Spell1), spell1_setup)
    ;
}

fn spell1_setup() {
    println!("Spell1 setup");
}
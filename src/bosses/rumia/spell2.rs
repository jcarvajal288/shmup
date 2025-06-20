use bevy::app::App;
use bevy::prelude::{AppExtStates, OnEnter, States};
use crate::bosses::boss_health_bar::spawn_boss_health_bar;
use crate::bosses::rumia::RumiaState;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum Spell2State {
    #[default]
    Inactive,
    Phase1,
}

pub fn spell2_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(RumiaState::Spell2), (enter_spell2, spawn_boss_health_bar))
        .init_state::<Spell2State>()
    ;
}

fn enter_spell2() {
    println!("Enter spell 2");
}
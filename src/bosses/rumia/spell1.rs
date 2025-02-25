use bevy::prelude::*;
use crate::bosses::boss::Boss;
use crate::bosses::rumia::RumiaState;
use crate::resources::sprites::{set_one_off_animation, AnimationIndices};

pub fn spell1_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(RumiaState::Spell1), spell1_setup)
    ;
}

fn spell1_setup(
    mut rumia_query: Query<(&Boss, &mut AnimationIndices)>,
) {
    for (_boss, mut animation_indices) in rumia_query.iter_mut() {
        set_one_off_animation(&mut *animation_indices, 0, 3);
    }
}
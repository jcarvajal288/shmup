use std::f32::consts::PI;
use bevy::prelude::*;
use crate::bosses::boss::Boss;
use crate::bosses::rumia::RumiaState;
use crate::bullet::BulletType;
use crate::bullet_patterns::{BoxedBulletPattern, BulletPatternAngle};
use crate::bullet_patterns::bullet_stream::BulletStream;
use crate::bullet_patterns::BulletPatternTarget::Player;
use crate::enemy::Enemy;
use crate::movement_patterns::BoxedMovementPattern;
use crate::resources::sprites::{set_one_off_animation, AnimationIndices, Sprites};

// #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
// pub enum Spell1State {
//     #[default]
//     Setup,
//     PreRumia,
//     Rumia,
// }

pub fn spell1_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(RumiaState::Spell1), spell1_setup)
        .add_systems(Update, (update_spellcard).run_if(in_state(RumiaState::Spell1)))
    ;
}

fn spell1_setup(
    mut commands: Commands,
    mut rumia_query: Query<(&Boss, &Transform, &mut AnimationIndices)>,
) {
    for (_boss, transform, mut animation_indices) in rumia_query.iter_mut() {
        set_one_off_animation(&mut *animation_indices, 0, 3);
        commands.spawn((
            Name::new("spell1"),
            BoxedBulletPattern(Box::new(BulletStream {
                bullet_type: BulletType::BlueRimmedCircle,
                bullets_per_wave: 11,
                waves_per_iteration: 7,
                num_iterations: 1,
                angle: BulletPatternAngle {
                    target: Player,
                    spread: PI * 2.0,
                },
                speed: 200.0,
                acceleration: 0.0,
                startup_timer: Default::default(),
                wave_timer: Timer::from_seconds(0.1, TimerMode::Once),
                iteration_timer: Default::default(),
                waves_left: 0,
                iterations_left: 0,
            })),
            transform.clone(),
        ));
    }
}

fn update_spellcard(
    time: Res<Time>,
    mut commands: Commands,
    sprites: ResMut<Sprites>,
    mut bullet_pattern_query: Query<(&mut BoxedBulletPattern, &Transform)>,
    player_query: Query<&Transform, (With<crate::player::Player>, Without<Enemy>)>,
) {
    for (mut bullet_pattern, transform) in bullet_pattern_query.iter_mut() {
        for player_transform in player_query.iter() {
            bullet_pattern.0.fire(&mut commands, &sprites, *transform, &time, player_transform);
        }
    }
}
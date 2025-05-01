pub mod bullet_stream;
pub mod circle_spawn;

use crate::bullet_patterns::BulletPatternTarget::Player;
use crate::movement_patterns::BoxedBulletMovementPattern;
use crate::resources::sprites::Sprites;
use bevy::prelude::{Commands, Component, Res, Time, Transform};
use dyn_clone::DynClone;

pub trait BulletPattern: DynClone {
    fn fire(
        &mut self,
        commands: &mut Commands,
        sprites: &Res<Sprites>,
        transform: Transform,
        time: &Res<Time>,
        player_transform: &Transform,
        movement_pattern: &mut BoxedBulletMovementPattern
    );
}

dyn_clone::clone_trait_object!(BulletPattern);

#[derive(Component, Clone)]
pub struct BoxedBulletPattern(pub Box<dyn BulletPattern + Send + Sync>);

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum BulletPatternTarget {
    Player,
    Down,
}

#[derive(Clone, Copy)]
pub struct BulletPatternAngle {
    pub target: BulletPatternTarget,
    pub spread: f32,
    pub offset: f32,
}

fn get_target_transform(target: &BulletPatternTarget, starting_transform: &Transform, player_transform: &Transform) -> Transform {
    if *target == Player {
        Transform::from_translation(player_transform.translation - starting_transform.translation)
    } else {
        Transform::from_translation(*starting_transform.down())
    }
}


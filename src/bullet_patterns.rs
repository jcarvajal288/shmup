pub mod bullet_stream;

use bevy::prelude::{Commands, Component, Res, Time, Transform};
use crate::images::Images;
use crate::movement_patterns::BoxedMovementPattern;

pub trait BulletPattern {
    fn fire(
        &mut self,
        commands: &mut Commands,
        images: &Res<Images>,
        transform: Transform,
        time: &Res<Time>,
        player_transform: &Transform,
    ) -> ();
}

#[derive(Component)]
pub struct BoxedBulletPattern(pub Box<dyn BulletPattern + Send + Sync>);

#[derive(Eq, PartialEq)]
pub enum BulletPatternTarget {
    Player,
    Down,
}

pub struct BulletPatternAngle {
    pub target: BulletPatternTarget,
    pub offset: f32,
}
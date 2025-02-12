pub mod bullet_stream;

use crate::images::Images;
use bevy::prelude::{Commands, Component, Res, ResMut, Time, Transform};
use crate::sprites::Sprites;

pub trait BulletPattern {
    fn fire(
        &mut self,
        commands: &mut Commands,
        sprites: &ResMut<Sprites>,
        transform: Transform,
        time: &Res<Time>,
        player_transform: &Transform,
    ) -> ();
}

#[derive(Component)]
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
}
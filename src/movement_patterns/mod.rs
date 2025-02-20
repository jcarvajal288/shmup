pub mod move_straight;
pub mod move_to;

use bevy::prelude::{Component, Res, Time, Transform};

pub trait MovementPattern {
    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>) -> ();
}

#[derive(Component)]
pub struct BoxedMovementPattern(pub Box<dyn MovementPattern + Send + Sync>);


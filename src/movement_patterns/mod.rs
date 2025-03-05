pub mod move_straight;
pub mod move_to;

use bevy::prelude::{Component, Move, Res, Time, Transform};

pub trait MovementPattern {
    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>) -> ();

    fn lateral_movement(&mut self) -> f32;

    fn is_finished(&self) -> bool;
}

#[derive(Component)]
pub struct BoxedMovementPattern(pub Box<dyn MovementPattern + Send + Sync>);

#[derive(Clone)]
pub struct DontMove;

impl Default for DontMove {
    fn default() -> Self { Self {}}
}

impl MovementPattern for DontMove {
    fn do_move(&mut self, _: &mut Transform, _: &Res<Time>) -> () {}
    fn lateral_movement(&mut self) -> f32 { 0.0 }
    fn is_finished(&self) -> bool { true }
}


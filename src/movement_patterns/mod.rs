pub mod move_straight;
pub mod move_to;
pub mod move_direction;
pub mod move_away;
pub mod move_distance_away;

use std::f32::consts::PI;
use bevy::math::{Quat, Rot2, Vec3};
use bevy::prelude::{Component, Res, Time, Transform};
use dyn_clone::DynClone;

pub trait MovementPattern: DynClone {
    fn name(&self) -> &str;

    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>) -> ();

    fn lateral_movement(&mut self) -> f32;

    fn is_finished(&self) -> bool;
}

dyn_clone::clone_trait_object!(MovementPattern);

#[derive(Component, Clone)]
pub struct BoxedMovementPattern(pub Box<dyn MovementPattern + Send + Sync>);

impl Default for BoxedMovementPattern {
    fn default() -> Self {
        BoxedMovementPattern(Box::new(DontMove::default()))
    }
}

#[derive(Component, Clone)]
pub struct BoxedBulletMovementPattern(pub Box<dyn MovementPattern + Send + Sync>);

impl Default for BoxedBulletMovementPattern {
    fn default() -> Self {
        BoxedBulletMovementPattern(Box::new(DontMove::default()))
    }
}

#[derive(Clone)]
pub struct DontMove;

impl Default for DontMove {
    fn default() -> Self { Self {}}
}

impl MovementPattern for DontMove {
    fn name(&self) -> &str { "DontMove" }

    fn do_move(&mut self, _: &mut Transform, _: &Res<Time>) -> () {}
    fn lateral_movement(&mut self) -> f32 { 0.0 }
    fn is_finished(&self) -> bool { true }
}

pub fn face_travel_direction(transform: &mut Transform, direction: Vec3) {
    let angle = direction.y.atan2(direction.x);
    transform.rotation = Quat::from_axis_angle(Vec3::Z, angle + (-PI / 2.0));
}

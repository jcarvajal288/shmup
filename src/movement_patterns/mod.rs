pub mod move_straight;
pub mod move_to;
pub mod move_direction;
pub mod move_away;
pub mod move_distance_away;
pub mod sine_wave;

use std::f32::consts::PI;
use bevy::math::{Quat, Rot2, Vec3};
use bevy::prelude::{Component, Res, Time, Transform};
use dyn_clone::DynClone;
use crate::movement_patterns::MovementPatterns::{StraightAtPlayer, StraightLine};

#[derive(Component, Clone, PartialEq)]
pub enum MovementPatterns {
    StraightLine(Rot2, f32, bool), // angle, speed, face travel direction
    StraightAtPlayer(f32), // speed
}

pub fn run_movement_pattern(movement_pattern: &MovementPatterns, transform: &mut Transform, time: &Res<Time>) {
    match movement_pattern {
        StraightLine(angle, speed, face_travel_direction) => move_straight_line(*angle, *speed, *face_travel_direction, transform, time),
        StraightAtPlayer(_speed) => { /* this is run as StraightLine */}
    }
}

fn move_straight_line(angle: Rot2, speed: f32, face_travel_direction: bool, transform: &mut Transform, time: &Res<Time>) {
    let movement_direction = Vec3::new(angle.cos, angle.sin, 0.0);
    let movement_distance = speed * time.delta_secs();
    let translation_delta = movement_direction * movement_distance;
    transform.translation += translation_delta;
    if face_travel_direction {
        transform.rotation = Quat::from_axis_angle(Vec3::Z, angle.as_radians() + (-PI / 2.0));
    }
}

pub trait MovementPattern: DynClone {
    fn name(&self) -> &str;

    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>);

    fn lateral_movement(&mut self) -> f32;

    fn is_finished(&self) -> bool;
}

dyn_clone::clone_trait_object!(MovementPattern);

#[derive(Component, Clone)]
pub struct BoxedMovementPattern(pub Box<dyn MovementPattern + Send + Sync>);

impl Default for BoxedMovementPattern {
    fn default() -> Self {
        BoxedMovementPattern(Box::new(DontMove))
    }
}

#[derive(Component, Clone)]
pub struct BoxedBulletMovementPattern(pub Box<dyn MovementPattern + Send + Sync>);

impl Default for BoxedBulletMovementPattern {
    fn default() -> Self {
        BoxedBulletMovementPattern(Box::new(DontMove))
    }
}

#[derive(Clone)]
#[derive(Default)]
pub struct DontMove;


impl MovementPattern for DontMove {
    fn name(&self) -> &str { "DontMove" }

    fn do_move(&mut self, _: &mut Transform, _: &Res<Time>) {}
    fn lateral_movement(&mut self) -> f32 { 0.0 }
    fn is_finished(&self) -> bool { true }
}

pub fn face_travel_direction(transform: &mut Transform, direction: Vec3) {
    let angle = direction.y.atan2(direction.x);
    transform.rotation = Quat::from_axis_angle(Vec3::Z, angle + (-PI / 2.0));
}

pub mod move_to;
pub mod move_away;
pub mod move_distance_away;
mod sine_wave;
mod decelerate;
mod straight_line;

use std::f32::consts::PI;
use bevy::math::{Quat, Rot2, Vec2, Vec3};
use bevy::prelude::{Component, Res, Time, Transform};
use dyn_clone::DynClone;
use crate::movement_patterns::decelerate::move_decelerate;
use crate::movement_patterns::MovementPatterns::{Decelerate, SineWave, StraightAtPlayer, StraightLine};
use crate::movement_patterns::sine_wave::move_sine_wave;
use crate::movement_patterns::straight_line::move_straight_line;

#[derive(Component, Clone, PartialEq)]
pub enum MovementPatterns {
    StraightLine(Rot2, f32), // angle, speed
    StraightAtPlayer(f32), // speed
    Decelerate(Rot2, f32, f32, f32), // angle, current speed, final speed, deceleration
    SineWave(f32, f32, f32, Vec2) // amplitude, wavelength, frequency, starting position
}

pub fn run_movement_pattern(movement_pattern: &mut MovementPatterns, transform: &mut Transform, time: &Res<Time>, face_travel_direction: bool) {
    match movement_pattern {
        StraightLine(angle, speed) => {
            move_straight_line(*angle, *speed, transform, time, face_travel_direction)
        }
        StraightAtPlayer(_speed) => { /* this is run as StraightLine */},
        Decelerate(angle, current_speed, final_speed, deceleration) => {
            move_decelerate(*angle, current_speed, *final_speed, *deceleration, transform, time, face_travel_direction)
        }
        SineWave(amplitude, wavelength, frequency, starting_position) => {
            move_sine_wave(*amplitude, *wavelength, *frequency, *starting_position, transform, time, face_travel_direction)
        }
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

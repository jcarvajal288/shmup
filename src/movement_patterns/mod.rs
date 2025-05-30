pub mod move_to;
pub mod move_away;
pub mod move_distance_away;

use std::f32::consts::PI;
use std::time::Duration;
use bevy::math::{Quat, Rot2, Vec2, Vec3};
use bevy::prelude::{Component, Res, Time, Transform};
use dyn_clone::DynClone;
use crate::movement_patterns::MovementPatterns::{Decelerate, SineWave, StraightAtPlayer, StraightLine};

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

fn move_sine_wave(
    amplitude: f32,
    wavelength: f32,
    frequency: f32,
    starting_position: Vec2,
    transform: &mut Transform,
    time: &Res<Time>,
    face_travel: bool
) {
    let new_y = transform.translation.y - frequency * time.delta_secs();
    let x_increment = amplitude * f32::sin(2.0 * PI / wavelength * (new_y - starting_position.y));
    let old_translation = transform.translation;
    let new_translation = Vec3::new(starting_position.x + x_increment, new_y, 0.0);
    transform.translation = new_translation;
    if face_travel {
        let direction = new_translation - old_translation;
        face_travel_direction(transform, direction);
    }
}

pub fn create_decelerate_pattern(direction: Rot2, starting_speed: f32, final_speed: f32, time_to_decelerate: Duration) -> MovementPatterns {
    let deceleration = if time_to_decelerate.as_secs_f32() == 0.0 {
        0.0
    } else {
        (final_speed - starting_speed) / time_to_decelerate.as_secs_f32()
    };
    Decelerate(direction, starting_speed, final_speed, deceleration)
}

fn move_decelerate(
    angle: Rot2,
    current_speed: &mut f32,
    final_speed: f32,
    deceleration: f32,
    transform: &mut Transform,
    time: &Res<Time>,
    face_travel: bool
) {
    let direction = Vec3::new(angle.cos, angle.sin, 0.0);
    let delta_time = time.delta_secs();
    if *current_speed > final_speed {
        *current_speed += deceleration * delta_time;
    }
    transform.translation += direction * *current_speed * delta_time;
    if face_travel {
        face_travel_direction(transform, direction);
    }
}

fn move_straight_line(angle: Rot2, speed: f32, transform: &mut Transform, time: &Res<Time>, face_travel: bool) {
    let movement_direction = Vec3::new(angle.cos, angle.sin, 0.0);
    let movement_distance = speed * time.delta_secs();
    let translation_delta = movement_direction * movement_distance;
    transform.translation += translation_delta;
    if face_travel {
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

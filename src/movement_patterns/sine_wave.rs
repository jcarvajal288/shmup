use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Res, Time, Transform};
use std::f32::consts::PI;
use crate::movement_patterns;

pub fn move_sine_wave(
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
    let new_translation = Vec3::new(starting_position.x + x_increment, new_y, old_translation.z);
    transform.translation = new_translation;
    if face_travel {
        let direction = new_translation - old_translation;
        movement_patterns::face_travel_direction(transform, direction);
    }
}
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Res, Time, Transform};
use std::f32::consts::PI;
use crate::movement_patterns;
use crate::movement_patterns::MovementPatterns;
use crate::movement_patterns::MovementPatterns::SineWavePattern;

#[derive(Clone, PartialEq)]
pub struct SineWave {
    pub amplitude: f32,
    pub wavelength: f32,
    pub frequency: f32,
    pub starting_position: Vec2,
}

impl SineWave {
    pub fn do_move(
        &self,
        transform: &mut Transform,
        time: &Res<Time>,
        face_travel: bool
    ) {
        let new_y = transform.translation.y - self.frequency * time.delta_secs();
        let x_increment = self.amplitude * f32::sin(2.0 * PI / self.wavelength * (new_y - self.starting_position.y));
        let old_translation = transform.translation;
        let new_translation = Vec3::new(self.starting_position.x + x_increment, new_y, old_translation.z);
        transform.translation = new_translation;
        if face_travel {
            let direction = new_translation - old_translation;
            movement_patterns::face_travel_direction(transform, direction);
        }
    }
}

pub fn create_sine_wave_pattern(amplitude: f32, wavelength: f32, frequency: f32, starting_position: Vec2) -> MovementPatterns {
    SineWavePattern(
        SineWave {
            amplitude,
            wavelength,
            frequency,
            starting_position,
        }
    )
}
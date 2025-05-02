use crate::movement_patterns::MovementPattern;
use bevy::prelude::{Res, Time, Transform, Vec2};
use std::f32::consts::PI;

const DOWN: f32 = -PI / 2.0;

#[derive(Clone)]
pub struct MoveSineWave {
    pub amplitude: f32,
    pub wavelength: f32,
    pub frequency: f32,
    pub starting_position: Vec2,
}

impl MoveSineWave {
    pub fn sin(&self, y: f32) -> f32 {
        self.amplitude * f32::sin(2.0 * PI / self.wavelength * (y - self.starting_position.y))
    }
}

impl MovementPattern for MoveSineWave {
    fn name(&self) -> &str {
        "Sine Wave"
    }

    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>) {
        let new_y = transform.translation.y - self.frequency * time.delta_secs();
        let x_increment = self.sin(new_y);
        transform.translation.x = self.starting_position.x + x_increment;
        transform.translation.y = new_y;
    }

    fn lateral_movement(&mut self) -> f32 {
        0.0
    }

    fn is_finished(&self) -> bool {
        false
    }
}
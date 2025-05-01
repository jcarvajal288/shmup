use crate::movement_patterns::MovementPattern;
use bevy::math::{Curve, Vec3};
use bevy::prelude::{Res, Time, Transform};
use std::f32::consts::PI;

const DOWN: f32 = -PI / 2.0;

#[derive(Clone)]
pub struct MoveSineWave {
    pub amplitude: f32,
    pub wavelength: f32,
    pub speed: f32,
}

impl MoveSineWave {
    pub fn sin(&self, time: f32) -> f32 {
        self.amplitude * f32::sin((2.0 * PI / self.wavelength) * (time - self.speed))
    }
}

impl MovementPattern for MoveSineWave {
    fn name(&self) -> &str {
        "Sine Wave"
    }

    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>) {
        let speed = 100.0;
        let movement_angle = self.sin(time.elapsed_secs()) + DOWN;
        println!("movement angle: {}", movement_angle);

        let movement_direction = Vec3::new(movement_angle.cos(), movement_angle.sin(), 0.0);
        let movement_distance = speed * time.delta_secs();
        let translation_delta = movement_direction * movement_distance;
        transform.translation += translation_delta;
    }

    fn lateral_movement(&mut self) -> f32 {
        0.0
    }

    fn is_finished(&self) -> bool {
        false
    }
}
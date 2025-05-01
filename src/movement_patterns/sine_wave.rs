use crate::movement_patterns::MovementPattern;
use bevy::math::ops::sin;
use bevy::math::{Curve, Vec3};
use bevy::prelude::{FunctionCurve, Interval, Res, Time, Transform};
use std::f32::consts::PI;

#[derive(Clone)]
pub struct SineWave {
}

impl MovementPattern for SineWave {
    fn name(&self) -> &str {
        "Sine Wave"
    }

    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>) {
        let speed = 100.0;
        let sin_wave = FunctionCurve::new(Interval::EVERYWHERE, sin);
        let movement_angle = sin_wave.sample(time.elapsed_secs()).unwrap() * PI / 2.0 - PI / 2.0;
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
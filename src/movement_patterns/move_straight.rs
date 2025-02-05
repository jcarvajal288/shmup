use bevy::math::Vec3;
use bevy::prelude::{Res, Time, Transform};
use crate::movement_patterns::MovementPattern;

pub struct MoveStraight {
    pub angle: f32,
    pub speed: f32,
    pub acceleration: f32,
}

impl Default for MoveStraight {
    fn default() -> Self {
        Self {
            angle: -std::f32::consts::PI,
            speed: 1.0,
            acceleration: 0.0,
        }
    }
}

impl MovementPattern for MoveStraight {
    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>) {
        let movement_direction = self.angle * Vec3::Y;
        let movement_distance = self.speed * time.delta_secs();
        let translation_delta = movement_direction * movement_distance;
        transform.translation += translation_delta;
        self.speed += self.acceleration;
    }
}

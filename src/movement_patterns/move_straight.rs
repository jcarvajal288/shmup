use bevy::math::Vec3;
use bevy::prelude::{Res, Time, Transform};
use crate::movement_patterns::MovementPattern;

#[derive(Clone)]
pub struct MoveStraight {
    pub angle: f32,
    pub speed: f32,
    pub acceleration: f32,
    pub face_travel_direction: bool,
}

impl Default for MoveStraight {
    fn default() -> Self {
        Self {
            angle: -std::f32::consts::PI,
            speed: 1.0,
            acceleration: 0.0,
            face_travel_direction: false,
        }
    }
}

impl MovementPattern for MoveStraight {
    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>) {
        let movement_direction = Vec3::new(self.angle.cos(), self.angle.sin(), 0.0);
        let movement_distance = self.speed * time.delta_secs();
        let translation_delta = movement_direction * movement_distance;
        transform.translation += translation_delta;
        self.speed += self.acceleration;
    }
}

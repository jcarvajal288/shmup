use std::f32::consts::PI;
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{Res, Time, Transform};
use crate::movement_patterns::MovementPattern;

#[derive(Clone)]
pub struct MoveTo {
    pub destination: Vec2,
    pub speed: f32,
    pub acceleration: f32,
    pub face_travel_direction: bool,
}

impl Default for MoveTo {
    fn default() -> Self {
        Self {
            destination: Vec2::ZERO,
            speed: 0.0,
            acceleration: 0.0,
            face_travel_direction: false,
        }
    }
}

impl MovementPattern for MoveTo {
    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>) -> () {
        let diff = self.destination - transform.translation.truncate();
        let angle = diff.y.atan2(diff.x);
        let movement_direction = Vec3::new(angle.cos(), angle.sin(), 0.0);
        let movement_distance = self.speed * time.delta_secs();
        let translation_delta = movement_direction * movement_distance;
        transform.translation += translation_delta;
        if self.face_travel_direction {
            transform.rotation = Quat::from_axis_angle(Vec3::Z, angle + (-PI / 2.0));
        }
        self.speed += self.acceleration;
        if transform.translation.truncate().abs_diff_eq(self.destination, 1.0) {
            self.speed = 0.0;
        }
    }
}

impl MoveTo {
    pub fn move_finished(&self, transform: &mut Transform) -> bool {
        transform.translation.truncate() == self.destination
    }
}
use bevy::math::{Quat, Rot2, Vec3};
use bevy::prelude::{Res, Time, Transform};
use std::f32::consts::PI;
use crate::movement_patterns::MovementPatterns;
use crate::movement_patterns::MovementPatterns::StraightLinePattern;

#[derive(Clone, PartialEq)]
pub struct StraightLine {
    pub angle: Rot2,
    pub speed: f32,
}

impl Default for StraightLine {
    fn default() -> Self {
        Self {
            angle: Rot2::default(),
            speed: 0.0,
        }
    }
}

impl StraightLine {
    pub fn do_move(&self, transform: &mut Transform, time: &Res<Time>, face_travel: bool) {
        let movement_direction = Vec3::new(self.angle.cos, self.angle.sin, 0.0);
        let movement_distance = self.speed * time.delta_secs();
        let translation_delta = movement_direction * movement_distance;
        transform.translation += translation_delta;
        if face_travel {
            transform.rotation = Quat::from_axis_angle(Vec3::Z, self.angle.as_radians() + (-PI / 2.0));
        }
    }
}

pub fn create_straight_line_pattern(angle: Rot2, speed: f32) -> MovementPatterns {
    StraightLinePattern(
        StraightLine {
            angle,
            speed,
        }
    )
}
use std::cmp;
use crate::movement_patterns;
use crate::movement_patterns::MovementPattern;
use bevy::math::{Rot2, Vec2, Vec3};
use bevy::prelude::{Res, Time, Transform};

#[derive(Clone, PartialEq)]
pub struct CurvedLine {
    pub speed: f32,
    pub distance_before_curve: f32,
    pub current_angle: Rot2,
    pub max_angle: Rot2,
    pub rate_of_change: f32,
    pub starting_position: Vec2,
}

impl MovementPattern for CurvedLine {
    fn name(&self) -> &str {
        "Vertical Curve"
    }

    fn do_move(
        &mut self,
        transform: &mut Transform,
        time: &Res<Time>,
        face_travel: bool
    ) {
        let distance_from_starting_position = transform.translation.truncate().distance(self.starting_position);

        if distance_from_starting_position > self.distance_before_curve {
            let delta_radians = self.current_angle.as_radians() + self.rate_of_change * time.delta_secs();
            let new_radians = if self.rate_of_change > 0.0 {
                f32::min(self.max_angle.as_radians(), delta_radians)
            } else {
                f32::max(self.max_angle.as_radians(), delta_radians)
            };
            self.current_angle = Rot2::radians(new_radians);
        };

        let movement_direction = Vec3::new(self.current_angle.cos, self.current_angle.sin, 0.0);
        let movement_distance = self.speed * time.delta_secs();
        let translation_delta = movement_direction * movement_distance;
        let old_translation = transform.translation;
        let new_translation = transform.translation + translation_delta;
        transform.translation = new_translation;
        if face_travel {
            let direction = new_translation - old_translation;
            movement_patterns::face_travel_direction(transform, direction);
        }
    }

    fn lateral_movement(&self) -> f32 {
        self.current_angle.as_radians()
    }

    fn is_finished(&self) -> bool {
        false
    }
}
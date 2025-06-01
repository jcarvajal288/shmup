use bevy::math::{Rot2, Vec3};
use bevy::prelude::{Res, Time, Transform};
use std::time::Duration;
use crate::movement_patterns;
use crate::movement_patterns::MovementPatterns;

#[derive(Clone, PartialEq)]
pub struct Decelerate {
    pub angle: Rot2,
    pub current_speed: f32,
    pub final_speed: f32,
    pub deceleration: f32,
}


impl Decelerate {
    pub fn do_move(
        &mut self,
        transform: &mut Transform,
        time: &Res<Time>,
        face_travel: bool
    ) {
        let direction = Vec3::new(self.angle.cos, self.angle.sin, 0.0);
        let delta_time = time.delta_secs();
        if self.current_speed > self.final_speed {
            self.current_speed += self.deceleration * delta_time;
        }
        transform.translation += direction * self.current_speed * delta_time;
        if face_travel {
            movement_patterns::face_travel_direction(transform, direction);
        }
    }
}

pub fn create_decelerate_pattern(angle: Rot2, starting_speed: f32, final_speed: f32, time_to_decelerate: Duration) -> Decelerate {
    let deceleration = if time_to_decelerate.as_secs_f32() == 0.0 {
        0.0
    } else {
        (final_speed - starting_speed) / time_to_decelerate.as_secs_f32()
    };
    Decelerate {
        angle,
        current_speed: starting_speed,
        final_speed,
        deceleration,
    }
}

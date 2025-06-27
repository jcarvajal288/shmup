use std::f32::consts::PI;
use crate::movement_patterns;
use crate::movement_patterns::{MovementPattern, MovementPatterns};
use bevy::math::{Rot2, Vec2, Vec3};
use bevy::prelude::{Res, Time, Transform};
use std::time::Duration;
use crate::movement_patterns::MovementPatterns::DeceleratePattern;

#[derive(Clone, PartialEq)]
pub struct Decelerate {
    pub angle: Rot2,
    pub current_speed: f32,
    pub final_speed: f32,
    pub deceleration: f32,
}


impl MovementPattern for Decelerate {
    fn name(&self) -> &str {
        "Decelerate"
    }

    fn do_move(
        &mut self,
        transform: &mut Transform,
        time: &Res<Time>,
        face_travel: bool
    ) {
        if self.current_speed < 1.0 {
            self.current_speed = 0.0;
            return;
        }
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

    fn lateral_movement(&self) -> f32 {
        if self.current_speed == 0.0 {
            PI / 2.0
        } else {
            self.angle.as_radians()
        }
    }

    fn is_finished(&self) -> bool {
        !(self.current_speed > self.final_speed  + 1.0)
    }
}

pub fn create_decelerate_pattern(angle: Rot2, starting_speed: f32, final_speed: f32, time_to_decelerate: Duration) -> MovementPatterns {
    let deceleration = if time_to_decelerate.as_secs_f32() == 0.0 {
        0.0
    } else {
        (final_speed - starting_speed) / time_to_decelerate.as_secs_f32()
    };
    DeceleratePattern(
        Decelerate {
            angle,
            current_speed: starting_speed,
            final_speed,
            deceleration,
        }
    )
}

pub fn create_move_to_pattern(starting_position: Vec2, destination: Vec2, time: Duration) -> MovementPatterns {
    let displacement = destination - starting_position;
    let angle = displacement.y.atan2(displacement.x);
    let distance = displacement.length();
    let current_speed = distance * 2.0 / time.as_secs_f32();
    DeceleratePattern(
        Decelerate {
            angle: Rot2::radians(angle),
            current_speed,
            final_speed: 0.0,
            deceleration: -current_speed / time.as_secs_f32(),
        }
    )
}

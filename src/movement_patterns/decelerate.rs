use bevy::math::{Rot2, Vec3};
use bevy::prelude::{Res, Time, Transform};
use std::time::Duration;
use crate::movement_patterns;
use crate::movement_patterns::MovementPatterns;
use crate::movement_patterns::MovementPatterns::Decelerate;

pub fn move_decelerate(
    angle: Rot2,
    current_speed: &mut f32,
    final_speed: f32,
    deceleration: f32,
    transform: &mut Transform,
    time: &Res<Time>,
    face_travel: bool
) {
    let direction = Vec3::new(angle.cos, angle.sin, 0.0);
    let delta_time = time.delta_secs();
    if *current_speed > final_speed {
        *current_speed += deceleration * delta_time;
    }
    transform.translation += direction * *current_speed * delta_time;
    if face_travel {
        movement_patterns::face_travel_direction(transform, direction);
    }
}

pub fn create_decelerate_pattern(direction: Rot2, starting_speed: f32, final_speed: f32, time_to_decelerate: Duration) -> MovementPatterns {
    let deceleration = if time_to_decelerate.as_secs_f32() == 0.0 {
        0.0
    } else {
        (final_speed - starting_speed) / time_to_decelerate.as_secs_f32()
    };
    Decelerate(direction, starting_speed, final_speed, deceleration)
}
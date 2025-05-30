use bevy::math::{Quat, Rot2, Vec3};
use bevy::prelude::{Res, Time, Transform};
use std::f32::consts::PI;

pub fn move_straight_line(angle: Rot2, speed: f32, transform: &mut Transform, time: &Res<Time>, face_travel: bool) {
    let movement_direction = Vec3::new(angle.cos, angle.sin, 0.0);
    let movement_distance = speed * time.delta_secs();
    let translation_delta = movement_direction * movement_distance;
    transform.translation += translation_delta;
    if face_travel {
        transform.rotation = Quat::from_axis_angle(Vec3::Z, angle.as_radians() + (-PI / 2.0));
    }
}
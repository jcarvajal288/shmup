use crate::bullet::{spawn_bullet, BulletSpawnEvent, BulletSpawner, BulletType};
use crate::bullet_patterns::{get_target_transform, BulletPattern, BulletPatternAngle};
use crate::movement_patterns::{BoxedBulletMovementPattern, MovementPatterns};
use crate::resources::sprites::Sprites;
use bevy::prelude::*;
use std::f32::consts::PI;
use crate::movement_patterns::MovementPatterns::StraightLine;

pub struct Starburst {
    pub bullet_type: BulletType,
    pub num_lines: usize,
    pub num_bullets_in_line: usize,
    pub lowest_speed: f32,
    pub highest_speed: f32,
    pub origin: Vec2,
    pub target: Vec2,
}


pub fn fire_starburst(bullet_spawn_events: &mut EventWriter<BulletSpawnEvent>, starburst: Starburst) {
    let bullet_line = vec![starburst.bullet_type; starburst.num_bullets_in_line];
    let speed_increment = (starburst.highest_speed - starburst.lowest_speed) / starburst.num_bullets_in_line as f32;
    let speeds = (0..starburst.num_bullets_in_line).map(|i| {
        starburst.lowest_speed + (i as f32 * speed_increment)
    }).collect::<Vec<f32>>();
    let firing_angle = starburst.target.y.atan2(starburst.target.x);
    let step_size = (2.0 * PI) / starburst.num_lines as f32;
    let angles = (0..starburst.num_lines).map(|i: usize| {
        firing_angle - PI + (i as f32 * step_size)
    }).collect::<Vec<f32>>();
    for (bullet_type, speed) in bullet_line.iter()
        .zip(speeds.iter())
        .map(|(bullet_type, speed)| (bullet_type, speed))
    {
        for angle in &angles {
            bullet_spawn_events.send(BulletSpawnEvent {
                bullet_type: *bullet_type,
                position: starburst.origin,
                movement_pattern: StraightLine(Rot2::radians(*angle), *speed),
            });
        }
    }
}
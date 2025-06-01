use crate::bullet::{BulletSpawnEvent, BulletType};
use crate::movement_patterns::MovementPatterns::StraightLinePattern;
use bevy::prelude::*;
use std::f32::consts::PI;
use crate::movement_patterns::straight_line::create_straight_line_pattern;

pub struct Starburst {
    pub bullets: Vec<BulletType>,
    pub num_lines: usize,
    pub speed_range: (f32, f32),
    pub offset: f32,
    pub target: Vec2,
}

impl Default for Starburst {
    fn default() -> Self {
        Self {
            bullets: vec![BulletType::WhiteArrow],
            num_lines: 0,
            speed_range: (0.0, 0.0),
            offset: 0.0,
            target: Default::default(),
        }
    }
}


impl Starburst {

    pub fn fire(&self, origin: &Transform, bullet_spawn_events: &mut EventWriter<BulletSpawnEvent>) {
        let speed_increment = (self.speed_range.1 - self.speed_range.0) / self.bullets.len() as f32;
        let speeds = (0..self.bullets.len()).map(|i| {
            self.speed_range.0 + (i as f32 * speed_increment)
        }).collect::<Vec<f32>>();
        let firing_angle = self.target.y.atan2(self.target.x);
        let step_size = (2.0 * PI) / self.num_lines as f32;
        let angles = (0..self.num_lines).map(|i: usize| {
            firing_angle - PI + (i as f32 * step_size) + self.offset
        }).collect::<Vec<f32>>();
        for (bullet_type, speed) in self.bullets.iter()
            .zip(speeds.iter())
            .map(|(bullet_type, speed)| (bullet_type, speed))
        {
            for angle in &angles {
                bullet_spawn_events.send(BulletSpawnEvent {
                    bullet_type: *bullet_type,
                    position: origin.translation.truncate(),
                    movement_pattern: create_straight_line_pattern(Rot2::radians(*angle), *speed),
                });
            }
        }
    }
}


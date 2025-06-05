use bevy::math::Vec2;
use bevy::prelude::{Timer, TimerMode};
use crate::game::SpawnTimer;

pub struct SpawnTimeTracker {
    elapsed: f32
}

impl Default for SpawnTimeTracker {
    fn default() -> Self {
        Self {
            elapsed: 0.0,
        }
    }
}

impl SpawnTimeTracker {

    pub fn increment(&mut self, time_to_add: f32) {
        self.elapsed += time_to_add;
    }

    pub fn timer_with_increment(&mut self, time_to_add: f32) -> SpawnTimer {
        self.increment(time_to_add);
        SpawnTimer(Timer::from_seconds(self.elapsed, TimerMode::Once))
    }
}

pub fn horizontal_line(leftmost: f32, rightmost: f32, y: f32, num_spawns: usize) -> Vec<Vec2> {
    let interval = (rightmost - leftmost) / (num_spawns - 1) as f32;
    (0..num_spawns)
        .map(|i| leftmost + interval * i as f32)
        .map(|x| Vec2::new(x, y))
        .collect()
}
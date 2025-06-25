use bevy::prelude::Timer;
use bevy::time::TimerMode;

pub struct ShotSchedule {
    pub delay: Timer,
    pub interval: Timer,
    pub repetitions: i32,
}

impl Default for ShotSchedule {
    fn default() -> Self {
        Self {
            delay: Timer::default(),
            interval: Timer::default(),
            repetitions: 1,
        }
    }
}

pub fn create_shot_schedule(delay: f32, interval: f32, repetitions: i32) -> ShotSchedule {
    ShotSchedule {
        delay: Timer::from_seconds(delay, TimerMode::Once),
        interval: Timer::from_seconds(interval, TimerMode::Once),
        repetitions,
    }
}
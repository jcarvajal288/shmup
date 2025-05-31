use bevy::prelude::Timer;

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
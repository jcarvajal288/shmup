use bevy::prelude::Timer;

pub struct ShotSchedule {
    pub interval: Timer,
    pub repetitions: i32,
}

impl Default for ShotSchedule {
    fn default() -> Self {
        Self {
            interval: Timer::default(),
            repetitions: 0,
        }
    }
}
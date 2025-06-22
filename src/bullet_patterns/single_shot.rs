use crate::bullet::{BulletSpawnEvent, BulletType};
use crate::bullet_patterns::shot_schedule::ShotSchedule;
use crate::bullet_patterns::BulletPatterns::SingleShotPattern;
use crate::bullet_patterns::{BulletPatterns, Target};
use crate::movement_patterns::MovementPatterns::StraightLinePattern;
use bevy::math::Rot2;
use bevy::prelude::{default, EventWriter, Timer, TimerMode, Transform};
use crate::game::angle_to_transform;
use crate::movement_patterns::straight_line::create_straight_line_pattern;

pub struct SingleShot {
    pub bullet_type: BulletType,
    pub speed: f32
}

impl Default for SingleShot {
    fn default() -> Self {
        Self {
            bullet_type: BulletType::WhiteArrow,
            speed: 0.0,
        }
    }
}

impl SingleShot {

    pub fn fire(
        &self,
        origin: &Transform,
        angle: Rot2,
        bullet_spawn_events: &mut EventWriter<BulletSpawnEvent>,
    ) {
        bullet_spawn_events.send(BulletSpawnEvent {
            bullet_type: self.bullet_type,
            position: origin.translation.truncate(),
            movement_pattern: create_straight_line_pattern(angle, self.speed),
            ..default()
        });
    }
}

pub fn single_shot_at_player(
    bullet_type: BulletType,
    speed: f32,
    interval_secs: f32,
    repetitions: i32,
) -> BulletPatterns {
    SingleShotPattern(
        SingleShot {
            bullet_type,
            speed,
        },
        Target::Player,
        ShotSchedule {
            interval: Timer::from_seconds(interval_secs, TimerMode::Once),
            repetitions,
            ..default()
        }
    )
}
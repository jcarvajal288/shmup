use crate::bullet::{BulletSpawnEvent, BulletType};
use crate::bullet_patterns::shot_schedule::ShotSchedule;
use crate::bullet_patterns::BulletPatterns::ShootAtPlayerPattern;
use crate::bullet_patterns::BulletPatterns;
use crate::movement_patterns::MovementPatterns::StraightLinePattern;
use bevy::math::Rot2;
use bevy::prelude::{default, EventWriter, Timer, TimerMode, Transform};
use crate::movement_patterns::straight_line::create_straight_line_pattern;

pub struct ShootAtPlayer {
    pub bullet_type: BulletType,
    pub speed: f32
}

impl Default for ShootAtPlayer {
    fn default() -> Self {
        Self {
            bullet_type: BulletType::WhiteArrow,
            speed: 0.0,
        }
    }
}

impl ShootAtPlayer {

    pub fn fire(
        &self,
        origin: &Transform,
        player_transform: &Transform,
        bullet_spawn_events: &mut EventWriter<BulletSpawnEvent>,
    ) {
        let diff = player_transform.translation.truncate() - origin.translation.truncate();
        let angle = diff.y.atan2(diff.x);
        bullet_spawn_events.send(BulletSpawnEvent {
            bullet_type: self.bullet_type,
            position: origin.translation.truncate(),
            movement_pattern: create_straight_line_pattern(Rot2::radians(angle), self.speed),
            ..default()
        });
    }
}

pub fn shoot_at_player_pattern(
    bullet_type: BulletType,
    speed: f32,
    interval_secs: f32,
    repetitions: i32,
) -> BulletPatterns {
    ShootAtPlayerPattern(
        ShootAtPlayer {
            bullet_type,
            speed,
        },
        ShotSchedule {
            interval: Timer::from_seconds(interval_secs, TimerMode::Once),
            repetitions,
            ..default()
        }
    )
}
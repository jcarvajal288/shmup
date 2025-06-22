pub mod starburst;
pub mod single_shot;
pub mod shot_schedule;
pub mod shotgun;

use bevy::math::Rot2;
use crate::bullet::BulletSpawnEvent;
use crate::bullet_patterns::single_shot::SingleShot;
use crate::bullet_patterns::shotgun::Shotgun;
use crate::bullet_patterns::starburst::Starburst;
use crate::bullet_patterns::BulletPatterns::{SingleShotPattern, ShotgunPattern, StarburstPattern};
use bevy::prelude::{Component, EventWriter, Res, Time, Transform};
use shot_schedule::ShotSchedule;
use crate::game::angle_to_transform;

pub const ENDLESS: i32 = -1;

#[derive(Component)]
pub enum BulletPatterns {
    SingleShotPattern(SingleShot, Target, ShotSchedule),
    StarburstPattern(Starburst, Target, ShotSchedule),
    ShotgunPattern(Shotgun, Target, ShotSchedule),
}

#[derive(Component)]
pub enum Target {
    Player,
    Angle(Rot2),
}

impl Target {
    pub fn get_angle(&self, origin: &Transform, player_transform: &Transform) -> Rot2 {
        match self {
            Target::Player => angle_to_transform(*origin, *player_transform),
            Target::Angle(rot2) => *rot2,
        }
    }
}

pub fn fire_bullet_pattern(
    bullet_pattern: &mut BulletPatterns,
    time: &Res<Time>,
    origin: &Transform,
    player_transform: &Transform,
    bullet_spawn_events: &mut EventWriter<BulletSpawnEvent>,
) {
    match bullet_pattern {
        SingleShotPattern(shoot_at_player, target, shot_schedule) => {
            let angle = target.get_angle(origin, player_transform);
            let fire = || shoot_at_player.fire(origin, angle, bullet_spawn_events);
            run_schedule(fire, shot_schedule, time);
        }
        StarburstPattern(starburst, target, shot_schedule) => {
            let angle = target.get_angle(origin, player_transform);
            let fire = || starburst.fire(origin, angle, bullet_spawn_events);
            run_schedule(fire, shot_schedule, time);
        }
        ShotgunPattern(shotgun, target, shot_schedule) => {
            let angle = target.get_angle(origin, player_transform);
            let fire = || shotgun.fire(origin, angle, bullet_spawn_events);
            run_schedule(fire, shot_schedule, time);
        }
    }
}

fn run_schedule<F>(mut fire: F, shot_schedule: &mut ShotSchedule, time: &Res<Time>)
where F: FnMut()
{
    if shot_schedule.delay.tick(time.delta()).finished() {
        if shot_schedule.repetitions != 0 {
            if shot_schedule.interval.tick(time.delta()).just_finished() {
                fire();
                shot_schedule.interval.reset();
                if shot_schedule.repetitions > 0 {
                    shot_schedule.repetitions -= 1
                }
            }
        }
    }
}

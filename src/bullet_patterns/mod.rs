pub mod starburst;
pub mod shoot_at_player;
pub mod shot_schedule;
pub mod shotgun;

use crate::bullet::BulletSpawnEvent;
use crate::bullet_patterns::shoot_at_player::ShootAtPlayer;
use crate::bullet_patterns::shotgun::Shotgun;
use crate::bullet_patterns::starburst::Starburst;
use crate::bullet_patterns::BulletPatterns::{ShootAtPlayerPattern, ShotgunPattern, StarburstPattern};
use bevy::prelude::{Component, EventWriter, Res, Time, Transform};
use shot_schedule::ShotSchedule;

pub const ENDLESS: i32 = -1;

#[derive(Component)]
pub enum BulletPatterns {
    ShootAtPlayerPattern(ShootAtPlayer, ShotSchedule),
    StarburstPattern(Starburst, ShotSchedule),
    ShotgunPattern(Shotgun, ShotSchedule),
}

pub fn fire_bullet_pattern(
    bullet_pattern: &mut BulletPatterns,
    time: &Res<Time>,
    origin: &Transform,
    player_transform: &Transform,
    bullet_spawn_events: &mut EventWriter<BulletSpawnEvent>,
) {
    match bullet_pattern {
        ShootAtPlayerPattern(shoot_at_player, shot_schedule) => {
            let fire = || shoot_at_player.fire(origin, player_transform, bullet_spawn_events);
            run_schedule(fire, shot_schedule, time);
        }
        StarburstPattern(starburst, shot_schedule) => {
            let fire = || starburst.fire(origin, bullet_spawn_events);
            run_schedule(fire, shot_schedule, time);
        }
        ShotgunPattern(shotgun, shot_schedule) => {
            let fire = || shotgun.fire(origin, bullet_spawn_events);
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

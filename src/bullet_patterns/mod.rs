pub mod starburst;
pub mod shoot_at_player;
pub mod shot_schedule;

use crate::bullet::BulletSpawnEvent;
use crate::bullet_patterns::shoot_at_player::ShootAtPlayer;
use crate::bullet_patterns::BulletPatternTarget::Player;
use crate::bullet_patterns::BulletPatterns::ShootAtPlayerPattern;
use crate::movement_patterns::BoxedBulletMovementPattern;
use crate::resources::sprites::Sprites;
use bevy::prelude::{Commands, Component, EventWriter, Res, Time, Transform};
use dyn_clone::DynClone;
use shot_schedule::ShotSchedule;

pub const ENDLESS: i32 = -1;

#[derive(Component)]
pub enum BulletPatterns {
    ShootAtPlayerPattern(ShootAtPlayer, ShotSchedule),
}

pub fn fire_bullet_pattern(
    bullet_pattern: &mut BulletPatterns,
    time: &Res<Time>,
    transform: &Transform,
    player_transform: &Transform,
    bullet_spawn_events: &mut EventWriter<BulletSpawnEvent>,
) {
    match bullet_pattern {
        ShootAtPlayerPattern(shoot_at_player, shot_schedule) => {
            let fire = || shoot_at_player.fire(transform, player_transform, bullet_spawn_events);
            run_schedule(fire, shot_schedule, time)
        }
    }
}

fn run_schedule<F>(mut fire: F, shot_schedule: &mut ShotSchedule, time: &Res<Time>)
where F: FnMut()
{
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

pub trait BulletPattern: DynClone {
    fn fire(
        &mut self,
        commands: &mut Commands,
        sprites: &Res<Sprites>,
        transform: Transform,
        time: &Res<Time>,
        player_transform: &Transform,
        movement_pattern: &mut BoxedBulletMovementPattern
    );
}

dyn_clone::clone_trait_object!(BulletPattern);

#[derive(Component, Clone)]
pub struct BoxedBulletPattern(pub Box<dyn BulletPattern + Send + Sync>);

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum BulletPatternTarget {
    Player,
    Down,
}

#[derive(Clone, Copy)]
pub struct BulletPatternAngle {
    pub target: BulletPatternTarget,
    pub spread: f32,
    pub offset: f32,
}

fn get_target_transform(target: &BulletPatternTarget, starting_transform: &Transform, player_transform: &Transform) -> Transform {
    if *target == Player {
        Transform::from_translation(player_transform.translation - starting_transform.translation)
    } else {
        Transform::from_translation(*starting_transform.down())
    }
}


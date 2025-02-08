use std::f32::consts::PI;
use bevy::math::Vec3;
use crate::bullet::{spawn_bullet, BulletSpawner, BulletType};
use crate::bullet_patterns::{BulletPattern, BulletPatternAngle, BulletPatternTarget};
use crate::images::Images;
use crate::movement_patterns::BoxedMovementPattern;
use bevy::prelude::{Commands, Component, Res, Time, Timer, Transform, Vec2};
use crate::bullet_patterns::BulletPatternTarget::Player;
use crate::movement_patterns::move_straight::MoveStraight;

#[derive(Component)]
pub struct BulletStream {
    pub bullet_type: BulletType,
    pub num_bullets: usize,
    pub num_iterations: usize,
    pub angle: BulletPatternAngle,
    pub speed: f32,
    pub acceleration: f32,
    pub timer: Timer,
}

impl BulletPattern for BulletStream {

    fn fire(
        &mut self,
        commands: &mut Commands,
        images: &Res<Images>,
        transform: Transform,
        time: &Res<Time>,
        player_transform: &Transform,
    ) {
        self.timer.tick(time.delta());
        if self.timer.just_finished() && self.num_iterations > 0 {
            let target = get_target_transform(&self.angle.target, &transform, player_transform);
            let firing_angle = target.translation.y.atan2(target.translation.x);

            spawn_bullet(commands, &images, BulletSpawner {
                bullet_type: self.bullet_type,
                position: Vec2::new(transform.translation.x, transform.translation.y),
                movement_pattern: BoxedMovementPattern(Box::new(MoveStraight {
                    angle: firing_angle,
                    speed: self.speed,
                    acceleration: self.acceleration,
                    face_travel_direction: true,
                }))
            });
            self.num_iterations -= 1;
        }
    }
}

fn get_target_transform(target: &BulletPatternTarget, starting_transform: &Transform, player_transform: &Transform) -> Transform {
    if *target == Player {
        Transform::from_translation(player_transform.translation - starting_transform.translation)
    } else {
        Transform::from_translation(*starting_transform.down())
    }
}

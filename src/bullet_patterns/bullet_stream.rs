use std::f32::consts::PI;
use crate::bullet::{spawn_bullet, BulletSpawner, BulletType};
use crate::bullet_patterns::{BulletPattern, BulletPatternAngle, BulletPatternTarget};
use crate::images::Images;
use crate::movement_patterns::BoxedMovementPattern;
use bevy::prelude::{Commands, Component, Res, Time, Timer, Transform, Vec2};
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
        if self.timer.just_finished() {

            let diff = player_transform.translation - transform.translation;
            let firing_angle = diff.y.atan2(diff.x);

            spawn_bullet(commands, &images, BulletSpawner {
                bullet_type: self.bullet_type,
                position: Vec2::new(transform.translation.x, transform.translation.y),
                movement_pattern: BoxedMovementPattern(Box::new(MoveStraight {
                    angle: firing_angle,
                    speed: self.speed,
                    acceleration: self.acceleration,
                    face_travel_direction: true,
                }))
            })
        }
    }
}

use crate::bullet::{spawn_bullet, BulletSpawner, BulletType};
use crate::images::Images;
use crate::movement_patterns::move_straight::MoveStraight;
use crate::movement_patterns::BoxedMovementPattern;
use bevy::prelude::{Commands, Component, Res, Time, Timer, Transform, Vec2};
use crate::bullet_patterns::BulletPattern;

#[derive(Component)]
pub struct BulletStream {
    pub bullet_type: BulletType,
    pub num_bullets: usize,
    pub num_iterations: usize,
    pub timer: Timer,
}

impl BulletPattern for BulletStream {

    fn fire(
        &mut self,
        commands: &mut Commands,
        images: &Res<Images>,
        transform: Transform,
        time: &Res<Time>,
        movement_pattern: BoxedMovementPattern,
    ) {
        self.timer.tick(time.delta());
        if self.timer.just_finished() {
            spawn_bullet(commands, &images, BulletSpawner {
                bullet_type: self.bullet_type,
                position: Vec2::new(transform.translation.x, transform.translation.y),
                movement_pattern,
            })
        }
    }
}

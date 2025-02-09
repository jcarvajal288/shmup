use std::f32::consts::PI;
use std::time::Duration;
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
    pub bullets_per_wave: usize,
    pub waves_per_iteration: usize,
    pub num_iterations: usize,
    pub angle: BulletPatternAngle,
    pub speed: f32,
    pub acceleration: f32,

    pub startup_timer: Timer,
    pub wave_timer: Timer,
    pub iteration_timer: Timer,

    pub waves_left: usize,
    pub iterations_left: usize,
}

impl Default for BulletStream {
    fn default() -> Self {
        Self {
            bullet_type: BulletType::WhiteArrow,
            bullets_per_wave: 0,
            waves_per_iteration: 0,
            num_iterations: 0,
            angle: BulletPatternAngle { target: Player, offset: 0.0 },
            speed: 0.0,
            acceleration: 0.0,
            startup_timer: Default::default(),
            wave_timer: Default::default(),
            iteration_timer: Default::default(),
            waves_left: 0,
            iterations_left: 0,
        }
    }
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
        if self.startup_timer.tick(time.delta()).just_finished() {
            self.waves_left = self.waves_per_iteration;
            self.iterations_left = self.num_iterations;
        }
        if !self.startup_timer.finished() { return }

        if self.wave_timer.tick(time.delta()).just_finished() && self.waves_left > 0 {
            self.fire_bullet(commands, &images, &transform, player_transform);
            self.waves_left -= 1;

            if  self.waves_left == 0 && self.iterations_left > 0 {
                self.iterations_left -= 1;
                self.iteration_timer.reset();
            } else {
                self.wave_timer.reset();
            }
        }
        if self.waves_left == 0 && self.iterations_left > 0 && self.iteration_timer.tick(time.delta()).just_finished() {
            self.waves_left = self.waves_per_iteration;
            self.wave_timer.reset();
        }
    }
}

impl BulletStream {

    fn fire_bullet(&mut self, commands: &mut Commands, images: &&Res<Images>, transform: &Transform, player_transform: &Transform) {
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
    }
}

fn get_target_transform(target: &BulletPatternTarget, starting_transform: &Transform, player_transform: &Transform) -> Transform {
    if *target == Player {
        Transform::from_translation(player_transform.translation - starting_transform.translation)
    } else {
        Transform::from_translation(*starting_transform.down())
    }
}

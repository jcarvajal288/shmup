use crate::bullet::{spawn_bullet, BulletSpawner, BulletType};
use crate::bullet_patterns::BulletPatternTarget::Player;
use crate::bullet_patterns::{get_target_transform, BulletPattern, BulletPatternAngle};
use crate::movement_patterns::{BoxedBulletMovementPattern, MovementPatterns};
use crate::resources::sprites::Sprites;
use bevy::prelude::{Commands, Component, Res, Time, Timer, Transform, Vec2};
use std::f32::consts::PI;
use bevy::math::Rot2;

#[derive(Component, Clone)]
pub struct BulletStream {
    pub bullet_type: BulletType,
    pub bullets_per_wave: usize,
    pub waves_per_iteration: usize,
    pub num_iterations: usize,
    pub angle: BulletPatternAngle,

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
            bullets_per_wave: 1,
            waves_per_iteration: 1,
            num_iterations: 1,
            angle: BulletPatternAngle {
                target: Player,
                spread: PI / 2.0,
                offset: 0.0,
            },
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
        sprites: &Res<Sprites>,
        transform: Transform,
        time: &Res<Time>,
        player_transform: &Transform,
        movement_pattern: &mut BoxedBulletMovementPattern,
    ) {
        if self.startup_timer.tick(time.delta()).just_finished() {
            self.waves_left = self.waves_per_iteration;
            self.iterations_left = self.num_iterations;
        }
        if !self.startup_timer.finished() { return }

        if self.wave_timer.tick(time.delta()).just_finished() && self.waves_left > 0 {
            self.fire_wave(commands, sprites, &transform, player_transform, movement_pattern);
            self.waves_left -= 1;

            if self.waves_left == 0 && self.iterations_left > 0 {
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

    fn fire_bullet(&mut self, commands: &mut Commands, sprites: &Res<Sprites>, transform: &Transform, movement_pattern: &mut BoxedBulletMovementPattern) {
        let new_movement_pattern = std::mem::take(movement_pattern);
        spawn_bullet(commands, sprites, BulletSpawner {
            bullet_type: self.bullet_type,
            position: Vec2::new(transform.translation.x, transform.translation.y),
            movement_pattern: MovementPatterns::StraightLine(Rot2::degrees(0.0), 0.0, true), // placeholder for refactor
        });
    }

    fn fire_wave(&mut self, commands: &mut Commands, sprites: &Res<Sprites>, transform: &Transform, player_transform: &Transform, movement_pattern: &mut BoxedBulletMovementPattern) {
        let target = get_target_transform(&self.angle.target, transform, player_transform);
        let firing_angle = target.translation.y.atan2(target.translation.x);

        if self.bullets_per_wave == 1 {
            self.fire_bullet(commands, sprites, transform, movement_pattern);
        } else {
            let step_size = self.angle.spread / (self.bullets_per_wave as f32 - 1.0);
            let angles = (0..self.bullets_per_wave as i32).map(|i: i32| {
                firing_angle - (self.angle.spread / 2.0) + (i as f32 * step_size) + self.angle.offset
            }).collect::<Vec<_>>();

            for _angle in angles {
                self.fire_bullet(commands, sprites, transform, movement_pattern);
            }
        }
    }
}


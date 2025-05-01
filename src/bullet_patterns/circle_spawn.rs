use crate::bullet::{spawn_bullet, BulletSpawner, BulletType};
use crate::bullet_patterns::{get_target_transform, BulletPattern, BulletPatternAngle};
use crate::movement_patterns::BoxedBulletMovementPattern;
use crate::resources::sprites::Sprites;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct CircleSpawn {
    pub bullet_type: BulletType,
    pub bullets_in_circle: u32,
    pub angle: BulletPatternAngle,
    pub spawn_circle_radius: f32,
}

impl BulletPattern for CircleSpawn {
    fn fire(
        &mut self,
        commands: &mut Commands,
        sprites: &Res<Sprites>,
        transform: Transform,
        _time: &Res<Time>,
        player_transform: &Transform,
        movement_pattern: &mut BoxedBulletMovementPattern
    ) {
        self.fire_wave(commands, sprites, &transform, player_transform, movement_pattern);
    }
}

impl CircleSpawn {

    fn fire_wave(
        &self,
        commands: &mut Commands,
        sprites: &Res<Sprites>,
        transform: &Transform,
        player_transform: &Transform,
        movement_pattern: &mut BoxedBulletMovementPattern
    ) {
        let target = get_target_transform(&self.angle.target, transform, player_transform);
        let firing_angle = target.translation.y.atan2(target.translation.x);
        let step_size = if self.bullets_in_circle <= 1 { 0.0 } else { self.angle.spread / (self.bullets_in_circle as f32 - 1.0) };
        let angles = (0..self.bullets_in_circle as i32).map(|i: i32| {
            firing_angle - (self.angle.spread / 2.0) + (i as f32 * step_size) + self.angle.offset
        }).collect::<Vec<_>>();

        for angle in angles {
            self.fire_bullet(commands, sprites, transform, angle, movement_pattern);
        }
    }

    fn fire_bullet(&self, commands: &mut Commands, sprites: &Res<Sprites>, transform: &Transform, angle: f32, movement_pattern: &mut BoxedBulletMovementPattern) {
        let translation_offset = Vec2::new(angle.cos(), angle.sin()) * self.spawn_circle_radius;
        spawn_bullet(commands, sprites, BulletSpawner {
            bullet_type: self.bullet_type,
            position: transform.translation.truncate() + translation_offset,
            movement_pattern: movement_pattern.clone(),
        });
    }
}
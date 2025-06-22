use bevy::math::Rot2;
use bevy::prelude::{EventWriter, ResMut, Transform};
use crate::bullet::{BulletSpawnEvent, BulletType};
use rand::Rng;
use crate::bullet_patterns::Target;
use crate::movement_patterns::straight_line::create_straight_line_pattern;

pub struct Shotgun {
    pub bullets: Vec<BulletType>,
    pub spread: f32,
    pub speed_range: (f32, f32),
}

impl Default for Shotgun {
    fn default() -> Self {
        Self {
            bullets: vec![],
            spread: 0.0,
            speed_range: (0.0, 0.0),
        }
    }
}

impl Shotgun {
    pub fn fire(
        &self,
        origin: &Transform,
        angle: Rot2,
        bullet_spawn_events: &mut EventWriter<BulletSpawnEvent>,
    ) {
        let mut rng = rand::rng();
        for bullet_type in &self.bullets {
            let spread_limit_left = angle.as_radians() - self.spread / 2.0;
            let spread_limit_right = angle.as_radians() + self.spread / 2.0;
            let direction = rng.random_range(spread_limit_left..spread_limit_right);

            let speed = rng.random_range(self.speed_range.0..self.speed_range.1);
            bullet_spawn_events.send(BulletSpawnEvent {
                bullet_type: *bullet_type,
                position: origin.translation.truncate(),
                movement_pattern: create_straight_line_pattern(Rot2::radians(direction), speed),
            });
        }
    }
}
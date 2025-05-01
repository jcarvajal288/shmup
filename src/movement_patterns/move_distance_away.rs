use std::time::Duration;
use bevy::math::Vec3;
use bevy::prelude::{Res, Time, Transform};
use crate::movement_patterns::MovementPattern;

#[derive(Clone)]
pub struct MoveDistanceAway {
    pub name: &'static str,
    pub repulsion_point: Vec3,
    pub velocity: f32,
    pub acceleration: f32,
    pub direction: Vec3,
}

impl Default for MoveDistanceAway {
    fn default() -> Self {
        Self {
            name: Default::default(),
            repulsion_point: Default::default(),
            direction: Vec3::ZERO,
            velocity: 0.0,
            acceleration: 0.0,
        }
    }
}

impl MovementPattern for MoveDistanceAway {
    fn name(&self) -> &str {
        self.name
    }

    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>) {
        if self.direction == Vec3::ZERO {
            self.direction = (transform.translation - self.repulsion_point).normalize().with_z(0.0);
        }
        if !self.is_finished() {
            let delta_time = time.delta_secs();
            self.velocity += self.acceleration * delta_time;
            transform.translation += self.direction * self.velocity * delta_time;
        }
    }

    fn lateral_movement(&mut self) -> f32 {
        (self.direction * self.velocity).x
    }

    fn is_finished(&self) -> bool {
        self.velocity < 0.5
    }
}

pub struct MoveDistanceAwayBuilder {
    pub name: &'static str,
    pub repulsion_point: Vec3,
    pub duration: Duration,
    pub distance: f32,
}

pub fn build_move_distance_away(builder: MoveDistanceAwayBuilder) -> MoveDistanceAway {

    let velocity = builder.distance * 2.0 / builder.duration.as_secs_f32();
    MoveDistanceAway {
        name: builder.name,
        repulsion_point: builder.repulsion_point,
        velocity,
        acceleration: -1.0 * velocity / builder.duration.as_secs_f32(),
        direction: Vec3::ZERO,
    }
}

use std::time::Duration;
use bevy::prelude::*;
use crate::movement_patterns::MovementPattern;

#[derive(Clone)]
pub struct MoveAway {
    pub repulsion_point: Vec3,
    pub velocity: f32,
    pub final_velocity: f32,
    pub acceleration: f32,
    pub direction: Vec3,
    pub is_accelerating: bool,
}

impl MovementPattern for MoveAway {
    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>) -> () {
        if self.direction == Vec3::ZERO {
            self.direction = (transform.translation - self.repulsion_point).normalize().with_z(0.0);
        }
        let delta_time = time.delta_secs();
        if self.is_accelerating && self.velocity < self.final_velocity
        || !self.is_accelerating && self.velocity > self.final_velocity {
            self.velocity += self.acceleration * delta_time;
        }
        transform.translation += self.direction * self.velocity * delta_time;
    }

    fn lateral_movement(&mut self) -> f32 {
        self.direction.x * self.final_velocity
    }

    fn is_finished(&self) -> bool {
        self.velocity == self.final_velocity
    }
}

pub struct MoveAwayBuilder {
    pub repulsion_point: Vec3,
    pub starting_velocity: f32,
    pub final_velocity: f32,
    pub time_to_final_velocity: Duration,
}

pub fn build_move_away(builder: MoveAwayBuilder) -> MoveAway {
    MoveAway {
        repulsion_point: builder.repulsion_point,
        velocity: builder.starting_velocity,
        final_velocity: builder.final_velocity,
        acceleration: (builder.final_velocity - builder.starting_velocity) / builder.time_to_final_velocity.as_secs_f32(),
        direction: Vec3::ZERO,
        is_accelerating: builder.starting_velocity < builder.final_velocity,
    }
}


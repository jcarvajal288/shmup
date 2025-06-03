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
    fn name(&self) -> &str {
        "MoveAway"
    }

    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>, _face_travel: bool) {
        if self.direction == Vec3::ZERO {
            self.direction = (transform.translation - self.repulsion_point).normalize().with_z(0.0);
        }
        let delta_time = time.delta_secs();
        if !self.is_finished() {
            self.velocity += self.acceleration * delta_time;
        }
        transform.translation += self.direction * self.velocity * delta_time;
    }

    fn lateral_movement(&self) -> f32 {
        (self.direction * self.velocity).x
    }

    fn is_finished(&self) -> bool {
        if self.is_accelerating {
            self.velocity >= self.final_velocity
        } else {
            self.velocity <= self.final_velocity
        }
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


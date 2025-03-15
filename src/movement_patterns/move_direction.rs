use std::time::Duration;
use bevy::prelude::*;
use crate::movement_patterns::MovementPattern;

#[derive(Clone)]
pub struct MoveDirection {
    pub direction: Vec3,
    pub velocity: f32,
    pub final_velocity: f32,
    pub acceleration: f32,
}

impl MovementPattern for MoveDirection {
    fn name(&self) -> &str {
        "MoveDirection"
    }

    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>) -> () {
        let delta_time = time.delta_secs();
        self.velocity += self.acceleration * delta_time;
        transform.translation += self.direction * self.velocity * delta_time;
    }

    fn lateral_movement(&mut self) -> f32 {
        self.direction.x * self.final_velocity
    }

    fn is_finished(&self) -> bool {
        self.velocity == self.final_velocity
    }
}
pub struct MoveDirectionBuilder {
    pub direction: Rot2,
    pub starting_velocity: f32,
    pub final_velocity: f32,
    pub time_to_decelerate: Duration,
}

pub fn build_move_direction(builder: MoveDirectionBuilder) -> MoveDirection {
    MoveDirection {
        direction: Vec3::new(builder.direction.cos, builder.direction.sin, 0.0),
        velocity: builder.starting_velocity,
        final_velocity: builder.final_velocity,
        acceleration: (builder.final_velocity - builder.starting_velocity) / builder.time_to_decelerate.as_secs_f32(),
    }
}


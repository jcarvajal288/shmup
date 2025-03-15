use crate::movement_patterns::MovementPattern;
use bevy::math::Vec2;
use bevy::prelude::{Res, Time, Transform, Vec3};

#[derive(Clone)]
pub struct MoveTo {
    pub direction: Vec3,
    pub velocity: f32,
    pub acceleration: f32,
    pub duration: f32,
    pub elapsed_time: f32,
}

impl Default for MoveTo {
    fn default() -> Self {
        Self {
            direction: Vec3::ZERO,
            velocity: 0.0,
            acceleration: 0.0,
            duration: 0.0,
            elapsed_time: 0.0,
        }
    }
}

impl MovementPattern for MoveTo {
    fn name(&self) -> &str { "MoveTo" }

    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>) -> () {
        let delta_time = time.delta_secs();
        if self.elapsed_time > self.duration {
            return;
        }
        self.velocity += self.acceleration * delta_time;
        transform.translation += self.direction * self.velocity * delta_time;
        self.elapsed_time += delta_time;
    }

    fn lateral_movement(&mut self) -> f32 {
        self.direction.x * self.velocity
    }

    fn is_finished(&self) -> bool {
         self.elapsed_time > self.duration
    }
}

pub struct MoveToBuilder {
    pub start: Vec2,
    pub destination: Vec2,
    pub time: f32,
}

pub fn build_move_to(builder: MoveToBuilder) -> MoveTo {
    let displacement = builder.destination - builder.start;
    let distance = displacement.length();
    let direction = displacement.normalize().extend(0.0);
    let velocity = distance * 2.0 / builder.time;
    let duration = builder.time;
    MoveTo {
        direction,
        velocity,
        acceleration: -velocity / builder.time,
        duration,
        elapsed_time: 0.0,
    }
}

fn find_accel_to_stop_at_destination(start: Vec2, dest: Vec2, speed: f32, time: f32) -> Vec2 {
    let displacement = dest - start;
    let velocity = displacement.normalize() * speed;
    let acceleration = (2.0 * (displacement - (velocity * time))) / (time * time);
    acceleration
}
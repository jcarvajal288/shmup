use crate::movement_patterns::MovementPattern;
use bevy::math::Vec2;
use bevy::prelude::{Res, Time, Transform, Vec3};
use crate::movement_patterns;

#[derive(Clone, PartialEq)]
pub struct MoveTo {
    pub destination: Vec2,
    pub current_speed: f32,
    pub acceleration: f32,
}

impl MoveTo {
    pub fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>, face_travel: bool) {
        let diff = self.destination - transform.translation.truncate();
        let angle = diff.y.atan2(diff.x);
        let direction = Vec3::new(f32::cos(angle), f32::sin(angle), 0.0);
        let distance = self.current_speed * time.delta_secs();
        let translation_delta = direction * distance;
        transform.translation += translation_delta;
        self.current_speed += self.acceleration;
        if face_travel {
            movement_patterns::face_travel_direction(transform, direction);
        }
    }
}


#[derive(Clone)]
pub struct MoveToOld {
    pub direction: Vec3,
    pub velocity: f32,
    pub acceleration: f32,
    pub duration: f32,
    pub elapsed_time: f32,
}

impl Default for MoveToOld {
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

impl MovementPattern for MoveToOld {
    fn name(&self) -> &str { "MoveTo" }

    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>) {
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

pub fn build_move_to(builder: MoveToBuilder) -> MoveToOld {
    let displacement = builder.destination - builder.start;
    let distance = displacement.length();
    let direction = displacement.normalize().extend(0.0);
    let velocity = distance * 2.0 / builder.time;
    let duration = builder.time;
    MoveToOld {
        direction,
        velocity,
        acceleration: -velocity / builder.time,
        duration,
        elapsed_time: 0.0,
    }
}

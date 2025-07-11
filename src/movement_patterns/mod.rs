pub mod sine_wave;
pub mod decelerate;
pub mod straight_line;
pub mod curved_line;

use crate::movement_patterns::decelerate::Decelerate;
use crate::movement_patterns::sine_wave::SineWave;
use crate::movement_patterns::straight_line::StraightLine;
use crate::movement_patterns::MovementPatterns::{DeceleratePattern, DontMovePattern, SineWavePattern, StraightLinePattern, CurvedLinePattern};
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Component, Res, Time, Transform};
use std::f32::consts::PI;
use crate::movement_patterns::curved_line::CurvedLine;

#[derive(Component, Clone, PartialEq)]
pub enum MovementPatterns {
    DontMovePattern(DontMove),
    StraightLinePattern(StraightLine),
    DeceleratePattern(Decelerate),
    SineWavePattern(SineWave),
    CurvedLinePattern(CurvedLine)
}

pub fn run_movement_pattern(movement_pattern: &mut MovementPatterns, transform: &mut Transform, time: &Res<Time>, face_travel_direction: bool) {
    match movement_pattern {
        StraightLinePattern(straight_line) => {
            straight_line.do_move(transform, time, face_travel_direction)
        }
        DeceleratePattern(decelerate) => {
            decelerate.do_move(transform, time, face_travel_direction)
        }
        SineWavePattern(sine_wave) => {
            sine_wave.do_move(transform, time, face_travel_direction)
        }
        CurvedLinePattern(curved_line) => {
            curved_line.do_move(transform, time, face_travel_direction)
        }
        DontMovePattern(_dont_move) => {}
    }
}

pub fn get_lateral_movement(movement_pattern: &MovementPatterns) -> f32 {
    match movement_pattern {
        DontMovePattern(pattern) => { pattern.lateral_movement() }
        StraightLinePattern(pattern) => { pattern.lateral_movement() }
        DeceleratePattern(pattern) => { pattern.lateral_movement() }
        SineWavePattern(pattern) => { pattern.lateral_movement() }
        CurvedLinePattern(pattern) => { pattern.lateral_movement() }
    }
}

pub fn is_finished(movement_pattern: &MovementPatterns) -> bool {
    match movement_pattern {
        DontMovePattern(pattern) => { pattern.is_finished() }
        StraightLinePattern(pattern) => { pattern.is_finished() }
        DeceleratePattern(pattern) => { pattern.is_finished() }
        SineWavePattern(pattern) => { pattern.is_finished() }
        CurvedLinePattern(pattern) => { pattern.is_finished() }
    }
}

pub trait MovementPattern {
    fn name(&self) -> &str;

    fn do_move(&mut self, transform: &mut Transform, time: &Res<Time>, face_travel: bool);

    fn lateral_movement(&self) -> f32;

    fn is_finished(&self) -> bool;
}


#[derive(Clone, PartialEq)]
#[derive(Default)]
pub struct DontMove;

impl MovementPattern for DontMove {
    fn name(&self) -> &str { "DontMove" }

    fn do_move(&mut self, _: &mut Transform, _: &Res<Time>, _: bool) {}
    fn lateral_movement(&self) -> f32 { 0.0 }
    fn is_finished(&self) -> bool { true }
}

pub fn face_travel_direction(transform: &mut Transform, direction: Vec3) {
    let angle = direction.y.atan2(direction.x);
    transform.rotation = Quat::from_axis_angle(Vec3::Z, angle + (-PI / 2.0));
}

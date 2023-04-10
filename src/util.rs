use crate::util::RotationDirection::*;
use std::f32::consts::PI;

pub const STEPS_PER_ROTATION: usize = 100;
pub const ROTATION_SPEED: f32 = 0.5 * PI / STEPS_PER_ROTATION as f32;

#[derive(Copy, Clone)]
pub enum RotationDirection {
    Right,
    Left,
}

impl RotationDirection {
    pub fn jump_offset(self) -> usize {
        match self {
            Right => 3,
            Left => 1,
        }
    }
}

#[derive(Copy, Clone)]
pub enum RotationAxis {
    XAxis,
    YAxis,
    ZAxis,
}

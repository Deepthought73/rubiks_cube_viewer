use crate::util::RotationDirection::*;
use std::f32::consts::PI;
use strum_macros::EnumIter;

pub const STEPS_PER_ROTATION: usize = 20;
pub const ROTATION_SPEED: f32 = 0.5 * PI / STEPS_PER_ROTATION as f32;

#[derive(Copy, Clone, PartialEq, PartialOrd, EnumIter)]
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

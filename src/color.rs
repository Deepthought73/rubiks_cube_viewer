use crate::color::Color::*;
use crate::util::RotationAxis::*;
use crate::util::{RotationAxis, ROTATION_SPEED};
use std::f32::consts::PI;
use std::fmt::{Debug, Formatter};
use strum_macros::EnumIter;

/** Color Faces
*
*  Top:    White
*  Bottom: Yellow
*  Front:  Red
*  Back:   Orange
*  Right:  Blue
*  Left:   Green
*
 */

#[derive(Copy, Clone, EnumIter, PartialEq, PartialOrd)]
pub enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}

impl Color {
    pub fn neighbor_order(self) -> [Self; 4] {
        match self {
            White => [Orange, Blue, Red, Green],
            Yellow => [Red, Blue, Orange, Green],
            Red => [White, Blue, Yellow, Green],
            Orange => [White, Green, Yellow, Blue],
            Blue => [White, Orange, Yellow, Red],
            Green => [White, Red, Yellow, Orange],
        }
    }

    pub fn neighbor_pos(self, side: Color) -> usize {
        self.neighbor_order()
            .iter()
            .position(|it| *it == side)
            .unwrap()
    }

    pub fn rgb(self) -> (f32, f32, f32) {
        match self {
            White => (1.0, 1.0, 1.0),
            Yellow => (1.0, 1.0, 0.0),
            Red => (1.0, 0.0, 0.0),
            Orange => (1.0, 0.5, 0.2),
            Blue => (0.0, 0.0, 1.0),
            Green => (0.0, 1.0, 0.0),
        }
    }

    pub fn rotation_and_angle(self) -> (RotationAxis, f32) {
        match self {
            White => (ZAxis, -0.5 * PI),
            Yellow => (ZAxis, 0.5 * PI),
            Red => (ZAxis, 0.0),
            Orange => (YAxis, PI),
            Blue => (YAxis, 0.5 * PI),
            Green => (YAxis, -0.5 * PI),
        }
    }

    fn rotation_axis(self) -> RotationAxis {
        match self {
            White | Yellow => YAxis,
            Red | Orange => XAxis,
            Blue | Green => ZAxis,
        }
    }

    fn rotation_speed(self) -> f32 {
        match self {
            White | Orange | Blue => -ROTATION_SPEED,
            _ => ROTATION_SPEED,
        }
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                White => 'W',
                Yellow => 'Y',
                Red => 'R',
                Orange => 'O',
                Blue => 'B',
                Green => 'G',
            }
        )
    }
}

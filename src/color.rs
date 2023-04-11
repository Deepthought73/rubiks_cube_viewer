use crate::color::Color::*;
use crate::util::RotationDirection::Left;
use crate::util::{RotationDirection, ROTATION_SPEED};
use kiss3d::nalgebra::{OVector, Unit, Vector, U3};
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
            Yellow => (247.0 / 255.0, 181.0 / 255.0, 25.0 / 255.0),
            Red => (214.0 / 255.0, 8.0 / 255.0, 4.0 / 255.0),
            Orange => (247.0 / 255.0, 106.0 / 255.0, 25.0 / 255.0),
            Blue => (0.0 / 255.0, 65.0 / 255.0, 145.0 / 255.0),
            Green => (25.0 / 255.0, 138.0 / 255.0, 45.0 / 255.0),
        }
    }

    pub fn rotation_and_angle(self) -> (Unit<OVector<f32, U3>>, f32) {
        match self {
            White => (Vector::z_axis(), -0.5 * PI),
            Yellow => (Vector::z_axis(), 0.5 * PI),
            Red => (Vector::y_axis(), 0.0),
            Orange => (Vector::y_axis(), PI),
            Blue => (Vector::y_axis(), 0.5 * PI),
            Green => (Vector::y_axis(), -0.5 * PI),
        }
    }

    pub fn rotation_axis(self) -> Unit<OVector<f32, U3>> {
        match self {
            Red | Orange => Vector::x_axis(),
            White | Yellow => Vector::y_axis(),
            Blue | Green => Vector::z_axis(),
        }
    }

    pub fn rotation_speed(self, direction: RotationDirection) -> f32 {
        (match self {
            White | Orange | Blue => -ROTATION_SPEED,
            _ => ROTATION_SPEED,
        }) * (if direction == Left { -1.0 } else { 1.0 })
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

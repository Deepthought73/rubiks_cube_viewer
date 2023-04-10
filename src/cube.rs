use crate::color::Color;
use crate::color::Color::{White, Yellow};
use crate::side::Side;
use crate::util::RotationDirection;
use std::fmt::{Debug, Formatter};
use strum::IntoEnumIterator;

pub struct Cube {
    sides: Vec<Side>,
}

impl Cube {
    pub fn new() -> Self {
        let sides = Color::iter().map(|c| Side::new(c)).collect();
        Self { sides }
    }

    pub fn side(&self, color: Color) -> &Side {
        &self.sides[color as usize]
    }

    pub fn side_mut(&mut self, color: Color) -> &mut Side {
        &mut self.sides[color as usize]
    }

    pub fn rotate(&mut self, side: Color, direction: RotationDirection) {
        let surrounding_rows = side.neighbor_order().map(|n| self.side(n).get_row(side));
        for (i, n) in side.neighbor_order().iter().enumerate() {
            self.side_mut(*n)
                .set_row(side, surrounding_rows[(i + direction.jump_offset()) % 4]);
        }
        self.side_mut(side).rotate(direction);
    }
}

impl Debug for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}\n\n", self.side(White))?;

        let mut rows = ["".to_string(), "".to_string(), "".to_string()];
        for side in Yellow.neighbor_order().map(|c| self.side(c).string_array()) {
            for i in 0..3 {
                rows[i] += &side[i];
                rows[i] += "    ";
            }
        }
        write!(f, "{}\n{}\n{}\n\n", rows[0], rows[1], rows[2])?;

        write!(f, "{:?}\n", self.side(Yellow))?;

        Ok(())
    }
}

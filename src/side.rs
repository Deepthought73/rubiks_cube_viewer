use crate::color::Color;
use crate::util::RotationDirection;
use std::fmt::{Debug, Formatter};

/** Tiles
*  0  1  2
*  7     3
*  6  5  4
 */

#[derive(Clone)]
pub struct Side {
    color: Color,
    tiles: [Color; 8],
    rotation_offset: usize,
}

impl Side {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            tiles: [color; 8],
            rotation_offset: 0,
        }
    }

    pub fn rotate(&mut self, direction: RotationDirection) {
        self.rotation_offset = (self.rotation_offset + direction.jump_offset() * 2) % 8;
    }

    pub fn get_row(&self, side: Color) -> [Color; 3] {
        let pos = self.rotation_offset + self.color.neighbor_pos(side) * 2;
        [
            self.tiles[pos % 8],
            self.tiles[(pos + 1) % 8],
            self.tiles[(pos + 2) % 8],
        ]
    }

    pub fn set_row(&mut self, side: Color, row: [Color; 3]) {
        let pos = self.rotation_offset + self.color.neighbor_pos(side) * 2;
        self.tiles[pos % 8] = row[0];
        self.tiles[(pos + 1) % 8] = row[1];
        self.tiles[(pos + 2) % 8] = row[2];
    }

    /*
    fn render_top_rotation(&mut self, rotation: &UnitQuaternion<f32>) {
        if let Some(cubes) = &mut self.cubes {
            for tile in self.tiles[0] {
                let (i, j) = tile.cube_position;
                cubes[i][j].append_rotation(rotation);
            }
        }
    }

    fn render_rotation(&mut self, rotation: &UnitQuaternion<f32>) {
        if let Some(cubes) = &mut self.cubes {
            for tile in self.tiles.iter().flatten() {
                let (i, j) = tile.cube_position;
                cubes[i][j].append_rotation(rotation);
            }
        }
    }*/

    pub fn normalized_tiles(&self) -> [Color; 8] {
        let mut ret = self.tiles;
        ret.rotate_left(self.rotation_offset);
        ret
    }

    pub fn string_array(&self) -> [String; 3] {
        let sides = self.color.neighbor_order().map(|n| self.get_row(n));
        [
            format!("{:?}  {:?}  {:?}", sides[0][0], sides[0][1], sides[0][2]),
            format!("{:?}  {:?}  {:?}", sides[3][1], self.color, sides[1][1]),
            format!("{:?}  {:?}  {:?}", sides[2][2], sides[2][1], sides[2][0]),
        ]
    }
}

impl Debug for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string_array().join("\n"))
    }
}

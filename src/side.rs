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

    /*
        fn build_cubes(color: Color, window: &mut Window) -> [[SceneNode; 3]; 3] {
            let (r, g, b) = color.rgb();
            [-1, 0, 1].map(|y| {
                [-1, 0, 1].map(|z| {
                    let mut cube = window.add_cube(0.0005, 0.0995, 0.0995);
                    cube.set_local_translation(Translation3::new(
                        -0.15,
                        -0.1 * y as f32,
                        0.1 * z as f32,
                    ));
                    cube.set_lines_width(10.0);
                    let (rotation_axis, angle) = color.rotation_and_angle();
                    let rotation_axis = match rotation_axis {
                        XAxis => Vector::x_axis(),
                        YAxis => Vector::y_axis(),
                        ZAxis => Vector::z_axis(),
                    };
                    let rotation = UnitQuaternion::from_axis_angle(&rotation_axis, angle);
                    cube.append_rotation(&rotation);
                    cube.set_color(r, g, b);
                    cube
                })
            })
        }
    */

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
    fn orientate_right(&mut self) {
        let tmp = self.tiles[1][0];
        self.tiles[1][0] = self.tiles[2][1];
        self.tiles[2][1] = self.tiles[1][2];
        self.tiles[1][2] = self.tiles[0][1];
        self.tiles[0][1] = tmp;

        let tmp = self.tiles[0][0];
        self.tiles[0][0] = self.tiles[2][0];
        self.tiles[2][0] = self.tiles[2][2];
        self.tiles[2][2] = self.tiles[0][2];
        self.tiles[0][2] = tmp;

        self.rotation_offset = (self.rotation_offset + 3) % 4;
        self.top = self.color.neighbor_order()[self.rotation_offset];
    }

    fn orientate_to(&mut self, new_top: Color) {
        while self.top != new_top {
            self.orientate_right()
        }
    }

    fn get_top_colors(&self) -> [Color; 3] {
        self.tiles[0].map(|tile| tile.color)
    }

    fn set_top_colors(&mut self, row: [Color; 3]) {
        for (tile, color) in self.tiles[0].iter_mut().zip(row.iter()) {
            tile.color = *color;
            if let Some(cubes) = &mut self.cubes {
                let (i, j) = tile.cube_position;
                let (r, g, b) = color.rgb();
                cubes[i][j].set_color(r, g, b);
            }
        }
    }

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

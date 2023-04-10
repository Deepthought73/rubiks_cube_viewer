extern crate kiss3d;

use kiss3d::light::Light;
use kiss3d::window::Window;
use std::f32::consts::PI;

use crate::Color::*;
use crate::RotationAxis::*;
use itertools::Itertools;
use kiss3d::camera::ArcBall;
use kiss3d::nalgebra::{
    ArrayStorage, Const, OVector, Point3, RealField, Translation3, Unit, UnitQuaternion, Vector,
    Vector3,
};
use kiss3d::ncollide3d::math::Matrix;
use kiss3d::scene::SceneNode;
use kiss3d::window;
use std::fmt::{write, Debug, Formatter};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/** Axis orientation
 *
 *    y axis
 *      |
 *      |         x axis
 *      |          /
 *      |       /
 *      |    /
 *      | /
 *      x------------------ z axis
 *
 */

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

const STEPS_PER_ROTATION: usize = 100;
const ROTATION_SPEED: f32 = 0.5 * PI / STEPS_PER_ROTATION as f32;

#[derive(Copy, Clone)]
enum RotationAxis {
    XAxis,
    YAxis,
    ZAxis,
}

#[derive(Copy, Clone, EnumIter, PartialEq, PartialOrd)]
enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}

impl Color {
    fn default_top(self) -> Self {
        self.neighbor_order()[0]
    }

    fn neighbor_order(self) -> [Self; 4] {
        match self {
            White => [Orange, Green, Red, Blue],
            Yellow => [Red, Green, Orange, Blue],
            Red => [White, Blue, Yellow, Green],
            Orange => [White, Green, Yellow, Blue],
            Blue => [White, Orange, Yellow, Red],
            Green => [White, Red, Yellow, Orange],
        }
    }

    fn rgb(self) -> (f32, f32, f32) {
        match self {
            White => (1.0, 1.0, 1.0),
            Yellow => (1.0, 1.0, 0.0),
            Red => (1.0, 0.0, 0.0),
            Orange => (1.0, 0.5, 0.2),
            Blue => (0.0, 0.0, 1.0),
            Green => (0.0, 1.0, 0.0),
        }
    }

    fn rotation_and_angle(self) -> (RotationAxis, f32) {
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

#[derive(Copy, Clone)]
enum RotationDirection {
    Right,
    Left,
}

#[derive(Copy, Clone)]
struct Tile {
    color: Color,
    cube_position: (usize, usize),
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.color)
    }
}

#[derive(Clone)]
struct Side {
    color: Color,
    tiles: [[Tile; 3]; 3],
    cubes: Option<[[SceneNode; 3]; 3]>,
    top: Color,
    rotation_offset: usize,
}

impl Side {
    fn new(color: Color, window: Option<&mut Window>) -> Self {
        Self {
            color,
            tiles: [0, 1, 2].map(|i| {
                (if color == White || color == Yellow {
                    [2, 1, 0]
                } else {
                    [0, 1, 2]
                })
                .map(|j| Tile {
                    color,
                    cube_position: (i, j),
                })
            }),
            cubes: window.map(|window| Self::build_cubes(color, window)),
            top: color.default_top(),
            rotation_offset: 0,
        }
    }

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

    fn rotate_right(&mut self) {
        self.rotation_offset = (self.rotation_offset + 1) % 4;
        self.top = self.color.neighbor_order()[self.rotation_offset];
    }

    fn rotate_left(&mut self) {
        self.rotation_offset = (self.rotation_offset + 3) % 4;
        self.top = self.color.neighbor_order()[self.rotation_offset];
    }

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
    }

    fn string_array(&self) -> [String; 3] {
        self.tiles.map(|row| {
            row.iter()
                .map(|c| format!("{c:?}"))
                .intersperse("  ".to_string())
                .collect::<String>()
        })
    }
}

impl Debug for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Current top: {:?}\n{}",
            self.top,
            self.string_array().join("\n")
        )
    }
}

struct Cube {
    sides: Vec<Side>,
}

impl Cube {
    fn new(window: &mut Window) -> Self {
        let sides = Color::iter().map(|c| Side::new(c, Some(window))).collect();
        Self { sides }
    }

    fn side(&self, color: Color) -> &Side {
        &self.sides[color as usize]
    }

    fn side_mut(&mut self, color: Color) -> &mut Side {
        &mut self.sides[color as usize]
    }

    fn orientate_neighbors_to(&mut self, side: Color) {
        for n in side.neighbor_order() {
            self.side_mut(n).orientate_to(side);
        }
    }

    // TODO fix White rotate_right = rotate_left
    fn rotate_right(&mut self, side: Color) {
        self.orientate_neighbors_to(side);
        let top_rows = side
            .neighbor_order()
            .map(|n| self.side_mut(n).get_top_colors());
        for (i, n) in side.neighbor_order().iter().enumerate() {
            let next_index = if side == White || side == Yellow {
                (i + 1) % 4
            } else {
                (i + 3) % 4
            };
            self.side_mut(*n).set_top_colors(top_rows[next_index])
        }
        self.side_mut(side).rotate_right();
    }

    fn rotate_left(&mut self, side: Color) {
        (0..3).for_each(|_| self.rotate_right(side))
    }

    fn render_rotation(&mut self, side: Color, angle: f32) {
        let axis = side.rotation_axis();
        let axis = match axis {
            XAxis => Vector::x_axis(),
            YAxis => Vector::y_axis(),
            ZAxis => Vector::z_axis(),
        };
        let rotation = UnitQuaternion::from_axis_angle(&axis, angle);
        self.orientate_neighbors_to(side);
        for n in side.neighbor_order() {
            self.side_mut(n).render_top_rotation(&rotation);
        }
        self.side_mut(side).render_rotation(&rotation);
    }
}

#[derive(Copy, Clone)]
struct RotationAnimation {
    side: Color,
    direction: RotationDirection,
    step: usize,
}

impl RotationAnimation {
    pub fn new(side: Color, direction: RotationDirection) -> Self {
        Self {
            side,
            direction,
            step: 0,
        }
    }
}

struct CubeWindow {
    window: Window,
    cube: Cube,
    rotation_queue: Vec<RotationAnimation>,
    current_rotation: Option<RotationAnimation>,
}

impl CubeWindow {
    pub fn new() -> Self {
        let mut window = Window::new("Cube window");
        window.set_light(Light::StickToCamera);
        let cube = Cube::new(&mut window);

        Self {
            window,
            cube,
            rotation_queue: vec![
                RotationAnimation::new(Blue, RotationDirection::Right),
                RotationAnimation::new(White, RotationDirection::Right),
                // RotationAnimation::new(Red, RotationDirection::Right),
                // RotationAnimation::new(Green, RotationDirection::Right),
                // RotationAnimation::new(White, RotationDirection::Right),
                // RotationAnimation::new(Blue, RotationDirection::Right),
                // RotationAnimation::new(Orange, RotationDirection::Right),
                // RotationAnimation::new(Green, RotationDirection::Right),
                // RotationAnimation::new(Yellow, RotationDirection::Right),
                // RotationAnimation::new(Green, RotationDirection::Right),
            ],
            current_rotation: None,
        }
    }

    fn render(&mut self) {
        self.add_coordinate_system();

        let eye = Point3::from_slice(&[-1.0, 0.0, 0.0]);
        let at = Point3::from_slice(&[0.0, 0.0, 0.0]);
        let mut camera = ArcBall::new(eye, at);

        while self.window.render_with(Some(&mut camera), None, None) {
            camera.set_at(camera.at() * 0.96);

            if self.current_rotation.is_none() && self.rotation_queue.is_empty() {
                continue;
            }

            if let Some(rotation) = &mut self.current_rotation {
                let speed = rotation.side.rotation_speed();
                if rotation.step == STEPS_PER_ROTATION {
                    self.cube
                        .render_rotation(rotation.side, -speed * STEPS_PER_ROTATION as f32);
                    self.cube.rotate_right(rotation.side);
                    self.current_rotation = None;
                } else {
                    rotation.step += 1;
                    self.cube.render_rotation(rotation.side, speed);
                }
            } else {
                self.current_rotation = Some(self.rotation_queue.remove(0));
            }
        }
    }

    fn add_coordinate_system(&mut self) {
        println!("=========[ Coordinate System ]=========");
        println!("X: Red");
        println!("Y: Green");
        println!("Z: Blue");
        println!("=======================================");

        let mut coord_origin = self.window.add_cube(0.02, 0.02, 0.02);
        coord_origin.set_color(1.0, 1.0, 1.0);
        coord_origin.set_local_translation(Translation3::new(-0.5, -0.5, -0.5));
        let mut x_scale = self.window.add_cube(1.0, 0.01, 0.01);
        x_scale.set_color(1.0, 0.0, 0.0);
        x_scale.set_local_translation(Translation3::new(0.0, -0.5, -0.5));
        let mut y_scale = self.window.add_cube(0.01, 1.0, 0.01);
        y_scale.set_color(0.0, 1.0, 0.0);
        y_scale.set_local_translation(Translation3::new(-0.5, 0.0, -0.5));
        let mut z_scale = self.window.add_cube(0.01, 0.01, 1.0);
        z_scale.set_color(0.0, 0.0, 1.0);
        z_scale.set_local_translation(Translation3::new(-0.5, -0.5, 0.0));
    }
}

impl Debug for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut red = self.side(Red).clone();
        red.orientate_to(Yellow);
        write!(
            f,
            "{}\n\n",
            red.string_array()
                .map(|it| "           ".to_string() + &it)
                .join("\n")
        )?;

        let mut blue = self.side(Blue).clone();
        blue.orientate_to(Red);
        let blue = blue.string_array();

        let mut white = self.side(White).clone();
        white.orientate_to(Red);
        let white = white.string_array();

        let mut green = self.side(Green).clone();
        green.orientate_to(Red);
        let green = green.string_array();

        for i in 0..3 {
            write!(f, "{}    {}    {}\n", blue[i], white[i], green[i])?;
        }

        let mut orange = self.side(Orange).clone();
        orange.orientate_to(White);
        write!(
            f,
            "\n{}\n\n",
            orange
                .string_array()
                .map(|it| "           ".to_string() + &it)
                .join("\n")
        )?;

        let mut yellow = self.side(Yellow).clone();
        yellow.orientate_to(Orange);
        write!(
            f,
            "{}\n\n",
            yellow
                .string_array()
                .map(|it| "           ".to_string() + &it)
                .join("\n")
        )?;

        Ok(())
    }
}

fn main() {
    //let mut cube_window = CubeWindow::new();
    //cube_window.render();

    let mut s = Side::new(Orange, None);

    s.tiles[0][0].color = White;
    s.tiles[2][2].color = Yellow;

    s.rotate_right();
    s.rotate_right();
    s.orientate_to(White);

    println!("{:?}", s);
}

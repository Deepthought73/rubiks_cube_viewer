use crate::color::Color;
use crate::cube::Cube;
use crate::util::{RotationDirection, STEPS_PER_ROTATION};
use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point3, Translation3, UnitQuaternion, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use rand::prelude::SliceRandom;
use std::path::Path;
use strum::IntoEnumIterator;

struct Tile {
    edge: SceneNode,
    center: SceneNode,
}

impl Tile {
    fn new(color: Color, y: i32, z: i32, window: &mut Window) -> Self {
        let mut edge = window.add_obj(
            Path::new("res/edge.obj"),
            Path::new("res"),
            Vector3::new(0.05, 0.05, 0.05),
        );
        let mut center = window.add_obj(
            Path::new("res/center.obj"),
            Path::new("res"),
            Vector3::new(0.05, 0.05, 0.05),
        );

        let translation = Translation3::new(-0.1, -0.1 * y as f32, 0.1 * z as f32);
        center.set_local_translation(translation);
        edge.set_local_translation(translation);

        let (rotation_axis, angle) = color.rotation_and_angle();
        let rotation = UnitQuaternion::from_axis_angle(&rotation_axis, angle);
        center.append_rotation(&rotation);
        edge.append_rotation(&rotation);

        let (r, g, b) = color.rgb();
        center.set_color(r, g, b);

        Self { edge, center }
    }

    fn append_rotation(&mut self, rotation: &UnitQuaternion<f32>) {
        self.edge.append_rotation(rotation);
        self.center.append_rotation(rotation);
    }

    fn set_color(&mut self, color: &Color) {
        let (r, g, b) = color.rgb();
        self.center.set_color(r, g, b);
    }
}

#[derive(Copy, Clone)]
struct RotationAnimation {
    side: Color,
    direction: RotationDirection,
    step: usize,
}

pub struct CubeWindow {
    window: Window,
    cube: Cube,
    sides: Vec<[Tile; 8]>,
    centers: Vec<Tile>,
    rotation_queue: Vec<RotationAnimation>,
}

impl CubeWindow {
    pub fn new() -> Self {
        let mut window = Window::new("Cube Window");
        window.set_light(Light::StickToCamera);
        // window.set_background_color(0.7, 0.7, 0.7);

        // let mut w = window.add_cube(1.0, 5.0, 5.0);
        // w.set_color(1.0, 1.0, 1.0);
        // w.set_local_translation(Translation3::new(-2.5, 0.0, 0.0));
        // let mut w = window.add_cube(1.0, 5.0, 5.0);
        // w.set_color(1.0, 1.0, 1.0);
        // w.set_local_translation(Translation3::new(2.5, 0.0, 0.0));
        // let mut w = window.add_cube(5.0, 1.0, 5.0);
        // w.set_color(1.0, 1.0, 1.0);
        // w.set_local_translation(Translation3::new(0.0, 2.5, 0.0));
        // let mut w = window.add_cube(5.0, 1.0, 5.0);
        // w.set_color(1.0, 1.0, 1.0);
        // w.set_local_translation(Translation3::new(0.0, -2.5, 0.0));
        // let mut w = window.add_cube(5.0, 5.0, 1.0);
        // w.set_color(1.0, 1.0, 1.0);
        // w.set_local_translation(Translation3::new(0.0, 0.0, 2.5));
        // let mut w = window.add_cube(5.0, 5.0, 1.0);
        // w.set_color(1.0, 1.0, 1.0);
        // w.set_local_translation(Translation3::new(0.0, 0.0, -2.5));

        let cube = Cube::new();
        let sides = Color::iter()
            .map(|c| Self::build_tiles(c, &mut window))
            .collect();
        let centers = Color::iter()
            .map(|c| Tile::new(c, 0, 0, &mut window))
            .collect();

        let mut rng = rand::thread_rng();

        let rotation_queue = (0..1000)
            .map(|_| RotationAnimation {
                side: Color::iter()
                    .collect::<Vec<_>>()
                    .choose(&mut rng)
                    .unwrap()
                    .clone(),
                direction: RotationDirection::iter()
                    .collect::<Vec<_>>()
                    .choose(&mut rng)
                    .unwrap()
                    .clone(),
                step: 0,
            })
            .collect();

        Self {
            window,
            cube,
            sides,
            centers,
            rotation_queue,
        }
    }

    fn build_tiles(color: Color, window: &mut Window) -> [Tile; 8] {
        [
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
        ]
        .map(|(z, y)| Tile::new(color, y, z, window))
    }

    fn rotate(&mut self, side: Color, direction: RotationDirection) {
        self.cube.rotate(side, direction);
        self.synchronize_colors(side);
        for n in side.neighbor_order() {
            self.synchronize_colors(n)
        }
    }

    fn synchronize_colors(&mut self, side: Color) {
        let colors = self.cube.side(side).normalized_tiles();
        for (tile, color) in self.sides[side as usize].iter_mut().zip(colors.iter()) {
            tile.set_color(color);
        }
    }

    pub fn render(&mut self) {
        let eye = Point3::from_slice(&[-1.0, 0.0, 0.0]);
        let at = Point3::from_slice(&[0.0, 0.0, 0.0]);
        let mut camera = ArcBall::new(eye, at);

        while self.window.render_with(Some(&mut camera), None, None) {
            camera.set_at(camera.at() * 0.9);

            camera.set_yaw(camera.yaw() + 0.01);

            if let Some(rotation) = self.rotation_queue.first().cloned() {
                let speed = rotation.side.rotation_speed(rotation.direction);
                if rotation.step == STEPS_PER_ROTATION {
                    self.append_rotation(rotation.side, -speed * STEPS_PER_ROTATION as f32);
                    self.rotate(rotation.side, rotation.direction);
                    self.rotation_queue.remove(0);
                } else {
                    self.append_rotation(rotation.side, speed);
                    self.rotation_queue
                        .first_mut()
                        .map(|rotation| rotation.step += 1);
                }
            }
        }
    }

    fn append_rotation(&mut self, side: Color, angle: f32) {
        let axis = side.rotation_axis();
        let rotation = UnitQuaternion::from_axis_angle(&axis, angle);

        for tile in self.sides[side as usize].iter_mut() {
            tile.append_rotation(&rotation)
        }
        self.centers[side as usize].append_rotation(&rotation);

        for n in side.neighbor_order() {
            let s = &mut self.sides[n as usize];
            for i in 0..3 {
                s[(n.neighbor_pos(side) * 2 + i) % 8].append_rotation(&rotation)
            }
        }
    }

    #[allow(unused)]
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

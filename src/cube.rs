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

    /*
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
    }*/
}

/*
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
*/

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

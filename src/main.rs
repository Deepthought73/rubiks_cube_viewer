use crate::cube_window::CubeWindow;

mod color;
mod cube;
mod cube_window;
mod side;
mod util;

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

fn main() {
    let mut cube_window = CubeWindow::new();
    cube_window.render();
}

use minifb::{Key, Window, WindowOptions};

pub mod transform;
pub mod math_utils;
pub mod object;
pub mod scene;

use scene::{Scene, RenderMode};
use object::{Object3D, Camera};
use math_utils::Vector3D;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 360;

pub fn main() {

    // Boilerplate section for testing
    let mut cube = Object3D::load_obj("test_scene/tinker.obj".to_string()).unwrap();
    cube.transform.translate(&Vector3D::new(0, 0, 5));

    let camera = Camera::new(1, 30, 90);

    let mut scene = Scene {
        objects: vec![cube],
        render_mode: RenderMode::VertexOnly,
        camera
    };
    // End boilerplate section
}

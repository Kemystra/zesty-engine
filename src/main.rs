use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

pub mod transform;
pub mod math_utils;
pub mod object;
pub mod scene;
pub mod graphic;

use scene::Scene;
use object::{Object3D, Camera};
use math_utils::vector3d::Vector3D;
use graphic::Buffer;

pub const SCREEN_WIDTH: usize = 640;
pub const SCREEN_HEIGHT: usize = 360;

pub fn main() {

    // NOTE: the coordinates are left-handed
    // Thank you, past me

    // Boilerplate section for testing
    let mut cube = Object3D::load_obj("test_scene/tinker.obj".to_string()).unwrap();
    cube.transform.translate(Vector3D::new(0, 0, 5));

    let camera = Camera::new(1, 30, 90);

    let mut scene = Scene {
        objects: vec![cube],
        camera
    };
    // End boilerplate section
    
    let mut window = Window::new(
        "Zesty Engine v0.5",
        SCREEN_WIDTH, 
        SCREEN_HEIGHT,
        WindowOptions::default()
    ).unwrap_or_else(|e| panic!("{}", e));
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    let mut buffer = Buffer::new(&window);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        scene.render(&mut buffer);
        
        window
            .update_with_buffer(&buffer.raw_buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();

        buffer.clear();
    }
    
}

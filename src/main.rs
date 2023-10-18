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
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    
    let context = unsafe { Context::new(&window) }.unwrap();
    let mut surface = unsafe { Surface::new(&context, &window) }.unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { window_id, event: WindowEvent::CloseRequested }
                if window_id == window.id() => { *control_flow = ControlFlow::Exit }

            // Exit window with ANY keypress
            Event::WindowEvent { window_id, event: WindowEvent::KeyboardInput {..} }
                if window_id == window.id() => { *control_flow = ControlFlow::Exit }

            Event::RedrawRequested(window_id)
                if window_id == window.id() => {
                    let (width, height) = { let size = window.inner_size(); (size.width, size.height) };
                    surface.resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap()
                    ).unwrap();

                    let mut buffer = surface.buffer_mut().unwrap();
                    for index in 0..(width * height) {
                        let y = index / width;
                        let x = index % width;

                        let red = x % 255;
                        let green = y % 255;
                        let blue = (x*y) % 255;

                        buffer[index as usize] = blue | (green << 8) | (red << 16);
                    }

                    buffer.present().unwrap();
                }

            _ => {}
        }
    });
}

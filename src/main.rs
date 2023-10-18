use std::num::NonZeroU32;

use winit::window::WindowBuilder;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::{Event, WindowEvent};

use softbuffer::{Context, Surface};

pub mod transform;
pub mod math_utils;
pub mod object;
pub mod scene;
pub mod renderer;

use scene::Scene;
use object::{Object3D, Camera};
use math_utils::vector3d::Vector3D;
use renderer::{Color, Renderer};

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

    // Renderer init
    let (width, height) = { let size = window.inner_size(); (size.width, size.height) };
    let pitch = width;
 
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

                    let renderer = Renderer::new(width as usize, height as usize,
                        |x: usize, y: usize, color: Color| {
                            buffer[x + (y*pitch)] = color.r | (color.g << 8) | (color.b << 16);
                    });


                    buffer.present().unwrap();
                }

            _ => {}
        }
    });
}

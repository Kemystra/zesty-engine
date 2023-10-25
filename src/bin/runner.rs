use std::num::NonZeroU32;
use std::time::Instant;

use winit::window::WindowBuilder;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::{Event, WindowEvent, VirtualKeyCode, ElementState};
use winit::dpi::PhysicalSize;

use softbuffer::{Context, Surface};

use lib_engine::{scene, object, math_utils, renderer};

use scene::Scene;
use object::{Object3D, Camera, AspectRatio};
use math_utils::vector3d::Vector3D;
use renderer::Renderer;

pub fn main() {
    // NOTE: the coordinates are left-handed
    // Thank you, past me

    // Boilerplate section for testing
    let mut cube = Object3D::load_obj("test_scene/tinker.obj".to_string()).unwrap();
    cube.transform.translate(Vector3D::new(0, 0, 5));

    let camera = Camera::new(1, 30, 90, AspectRatio(16.0, 9.0));

    let mut scene = Scene {
        objects: vec![cube],
        camera
    };

    // End boilerplate section
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(640, 360))
        .with_resizable(false)
        .build(&event_loop).unwrap();

    // Renderer init
    let (width, height) = { let size = window.inner_size(); (size.width, size.height) };

    let context = unsafe { Context::new(&window) }.unwrap();
    let mut surface = unsafe { Surface::new(&context, &window) }.unwrap();

    let mut renderer = Renderer::new(width as usize, height as usize);
    surface.resize(
        NonZeroU32::new(width).unwrap(),
        NonZeroU32::new(height).unwrap()
    ).unwrap();


    let mut now = Instant::now();
    let mut delta_time = 0;
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { window_id, event: WindowEvent::CloseRequested }
                if window_id == window.id() => { *control_flow = ControlFlow::Exit }

            Event::WindowEvent { window_id, event: WindowEvent::KeyboardInput { input, ..} }
                if window_id == window.id() => {
                    if input.virtual_keycode == Some(VirtualKeyCode::Space) && input.state == ElementState::Pressed {
                        println!("FPS: {}", 1_000_000 / delta_time);
                    }
                }

            Event::MainEventsCleared => {
                let mut buffer = surface.buffer_mut().unwrap();
                let tmp = renderer.render(&mut scene);

                buffer.copy_from_slice(tmp);

                buffer.present().unwrap();
                renderer.clear_tmp_buffer();

                delta_time = now.elapsed().as_micros();
                now = Instant::now();
            }

            _ => {}
        }
    });
}

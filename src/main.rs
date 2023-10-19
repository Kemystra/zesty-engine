use std::num::NonZeroU32;

use winit::window::WindowBuilder;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::{Event, WindowEvent};
use winit::dpi::PhysicalSize;

use softbuffer::{Context, Surface};

pub mod transform;
pub mod math_utils;
pub mod object;
pub mod scene;
pub mod renderer;

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


    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { window_id, event: WindowEvent::CloseRequested }
                if window_id == window.id() => { *control_flow = ControlFlow::Exit }

            // Exit window with ANY keypress
            //Event::WindowEvent { window_id, event: WindowEvent::KeyboardInput {..} }
            //    if window_id == window.id() => { *control_flow = ControlFlow::Exit }

            Event::RedrawRequested(window_id)
                if window_id == window.id() => {

                    let mut buffer = surface.buffer_mut().unwrap();
                    let tmp = renderer.render(&mut scene);

                    buffer.copy_from_slice(tmp);

                    buffer.present().unwrap();
                }

            _ => {}
        }
    });
}

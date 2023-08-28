extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

mod transform;
mod math_utils;
mod object;
mod scene;
mod renderer;

use renderer::{Renderer, RenderMode};
use scene::Scene;
use object::Object3D;
use transform::Transform;
use math_utils::Vector3D;

const SCREEN_WIDTH: u32 = 480;
const SCREEN_HEIGHT: u32 = 360;

pub fn main() -> Result<(), String> {

    // Boilerplate section for testing
    let cube = Object3D {
        transform: Transform {
            translation: Vector3D::new(0, 0, -3),
            rotation: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0]
            ],
            scale: 1.0
        },
    };
    let scene = Scene {
        objects: vec![cube]
    };
    let mut renderer = Renderer::new(scene);
    renderer.render_mode = RenderMode::VertexOnly;
    // End boilerplate section

    // SDL2 Initialization
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, SCREEN_WIDTH, SCREEN_HEIGHT)
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        texture.with_lock(None, |a: &mut [u8], b: usize| { renderer.render(a,b) })?;
        canvas.clear();
        canvas.copy(&texture, None, None)?; 
        canvas.present();
    }

    Ok(())
}

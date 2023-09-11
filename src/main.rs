extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

pub mod transform;
pub mod math_utils;
pub mod object;
pub mod scene;

use scene::{Scene, RenderMode};
use object::{Object3D, Camera};
use math_utils::Vector3D;

pub const SCREEN_WIDTH: u32 = 480;
pub const SCREEN_HEIGHT: u32 = 360;

pub fn main() -> Result<(), String> {

    // Boilerplate section for testing
    let mut cube = Object3D::new(vec![
        Vector3D::new(1.0, -1.0, 1.0),
        Vector3D::new(1.0, -1.0, -1.0),
        Vector3D::new(1.0, 1.0, -1.0),
        Vector3D::new(1.0, 1.0, 1.0),
        Vector3D::new(-1.0, 1.0, 1.0),
        Vector3D::new(-1.0, 1.0, -1.0),
        Vector3D::new(-1.0, -1.0, -1.0),
        Vector3D::new(-1.0, -1.0, 1.0),
    ], vec![]);
    cube.transform.translate(&Vector3D::new(0, 0, 2));

    let camera = Camera::new(1, 10, 80);

    let mut scene = Scene {
        objects: vec![cube],
        render_mode: RenderMode::VertexOnly,
        camera
    };
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

        texture.with_lock(None, |a: &mut [u8], b: usize| { scene.render(a,b) })?;
        canvas.clear();
        canvas.copy(&texture, None, None)?; 
        canvas.present();
    }

    Ok(())
}

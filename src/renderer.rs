use crate::object;
use crate::scene::Scene;

use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};

#[derive(Debug)]
pub enum RenderMode {
    Wireframe,
    VertexOnly,
    Full
}


#[derive(Debug)]
pub struct Renderer {
    scene: Scene,
    pub render_mode: RenderMode
}

impl Renderer {
    pub fn new(scene: Scene) -> Self {
        Self {
            scene,
            render_mode: RenderMode::Full
        }
    }

    pub fn render(&mut self, buffer: &mut [u8], pitch: usize) {
        let mut i = 0;
        for y in 0..SCREEN_HEIGHT as usize{
            for x in 0..SCREEN_WIDTH as usize{
                let offset = y*pitch + x*3;
                buffer[offset] = i;
                buffer[offset + 1] = 255 - i;
                buffer[offset + 2] = 10;
                i = (i+1) % 255;
            }
        }
    }
}

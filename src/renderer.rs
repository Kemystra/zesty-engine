use crate::object;
use crate::scene::Scene;

use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};

#[derive(Debug)]
pub enum RenderMode {
    VertexOnly,
    Wireframe,
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

    pub fn render(&mut self, buffer: &mut [u8], pitch: usize) -> () {
        match self.render_mode {
            RenderMode::VertexOnly => vertex_render(buffer, pitch),
            RenderMode::Wireframe => wireframe_render(buffer, pitch),
            RenderMode::Full => full_render(buffer, pitch)
        }
    }

    fn vertex_render(&mut self, buffer: &mut [u8], pitch: usize) -> () {

    }
}

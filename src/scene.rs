use crate::object::Object3D;


#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Object3D>,
    pub render_mode: RenderMode
}

#[derive(Debug)]
pub enum RenderMode {
    VertexOnly,
    Wireframe,
    Full
}


impl Scene {
    pub fn render(&mut self, buffer: &mut [u8], pitch: usize) -> () {

    }
}

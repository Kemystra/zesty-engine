use crate::object::{Object3D, Camera};

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Object3D>,
    pub render_mode: RenderMode,
    pub camera: Camera
}

#[derive(Debug)]
pub enum RenderMode {
    VertexOnly,
    Wireframe,
    Full
}


impl Scene {
    pub fn render(&mut self, buffer: &mut [u8], pitch: usize) -> () {
        match self.render_mode {
            RenderMode::VertexOnly => self.vertex_render(buffer, pitch),
            RenderMode::Wireframe => self.wireframe_render(buffer, pitch),
            RenderMode::Full => self.full_render(buffer, pitch)
        }
    }

    fn vertex_render(&mut self, buffer: &mut [u8], pitch: usize) -> () {
        for obj in self.objects.iter() {
            for vertex in obj.get_vertices() {
                let vertex_in_world = obj.transform.to_world_space(&vertex);
                let vertex_in_cam = self.camera.transform.to_local_space(&vertex_in_world);
            }
        }
    }

    fn wireframe_render(&mut self, buffer: &mut [u8], pitch: usize) -> () {
        
    }

    fn full_render(&mut self, buffer: &mut [u8], pitch: usize) -> () {
        
    }
}

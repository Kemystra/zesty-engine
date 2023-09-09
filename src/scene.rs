use crate::object::{Object3D, Camera};
use crate::transform::convert_space;

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
        let world_to_cam_transform = self.camera.transform.invert_matrix().unwrap();
        for obj in self.objects.iter() {
            for vertex in obj.get_vertices() {
                let vertex_in_world = convert_space(&obj.transform, &vertex);
                let vertex_in_cam = convert_space(&world_to_cam_transform, &vertex_in_world);
            }
        }
    }

    fn wireframe_render(&mut self, buffer: &mut [u8], pitch: usize) -> () {
        
    }

    fn full_render(&mut self, buffer: &mut [u8], pitch: usize) -> () {
        
    }
}

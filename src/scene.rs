use crate::object::{Object3D, Camera};
use crate::math_utils::Vector3D;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};

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
                let screen_coords = self.camera.project_to_screen_space(vertex_in_cam);

                let ncd_coords = Vector3D {
                    x: (screen_coords.x + 1.0)/2.0,
                    y: (screen_coords.y + 1.0)/2.0,
                    z: (screen_coords.z + 1.0)/2.0,
                };

            }
        }
    }

    fn wireframe_render(&mut self, buffer: &mut [u8], pitch: usize) -> () {
        
    }

    fn full_render(&mut self, buffer: &mut [u8], pitch: usize) -> () {
        
    }
}

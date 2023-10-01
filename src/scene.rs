use crate::object::{Object3D, Camera};
use crate::math_utils::vector3d::Vector3D;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::graphic::Buffer;

use std::f64::consts::PI;

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Object3D>,
    pub camera: Camera
}

const WHITE: (u8, u8, u8) = (255, 255, 255);

impl Scene {
    pub fn render(&mut self, buffer: &mut Buffer) -> () {
        let rot = (PI/4.0) * (1.0/60.0);
        for obj in self.objects.iter_mut() {

            let mut tmp_vertex: Vec<Vector3D> = vec![];
            for vertex in obj.get_vertices() {
                let vertex_in_world = obj.transform.to_world_space(*vertex);
                let vertex_in_cam = self.camera.transform.to_local_space(vertex_in_world);
                let screen_coords = self.camera.project_to_screen_space(vertex_in_cam);

                let ncd_coords = Vector3D {
                    x: (screen_coords.x + 1.0) * 0.5,
                    y: (screen_coords.y + 1.0) * 0.5,
                    z: (screen_coords.z + 1.0) * 0.5,
                };

                let final_x = ncd_coords.x * SCREEN_WIDTH as f64;
                let final_y = ncd_coords.y * SCREEN_HEIGHT as f64;
                tmp_vertex.push(Vector3D::new(final_x, final_y, screen_coords.z));
            }

            for face in obj.get_triangles() {
                let v1 = tmp_vertex[face[0] - 1];
                let v2 = tmp_vertex[face[1] - 1];
                buffer.bresenham_line(WHITE,
                    v1.x as usize, v1.y as usize,
                    v2.x as usize, v2.y as usize);
            }

            obj.transform.rotate(rot, 0.0, rot);
        }
    }
}

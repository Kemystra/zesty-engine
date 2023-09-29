use crate::object::{Object3D, Camera};
use crate::math_utils::{vector3d::Vector3D, clamp};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};

use std::f64::consts::PI;

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

const WHITE: (u8, u8, u8) = (255, 255, 255);

fn color_pixel(
    x: usize, y: usize,
    color: (u8, u8, u8),
    buffer: &mut [u32]) -> () {

    let (r,g,b) = (color.0 as u32, color.1 as u32, color.2 as u32);
    let color_32bit = (r << 16) | (g << 8) | b;
    let offset = x + (y*SCREEN_WIDTH);

    buffer[offset] = color_32bit;
}

impl Scene {
    pub fn render(&mut self, buffer: &mut [u32]) -> () {
        match self.render_mode {
            RenderMode::VertexOnly => self.vertex_render(buffer),
            RenderMode::Wireframe => self.wireframe_render(buffer),
            RenderMode::Full => self.full_render(buffer)
        }
    }

    fn vertex_render(&mut self, buffer: &mut [u32]) -> () {
        let rot = (PI/4.0) * (1.0/60.0);
        for obj in self.objects.iter_mut() {
            for vertex in obj.get_vertices() {
                let vertex_in_world = obj.transform.to_world_space(*vertex);
                let vertex_in_cam = self.camera.transform.to_local_space(vertex_in_world);
                let screen_coords = self.camera.project_to_screen_space(vertex_in_cam);

                if screen_coords.x > 1.0 || screen_coords.x < -1.0 {
                    continue;
                }

                if screen_coords.y > 1.0 || screen_coords.y < -1.0 {
                    continue;
                }

                let ncd_coords = Vector3D {
                    x: (screen_coords.x + 1.0)/2.0,
                    y: (screen_coords.y + 1.0)/2.0,
                    z: (screen_coords.z + 1.0)/2.0,
                };

                let final_x = (ncd_coords.x * SCREEN_WIDTH as f64) as usize; 
                let final_y = (ncd_coords.y * SCREEN_HEIGHT as f64) as usize;


                for i in 0..5 {
                    for j in 0..5 {
                        color_pixel(
                        clamp(final_x + i, 0, SCREEN_WIDTH-1),
                        clamp(final_y + j, 0, SCREEN_HEIGHT-1),
                        WHITE,
                        buffer);
                    }
                }
            }
            obj.transform.rotate(rot, 0.0, rot);
        }
    }

    fn wireframe_render(&mut self, buffer: &mut [u32]) -> () {
        
    }

    fn full_render(&mut self, buffer: &mut [u32]) -> () {
        
    }
}

use std::f64::consts::PI;

use crate::scene::Scene;
use crate::math_utils::{vector3d::Vector3D, matrix4x4};
use matrix4x4::{matrix_multiply, vector_matrix_multiply};
use crate::component::mesh::Mesh;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8, pub g: u8, pub b: u8
}

impl Color {
    pub fn new<T: TryInto<u8>>(r: T, g: T, b: T) -> Self {
        Self {
            r: r.try_into().unwrap_or_default(),
            g: g.try_into().unwrap_or_default(),
            b: b.try_into().unwrap_or_default()
        }
    }

    pub fn rgb_u32(&self) -> u32 {
        (self.r as u32) | ((self.g as u32) << 8) | ((self.b as u32) << 16)
    }
}

const WHITE: Color = Color {
    r: 255, g: 255, b: 255
};

pub struct Renderer {
    width: usize,
    height: usize,
    tmp_buffer: Vec<u32>
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tmp_buffer: vec![0; width*height]
        }
    }

    pub fn render(&mut self, scene: &mut Scene) {
        let rot = (PI/4.0) * (1.0/200.0);
        let camera = &scene.camera;

        for obj in scene.objects.iter_mut() {
            let mesh = obj.get_component::<Mesh>().unwrap();
            let obj_to_cam_matrix = matrix_multiply(&obj.transform.matrix(), &camera.transform.matrix());

            for vertex in mesh.vertices() {
                let vertex_in_cam = vector_matrix_multiply(&obj_to_cam_matrix, *vertex, true);
                let screen_coords = camera.project_to_screen_space(vertex_in_cam);

                let ncd_coords = Vector3D {
                    x: (screen_coords.x + 1.0) * 0.5,
                    y: (screen_coords.y + 1.0) * 0.5,
                    z: screen_coords.z,
                };

                let final_x = (ncd_coords.x * self.width as f64) as usize;
                let final_y = (ncd_coords.y * self.height as f64) as usize;
            }

            obj.transform.rotate(rot, 0.0, rot);
        }
    }

    pub fn buffer(&self) -> &Vec<u32> {
        &self.tmp_buffer
    }

    pub fn clear_tmp_buffer(&mut self) {
        self.tmp_buffer.iter_mut().for_each(|x| *x = 0);
    }

    pub fn plot_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.tmp_buffer[x + (y*self.width)] = color.rgb_u32();
    }

    pub fn bresenham_line(
        &mut self, color: Color,
        x0: usize, y0: usize,
        end_x: usize, end_y: usize) {
        let mut curr_x = x0 as isize;
        let mut curr_y = y0 as isize;

        let end_x = end_x as isize;
        let end_y = end_y as isize;

        let dx = (end_x - curr_x).abs();
        let dy = -(end_y - curr_y).abs();
        let mut error = dx + dy;

        let sx = if curr_x < end_x {1} else {-1};
        let sy = if curr_y < end_y {1} else {-1};

        loop {
            self.plot_pixel(curr_x as usize, curr_y as usize, color);
            if curr_x == end_x && curr_y == end_y {break}
            let e2 = error * 2;

            if e2 >= dy {
                if curr_x == end_x {break}
                error += dy;
                curr_x += sx;
            }

            if e2 <= dx {
                if curr_y == end_y {break}
                error += dx;
                curr_y += sy
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_color() {
        let color = Color::new(255, 100, 12);

        assert_eq!(color, Color {
            r: 255, g: 100, b: 12
        });
    }

    #[test]
    fn new_invalid_color() {
        let color = Color::new(256, 34, -1);

        assert_eq!(color, Color {
            r: 0, g: 34, b: 0
        })
    }

    #[test]
    fn output_rgb_as_u32() {
        let color = Color::new(100, 234, 88);
        let expected = 100_u32 | (234 << 8) | (88 << 16);

        assert_eq!(color.rgb_u32(), expected);
    }
}

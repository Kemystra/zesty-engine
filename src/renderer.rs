use std::collections::HashMap;
use std::f64::consts::PI;
use std::cmp::{min, max};

use crate::scene::Scene;
use crate::math_utils::{vector3d::Vector3D, matrix4x4};
use matrix4x4::{matrix_multiply, vector_matrix_multiply};
use crate::component::mesh::Mesh;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color(u32);

impl Color {
    pub fn new<T: TryInto<u8>>(r: T, g: T, b: T) -> Self {
        Self(
            r.try_into().unwrap_or_default() as u32 |
            ((g.try_into().unwrap_or_default() as u32) << 8) |
            ((b.try_into().unwrap_or_default() as u32) << 16)
        )
    }

    pub fn rgb_u32(&self) -> u32 {
        self.0
    }
}

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
        let camera = &mut scene.camera;

        let color_list: [Color; 8] = [
            Color::new(255, 255, 255),
            Color::new(0, 0, 255),
            Color::new(255, 0, 0),
            Color::new(0, 0, 255),
            Color::new(0, 255, 255),
            Color::new(255, 0, 255),
            Color::new(255, 255, 0),
            Color::new(100, 100, 100)
        ];

        for obj in scene.objects.iter_mut() {
            let mesh = obj.get_component::<Mesh>().unwrap();
            let obj_to_cam_matrix = matrix_multiply(
                &obj.transform.matrix(),
                &camera.transform.matrix()
            );

            let all_vertices = mesh.vertices();
            let mut obj_vertex_loopkup: HashMap<usize, Vector3D> = HashMap::new();

            let mut i = 0;
            
            for triangle in mesh.triangles() {
                let triangle_vertices = triangle.iter().map(|&vertex_index| {
                    if let Some(i) = obj_vertex_loopkup.get(&vertex_index) {
                        return *i;
                    }

                    let vertex_in_cam = vector_matrix_multiply(
                        &obj_to_cam_matrix,
                        all_vertices[vertex_index],
                        true
                    );

                    let screen_space_coords =
                    camera.project_to_screen_space(vertex_in_cam);

                    obj_vertex_loopkup.insert(vertex_index, screen_space_coords);
                    screen_space_coords
                });

                let triangle_tuple = triangle_vertices.map(|point| {
                    let ncd_coords = self.to_ncd_space(point);

                    let final_x = (ncd_coords.x * self.width as f64) as isize;
                    let final_y = (ncd_coords.y * self.height as f64) as isize;

                    (final_x, final_y)
                }).collect::<Vec<(isize, isize)>>();

                self.draw_triangles(triangle_tuple, color_list[i % 8]);
                i += 1;
            }

            obj.transform.rotate(rot, 0.0, rot);
        }
    }

    fn to_ncd_space(&self, vector: Vector3D) -> Vector3D {
        Vector3D {
            x: (vector.x + 1.0) * 0.5,
            y: (vector.y + 1.0) * 0.5,
            z: vector.z,
        }
    }

    pub fn buffer(&self) -> &Vec<u32> {
        &self.tmp_buffer
    }

    pub fn clear_tmp_buffer(&mut self) {
        self.tmp_buffer.iter_mut().for_each(|x| *x = 0);
    }

    pub fn plot_pixel<T: Into<usize>>(&mut self, x: T, y: T, color: Color) {
        self.tmp_buffer[x.into() + (y.into()*self.width)] = color.rgb_u32();
    }

    pub fn bresenham_line(
        &mut self, color: Color,
        x0: isize, y0: isize,
        end_x: isize, end_y: isize) {

        let mut curr_x = x0;
        let mut curr_y = y0;

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

    pub fn draw_triangles(&mut self, triangle_tuple: Vec<(isize, isize)>, color: Color) {
        let mut max_x = triangle_tuple[0].0;
        let mut max_y = triangle_tuple[0].1;
        let mut min_x = max_x;
        let mut min_y = max_y;

        for index in 1..3 {
            let x_part = triangle_tuple[index].0;
            let y_part = triangle_tuple[index].1;

            max_x = max(x_part, max_x);
            min_x = min(x_part, min_x);

            max_y = max(y_part, max_y);
            min_y = min(y_part, min_y);
        }

        let edge_results = triangle_tuple.iter().enumerate().map(|pair| {
            let (i, point) = pair;
            let next_point = triangle_tuple[(i+1) % 3];
            let diff_x = point.0 - next_point.0;
            let diff_y = point.1 - next_point.1;

            // Based on the edge function
            let first_result = ((min_y - point.1) * diff_x) - ((min_x - point.0) * diff_y);
            (first_result, diff_x, diff_y)
        }).collect::<Vec<(isize, isize, isize)>>();

        for offset_x in 0..(max_x - min_x) {
            for offset_y in 0..(max_y - min_y) {
                let is_in_triangle = edge_results.iter().all(|results_group| {
                    let (first_result, diff_x, diff_y) = results_group;
                    let curr_result = first_result + (diff_x*offset_y) - (diff_y*offset_x);

                    curr_result >= 0
                });

                if is_in_triangle {
                    self.plot_pixel(
                        (min_x + offset_x) as usize,
                        (min_y + offset_y) as usize, color);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_invalid_color() {
        let color = Color::new(256, 34, -1);

        assert_eq!(color, Color::new(0, 34, 0));
    }

    #[test]
    fn output_rgb_as_u32() {
        let color = Color::new(100, 234, 88);
        let expected = 100_u32 | (234 << 8) | (88 << 16);

        assert_eq!(color.rgb_u32(), expected);
    }
}

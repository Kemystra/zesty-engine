use crate::scene::Scene;
use crate::math_utils::vector3d::Vector3D;
use std::f64::consts::PI;

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
}
const WHITE: Color = Color {
    r: 255, g: 255, b: 255
};

pub struct Renderer {
    width: usize,
    height: usize,
    draw_func: Box<dyn FnMut(usize, usize, Color) -> ()>
}

impl Renderer {
    pub fn new(width: usize, height: usize,
        draw_func: impl FnMut(usize, usize, Color) -> () + 'static) -> Self {
        Self {
            width,
            height,
            draw_func: Box::new(draw_func)
        }
    }

    pub fn render(&mut self, scene: &mut Scene) -> () {
        let rot = (PI/4.0) * (1.0/60.0);

        for obj in scene.objects.iter_mut() {

            let mut tmp_vertex: Vec<[usize; 2]> = vec![];
            for vertex in obj.get_vertices() {
                let vertex_in_world = obj.transform.to_world_space(*vertex);
                let vertex_in_cam = scene.camera.transform.to_local_space(vertex_in_world);
                let screen_coords = scene.camera.project_to_screen_space(vertex_in_cam);

                let ncd_coords = Vector3D {
                    x: (screen_coords.x + 1.0) * 0.5,
                    y: (screen_coords.y + 1.0) * 0.5,
                    z: (screen_coords.z + 1.0) * 0.5,
                };

                let final_x = (ncd_coords.x * self.width as f64) as usize;
                let final_y = (ncd_coords.y * self.height as f64) as usize;

                tmp_vertex.push([final_x, final_y]);
            }

            for face in obj.get_triangles() {
                let v1 = tmp_vertex[face[0]];
                let v2 = tmp_vertex[face[1]];
                let v3 = tmp_vertex[face[2]];
                self.bresenham_line(WHITE, v1[0], v1[1], v2[0], v2[1]);
                self.bresenham_line(WHITE, v2[0], v2[1], v3[0], v3[1]);
            }

            obj.transform.rotate(rot, 0.0, rot);
        }
    }

    pub fn plot_pixel(&mut self, x: usize, y: usize, color: Color) {
        (self.draw_func)(x, y, color);
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
}

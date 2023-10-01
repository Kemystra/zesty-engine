use crate::SCREEN_WIDTH;

#[derive(Debug)]
pub struct Buffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize
}

impl Buffer {
    pub fn new(window: minifb::Window) -> Buffer {
        let (width, height) = window.get_size();
        Buffer {
            buffer: vec![0_u32; width * height],
            width,
            height
        }
    }

    pub fn plot_pixel(
        &mut self,
        x: usize, y: usize,
        color: (u8, u8, u8)) -> () {

        let (r,g,b) = (color.0 as u32, color.1 as u32, color.2 as u32);
        let color_32bit = (r << 16) | (g << 8) | b;
        let offset = x + (y*SCREEN_WIDTH);

        self.buffer[offset] = color_32bit;
    }

    pub fn bresenham_line(
        &mut self, color: (u8, u8, u8),
        x0: usize, y0: usize,
        x1: usize, y1: usize) {
        let mut curr_x = x0 as isize;
        let mut curr_y = y0 as isize;

        let dx = ((x1 - x0) as isize).abs();
        let dy = -((y1 - y0) as isize).abs();
        let mut error = dx + dy;

        let sx = if x0 < x1 {1} else {-1};
        let sy = if y0 < y1 {1} else {-1};

        loop {
            self.plot_pixel(curr_x as usize, curr_y as usize, color);
            if curr_x == x1 as isize || curr_y == y1 as isize {break}
            let e2 = error * 2;

            if e2 >= dy {
                if curr_x == x1 as isize {break}
                error += dy;
                curr_x += sx;
            }

            if e2 <= dx {
                if curr_y == y1 as isize {break}
                error += dx;
                curr_y += sy
            }
        }
    }
}

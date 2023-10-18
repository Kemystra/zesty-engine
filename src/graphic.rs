#[derive(Debug)]
struct Color(u8, u8, u8);
    pub raw_buffer: Vec<u32>,
    width: usize,
    height: usize
}

impl Buffer {
    pub fn new(window: &minifb::Window) -> Buffer {
        let (width, height) = window.get_size();
        Buffer {
            raw_buffer: vec![0_u32; width * height],
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
        let offset = x + (y*self.width);

        self.raw_buffer[offset] = color_32bit;
    }

    pub fn bresenham_line(
        &mut self, color: (u8, u8, u8),
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

    pub fn clear(&mut self) {
        self.raw_buffer.iter_mut().for_each(|pixel| *pixel = 0);
    }
}

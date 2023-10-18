#[derive(Debug)]
struct Color(u8, u8, u8);


#[derive(Debug)]
pub struct BufferInterface<F>
    where F: FnMut(usize, usize, Color) -> () {
    width: usize,
    height: usize,
    draw_func: F
}

impl<F> BufferInterface<F>
    where F: FnMut(usize, usize, Color) -> () {
    pub fn new(width: usize, height: usize, draw_func: F) -> Self {
        Self {
            width,
            height,
            draw_func
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

    pub fn clear(&mut self) {
        self.raw_buffer.iter_mut().for_each(|pixel| *pixel = 0);
    }
}

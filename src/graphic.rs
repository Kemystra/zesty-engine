use crate::SCREEN_WIDTH;


pub fn plot_pixel(
    x: usize, y: usize,
    color: (u8, u8, u8),
    buffer: &mut [u32]) -> () {

    let (r,g,b) = (color.0 as u32, color.1 as u32, color.2 as u32);
    let color_32bit = (r << 16) | (g << 8) | b;
    let offset = x + (y*SCREEN_WIDTH);

    buffer[offset] = color_32bit;
}

pub fn bresenham_line(x0: usize, y0: usize, x1: usize, y1: usize) {
    let curr_x = x0 as isize;
    let curr_y = y0 as isize;

    let dx = ((x1 - x0) as isize).abs();
    let dy = -((y1 - y0) as isize).abs();
    let error = dx + dy;

    let sx = if x0 < x1 {1} else {-1};
    let sy = if y0 < y1 {1} else {-1};

    loop {
        plot_pixel()
    }
}

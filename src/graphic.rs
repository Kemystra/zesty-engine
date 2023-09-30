use crate::SCREEN_WIDTH;


pub fn color_pixel(
    x: usize, y: usize,
    color: (u8, u8, u8),
    buffer: &mut [u32]) -> () {

    let (r,g,b) = (color.0 as u32, color.1 as u32, color.2 as u32);
    let color_32bit = (r << 16) | (g << 8) | b;
    let offset = x + (y*SCREEN_WIDTH);

    buffer[offset] = color_32bit;
}

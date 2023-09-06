use crate::object::Object3D;


#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Object3D>
}

impl Scene {
    pub fn render(&mut self, buffer: &mut [u8], pitch: usize) -> () {

    }
}

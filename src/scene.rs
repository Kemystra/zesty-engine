use crate::object::{Object, Camera};

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub camera: Camera
}

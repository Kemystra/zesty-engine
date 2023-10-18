use crate::object::{Object3D, Camera};

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Object3D>,
    pub camera: Camera
}

use crate::transform::Transform;
use crate::math_utils::Vector3D;


#[derive(Debug)]
pub struct Object3D {
    pub transform: Transform,
    vertices: Vec<Vector3D>,
    triangles: Vec<[usize; 3]>
}

impl Object3D {
    pub fn new(vertices: Vec<Vector3D>, triangles: Vec<[usize; 3]>) -> Self {
        Self {
            transform: Transform::new(),
            vertices,
            triangles
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    pub transform: Transform,
    pub near_clip_distance: f64,
    pub far_clip_distance: f64,
    pub field_of_view: u32
}

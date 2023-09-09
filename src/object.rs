use crate::transform::{Transform, NIL_TRANSFORM};
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
            transform: NIL_TRANSFORM,
            vertices,
            triangles
        }
    }

    // Note that we use read-only borrow here
    // because we don't need to edit anything really
    pub fn get_vertices(&self) -> &Vec<Vector3D> {
        &self.vertices
    }
}

#[derive(Debug)]
pub struct Camera {
    pub transform: Transform,
    pub near_clip_distance: f64,
    pub far_clip_distance: f64,
    pub field_of_view: usize
}

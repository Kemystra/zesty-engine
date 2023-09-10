use std::f64::consts::PI;

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

    // Note that we use read-only borrow here
    // because we don't need to edit anything really
    pub fn get_vertices(&self) -> &Vec<Vector3D> {
        &self.vertices
    }
}

#[derive(Debug)]
pub struct Camera {
    pub transform: Transform,
    near_clip_distance: f64,
    far_clip_distance: f64,
    field_of_view: f64,
    projection_data: ProjectionData,
    dirty_flag: bool
}


impl Camera {
    pub fn new<T: Into<f64>+Copy>(n: T, f: T, fov: T) -> Self {
        Self {
            transform: Transform::new(),
            near_clip_distance: n.into(),
            far_clip_distance: f.into(),
            field_of_view: fov.into(),
            projection_data: Self::calc_projection_data(n.into(), f.into(), fov.into()),
            dirty_flag: false
        }
    } 
}

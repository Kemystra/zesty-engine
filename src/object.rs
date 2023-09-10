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
    transform: Transform,
    near_clip_distance: f64,
    far_clip_distance: f64,
    field_of_view: f64,
    projection_data: ProjectionData,
    dirty_flag: bool
}

#[derive(Debug)]
pub struct ProjectionData {
    w_scaler: f64,
    h_scaler: f64,
    m1: f64,
    m2: f64,
}


impl Camera {
    pub fn new(n: f64, f: f64, fov: f64) -> Self {
        Self {
            transform: Transform::new(),
            near_clip_distance: n,
            far_clip_distance: f,
            field_of_view: fov,
            projection_data: Self::calc_projection_data(n, f, fov),
            dirty_flag: false
        }
    }

    fn calc_projection_data(n: f64, f: f64, fov: f64) -> ProjectionData {
        let fov_tan_val = (fov/2.0 * PI/180.0).tan();
        let near_far_interval = f - n;
        ProjectionData {
            w_scaler: 9.0 / (16.0*fov_tan_val),
            h_scaler: 1.0 / fov_tan_val,
            m1: -f / near_far_interval,
            m2: -f*n / near_far_interval
        }
    }
}

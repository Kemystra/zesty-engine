use std::collections::HashMap;
use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::f64::consts::PI;

use crate::transform::Transform;
use crate::math_utils::vector3d::Vector3D;
use crate::component::Component;


#[derive(Debug)]
pub struct Object {
    pub transform: Transform,
    components: HashMap<String, Box<dyn Component>>
}

impl Object {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            components: HashMap::new()
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    pub transform: Transform,
    near_clip_distance: f64,
    far_clip_distance: f64,
    field_of_view: f64,
    aspect_ratio: AspectRatio,
    projection_data: ProjectionData,
    dirty_flag: bool
}


impl Camera {
    pub fn new<T: Into<f64>+Copy>(n: T, f: T, fov: T, aspect_ratio: AspectRatio) -> Self {
        Self {
            transform: Transform::new(),
            near_clip_distance: n.into(),
            far_clip_distance: f.into(),
            field_of_view: fov.into(),
            aspect_ratio,
            projection_data: ProjectionData::generate(
                n.into(), f.into(), fov.into(), aspect_ratio
            ),
            dirty_flag: false
        }
    }

    pub fn projection_data(&mut self) -> &ProjectionData {
        if self.dirty_flag {
            self.projection_data = ProjectionData::generate(
                self.near_clip_distance, 
                self.far_clip_distance,
                self.field_of_view,
                self.aspect_ratio
            )
        }
        &self.projection_data
    }

    pub fn project_to_screen_space(&mut self, point: Vector3D) -> Vector3D {
        // Deconstructing the data
        // What the actual frick
        let ProjectionData(w_scaler, h_scaler, m1, m2) = self.projection_data();
        let x = point.x * w_scaler;
        let y = point.y * h_scaler;
        let z = (point.z * m1) + m2;
        let w = -point.z;

        Vector3D {
            x: x/w,
            y: y/w,
            z: z/w,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct AspectRatio(pub f64, pub f64);


#[derive(Debug)]
pub struct ProjectionData(f64, f64, f64, f64);

// I'm just gonna hard code the aspect ratio lol
impl ProjectionData {
    pub fn generate(n: f64, f: f64, fov: f64, ratio: AspectRatio) -> ProjectionData {
        let fov_tan_val = n * (fov/2.0 * PI/180.0).tan();
        let near_far_interval = f - n;
        ProjectionData(
            1.0 / (fov_tan_val),
            ratio.0 / (ratio.1*fov_tan_val),
            -f / near_far_interval,
            -f*n / near_far_interval
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_obj() {
        let obj = Object::new();
        assert_eq!(obj.transform, Transform::new());
        assert_eq!(obj.components.len(), 0);
    }
}

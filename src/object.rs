use std::io::{self, BufReader, BufRead};
use std::fs::File;

use crate::transform::Transform;
use crate::math_utils::{vector3d::Vector3D, ProjectionData};


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

    pub fn load_obj(file_path: String) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let mut lines = BufReader::new(file).lines();

        let mut vertices: Vec<Vector3D> = vec![];
        let mut triangles: Vec<[usize; 3]> = vec![];

        // I just want to parse text
        // Why tf is it so complicated
        while let Some(Ok(line)) = lines.next() {
            if line.chars().nth(0) == Some('v') {
                let mut vertex_data = line.split_whitespace();
                // Hacky way to consume the first element
                vertex_data.next();
                let vertex = Vector3D::new(
                    vertex_data.next().unwrap().parse::<f64>().unwrap(),
                    vertex_data.next().unwrap().parse::<f64>().unwrap(),
                    vertex_data.next().unwrap().parse::<f64>().unwrap(),
                );

                vertices.push(vertex);
            }

            if line.chars().nth(0) == Some('f') {
                let mut facet_data = line.split_whitespace();

                facet_data.next();
                let facet = [
                    facet_data.next().unwrap().parse::<usize>().unwrap(),
                    facet_data.next().unwrap().parse::<usize>().unwrap(),
                    facet_data.next().unwrap().parse::<usize>().unwrap(),
                ];

                triangles.push(facet);
            }
        }

        if vertices.len() == 0 || triangles.len() == 0 {
            let no_3d_data_error = io::Error::new(io::ErrorKind::Other, "No 3D data found.");
            return Err(no_3d_data_error)
        }

        Ok(Self::new(vertices, triangles))
    }

    // Note that we use read-only borrow here
    // because we don't need to edit anything really
    pub fn get_vertices(&self) -> &Vec<Vector3D> {
        &self.vertices
    }

    pub fn get_triangles(&self) -> &Vec<[usize; 3]> {
        &self.triangles
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
            projection_data: ProjectionData::generate(n.into(), f.into(), fov.into()),
            dirty_flag: false
        }
    }

    pub fn get_projection_data(&mut self) -> &ProjectionData {
        if self.dirty_flag {
            self.projection_data = ProjectionData::generate(
                self.near_clip_distance, 
                self.far_clip_distance,
                self.field_of_view
            )
        }
        &self.projection_data
    }

    pub fn project_to_screen_space(&mut self, point: Vector3D) -> Vector3D {
        // Deconstructing the data
        // What the actual frick
        let ProjectionData(w_scaler, h_scaler, m1, m2) = self.get_projection_data();
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

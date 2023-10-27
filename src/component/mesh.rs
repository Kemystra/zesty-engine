use std::fs::File;
use std::io::{self, BufRead, BufReader};

use crate::math_utils::vector3d::Vector3D;
use super::{Component, ComponentType};


#[derive(Debug, Component, ComponentType)]
pub struct Mesh {
    src: String,
    vertices: Vec<Vector3D>,
    triangles: Vec<[usize; 3]>
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            src: "".to_string(),
            vertices: vec![],
            triangles: vec![]
        }
    }

    pub fn load_obj(&mut self, file_path: String) -> io::Result<()> {
        self.src = file_path;

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
                // Substract by one to make it easy for indexing later
                let facet = [
                    facet_data.next().unwrap().parse::<usize>().unwrap() - 1,
                    facet_data.next().unwrap().parse::<usize>().unwrap() - 1,
                    facet_data.next().unwrap().parse::<usize>().unwrap() - 1,
                ];

                triangles.push(facet);
            }
        }

        if vertices.len() == 0 || triangles.len() == 0 {
            let no_3d_data_error = io::Error::new(io::ErrorKind::Other, "No 3D data found.");
            return Err(no_3d_data_error)
        }

        Ok(())
    }

    // Note that we use read-only borrow here
    // because we don't need to edit anything really
    pub fn vertices(&self) -> &Vec<Vector3D> {
        &self.vertices
    }

    pub fn triangles(&self) -> &Vec<[usize; 3]> {
        &self.triangles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_init() {
        let mesh = Mesh::new();
        assert_eq!(mesh.vertices.len(), 0);
        assert_eq!(mesh.triangles.len(), 0);
    }
}

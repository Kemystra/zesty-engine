use crate::math_utils::vector3d::Vector3D;


#[derive(Debug, )]
pub struct Mesh {
    src: String,
    vertices: Vec<Vector3D>,
    triangles: Vec<[usize; 3]>
}

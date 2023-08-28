use crate::transform::Transform;
use crate::math_utils::Vector3D;


#[derive(Debug)]
pub struct Object {
    pub transform: Transform,
    triangles: Vec<Vector3D> 
}

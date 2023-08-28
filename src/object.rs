use crate::transform::Transform;
use crate::math_utils::Vector3D;


pub struct Object {
    pub transform: Transform,
    triangles: Vec<Vector3D> 
}

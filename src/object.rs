use crate::transform::Transform;
use crate::math_utils::Vector3D;


#[derive(Debug)]
pub struct Object3D {
    pub transform: Transform,
    triangles: Vec<Vector3D> 
}

impl Object3D {
    pub fn new(triangles: Vec<Vector3D>) -> Self {
        Self {
            transform: Transform {
                translation: Vector3D::new(0, 0, 0),
                rotation: [
                    [1.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, 0.0, 1.0]
                ],
                scale: 1.0
            },
            triangles
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    pub transform: Transform,

}

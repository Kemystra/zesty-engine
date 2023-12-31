use crate::math_utils::{vector3d, quaternion, matrix4x4};
use matrix4x4::{Matrix4x4, IDENTITY_MATRIX4X4, invert_matrix, vector_matrix_multiply};
use quaternion::{Quaternion, IDENTITY_QUATERNION};
use vector3d::Vector3D;


// The struct will have getter and setter
// So that we can use a dirty flag to track any changes
#[derive(Debug, PartialEq)]
pub struct Transform {
    matrix: Matrix4x4,
    inverse_matrix: Matrix4x4,
    rotation: Quaternion,
    scale: Vector3D,
    dirty_flag: bool
}

impl Transform {
    pub fn new() -> Self {
        Self {
            matrix: IDENTITY_MATRIX4X4,
            inverse_matrix: IDENTITY_MATRIX4X4,
            rotation: IDENTITY_QUATERNION,
            scale: Vector3D::new(1,1,1),
            dirty_flag: false
        }
    }

    pub fn from_matrix(matrix: Matrix4x4) -> Result<Self, String>{
        Ok(
        Self {
            matrix,
            inverse_matrix: invert_matrix(&matrix, true)?,
            rotation: IDENTITY_QUATERNION,
            scale: Vector3D::new(1,1,1),
            dirty_flag: false
        })
    }

    pub fn matrix(&self) -> Matrix4x4 {
       self.matrix 
    }

    pub fn inverse_matrix(&mut self) -> Matrix4x4 {
        if self.dirty_flag {
            self.inverse_matrix = invert_matrix(&self.matrix, true).unwrap();
        }

        self.inverse_matrix
    }

    #[inline]
    pub fn to_world_space(&self, coord: Vector3D) -> Vector3D {
        vector_matrix_multiply(&self.matrix, coord, true)
    }

    #[inline]
    pub fn to_local_space(&self, coord: Vector3D) -> Vector3D {
        vector_matrix_multiply(&self.inverse_matrix, coord, true)
    }

    pub fn has_changed(&self) -> bool {
        self.dirty_flag
    }

    pub fn translate(&mut self, amount: Vector3D) -> () {
        self.dirty_flag = true;
        self.matrix[3][0] += amount.x;
        self.matrix[3][1] += amount.y;
        self.matrix[3][2] += amount.z;
    }

    pub fn rotate<T: Into<f64>>(&mut self, x: T, y: T, z: T) {
        self.dirty_flag = true;
        let new_q = Quaternion::from_euler_angles(x,y,z);
        self.rotation *= new_q;
        self.rotation.lazy_normalize();
        self.rotation.update_3x4_matrix(&mut self.matrix, &self.scale);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::round_place;
    use rand::Rng;

    fn round_vector3d(vector: Vector3D) -> [f64; 3] {
        [
            round_place(vector.x, 2),
            round_place(vector.y, 2),
            round_place(vector.z, 2)
        ]
    }

    #[test]
    fn transform_to_world() {
        let mat_a = Vector3D::new(5.0,3.0,12.0);
        let matrix = [
                [0.9, 0.4, 0.12, 0.0],
                [0.7, 0.06, 0.5, 0.0],
                [0.1, 0.4, 2.7, 0.0],
                [10.0,12.0,11.0, 1.0],
            ];
        let transform_b = Transform::from_matrix(matrix).unwrap();
        let result = transform_b.to_world_space(mat_a);
        let rounded_result = round_vector3d(result);

        assert_eq!(rounded_result, [17.8, 18.98, 45.5]);
    }

    #[test]
    fn transform_to_local() {
        let mat_a = Vector3D::new(5,10,2);
        let matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [3.0, 4.0, 2.0, 1.0]
        ];
        let transform_b = Transform::from_matrix(matrix).unwrap();
        let result = transform_b.to_local_space(mat_a);
        let rounded_result = round_vector3d(result);

        assert_eq!(rounded_result, [2.0, 6.0, 0.0]);
    }

    #[test]
    fn translate_transform() {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0.0..10000.0);
        let b = rng.gen_range(0.0..10000.0);
        let c = rng.gen_range(0.0..10000.0);

        let vec_random = Vector3D::new(a,b,c);

        let mut transform = Transform::new();
        transform.translate(vec_random);
        assert_eq!(transform.has_changed(), true);

        let new_matrix = transform.matrix();
        assert_eq!(new_matrix[3], [a,b,c,1.0]);
    }

    #[test]
    fn rotate_transform() {
        let mut transform = Transform::new();
        transform.rotate(1.0, 2.0, 0.5);
        let result = transform.matrix();

        let expected = [
            [-0.36520, 0.41245, 0.83458, 0.0],
            [-0.19951, 0.84099, -0.50292, 0.0],
            [-0.90930, -0.35018, -0.22485, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        for (i,row) in result.iter().enumerate() {
            for (j,num) in row.iter().enumerate() {
                assert_eq!(round_place(*num, 5), expected[i][j]);
            }
        }
    }
}

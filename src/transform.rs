use crate::math_utils::{self, Vector3D, Matrix3x4, invert_matrix, Quaternion};


// The struct will have getter and setter
// So that we can use a dirty flag to track any changes
#[derive(Debug)]
pub struct Transform {
    matrix: Matrix3x4,
    inverse_matrix: Matrix3x4,
    rotation: Quaternion,
    scale: Vector3D,
    dirty_flag: bool
}

impl Transform {
    pub fn new() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0], 
                [0.0, 1.0, 0.0], 
                [0.0, 0.0, 1.0], 
                [0.0, 0.0, 0.0]
            ],
            inverse_matrix: [
                [1.0, 0.0, 0.0], 
                [0.0, 1.0, 0.0], 
                [0.0, 0.0, 1.0], 
                [0.0, 0.0, 0.0]
            ],
            rotation: math_utils::IDENTITY_QUATERNION,
            scale: Vector3D::new(1,1,1),
            dirty_flag: false
        }
    }

    pub fn from_matrix(matrix: Matrix3x4) -> Result<Self, String>{
        Ok(
        Self {
            matrix,
            inverse_matrix: invert_matrix(&matrix)?,
            rotation: math_utils::IDENTITY_QUATERNION,
            scale: Vector3D::new(1,1,1),
            dirty_flag: false
        })
    }

    pub fn get_matrix(&self) -> Matrix3x4 {
       self.matrix 
    }

    pub fn get_inverse_matrix(&mut self) -> Matrix3x4 {
        if self.dirty_flag {
            self.inverse_matrix = invert_matrix(&self.matrix).unwrap();
        }

        self.inverse_matrix
    }

    #[inline]
    pub fn to_world_space(&self, coord: &Vector3D) -> Vector3D {
        fast_3x4_multiply(&self.matrix, coord)
    }

    #[inline]
    pub fn to_local_space(&self, coord: &Vector3D) -> Vector3D {
        fast_3x4_multiply(&self.inverse_matrix, coord)
    }

    pub fn translate(&mut self, amount: &Vector3D) -> () {
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

// Helps to convert between local and world coord. system
// Note that transform can also be the inverted version
#[inline]
fn fast_3x4_multiply(matrix: &Matrix3x4, point: &Vector3D) -> Vector3D {
    Vector3D { 
        x: matrix[0][0]*point.x + matrix[1][0]*point.y + matrix[2][0]*point.z + matrix[3][0],
        y: matrix[0][1]*point.x + matrix[1][1]*point.y + matrix[2][1]*point.z + matrix[3][1],
        z: matrix[0][2]*point.x + matrix[1][2]*point.y + matrix[2][2]*point.z + matrix[3][2],
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn round_place(num: f64, place: usize) -> f64{
        let mult = 10_f64.powf(place as f64);
        (num*mult).round() / mult
    }

    fn round_vector3d(vector: &Vector3D) -> [f64; 3] {
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
                [0.9, 0.4, 0.12],
                [0.7, 0.06, 0.5],
                [0.1, 0.4, 2.7],
                [10.0,12.0,11.0],
            ];
        let transform_b = Transform::from_matrix(matrix).unwrap();
        let result = transform_b.to_world_space(&mat_a);
        let rounded_result = round_vector3d(&result);

        assert_eq!(rounded_result, [17.8, 18.98, 45.5]);
    }

    #[test]
    fn transform_to_local() {
        let mat_a = Vector3D::new(5,10,2);
        let matrix = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [3.0, 4.0, 2.0]
        ];
        let transform_b = Transform::from_matrix(matrix).unwrap();
        let result = transform_b.to_local_space(&mat_a);
        let rounded_result = round_vector3d(&result);

        assert_eq!(rounded_result, [2.0, 6.0, 0.0]);
    }

    #[test]
    fn translate_transform() {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0.0..10000.0);
        let b = rng.gen_range(0.0..10000.0);
        let c = rng.gen_range(0.0..10000.0);

        let vec_random = &Vector3D::new(a,b,c);

        let mut transform = Transform::new();
        transform.translate(vec_random);

        let new_matrix = transform.get_matrix();
        assert_eq!(new_matrix[3], [a,b,c]);
    }
}

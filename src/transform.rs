use crate::math_utils::Vector3D;


const IDENTITY_MATRIX3X3: [[f64; 3]; 3] = [
    [1.0, 0.0, 0.0], 
    [0.0, 1.0, 0.0], 
    [0.0, 0.0, 1.0], 
];

type Matrix3x4 = [[f64; 3]; 4];

// The struct will have getter and setter
// So that we can use a dirty flag to track any changes
#[derive(Debug)]
pub struct Transform {
    matrix: Matrix3x4,
    inverse_matrix: Matrix3x4,
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
            dirty_flag: false
        }
    }

    pub fn from_matrix(matrix: Matrix3x4) -> Result<Self, String>{
        Ok(
        Self {
            matrix,
            inverse_matrix: Self::invert_matrix(&matrix)?,
            dirty_flag: false
        })
    }

    pub fn get_matrix(&self) -> Matrix3x4 {
       self.matrix 
    }

    pub fn get_inverse_matrix(&self) -> Matrix3x4 {
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

    fn invert_matrix(matrix: &Matrix3x4) -> Result<Matrix3x4, String> {
        const ROW: usize = 4;
        const COL: usize = 3;

        let matrix = matrix;

        let mut inv_matrix = [[0.0; 3]; 4];
        inv_matrix[..3].copy_from_slice(&IDENTITY_MATRIX3X3);

        for column in 0..COL {
            // Making sure pivot is a non-zero number
            // If zero, swap row with one that has the biggest absolute value
            let mut pivot = column;
            let mut pivot_val = matrix[column][column];

            if pivot_val == 0.0 {
                for curr_row in 0..ROW {
                    if matrix[curr_row][column].abs() > pivot_val.abs() {
                        pivot = curr_row;
                        pivot_val = matrix[curr_row][column];
                    }
                }

                if pivot_val == 0.0 { return Err("Matrix has no inverse".to_string()) }
                let mut tmp = matrix[pivot];
                matrix[pivot] = matrix[column];
                matrix[column] = tmp;

                tmp = inv_matrix[pivot];
                inv_matrix[pivot] = inv_matrix[column];
                inv_matrix[column] = tmp;


            }

            // Forward substitution
            for row_under_pivot in (column + 1)..ROW {
                // Refer to scratchapixel.com, under Gauss-Jordan Matrix Inverse
                // cuz I dunno wth I'm doing
                let multiplier = matrix[row_under_pivot][column] / pivot_val;
                for i in 0..COL {
                    matrix[row_under_pivot][i] -= multiplier * matrix[column][i];
                    inv_matrix[row_under_pivot][i] -= multiplier * inv_matrix[column][i];
                }

                matrix[row_under_pivot][column] = 0.0;
            }
        }

        // Divide each row to turn the pivot into 1
        for column in 0..COL {
            let divisor = matrix[column][column];
            for i in 0..COL {
                matrix[column][i] /= divisor;
                inv_matrix[column][i] /= divisor;
            }
            matrix[column][column] = 1.0;
        }

        // Backward substitution
        for row in 0..ROW {
            for column in (row+1)..COL {
                let constant = matrix[row][column];
                for i in 0..COL {
                    matrix[row][i] -= matrix[column][i] * constant;
                    inv_matrix[row][i] -= inv_matrix[column][i] * constant;
                }

                matrix[row][column] = 0.0;
            }
        }
        Ok(inv_matrix)
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
        let transform_b = Transform {
            translation: Vector3D::new(10.0,12.0,11.0),
            rotation: [
                [0.3, 0.4, 0.12],
                [0.7, 0.02, 0.5],
                [0.1, 0.4, 0.9],
            ],
            scale: 3.0 
        };

        let result = convert_space(&transform_b, &mat_a);
        let rounded_result = round_vector3d(&result);

        assert_eq!(rounded_result, [17.8, 18.98, 45.5]);
    }

    #[test]
    fn transform_to_local() {
        let mat_a = Vector3D::new(5,10,2);
        let transform_b = Transform {
            translation: Vector3D::new(3,4,2),
            rotation: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0]
            ],
            scale: 1.0
        };

        let result = convert_space(&transform_b.invert_matrix().unwrap(), &mat_a);
        let rounded_result = round_vector3d(&result);

        assert_eq!(rounded_result, [2.0, 6.0, 0.0]);
    }
}

use std::f64::consts::PI;
use std::ops::{Add, Sub, Mul};


#[derive(Debug, PartialEq)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn new<T: Into<f64>>(x: T, y: T, z: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into()
        }
    }
}


impl Add for Vector3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}


impl Sub for Vector3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

// Dot product for vector
impl Mul for Vector3D {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}


pub type Matrix3x4 = [[f64; 3]; 4];

pub const NIL_MATRIX3X4: Matrix3x4 = [
    [1.0, 0.0, 0.0], 
    [0.0, 1.0, 0.0], 
    [0.0, 0.0, 1.0], 
    [0.0, 0.0, 0.0]
];

pub fn invert_matrix(matrix: &Matrix3x4) -> Result<Matrix3x4, String> {
    const ROW: usize = 4;
    const COL: usize = 3;

    let mut matrix = matrix.clone();
    let mut inv_matrix = NIL_MATRIX3X4.clone();

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

#[derive(Debug)]
pub struct ProjectionData(pub f64, pub f64, pub f64, pub f64);

// I'm just gonna hard code the aspect ratio lol
impl ProjectionData {
    pub fn generate(n: f64, f: f64, fov: f64) -> ProjectionData {
        let fov_tan_val = (fov/2.0 * PI/180.0).tan();
        let near_far_interval = f - n;
        ProjectionData(
            1.0 / (n*fov_tan_val),
            1.0 / (n*fov_tan_val),
            -f / near_far_interval,
            -f*n / near_far_interval
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn round_place(num: f64, place: usize) -> f64{
        let mult = 10_f64.powf(place as f64);
        (num*mult).round() / mult
    }

    #[test]
    fn add_vector3d() {
        let a = Vector3D::new(2,5,10);
        let b = Vector3D::new(5,6,11);

        assert_eq!(a+b, Vector3D::new(7,11,21));
    }

    #[test]
    fn substract_vector3d() {
        let a = Vector3D::new(4, 20, 5);
        let b = Vector3D::new(8, 11, 7);

        assert_eq!(a-b, Vector3D::new(-4, 9, -2));
    }

    #[test]
    fn dot_product_vector3d() {
        let a = Vector3D::new(5,6,7);
        let b = Vector3D::new(10,2,3);

        assert_eq!(a*b, 83_f64);
    }

    #[test]
    fn invert_trs_matrix() {
        let mat = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [3.0, 10.0, 1.0]
        ];
        let result = invert_matrix(&mat).unwrap();
        assert_eq!(round_place(result[3][0], 2), -3.0);
        assert_eq!(round_place(result[3][1], 2), -10.0);
        assert_eq!(round_place(result[3][2], 2), -1.0);
    }
}

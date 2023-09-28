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
        let fov_tan_val = n * (fov/2.0 * PI/180.0).tan();
        let near_far_interval = f - n;
        ProjectionData(
            1.0 / (fov_tan_val),
            16.0 / (9.0*fov_tan_val),
            -f / near_far_interval,
            -f*n / near_far_interval
        )
    }
}

pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if !(min <= max) { panic!("min bigger than max")}

    if val > max {
        return max;
    }
    else if val < min {
        return min;
    }

    val
}

// Reminder: Quaternion(w,x,y,z)
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Quaternion(f64, f64, f64, f64);

impl Mul for Quaternion {
    type Output = Quaternion;

    // Another abomination for quaternion multiplication
    // Credit to https://paroj.github.io/gltut/Positioning/Tut08%20Quaternions.html
    fn mul(self, rhs: Self) -> Self::Output {
        Quaternion(
            self.0*rhs.0 - self.1*rhs.1 - self.2*rhs.2 - self.3*rhs.3,
            self.0*rhs.1 + self.1*rhs.0 + self.2*rhs.3 - self.3*rhs.2,
            self.0*rhs.2 + self.2*rhs.0 + self.3*rhs.1 - self.1*rhs.3,
            self.0*rhs.3 + self.3*rhs.0 + self.1*rhs.2 - self.2*rhs.1
        )
    }
}

impl Quaternion {
    pub fn from_euler_angles<X,Y,Z>(x: X, y: Y, z: Z) -> Self
    where
        X: Into<f64> + Copy,
        Y: Into<f64> + Copy,
        Z: Into<f64> + Copy,
    {
        // Got this abomination from Wikipedia lul
        // https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles
        let a = x.into() * 0.5;
        let ca = a.cos();
        let sa = a.sin();

        let b = y.into() * 0.5;
        let cb = b.cos();
        let sb = b.sin();

        let c = z.into() * 0.5; 
        let cc = c.cos();
        let sc = c.sin();

        Quaternion(
            cc*cb*ca + sc*sb*sa,
            cc*cb*sa - sc*sb*ca,
            cc*sb*ca + sc*cb*sa,
            sc*cb*ca - cc*sb*sa
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

    fn compare_quaternions(q: Quaternion, precision: usize,
        a2: f64, b2: f64, c2: f64, d2: f64) {
        let Quaternion(a1, b1, c1, d1) = q;

        dbg!(a1, b1, c1, d1);

        assert_eq!(round_place(a1, precision), a2);
        assert_eq!(round_place(b1, precision), b2);
        assert_eq!(round_place(c1, precision), c2);
        assert_eq!(round_place(d1, precision), d2);

    }

    #[test]
    fn quaternion_from_euler_angle_x_only() {
        let q = Quaternion::from_euler_angles(1,0,0);
        compare_quaternions(q, 5, 0.87758, 0.47943, 0.0, 0.0);
    }
    
    #[test]
    fn quaternion_from_euler_angle_y_only() {
        let q = Quaternion::from_euler_angles(0,1,0);
        compare_quaternions(q, 5, 0.87758, 0.0, 0.47943, 0.0);
    }

    #[test]
    fn quaternion_from_euler_angle_z_only() {
        let q = Quaternion::from_euler_angles(0,0,1);
        compare_quaternions(q, 5, 0.87758, 0.0, 0.0, 0.47943);
    }

    #[test]
    fn quaternion_from_euler_angle_all() {
        let q = Quaternion::from_euler_angles(1,1,1);
        compare_quaternions(q, 5, 0.56568, 0.57094, 0.16752, 0.57094);
    }

    #[test]
    fn quaternion_multiply() {
        let q1 = Quaternion(1.0, 0.4, 0.5, 0.6);
        let q2 = Quaternion(2.0, 0.2, 0.34, 0.79);

        let res1 = q1 * q2;
        let res2 = q2 * q1;

        compare_quaternions(res1, 3, 1.276, 1.191, 1.144, 2.026);
        compare_quaternions(res2, 3, 1.276, 0.809, 1.536, 1.954);
        assert_ne!(res1, res2);
    }
}

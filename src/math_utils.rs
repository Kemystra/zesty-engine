use std::ops::{Add, Sub};

use crate::{Matrix2D, Matrix3D};


impl Add for Matrix3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        [
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2]
        ]
    }
}

impl Sub<Matrix3D> for Matrix3D {
    type Output = Matrix3D;

    fn add(self, rhs: Matrix3D) -> Self::Output {
        let x = self[0] - rhs[0];
        let y = self[1] - rhs[1];
        let z = self[2] - rhs[2];

        [x,y,z]
    }
}

use std::ops::{Add, Sub, Mul};


#[derive(Debug, PartialEq)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
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


#[cfg(test)]
mod tests {
    use super::*;

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
}

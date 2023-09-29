use std::ops::{Mul, MulAssign};

// Reminder: Quaternion(w,x,y,z)
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Quaternion(f64, f64, f64, f64);
pub const IDENTITY_QUATERNION: Quaternion = Quaternion(1.0, 0.0, 0.0, 0.0);

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

impl MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Quaternion {
    pub fn from_euler_angles<T: Into<f64>>(x: T, y: T, z: T) -> Self
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

    pub fn update_3x4_matrix(&self, matrix: &mut Matrix3x4, scale: &Vector3D) {
        let wx = self.0 * self.1 * 2.0;
        let wy = self.0 * self.2 * 2.0;
        let wz = self.0 * self.3 * 2.0;

        let xx = self.1 * self.1 * 2.0;
        let xy = self.1 * self.2 * 2.0;
        let xz = self.1 * self.3 * 2.0;

        let yy = self.2 * self.2 * 2.0;
        let yz = self.2 * self.3 * 2.0;

        let zz = self.3 * self.3 * 2.0;

        matrix[0][0] = (1.0 - yy - zz) * scale.x;
        matrix[0][1] = xy - wz;
        matrix[0][2] = xz + wy;
        matrix[1][0] = xy + wz;
        matrix[1][1] = (1.0 - xx - zz) * scale.y;
        matrix[1][2] = yz - wx;
        matrix[2][0] = xz - wy;
        matrix[2][1] = yz + wx;
        matrix[2][2] = (1.0 - xx - yy) * scale.z;
    }

    pub fn lazy_normalize(&mut self) {
        let magnitude_sq = (self.0*self.0)+(self.1*self.1)+(self.2*self.2)+(self.3*self.3);

        // Check if squared magnitude is off by less than 0.21
        // Why? Cuz 0.1^2 = 0.21
        // Why 0.1? Dunno
        if (1.0 - magnitude_sq).abs() < 0.21 { return }
        let magnitude = magnitude_sq.sqrt();

        self.0 /= magnitude;
        self.1 /= magnitude;
        self.2 /= magnitude;
        self.3 /= magnitude;
    }
}

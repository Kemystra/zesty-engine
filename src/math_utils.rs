use std::f64::consts::PI;

pub mod vector3d;
pub mod matrix3x4;
pub mod quaternion;

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

pub fn round_place(num: f64, place: usize) -> f64{
    let mult = 10_f64.powf(place as f64);
    (num*mult).round() / mult
}


#[cfg(test)]
mod tests {
    use super::*;

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

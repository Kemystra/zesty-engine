pub mod vector3d;
pub mod matrix4x4;
pub mod quaternion;

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




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_in_interval() {
        let raw_num = 10;
        let result = clamp(raw_num, 1, 100);

        assert_eq!(result, 10);
    }

    #[test]
    fn clamp_below_interval() {
        let raw_num = 5;
        let result = clamp(raw_num, 10, 100);

        assert_eq!(result, 10);
    }

    #[test]
    fn clamp_above_interval() {
        let raw_num = 6;
        let result = clamp(raw_num, 1, 5);
        
        assert_eq!(result, 5);
    }

    #[test]
    #[should_panic(expected = "min bigger than max")]
    fn clamp_panic() {
        let _ = clamp(9, 10, 7);
    }
}

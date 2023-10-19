pub mod vector3d;
pub mod matrix3x4;
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

pub fn round_place(num: f64, place: usize) -> f64{
    let mult = 10_f64.powf(place as f64);
    (num*mult).round() / mult
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_to_whole() {
        let fl = 12.45;
        let result = round_place(fl, 0);

        assert_eq!(result, 12_f64);
    }

    #[test]
    fn round_forward() {
        let fl = 12.55;
        let result = round_place(fl, 0);

        assert_eq!(result, 13_f64);
    }

    #[test]
    fn round_one_place() {
        let fl = 45.892;
        let result = round_place(fl, 1);

        assert_eq!(result, 45.9_f64);
    }

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
        let result = clamp(9, 10, 7);
    }
}

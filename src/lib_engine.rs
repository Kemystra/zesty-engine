pub mod transform;
pub mod math_utils;
pub mod object;
pub mod component;
pub mod scene;
pub mod renderer;

#[cfg(test)]
pub mod test_utils {
    pub fn round_place(num: f64, place: usize) -> f64{
        let mult = 10_f64.powf(place as f64);
        (num*mult).round() / mult
    }
}

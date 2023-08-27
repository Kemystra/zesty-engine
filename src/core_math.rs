pub struct Transform {
    translation: [f64; 3],
    rotation: [[f64; 3]; 3],
    scale: f64
}

impl Transform {
    pub fn translate(&mut self, x: f64, y: f64, z: f64) -> () {
        self.translation += [x,y,z];
    }

    pub fn scale(&mut self, scale: f64) -> () {
        self.scale *= scale;
    }

    pub fn rotate() -> () {
        
    }
}

pub fn local_to_world_coord(
    transform: &Transform, 
    local_coord: &[f64; 3]) -> [f64; 3] {

    let mut world_coord = [0.0; 3];
    for i in 0..3 {
        let mut sum = 0.0;
        for j in 0..3 {
            let norm_sum = local_coord[j]*transform.rotation[j][i];
            sum += if i==j {norm_sum*transform.scale} else {norm_sum};
        }
        sum += transform.translation[i];
        world_coord[i] = sum;
    }

    world_coord
}


fn round_place(num: f64, place: usize) -> f64{
    let mult = 10_f64.powf(place as f64);
    (num*mult).round() / mult
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_to_world() {
        let mat_a = [5.0,3.0,12.0];
        let transform_b = Transform {
            translation: [10.0,12.0,11.0],
            rotation: [
                [0.3, 0.4, 0.12],
                [0.7, 0.02, 0.5],
                [0.1, 0.4, 0.9],
            ],
            scale: 3.0 
        };

        let result = local_to_world_coord(&transform_b, &mat_a);
        let mut rounded_result = [0.0_f64; 3];
        for (i,&n) in result.iter().enumerate() {rounded_result[i] = round_place(n, 2)};

        assert_eq!(rounded_result, [17.8, 18.98, 45.5]);
    }
}

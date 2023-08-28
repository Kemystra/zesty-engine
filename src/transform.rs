use crate::math_utils::Vector3D;


#[derive(Debug)]
pub struct Transform {
    pub translation: Vector3D,
    pub rotation: [[f64; 3]; 3],
    pub scale: f64
}


pub fn local_to_world_coord(
    transform: &Transform, 
    local_coord: &Vector3D) -> Vector3D {

    Vector3D {
        x: local_coord.x*transform.rotation[0][0]*transform.scale +
            local_coord.y*transform.rotation[1][0] +
            local_coord.z*transform.rotation[2][0] + transform.translation.x,
        y: local_coord.x*transform.rotation[0][1] +
            local_coord.y*transform.rotation[1][1]*transform.scale +
            local_coord.z*transform.rotation[2][1] + transform.translation.y,
        z: local_coord.x*transform.rotation[0][2] +
            local_coord.y*transform.rotation[1][2] +
            local_coord.z*transform.rotation[2][2]*transform.scale + transform.translation.z,
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
    fn transform_to_world() {
        let mat_a = Vector3D::new(5.0,3.0,12.0);
        let transform_b = Transform {
            translation: Vector3D::new(10.0,12.0,11.0),
            rotation: [
                [0.3, 0.4, 0.12],
                [0.7, 0.02, 0.5],
                [0.1, 0.4, 0.9],
            ],
            scale: 3.0 
        };

        let result = local_to_world_coord(&transform_b, &mat_a);
        let rounded_result = [
            round_place(result.x, 2),
            round_place(result.y, 2),
            round_place(result.z, 2)
        ];

        assert_eq!(rounded_result, [17.8, 18.98, 45.5]);
    }
}

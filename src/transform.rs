use std::mem;

use crate::math_utils::Vector3D;


const IDENTITY_TRANSFORM: [[f64; 3]; 4] = [
    [1.0, 0.0, 0.0], 
    [0.0, 1.0, 0.0], 
    [0.0, 0.0, 1.0], 
    [0.0, 0.0, 0.0], 
];


#[derive(Debug)]
pub struct Transform {
    pub translation: Vector3D,
    pub rotation: [[f64; 3]; 3],
    pub scale: f64
}

// Helps to convert between local and world coord. system
// Note that transform can also be the inverted version
fn convert_coord_system(
    transform: &Transform, 
    coord: &Vector3D) -> Vector3D {

    Vector3D {
        x: coord.x*transform.rotation[0][0]*transform.scale +
            coord.y*transform.rotation[1][0] +
            coord.z*transform.rotation[2][0] + transform.translation.x,
        y: coord.x*transform.rotation[0][1] +
            coord.y*transform.rotation[1][1]*transform.scale +
            coord.z*transform.rotation[2][1] + transform.translation.y,
        z: coord.x*transform.rotation[0][2] +
            coord.y*transform.rotation[1][2] +
            coord.z*transform.rotation[2][2]*transform.scale + transform.translation.z,
    }
}


fn invert_transform(transform: &Transform) -> Result<Transform, &str> {

    const ROW: usize = 4;
    const COL: usize = 3;

    // Load into matrix
    let mut matrix = [[0.0; COL]; ROW];
    matrix[..3].clone_from_slice(&transform.rotation);
    matrix[3][0] = transform.translation.x;
    matrix[3][1] = transform.translation.y;
    matrix[3][2] = transform.translation.z;

    let mut inv_matrix = [[0.0; 3]; 4];
    inv_matrix.copy_from_slice(&IDENTITY_TRANSFORM);

    for i in 0..ROW {
        matrix[i][i] *= transform.scale;
    }

    for column in 0..COL {
        // Making sure pivot is a non-zero number
        // If zero, swap row with one that has the biggest absolute value
        let pivot = column;
        let pivot_val = matrix[column][column];

        if pivot_val == 0.0 {
            for curr_row in 0..ROW {
                if matrix[curr_row][column].abs() > pivot_val.abs() {
                    pivot = curr_row;
                    pivot_val = matrix[curr_row][column];
                }
            }

            if pivot_val == 0.0 { return Err("Matrix has no inverse") }
            mem::swap(&mut matrix[column], &mut matrix[pivot]);
            mem::swap(&mut inv_matrix[column], &mut inv_matrix[pivot]);
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
            for i in 0..ROW {
                matrix[row][i] -= matrix[column][i] * constant;
                inv_matrix[row][i] -= inv_matrix[column] * constant;
            }

            matrix[row][column] = 0.0;
        }
    }

    Ok(transform)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn round_place(num: f64, place: usize) -> f64{
        let mult = 10_f64.powf(place as f64);
        (num*mult).round() / mult
    }

    fn round_vector3d(vector: &Vector3D) -> [f64; 3] {
        [
            round_place(vector.x, 2),
            round_place(vector.y, 2),
            round_place(vector.z, 2)
        ]
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
        let rounded_result = round_vector3d(&result);

        assert_eq!(rounded_result, [17.8, 18.98, 45.5]);
    }

    #[test]
    fn transform_to_local() {
        let mat_a = Vector3D::new(5,10,2);
        let mut transform_b = Transform {
            translation: Vector3D::new(3,4,2),
            rotation: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0]
            ],
            scale: 1.0
        };

        let result = transform_b.world_to_local_coord(&mat_a);
        let rounded_result = round_vector3d(&result);

        assert_eq!(rounded_result, [2.0, 6, 0, 1]);
    }
}

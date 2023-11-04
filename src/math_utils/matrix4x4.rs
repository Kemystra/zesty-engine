use super::vector3d::Vector3D;


pub type Matrix4x4 = [[f64; 4]; 4];

pub const IDENTITY_MATRIX4X4: Matrix4x4 = [
    [1.0, 0.0, 0.0, 0.0], 
    [0.0, 1.0, 0.0, 0.0], 
    [0.0, 0.0, 1.0, 0.0], 
    [0.0, 0.0, 0.0, 1.0]
];

pub fn invert_matrix(matrix: &Matrix4x4, ignore_4th_col: bool) -> Result<Matrix4x4, String> {
    let row: usize = 4;
    let col: usize = if ignore_4th_col { 3 } else { 4 };

    let mut matrix = matrix.clone();
    let mut inv_matrix = IDENTITY_MATRIX4X4.clone();

    for column in 0..col {
        // Making sure pivot is a non-zero number
        // If zero, swap row with one that has the biggest absolute value
        let mut pivot = column;
        let mut pivot_val = matrix[column][column];

        if pivot_val == 0.0 {
            for curr_row in 0..row {
                if matrix[curr_row][column].abs() > pivot_val.abs() {
                    pivot = curr_row;
                    pivot_val = matrix[curr_row][column];
                }
            }

            if pivot_val == 0.0 { return Err("Matrix has no inverse".to_string()) }
            let mut tmp = matrix[pivot];
            matrix[pivot] = matrix[column];
            matrix[column] = tmp;

            tmp = inv_matrix[pivot];
            inv_matrix[pivot] = inv_matrix[column];
            inv_matrix[column] = tmp;
        }

        // Forward substitution
        for row_under_pivot in (column + 1)..row {
            // Refer to scratchapixel.com, under Gauss-Jordan Matrix Inverse
            // cuz I dunno wth I'm doing
            let multiplier = matrix[row_under_pivot][column] / pivot_val;
            for i in 0..col {
                matrix[row_under_pivot][i] -= multiplier * matrix[column][i];
                inv_matrix[row_under_pivot][i] -= multiplier * inv_matrix[column][i];
            }

            matrix[row_under_pivot][column] = 0.0;
        }
    }

    // Divide each row to turn the pivot into 1
    for column in 0..col {
        let divisor = matrix[column][column];
        for i in 0..col {
            matrix[column][i] /= divisor;
            inv_matrix[column][i] /= divisor;
        }
        matrix[column][column] = 1.0;
    }

    // Backward substitution
    for row in 0..row {
        for column in (row+1)..col {
            let constant = matrix[row][column];
            for i in 0..col {
                matrix[row][i] -= matrix[column][i] * constant;
                inv_matrix[row][i] -= inv_matrix[column][i] * constant;
            }

            matrix[row][column] = 0.0;
        }
    }
    Ok(inv_matrix)
}

pub fn vector_matrix_multiply(matrix: &Matrix4x4, vector: Vector3D, non_homogeneous: bool) -> Vector3D {
    let x = matrix[0][0]*vector.x + matrix[1][0]*vector.y + matrix[2][0]*vector.z + matrix[3][0];
    let y = matrix[0][1]*vector.x + matrix[1][1]*vector.y + matrix[2][1]*vector.z + matrix[3][1];
    let z = matrix[0][2]*vector.x + matrix[1][2]*vector.y + matrix[2][2]*vector.z + matrix[3][2];

    if non_homogeneous {
        return Vector3D::new(x, y, z)
    }

    let w = matrix[0][3]*vector.x + matrix[1][3]*vector.y + matrix[2][3]*vector.z + matrix[3][3];
    Vector3D {
        x: x/w,
        y: y/w,
        z: z/w
    }
}

pub fn matrix_multiply(matrix1: &Matrix4x4, matrix2: &Matrix4x4) -> Matrix4x4 {
    let mut result = IDENTITY_MATRIX4X4.clone();
    const SIZE: usize = 4;

    for x in 0..SIZE {
        for y in 0..SIZE {
            let mut sum = 0.0;
            for num in 0..4 {
                sum += matrix1[y][num] * matrix2[num][x];
                result[y][x] = sum;
            }
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::round_place;

    fn compare_matrices(mat1: &Matrix4x4, mat2: &Matrix4x4, precision: usize) {
        let flat_rounded_mat1 = mat1.iter().flatten().map(|x| round_place(*x, precision));
        let flat_rounded_mat2 = mat2.iter().flatten().map(|x| round_place(*x, precision));

        assert!(flat_rounded_mat1.eq(flat_rounded_mat2));
    }

    #[test]
    fn invert_trs_matrix() {
        let mat = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [3.0, 10.0, 1.0, 1.0]
        ];
        let result = invert_matrix(&mat, true).unwrap();
        assert_eq!(round_place(result[3][0], 2), -3.0);
        assert_eq!(round_place(result[3][1], 2), -10.0);
        assert_eq!(round_place(result[3][2], 2), -1.0);
    }

    #[test]
    fn invert_whole_matrix() {
        let matrix: Matrix4x4 = [
            [1.0, 0.5, 3.0, 0.0],
            [5.0, 6.0, 0.6, 0.0],
            [3.0, 1.0, 11.0, -12.0],
            [13.0, 14.0, 15.0, 2.0]
        ];

        let expected_result = [
            [3.85953, 1.42211, -0.12309, -0.73853],
            [-3.17385, -0.98748, 0.10014, 0.60083],
            [-0.4242, -0.30946, 0.02434, 0.14604],
            [0.31154, -0.01043, -0.08345, -0.0007]
        ];

        let result = invert_matrix(&matrix, false).unwrap();
        compare_matrices(&expected_result, &result, 5);
    }

    #[test]
    fn vector_multiply_normal() {
        let matrix = [
            [1.0, 2.0, 3.0, 0.0],
            [4.0, 5.0, 6.0, 0.0],
            [7.0, 8.0, 9.0, 0.0],
            [10.0, 11.0, 12.0, 1.0],
        ];
        let vector = Vector3D::new(1,2,3);

        let result = vector_matrix_multiply(&matrix, vector, true);
        assert_eq!(result, Vector3D::new(40, 47, 54));
    }

    #[test]
    fn vector_multiply_homogeneous() {
        let matrix = [
            [1.0, 2.0, 3.0, 2.0],
            [4.0, 5.0, 6.0, 5.0],
            [7.0, 8.0, 9.0, 9.0],
            [10.0, 11.0, 12.0, 1.0],
        ];
        let vector = Vector3D::new(1,2,3);

        let result = vector_matrix_multiply(&matrix, vector, false);
        assert_eq!(result, Vector3D::new(1.0, 1.175, 1.35));
    }

    #[test]
    fn matrix_multiply_trs() {
        let matrix = [
            [1.0, 2.0, 3.0, 0.0],
            [4.0, 5.0, 6.0, 0.0],
            [7.0, 8.0, 9.0, 0.0],
            [10.0, 11.0, 12.0, 1.0],
        ];

        let result = matrix_multiply(&matrix, &matrix);

        let expected_result = [
            [30.0, 36.0, 42.0, 0.0],
            [66.0, 81.0, 96.0, 0.0],
            [102.0, 126.0, 150.0, 0.0],
            [148.0, 182.0, 216.0, 1.0],
        ];

        dbg!("{:?}", result);

        compare_matrices(&result, &expected_result, 1);
    }

    #[test]
    fn matrix_multiply_whole() {
        let matrix = [
            [1.0, 2.0, 3.0, 5.0],
            [4.0, 5.0, 6.0, 0.0],
            [7.0, 8.0, 9.0, 3.0],
            [10.0, 11.0, 12.0, 1.0],
        ];

        let result = matrix_multiply(&matrix, &matrix);

        let expected_result = [
            [80.0, 91.0, 102.0, 19.0],
            [66.0, 81.0, 96.0, 38.0],
            [132.0, 159.0, 186.0, 65.0],
            [148.0, 182.0, 216.0, 87.0],
        ];

        compare_matrices(&result, &expected_result, 1);
    }
}

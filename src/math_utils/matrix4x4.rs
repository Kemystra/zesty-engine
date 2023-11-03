pub type Matrix4x4 = [[f64; 4]; 4];

pub const NIL_MATRIX4X4: Matrix4x4 = [
    [1.0, 0.0, 0.0, 0.0], 
    [0.0, 1.0, 0.0, 0.0], 
    [0.0, 0.0, 1.0, 0.0], 
    [0.0, 0.0, 0.0, 0.0]
];

pub fn invert_matrix(matrix: &mut Matrix4x4, ignore_4th_col: bool) -> Result<(), String> {
    let row: usize = 4;
    let col: usize = if ignore_4th_col { 3 } else { 4 };

    let mut inv_matrix = NIL_MATRIX4X4.clone();

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
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::round_place;

    #[test]
    fn invert_trs_matrix() {
        let mut mat = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [3.0, 10.0, 1.0, 1.0]
        ];

        invert_matrix(&mut mat, true).unwrap();
        assert_eq!(round_place(mat[3][0], 2), -3.0);
        assert_eq!(round_place(mat[3][1], 2), -10.0);
        assert_eq!(round_place(mat[3][2], 2), -1.0);
    }

    #[test]
    fn invert_whole_matrix() {
        let mut matrix: Matrix4x4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ];

        let result = [
            [3.85953, 1.42211, -0.12309, -0.73853],
            [-3.17385, -0.98748, 0.10014, 0.60083],
            [-0.4242, -0.30946, 0.02434, 0.14604],
            [0.31154, -0.01043, -0.08345, -0.0007]
        ];

        invert_matrix(&mut matrix, false).unwrap();
        assert_eq!(matrix, result);
    }
}

/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place.
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/
use std::cmp::max;
use std::fmt::{self, Display, Formatter};

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    let m = matrix[0].len();

    let n_f = max(n, m);

    if n > m {
        for _ in n - m..m {
            for j in 0..n {
                matrix[j].append(&mut vec![0; n - m])
            }
        }
    } else if m > n {
        for _ in m - n..m {
            matrix.push(vec![0; m]);
        }
    }

    // 1. 转置矩阵
    for i in 0..n_f {
        for j in i + 1..n_f {
            // 交换 matrix[i][j] 和 matrix[j][i]
            (matrix[i][j], matrix[j][i]) = (matrix[j][i], matrix[i][j]);
        }
    }

    println!("append 1 matrix: {:?}", matrix);

    // 2. 每一行进行反转
    for i in 0..n_f {
        matrix[i].reverse();
    }

    if n > m {
        for _ in 0..n - m {
            matrix.pop();
        }
    } else if m > n {
        for i in 0..n_f {
            for _ in 0..m - n {
                matrix[i].remove(0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3],]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![3, 1], vec![4, 2],]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![vec![1]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![1],]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![vec![5, 3, 1], vec![6, 4, 2],]);

        // 1 2 0
        // 3 4 0
        // 5 6 0
        //
        // 1 3 5
        // 2 4 6
        // 0 0 0
        //
        // 5 3 1
        // 6 4 2
    }
}

/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number.
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.

    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

// 矩阵乘法
fn matrix_multiply(a: [[i32; 2]; 2], b: [[i32; 2]; 2]) -> [[i32; 2]; 2] {
    [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0], // 计算第一行第一列
            a[0][0] * b[0][1] + a[0][1] * b[1][1], // 计算第一行第二列
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0], // 计算第二行第一列
            a[1][0] * b[0][1] + a[1][1] * b[1][1], // 计算第二行第二列
        ],
    ]
}

// 矩阵快速幂
fn matrix_pow(matrix: [[i32; 2]; 2], mut n: i32) -> [[i32; 2]; 2] {
    let mut result = [[1, 0], [0, 1]]; // 单位矩阵
    let mut base = matrix; // 基础矩阵

    while n > 0 {
        if n % 2 == 1 {
            result = matrix_multiply(result, base); // result = result * base
        }
        base = matrix_multiply(base, base); // base = base * base
        n /= 2; // n /= 2
    }

    result
}

pub fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let fib_matrix = [[1, 1], [1, 0]]; // 定义斐波那契的基础矩阵
    let result_matrix = matrix_pow(fib_matrix, n - 1); // 计算矩阵的 n-1 次幂
    result_matrix[0][0] // 返回 F(n)，即矩阵的 [0][0] 元素
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}

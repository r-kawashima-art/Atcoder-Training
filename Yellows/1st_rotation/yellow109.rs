use proconio::input;
// yukicoder No.526
// Q. Remainder of Nth Fibonacci number modulo M
// A. Matrix exponentiation: 行列累積乗
fn main() {
    input!{n: usize, m: usize}
    let base_matrix: [[usize; 2]; 2] = [[1, 1], [1, 0]];
    let result_matrix: [[usize; 2]; 2] = matrix_pow(base_matrix, n, m);
    println!("{}", result_matrix[1][1] % m);
}

fn matrix_pow(matrix: [[usize; 2]; 2], mut exponent: usize, modulus: usize) -> [[usize; 2]; 2] {
    let mut result: [[usize; 2]; 2] = [[1, 0], [0, 1]];
    let mut base: [[usize; 2]; 2] = matrix;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = matrix_mult(result, base, modulus);
        }
        base = matrix_mult(base, base, modulus);
        exponent /= 2;
    }
    return result;
}

fn matrix_mult(mat1: [[usize; 2]; 2], mat2: [[usize; 2]; 2], modulus: usize) -> [[usize; 2]; 2] {
    let mut result: [[usize; 2]; 2] = [[0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let prod = (mat1[i][k] as u128) * (mat2[k][j] as u128);
                result[i][j] = ((result[i][j] as u128 + prod) % (modulus as u128)) as usize;
            }
        }
    }
    return result;
}

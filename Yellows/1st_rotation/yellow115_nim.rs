use proconio::input;
use std::collections::HashSet;
// AtCoder Regular Contest 013 C
// Q. There is a cube of size X x Y x Z. Players A and B pick a cube (1x1x1). The player who takes prohibited cubes loses.
// A. Nim or Graundy number.
fn main() {
    input! {n: usize}
    let mut nim_sum: usize = 0;
    for _ in 0..n {
        input! {X: usize, Y: usize, Z: usize, m: usize, xyz: [(usize, usize, usize); m]}
        let (x_min, x_max) = (xyz.iter().map(|(x, _, _)| *x).min().unwrap(), xyz.iter().map(|(x, _, _)| *x).max().unwrap());
        let (y_min, y_max) = (xyz.iter().map(|(_, y, _)| *y).min().unwrap(), xyz.iter().map(|(_, y, _)| *y).max().unwrap());
        let (z_min, z_max) = (xyz.iter().map(|(_, _, z)| *z).min().unwrap(), xyz.iter().map(|(_, _, z)| *z).max().unwrap());
        nim_sum ^= x_min;
        nim_sum ^= X - x_max - 1;
        nim_sum ^= y_min;
        nim_sum ^= Y - y_max - 1;
        nim_sum ^= z_min;
        nim_sum ^= Z - z_max - 1;
    }
    if nim_sum == 0 {
        println!("LOSE");
    } else {
        println!("WIN");
    }
}

fn get_mex() {}

fn get_grundy() {}

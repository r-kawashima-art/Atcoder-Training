use proconio::input;
use std::collections::HashSet;
// AtCoder Regular Contest 013 C
// Q. There is a cube of size X x Y x Z. Players A and B pick a cube (1x1x1). The player who takes prohibited cubes loses.
// A. Nim or Graundy number.
fn main() {
    input! {n: usize}
    let mut nim_sum: usize = 0;
    let grundy_numbers: Vec<usize> = get_grundy(100); // Precompute Grundy numbers for pile sizes up to 10000.
    for _ in 0..n {
        input! {X: usize, Y: usize, Z: usize, m: usize, xyz: [(usize, usize, usize); m]}
        let (x_min, x_max) = (xyz.iter().map(|(x, _, _)| *x).min().unwrap(), xyz.iter().map(|(x, _, _)| *x).max().unwrap());
        let (y_min, y_max) = (xyz.iter().map(|(_, y, _)| *y).min().unwrap(), xyz.iter().map(|(_, y, _)| *y).max().unwrap());
        let (z_min, z_max) = (xyz.iter().map(|(_, _, z)| *z).min().unwrap(), xyz.iter().map(|(_, _, z)| *z).max().unwrap());
        let pile: [usize; 6] = [
            x_min,
            X - x_max - 1,
            y_min,
            Y - y_max - 1,
            z_min,
            Z - z_max - 1
        ];
        for &pile_size in &pile {
            nim_sum ^= grundy_numbers[pile_size];
        }
    }
    if nim_sum == 0 {
        println!("LOSE");
    } else {
        println!("WIN");
    }
}

fn get_mex(reachable: &HashSet<usize>) -> usize {
    /* Returns the Minimum EXcludant (mex) of a set.
    mex(S) = smallest non-negative integer NOT in S. */
    let mut mex: usize = 0;
    while reachable.contains(&mex) {
        mex += 1;
    }
    return mex;
}

fn get_grundy(max_pile_size: usize) -> Vec<usize> {
    let mut grundy_numbers = vec![0; max_pile_size + 1];
    for pile_size in 1..=max_pile_size {
        // 1. Identify all states reachable from the current state.
        // In this game, you can change a pile of size `pile_size` to any `next_size` < `pile_size`.
        let mut reachable_grundy_values = HashSet::new();
        for next_size in 0..pile_size {
            // 2. Collect the Grundy values of those next states.
            // For standard Nim, we know get_grundy(next_size) == next_size.
            reachable_grundy_values.insert(next_size);
        }

        // 3. The Grundy value is the mex of the reachable values.
        grundy_numbers[pile_size] = get_mex(&reachable_grundy_values);
    }
    grundy_numbers
}

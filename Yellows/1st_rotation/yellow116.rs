use proconio::input;
use std::collections::HashSet;
const LIMIT: usize = 100;
// AOJ 0401
// Q. The recurrence formula is as follows:
// (w-b) -> (w-1, b), (w, b-k), (w+1, b-1)
// A. nim value g(w, b)
// Sprague–Grundy theorem. nim sum.
fn main() {
    input! {n: usize, wb: [(usize, usize); n]}
    let mut nim_sum: i64 = 0;
    // dp[white][black]
    let mut dp: Vec<Vec<i64>> = vec![vec![-1; LIMIT + 1]; 2 * LIMIT + 1];
    for i in 0..n {
        let (w, b) = wb[i];
        let grundy_number: i64 = get_grundy(w, b, &mut dp);
        nim_sum ^= grundy_number;
    }

    if nim_sum == 0 {
        println!("1");
    } else {
        println!("0");
    }
}
fn get_mex(reachable: &HashSet<i64>) -> i64 {
    let mut mex: i64 = 0;
    while reachable.contains(&mex) {
        mex += 1;
    }
    return mex;
}

fn get_grundy(w: usize, b: usize, dp: &mut Vec<Vec<i64>>) -> i64 {
    /*Calculates the grundy number based on the topological sort
    g(w, b) = mex({g(w-1, b), g(w, b-k), g(w+1, b-1)})
    with memoization and topological order (b -> w)
     */
    if w >= 2 * LIMIT || b >= LIMIT {
        return 0;
    }
    if dp[w][b] != -1 {
        return dp[w][b];
    }

    for b in 0..=LIMIT {
        for w in 0..=2 * LIMIT {
            let mut reachable: HashSet<i64> = HashSet::new();
            // case (w, b) -> (w-1, b)
            if w >= 1 {
                reachable.insert(get_grundy(w - 1, b, dp));
            }
            // case (w, b) -> (w, b-k)
            for k in 1..=w {
                if b >= k {
                    reachable.insert(get_grundy(w, b - k, dp));
                }
            }
            // case (w, b) -> (w+1, b-1)
            if b >= 1 {
                reachable.insert(get_grundy(w + 1, b - 1, dp));
            }
            let mex = get_mex(&reachable);
            dp[w][b] = mex;
        }
    }
    return dp[w][b];
}

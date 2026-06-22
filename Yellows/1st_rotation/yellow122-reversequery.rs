use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;
// 全国統一プログラミング王決定戦本戦 D - Deforestation
// Q. Every Tree grows at 1 unit per second. We can cut trees in range [L, R] at a query's time.
// A. Reverse Query + BTreeSet + Greedy
fn main() {
    input! {
        n: usize,
        m: usize,
        queries: [(u64, Usize1, Usize1); m],
    }

    // Keep track of indices that have not been cut by future events
    let mut uncut: BTreeSet<usize> = (0..n).collect();
    let mut total_score: u64 = 0;

    // Process events in reverse order of time
    for &(time, left_idx, right_idx) in queries.iter().rev() {
        // Find all uncut indices in the range [left_idx, right_idx]
        let to_remove: Vec<usize> = uncut.range(left_idx..=right_idx).cloned().collect();
        
        for idx in to_remove {
            uncut.remove(&idx);
            total_score += time;
        }
    }

    println!("{}", total_score);
}

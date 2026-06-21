use proconio::input;
// ABC463D
// Q. What is the maximum minimum distance between any two selected points in a line, such that at least K points are selected?
// = Max of minimum
// A. Binary Searching by setting the minimum distance.
fn main() {
    input! {n: usize, k: usize, mut lr: [(i64, i64); n]}
    lr.sort_by_key(|tup| tup.1);
    // boundary condition checking
    if count_available(1, &lr) < k as i64 {
        println!("-1");
        return;
    }
    let mut score_l: i64 = -1;
    let mut score_r: i64 = 1 << 30;
    while score_r - score_l > 1 {
        let score_mid: i64 = (score_l + score_r) / 2;
        if count_available(score_mid, &lr) >= k as i64 {
            score_l = score_mid;
        } else {
            score_r = score_mid;
        }
    }
    println!("{}", score_l);
}

fn count_available(score: i64, lr: &Vec<(i64, i64)>) -> i64 {
    let n: usize = lr.len();
    if n == 0 {
        return 0;
    }
    let mut count: i64 = 1;
    let mut cur_pos: i64 = lr[0].1;
    for i in 1..n {
        if lr[i].0 - cur_pos >= score {
            count += 1;
            cur_pos = lr[i].1;
        }
    }
    count
}

use proconio::{input, marker::Usize1};

fn main() {
    input!{n: usize, q: usize, queries: [(usize, Usize1); n]}
    let mut row_poteintial: Vec<usize> = vec![n; n];
    let mut ans: usize = 0;
    for &(query_type, row_col) in queries.iter() {
        if query_type == 1 {
            ans += row_poteintial[row_col];
            row_poteintial[row_col] = 0;
        } else {
            
        }
    }
}

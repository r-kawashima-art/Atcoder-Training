use proconio::{input, marker::Chars};
fn main() {
    input!{h: usize, w: usize, k: usize, s: [Chars; h]}
    let mut prefix: Vec<Vec<usize>> = vec![vec![0; w + 1]; h + 1];
    for i in 1..=h {
        for j in 1..=w {
            if s[i-1][j-1] == '1' {
                prefix[i][j] = 1;
            }
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            prefix[i][j] = prefix[i - 1][j] + prefix[i][j];
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            prefix[i][j] += prefix[i][j-1];
        }
    }
    let mut ans: u64 = 0;
    let mut counts = vec![0u64; h * w + 1];
    for r1 in 1..=h {
        for r2 in r1..=h {
            counts[0] = 1;
            for c in 1..=w {
                let val = prefix[r2][c] - prefix[r1-1][c];
                if val >= k {
                    ans += counts[val - k];
                }
                counts[val] += 1;
            }
            // Reset frequency counts for the next row iteration
            for c in 0..=w {
                counts[prefix[r2][c] - prefix[r1-1][c]] = 0;
            }
        }
    }
    println!("{}", ans);
}

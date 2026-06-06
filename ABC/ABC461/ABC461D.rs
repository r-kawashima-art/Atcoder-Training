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
    let get_sum = |x1: usize, y1: usize, x2: usize, y2: usize| -> usize {
        prefix[x2][y2] + prefix[x1 - 1][y1 - 1] - prefix[x1 - 1][y2] - prefix[x2][y1 - 1]
    };
    let mut ans: usize = 0;
    for span_x in 1..=h {
        for span_y in 1..=w {
            for start_x in 1..h - span_x + 1 {
                for start_y in 1..w - span_y + 1 {
                    let sum_2d = get_sum(start_x, start_y, start_x + span_x, start_y + span_y);
                    if sum_2d == k {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}

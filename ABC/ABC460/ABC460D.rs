use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut db = vec![-1; h * w];
    let mut dw = vec![-1; h * w];

    let mut q_b = VecDeque::new();
    let mut q_w = VecDeque::new();

    for r in 0..h {
        for c in 0..w {
            let idx = r * w + c;
            if s[r][c] == '#' {
                db[idx] = 0;
                q_b.push_back((r, c));
            } else {
                dw[idx] = 0;
                q_w.push_back((r, c));
            }
        }
    }

    while let Some((r, c)) = q_b.pop_front() {
        let idx = r * w + c;
        let d = db[idx];
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < h as i32 && nc >= 0 && nc < w as i32 {
                    let n_idx = (nr as usize) * w + (nc as usize);
                    if db[n_idx] == -1 {
                        db[n_idx] = d + 1;
                        q_b.push_back((nr as usize, nc as usize));
                    }
                }
            }
        }
    }

    while let Some((r, c)) = q_w.pop_front() {
        let idx = r * w + c;
        let d = dw[idx];
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < h as i32 && nc >= 0 && nc < w as i32 {
                    let n_idx = (nr as usize) * w + (nc as usize);
                    if dw[n_idx] == -1 {
                        dw[n_idx] = d + 1;
                        q_w.push_back((nr as usize, nc as usize));
                    }
                }
            }
        }
    }

    for r in 0..h {
        let mut ans_row = String::with_capacity(w);
        for c in 0..w {
            let idx = r * w + c;
            if s[r][c] == '#' {
                let d = dw[idx];
                if d != -1 && d % 2 == 1 {
                    ans_row.push('#');
                } else {
                    ans_row.push('.');
                }
            } else {
                
                if d != -1 && d % 2 == 0 {
                    ans_row.push('#');
                } else {
                    ans_row.push('.');
                }
            }
        }
        println!("{}", ans_row);
    }
}


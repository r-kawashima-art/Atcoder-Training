use proconio::{input, marker::Chars};
use std::collections::VecDeque;
const INF: usize = 1 << 30;
const N1: usize = 1usize.wrapping_neg();
const D8: [(usize, usize); 8] = [
    (N1, N1), (N1, 0), (N1, 1),
    (0, N1), (0, 1),
    (1, N1), (1, 0), (1, 1)
];
// ABC460D
// Q. White turns into Black when there is an adjacent black cell of D8.
// Black turns into white
// A. Periodicity(turn on-off = parity) + Multi-start BFS 
fn main() {
    input!{h: usize, w: usize, s: [Chars; h]}
    let dist_from_blacks: Vec<Vec<usize>> = get_dist_multi_starts(&s, '#');
    let dist_from_whites: Vec<Vec<usize>> = get_dist_multi_starts(&s, '.');
    print_state(&dist_from_blacks, &dist_from_whites, &s);
}

fn print_state(dist_from_blacks: &Vec<Vec<usize>>, dist_from_whites: &Vec<Vec<usize>>, graph: &Vec<Vec<char>>) {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    for i in 0..h {
        for j in 0..w {
            if graph[i][j] == '#' {
                if dist_from_whites[i][j] != INF && dist_from_whites[i][j] % 2 == 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            } else {
                if dist_from_blacks[i][j] != INF && dist_from_blacks[i][j] % 2 == 0 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!("");
    }
}

fn get_dist_multi_starts(graph: &Vec<Vec<char>>, target_color: char) -> Vec<Vec<usize>> {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut dist: Vec<Vec<usize>> = vec![vec![INF; w]; h];
    let mut que = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if graph[i][j] == target_color {
                que.push_back((i, j));
                dist[i][j] = 0;
            }
        }
    }
    while let Some((row, col)) = que.pop_front() {
        for dir in 0..8 {
            let (dy, dx) = D8[dir];
            let next_row: usize = row.wrapping_add(dy);
            let next_col: usize = col.wrapping_add(dx);
            if next_row < h && next_col < w {
                if dist[next_row][next_col] > dist[row][col] + 1 {
                    dist[next_row][next_col] = dist[row][col] + 1;
                    que.push_back((next_row, next_col));
                }
            }
        }
    }
    return dist;
}
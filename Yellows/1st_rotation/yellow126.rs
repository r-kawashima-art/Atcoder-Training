use proconio::{input, marker::Usize1};
use std::collections::VecDeque;
// CODE FESTIVAL 2017 Qual B C - 3 Steps
// Q. Choose vetext u, v which are 3 edges away from each other and then add an edge. What is the maximum number of edges that can be added?
// A. Bipartite Graph (Undirected Graph)
// if Bipartite: Answer = Black * White - M
// else: Answer = N * (N - 1) / 2 - M
fn main() {
    input!{n: usize, m: usize, edges: [(Usize1, Usize1); m]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(v1, v2) in edges.iter() {
        graph[v1].push(v2);
        graph[v2].push(v1);
    }
    let (is_bipartite, black_times_white) = judge_bipartite(&graph);
    let ans = if is_bipartite {
        black_times_white - m
    } else {
        n * (n - 1) / 2 - m
    };
    println!("{}", ans);
}

fn judge_bipartite(graph: &Vec<Vec<usize>>) -> (bool, usize) {
    let n: usize = graph.len();
    if n == 0 {
        return (true, 0);
    }
    let mut colors: Vec<i32> = vec![-1; n];
    let mut que = VecDeque::new();

    colors[0] = 0;
    que.push_back(0);

    while let Some(u) = que.pop_front() {
        for &v in &graph[u] {
            if colors[v] == -1 {
                colors[v] = 1 - colors[u];
                que.push_back(v);
            } else if colors[v] == colors[u] {
                return (false, 0);
            }
        }
    }

    let mut black_count: usize = 0;
    let mut white_count: usize = 0;
    for i in 0..n {
        if colors[i] == 0 {
            black_count += 1;
        } else {
            white_count += 1;
        }
    }
    return (true, black_count * white_count);
}

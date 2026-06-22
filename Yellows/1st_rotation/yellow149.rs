use proconio::{input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
// JOI 2017 予選 6 - ヘビの JOI 君
// Q. Judge whether it is possible to go from 0 to n-1, following the rule of temperature adapatation within `x`minutes.
// A. Expanded Dijkstra Algorithm
const INF: i64 = i64::MAX;

fn main() {
    input! {
        n_node: usize,
        m_edge: usize,
        x_adapt: i64,
        temperatures: [usize; n_node],
        edges: [(Usize1, Usize1, i64); m_edge]
    }
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n_node];
    for &(v1, v2, time) in edges.iter() {
        graph[v1].push((v2, time));
        graph[v2].push((v1, time));
    }

    let dist = dijkstra(0, x_adapt, &graph, &temperatures);
    println!("{}", dist);
}

fn dijkstra(
    start: usize,
    x_adapt: i64,
    graph: &Vec<Vec<(usize, i64)>>,
    temperatures: &Vec<usize>,
) -> i64 {
    let n_node = graph.len();
    let x_size = x_adapt as usize;

    // dist[u * 2 * (X + 1) + state_type * (X + 1) + rem_time]
    // state_type: 0 for cold (temperature 0), 1 for hot (temperature 2)
    // rem_time: 0 to X
    let stride_type = x_size + 1;
    let stride_node = 2 * stride_type;
    let mut dist = vec![INF; n_node * stride_node];

    // que contains (Reverse(current_time), current_node, state_type, rem_time)
    let mut que = BinaryHeap::new();

    let start_idx = start * stride_node + 0 * stride_type + x_size;
    dist[start_idx] = 0;
    que.push((Reverse(0), start, 0, x_size));

    let goal_node = n_node - 1;

    while let Some((Reverse(cur_time), cur_node, cur_type, rem_time)) = que.pop() {
        let idx = cur_node * stride_node + cur_type * stride_type + rem_time;
        if cur_time > dist[idx] {
            continue;
        }

        if cur_node == goal_node {
            return cur_time;
        }

        for &(next_node, spent_time) in graph[cur_node].iter() {
            let next_temp = temperatures[next_node];

            let mut next_type = cur_type;
            let mut next_rem = rem_time;
            let mut possible = false;

            if next_temp == 0 {
                // Cold room: can only enter if we didn't just leave a hot room (cur_type == 1)
                // or if we have spent at least x_adapt time since leaving it (rem_time <= spent_time).
                if cur_type == 0 || (rem_time as i64) <= spent_time {
                    possible = true;
                    next_type = 0;
                    next_rem = x_size;
                }
            } else if next_temp == 2 {
                // Hot room: can only enter if we didn't just leave a cold room (cur_type == 0)
                // or if we have spent at least x_adapt time since leaving it (rem_time <= spent_time).
                if cur_type == 1 || (rem_time as i64) <= spent_time {
                    possible = true;
                    next_type = 1;
                    next_rem = x_size;
                }
            } else {
                // Comfortable room: always possible, temperature adaptation time decreases by spent_time
                possible = true;
                next_rem = (rem_time as i64 - spent_time).max(0) as usize;
            }

            if possible {
                let next_time = cur_time + spent_time;
                let next_idx = next_node * stride_node + next_type * stride_type + next_rem;
                if dist[next_idx] > next_time {
                    dist[next_idx] = next_time;
                    que.push((Reverse(next_time), next_node, next_type, next_rem));
                }
            }
        }
    }

    INF
}

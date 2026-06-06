use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
use std::time::Instant;

const N1: usize = 1usize.wrapping_neg();
// Directions: 0=Right, 1=Down, 2=Left, 3=Up
const D4: [(usize, usize); 4] = [(0, 1), (1, 0), (0, N1), (N1, 0)];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Forward,
    RightTurn,
    LeftTurn,
    Swap,
}

impl Operation {
    fn to_char(self) -> char {
        match self {
            Operation::Forward => 'F',
            Operation::RightTurn => 'R',
            Operation::LeftTurn => 'L',
            Operation::Swap => 'S',
        }
    }
}

struct Data {
    cells_n: usize,
    balltypes_m: usize,
    time_limit: usize,
    vertical_walls: Vec<Vec<bool>>,
    horizontal_walls: Vec<Vec<bool>>,
    ball_initial_positions: Vec<(usize, usize)>,
    basket_positions: Vec<(usize, usize)>,
}

impl Data {
    fn new() -> Self {
        input! {
            cells_n: usize,
            balltypes_m: usize,
            time_limit: usize,
            v_strings: [Chars; cells_n],
            h_strings: [Chars; cells_n - 1],
            ball_basket_positions: [(usize, usize, usize, usize); balltypes_m]
        }

        let mut vertical_walls = vec![vec![false; cells_n - 1]; cells_n];
        for i in 0..cells_n {
            for j in 0..cells_n - 1 {
                vertical_walls[i][j] = v_strings[i][j] == '1';
            }
        }

        let mut horizontal_walls = vec![vec![false; cells_n]; cells_n - 1];
        for i in 0..cells_n - 1 {
            for j in 0..cells_n {
                horizontal_walls[i][j] = h_strings[i][j] == '1';
            }
        }

        let mut ball_initial_positions = vec![(0, 0); balltypes_m];
        let mut basket_positions = vec![(0, 0); balltypes_m];
        for k in 0..balltypes_m {
            let (r1, c1, r2, c2) = ball_basket_positions[k];
            ball_initial_positions[k] = (r1, c1);
            basket_positions[k] = (r2, c2);
        }

        Self {
            cells_n,
            balltypes_m,
            time_limit,
            vertical_walls,
            horizontal_walls,
            ball_initial_positions,
            basket_positions,
        }
    }

    fn can_move(&self, r: usize, c: usize, dir: usize) -> bool {
        let n = self.cells_n;
        match dir {
            0 => c + 1 < n && !self.vertical_walls[r][c],
            1 => r + 1 < n && !self.horizontal_walls[r][c],
            2 => c > 0 && !self.vertical_walls[r][c - 1],
            3 => r > 0 && !self.horizontal_walls[r - 1][c],
            _ => false,
        }
    }

    fn encode_state(&self, r: usize, c: usize, d: usize) -> usize {
        (r * self.cells_n + c) * 4 + d
    }

    fn decode_state(&self, s: usize) -> (usize, usize, usize) {
        let d = s % 4;
        let c = (s / 4) % self.cells_n;
        let r = s / (self.cells_n * 4);
        (r, c, d)
    }
}

struct APSP {
    dist: Vec<Vec<u32>>,
    prev: Vec<Vec<Option<(usize, Operation)>>>,
}

impl APSP {
    fn new(data: &Data) -> Self {
        let num_states = data.cells_n * data.cells_n * 4;
        let mut dist = vec![vec![u32::MAX; num_states]; num_states];
        let mut prev = vec![vec![None; num_states]; num_states];

        for start_r in 0..data.cells_n {
            for start_c in 0..data.cells_n {
                for start_d in 0..4 {
                    let start_state = data.encode_state(start_r, start_c, start_d);
                    dist[start_state][start_state] = 0;
                    let mut q = VecDeque::new();
                    q.push_back(start_state);

                    while let Some(u) = q.pop_front() {
                        let (r, c, d) = data.decode_state(u);
                        let d_u = dist[start_state][u];

                        if data.can_move(r, c, d) {
                            let (dr, dc) = D4[d];
                            let nr = r.wrapping_add(dr);
                            let nc = c.wrapping_add(dc);
                            let v = data.encode_state(nr, nc, d);
                            if dist[start_state][v] == u32::MAX {
                                dist[start_state][v] = d_u + 1;
                                prev[start_state][v] = Some((u, Operation::Forward));
                                q.push_back(v);
                            }
                        }

                        let nd_r = (d + 1) % 4;
                        let v_r = data.encode_state(r, c, nd_r);
                        if dist[start_state][v_r] == u32::MAX {
                            dist[start_state][v_r] = d_u + 1;
                            prev[start_state][v_r] = Some((u, Operation::RightTurn));
                            q.push_back(v_r);
                        }

                        let nd_l = (d + 3) % 4;
                        let v_l = data.encode_state(r, c, nd_l);
                        if dist[start_state][v_l] == u32::MAX {
                            dist[start_state][v_l] = d_u + 1;
                            prev[start_state][v_l] = Some((u, Operation::LeftTurn));
                            q.push_back(v_l);
                        }
                    }
                }
            }
        }

        Self { dist, prev }
    }
}

fn evaluate_permutation(
    perm: &[usize],
    data: &Data,
    apsp: &APSP,
) -> (u32, Vec<usize>) {
    let m = data.balltypes_m;
    let num_stops = 2 * m;
    let mut dp = vec![vec![u32::MAX; 4]; num_stops + 1];
    let mut trace = vec![vec![0usize; 4]; num_stops + 1];

    for d in 0..4 {
        dp[0][d] = apsp.dist[data.encode_state(0, 0, 0)][data.encode_state(0, 0, d)];
    }

    for i in 0..num_stops {
        let prev_loc = if i == 0 {
            (0, 0)
        } else if (i - 1) % 2 == 0 {
            data.ball_initial_positions[perm[(i - 1) / 2]]
        } else {
            data.basket_positions[perm[(i - 1) / 2]]
        };

        let curr_loc = if i % 2 == 0 {
            data.ball_initial_positions[perm[i / 2]]
        } else {
            data.basket_positions[perm[i / 2]]
        };

        for curr_d in 0..4 {
            let mut min_val = u32::MAX;
            let mut best_prev_d = 0;
            let target_state = data.encode_state(curr_loc.0, curr_loc.1, curr_d);

            for prev_d in 0..4 {
                if dp[i][prev_d] != u32::MAX {
                    let from_state = data.encode_state(prev_loc.0, prev_loc.1, prev_d);
                    let move_cost = apsp.dist[from_state][target_state];
                    if move_cost != u32::MAX {
                        let val = dp[i][prev_d] + move_cost + 1;
                        if val < min_val {
                            min_val = val;
                            best_prev_d = prev_d;
                        }
                    }
                }
            }
            dp[i + 1][curr_d] = min_val;
            trace[i + 1][curr_d] = best_prev_d;
        }
    }

    let mut best_final_d = 0;
    let mut min_total = u32::MAX;
    for d in 0..4 {
        if dp[num_stops][d] < min_total {
            min_total = dp[num_stops][d];
            best_final_d = d;
        }
    }

    let mut best_dirs = vec![0; num_stops + 1];
    best_dirs[num_stops] = best_final_d;
    for i in (1..=num_stops).rev() {
        best_dirs[i - 1] = trace[i][best_dirs[i]];
    }

    (min_total, best_dirs)
}

fn reconstruct_path(
    perm: &[usize],
    best_dirs: &[usize],
    data: &Data,
    apsp: &APSP,
) -> Vec<Operation> {
    let mut ops = Vec::new();
    let m = data.balltypes_m;

    let mut current_state = data.encode_state(0, 0, 0);

    let start_target_state = data.encode_state(0, 0, best_dirs[0]);
    let mut path = Vec::new();
    let mut curr = start_target_state;
    while curr != current_state {
        let (prev, op) = apsp.prev[current_state][curr].unwrap();
        path.push(op);
        curr = prev;
    }
    path.reverse();
    ops.extend(path);
    current_state = start_target_state;

    for i in 0..2 * m {
        let target_loc = if i % 2 == 0 {
            data.ball_initial_positions[perm[i / 2]]
        } else {
            data.basket_positions[perm[i / 2]]
        };
        
        let target_dir = best_dirs[i + 1];
        let target_state = data.encode_state(target_loc.0, target_loc.1, target_dir);

        let mut path = Vec::new();
        let mut curr = target_state;
        while curr != current_state {
            let (prev, op) = apsp.prev[current_state][curr].unwrap();
            path.push(op);
            curr = prev;
        }
        path.reverse();
        ops.extend(path);
        
        ops.push(Operation::Swap);
        current_state = target_state;
    }

    ops
}

fn optimal_compression(ops: &[Operation]) -> Vec<char> {
    let chars: Vec<char> = ops.iter().map(|op| op.to_char()).collect();
    let n = chars.len();
    
    let mut dp = vec![0; n + 1];
    let mut next_idx = vec![0; n + 1];
    let mut best_macro = vec![None; n + 1]; 

    for i in (0..n).rev() {
        dp[i] = 1 + dp[i + 1];
        next_idx[i] = i + 1;
        best_macro[i] = None;

        for len in 2..=50 {
            if i + len > n { break; }
            let sub = &chars[i..i+len];
            
            let mut count = 1;
            let mut curr = i + len;
            while curr + len <= n {
                if &chars[curr..curr+len] == sub {
                    count += 1;
                    curr += len;
                    
                    let savings = (len as i32 - 1) * (count as i32 - 1) - 2;
                    if savings > 0 {
                        let cost = (curr - i) as i32 - savings + dp[curr] as i32;
                        if cost < dp[i] as i32 {
                            dp[i] = cost as usize;
                            next_idx[i] = curr;
                            best_macro[i] = Some((len, count));
                        }
                    }
                } else {
                    curr += 1;
                }
            }
        }
    }

    let mut final_chars = Vec::new();
    let mut curr = 0;
    while curr < n {
        if let Some((len, _count)) = best_macro[curr] {
            let sub = &chars[curr..curr+len];
            final_chars.push('M');
            final_chars.extend_from_slice(sub);
            final_chars.push('M');
            
            let mut ptr = curr + len;
            let end = next_idx[curr];
            
            while ptr < end {
                if ptr + len <= end && &chars[ptr..ptr+len] == sub {
                    final_chars.push('P');
                    ptr += len;
                } else {
                    final_chars.push(chars[ptr]);
                    ptr += 1;
                }
            }
            curr = end;
        } else {
            final_chars.push(chars[curr]);
            curr += 1;
        }
    }
    final_chars
}

fn solve() {
    let start_time = Instant::now();
    let data = Data::new();
    let apsp = APSP::new(&data);

    let mut best_compressed_len = usize::MAX;
    let mut best_final_chars = Vec::new();
    let time_limit = 1.90;

    let mut rng_seed: u64 = 88172645463325252;
    fn next_rand(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }

    let iters_per_restart = 200_000;
    let mut total_restarts = 0;

    loop {
        if start_time.elapsed().as_secs_f64() > time_limit {
            break;
        }

        let mut perm: Vec<usize> = (0..data.balltypes_m).collect();
        if total_restarts > 0 {
            for i in 0..data.balltypes_m {
                let j = (next_rand(&mut rng_seed) as usize) % data.balltypes_m;
                perm.swap(i, j);
            }
        }

        let (mut best_cost, _) = evaluate_permutation(&perm, &data, &apsp);
        let mut current_cost = best_cost;
        let mut best_perm = perm.clone();

        let t0 = 10.0_f64;
        let t1 = 0.1_f64;

        for iter in 0..iters_per_restart {
            let temp = t0 * (t1 / t0).powf(iter as f64 / iters_per_restart as f64);

            let m = data.balltypes_m;
            let type_of_move = next_rand(&mut rng_seed) % 2;
            let i = (next_rand(&mut rng_seed) as usize) % m;
            let mut j = (next_rand(&mut rng_seed) as usize) % m;
            while i == j {
                j = (next_rand(&mut rng_seed) as usize) % m;
            }

            if type_of_move == 0 {
                perm.swap(i, j);
                let (new_cost, _) = evaluate_permutation(&perm, &data, &apsp);
                
                let diff = new_cost as f64 - current_cost as f64;
                if diff <= 0.0 || f64::exp(-diff / temp) > (next_rand(&mut rng_seed) % 10000) as f64 / 10000.0 {
                    current_cost = new_cost;
                    if current_cost < best_cost {
                        best_cost = current_cost;
                        best_perm = perm.clone();
                    }
                } else {
                    perm.swap(i, j);
                }
            } else {
                let start = i.min(j);
                let end = i.max(j);
                perm[start..=end].reverse();
                let (new_cost, _) = evaluate_permutation(&perm, &data, &apsp);
                
                let diff = new_cost as f64 - current_cost as f64;
                if diff <= 0.0 || f64::exp(-diff / temp) > (next_rand(&mut rng_seed) % 10000) as f64 / 10000.0 {
                    current_cost = new_cost;
                    if current_cost < best_cost {
                        best_cost = current_cost;
                        best_perm = perm.clone();
                    }
                } else {
                    perm[start..=end].reverse();
                }
            }
        }

        let (_, best_dirs) = evaluate_permutation(&best_perm, &data, &apsp);
        let ops = reconstruct_path(&best_perm, &best_dirs, &data, &apsp);
        let final_chars = optimal_compression(&ops);

        if final_chars.len() < best_compressed_len {
            best_compressed_len = final_chars.len();
            best_final_chars = final_chars;
        }

        total_restarts += 1;
    }

    for &ch in best_final_chars.iter().take(data.time_limit) {
        print!("{}", ch);
    }
    println!();
}

fn main() {
    solve();
}

use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

const N1: usize = 1usize.wrapping_neg();
// Directions: 0=Right, 1=Down, 2=Left, 3=Up
const D4: [(usize, usize); 4] = [
    (0, 1), (1, 0), (0, N1), (N1, 0)
];

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
}

struct State {
    r: usize,
    c: usize,
    dir: usize,
    grid: Vec<Vec<i32>>, // -1 for empty, 0..M for balls
    holding: i32, // -1 for empty
}

fn solve() {
    let data = Data::new();
    let mut state = State {
        r: 0,
        c: 0,
        dir: 0,
        grid: vec![vec![-1; data.cells_n]; data.cells_n],
        holding: -1,
    };

    for k in 0..data.balltypes_m {
        let (r, c) = data.ball_initial_positions[k];
        state.grid[r][c] = k as i32;
    }

    let mut operations = Vec::new();
    let mut delivered = vec![false; data.balltypes_m];

    for _ in 0..data.balltypes_m {
        // Find nearest ball
        let (path_to_ball, ball_k, r_ball, c_ball) = find_nearest_ball(&data, &state, &delivered);
        if path_to_ball.is_none() {
            break; // No more balls?
        }
        let path = path_to_ball.unwrap();
        
        apply_operations(&data, &mut state, &path, &mut operations);
        
        // Pick it up
        operations.push(Operation::Swap);
        state.holding = ball_k;
        state.grid[r_ball][c_ball] = -1;

        // Find path to basket
        let (r_basket, c_basket) = data.basket_positions[ball_k as usize];
        let path_to_basket = find_path_to_cell(&data, &state, r_basket, c_basket);
        if let Some(path_basket) = path_to_basket {
            apply_operations(&data, &mut state, &path_basket, &mut operations);
            
            // Drop it off
            operations.push(Operation::Swap);
            // Since there might be another ball there, swap it
            let swapped_ball = state.grid[r_basket][c_basket];
            state.grid[r_basket][c_basket] = state.holding;
            state.holding = swapped_ball;
            delivered[ball_k as usize] = true;
            
            // If we picked up a ball that was on the basket, we need to deal with it later.
            // Our simple heuristic doesn't handle carrying it immediately, 
            // but the loop will just find the next nearest.
            // Wait, if we are holding a ball, we should just deliver it!
            while state.holding != -1 {
                let current_holding = state.holding;
                let (r_b, c_b) = data.basket_positions[current_holding as usize];
                let path_b = find_path_to_cell(&data, &state, r_b, c_b);
                if let Some(p_b) = path_b {
                    apply_operations(&data, &mut state, &p_b, &mut operations);
                    operations.push(Operation::Swap);
                    let swapped_b = state.grid[r_b][c_b];
                    state.grid[r_b][c_b] = state.holding;
                    state.holding = swapped_b;
                    delivered[current_holding as usize] = true;
                } else {
                    break;
                }
            }
        }
    }

    // Print
    for op in operations.iter().take(data.time_limit) {
        print!("{}", op.to_char());
    }
    println!();
}

fn apply_operations(data: &Data, state: &mut State, path: &Vec<Operation>, operations: &mut Vec<Operation>) {
    for &op in path {
        match op {
            Operation::Forward => {
                if data.can_move(state.r, state.c, state.dir) {
                    let (dr, dc) = D4[state.dir];
                    state.r = state.r.wrapping_add(dr);
                    state.c = state.c.wrapping_add(dc);
                }
            }
            Operation::RightTurn => {
                state.dir = (state.dir + 1) % 4;
            }
            Operation::LeftTurn => {
                state.dir = (state.dir + 3) % 4;
            }
            _ => {}
        }
        operations.push(op);
    }
}

// Find path to any undelivered ball
fn find_nearest_ball(data: &Data, state: &State, delivered: &[bool]) -> (Option<Vec<Operation>>, i32, usize, usize) {
    let mut q = VecDeque::new();
    let mut dist = vec![vec![vec![usize::MAX; 4]; data.cells_n]; data.cells_n];
    let mut prev = vec![vec![vec![None; 4]; data.cells_n]; data.cells_n];

    q.push_back((state.r, state.c, state.dir));
    dist[state.r][state.c][state.dir] = 0;

    let mut best_dist = usize::MAX;
    let mut target_state = None;
    let mut target_k = -1;

    while let Some((r, c, d)) = q.pop_front() {
        let current_dist = dist[r][c][d];
        if current_dist >= best_dist {
            continue;
        }

        if state.grid[r][c] != -1 && !delivered[state.grid[r][c] as usize] {
            if current_dist < best_dist {
                best_dist = current_dist;
                target_state = Some((r, c, d));
                target_k = state.grid[r][c];
            }
        }

        // Try moving forward
        if data.can_move(r, c, d) {
            let (dr, dc) = D4[d];
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if dist[nr][nc][d] == usize::MAX {
                dist[nr][nc][d] = current_dist + 1;
                prev[nr][nc][d] = Some(((r, c, d), Operation::Forward));
                q.push_back((nr, nc, d));
            }
        }
        // Try turning right
        let nd_r = (d + 1) % 4;
        if dist[r][c][nd_r] == usize::MAX {
            dist[r][c][nd_r] = current_dist + 1;
            prev[r][c][nd_r] = Some(((r, c, d), Operation::RightTurn));
            q.push_back((r, c, nd_r));
        }
        // Try turning left
        let nd_l = (d + 3) % 4;
        if dist[r][c][nd_l] == usize::MAX {
            dist[r][c][nd_l] = current_dist + 1;
            prev[r][c][nd_l] = Some(((r, c, d), Operation::LeftTurn));
            q.push_back((r, c, nd_l));
        }
    }

    if let Some((mut r, mut c, mut d)) = target_state {
        let mut path = Vec::new();
        while r != state.r || c != state.c || d != state.dir {
            let (prev_state, op) = prev[r][c][d].unwrap();
            path.push(op);
            r = prev_state.0;
            c = prev_state.1;
            d = prev_state.2;
        }
        path.reverse();
        return (Some(path), target_k, target_state.unwrap().0, target_state.unwrap().1);
    }
    
    (None, -1, 0, 0)
}

// Find path to a specific cell
fn find_path_to_cell(data: &Data, state: &State, target_r: usize, target_c: usize) -> Option<Vec<Operation>> {
    let mut q = VecDeque::new();
    let mut dist = vec![vec![vec![usize::MAX; 4]; data.cells_n]; data.cells_n];
    let mut prev = vec![vec![vec![None; 4]; data.cells_n]; data.cells_n];

    q.push_back((state.r, state.c, state.dir));
    dist[state.r][state.c][state.dir] = 0;

    let mut target_state = None;

    while let Some((r, c, d)) = q.pop_front() {
        let current_dist = dist[r][c][d];

        if r == target_r && c == target_c {
            target_state = Some((r, c, d));
            break;
        }

        if data.can_move(r, c, d) {
            let (dr, dc) = D4[d];
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if dist[nr][nc][d] == usize::MAX {
                dist[nr][nc][d] = current_dist + 1;
                prev[nr][nc][d] = Some(((r, c, d), Operation::Forward));
                q.push_back((nr, nc, d));
            }
        }
        let nd_r = (d + 1) % 4;
        if dist[r][c][nd_r] == usize::MAX {
            dist[r][c][nd_r] = current_dist + 1;
            prev[r][c][nd_r] = Some(((r, c, d), Operation::RightTurn));
            q.push_back((r, c, nd_r));
        }
        let nd_l = (d + 3) % 4;
        if dist[r][c][nd_l] == usize::MAX {
            dist[r][c][nd_l] = current_dist + 1;
            prev[r][c][nd_l] = Some(((r, c, d), Operation::LeftTurn));
            q.push_back((r, c, nd_l));
        }
    }

    if let Some((mut r, mut c, mut d)) = target_state {
        let mut path = Vec::new();
        while r != state.r || c != state.c || d != state.dir {
            let (prev_state, op) = prev[r][c][d].unwrap();
            path.push(op);
            r = prev_state.0;
            c = prev_state.1;
            d = prev_state.2;
        }
        path.reverse();
        return Some(path);
    }
    None
}

fn main() {
    solve();
}

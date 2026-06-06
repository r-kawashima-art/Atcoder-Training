use proconio::input;
const INF: usize = 1 << 60;
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (0, 1), (1, 0), (0, N1), (N1, 0)
];
#[derive(Debug, Clone, Copy)]
enum Operation {
    Forward,
    RightTurn,
    LeftTurn,
    Swap,
    Macro(bool),
    Play,
}
struct Data {
    cells_n: usize,
    balltypes_m: usize,
    time_limit: usize,
    vertical_walls: Vec<Vec<usize>>,
    horizontal_walls: Vec<Vec<usize>>,
    ball_positions: Vec<(usize, usize)>,
    bascket_positions: Vec<(usize, usize)>,
}
impl Data {
    fn new() -> Self {
        input!{
            cells_n: usize, balltypes_m: usize, time_limit: usize, 
            vertical_walls: [[usize; cells_n - 1]; cells_n], horizontal_walls: [[usize; cells_n]; cells_n - 1], 
            ball_bascket_positions: [(usize, usize, usize, usize); balltypes_m]
        }
        let mut ball_positions: Vec<(usize, usize)> = vec![];
        let mut bascket_positions: Vec<(usize, usize)> = vec![];
        for &(x1, y1, x2, y2) in &ball_bascket_positions {
            ball_positions.push((x1, y1));
            bascket_positions.push((x2, y2));
        }
        Self {
            cells_n, balltypes_m, time_limit, 
            vertical_walls, horizontal_walls, 
            ball_positions, bascket_positions,
        }
    }
}
struct State {
    cur_position: (usize, usize),
    cur_direction: usize,
    cur_score: i64,
    cur_time: usize,
    registered_macro: Vec<Operation>,
}
struct Solver {
    data: Data,
    state: State,
    operations: Vec<Operation>,
}
fn main() {
    

}

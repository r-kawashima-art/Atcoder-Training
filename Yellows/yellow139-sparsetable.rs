use proconio::input;
// DSL_2_A - Range Minimum Query
// Q. Range Minimum Query
// A. Sparse Table (Algorithm itself is similiar to Doubling DP)
struct SparseTable {
    // doubling[k][i] is min in a[i .. i + 2^k]
    doubling: Vec<Vec<usize>>,
    // n is the size of the array
    n: usize,
    layers: usize,
}

impl SparseTable {
    const INF: usize = 1 << 31 - 1;
    #[inline]
    fn log2_floor(num: usize) -> usize {
        let mut layers: usize = 0;
        while 1 << (layers + 1) < num {
            layers += 1;
        }
        return layers;
    }
    fn new(source: &Vec<usize>) -> Self {
        let n: usize = source.len();
        let layers: usize = Self::log2_floor(n);
        let mut doubling: Vec<Vec<usize>> = vec![vec![Self::INF; n]; layers + 1];
        for i in 0..n {
            doubling[0][i] = source[i];
        }
        for layer in 1..=layers {
            let length: usize = 1 << (layer - 1);
            for i in 0..n {
                if i + length >= n {
                    break;
                }
                doubling[layer][i] = doubling[layer - 1][i].min(doubling[layer-1][i+length]);
            }
        }
        return SparseTable {
            doubling,
            n,
            layers,
        };
    }
    fn find(&mut self, left: usize, right: usize) -> usize {
        if left > right {
            return Self::INF;
        } 
        let diff: usize = right + 1 - left;
        let layer: usize = Self::log2_floor(diff);
        let first: usize = self.doubling[layer][left];
        let second: usize = self.doubling[layer][right + 1 - (1 << layer)];
        return first.min(second);
    }
    fn update(&mut self, index: usize, value: usize) {
        self.doubling[0][index] = value;
        for phase in 1..=self.layers {
            let length: usize = 1 << (phase - 1);
            if index + length >= self.n {
                break;
            }
            self.doubling[phase][index] = self.doubling[phase - 1][index].min(self.doubling[phase - 1][index + length]);
        }
    }
}

fn main() {
    input!{n: usize, q: usize, query: [(usize, usize, usize); q]}
    let a = vec![SparseTable::INF; n];
    let mut sparse_table = SparseTable::new(&a);
    // 0-indexed but x..=y
    for &(query_type, x, y) in query.iter() {
        // update
        if query_type == 0 {
            sparse_table.update(x, y);
        } else {
        // find
            let ans = sparse_table.find(x, y);
            println!("{}", ans);
        }
    }
}
use proconio::input;
// DSL_2_F - RMQ and RUQ
// Q. Range Minimum Query and Range Update Query
// A. Segment Tree (with lazy propagation)
#[derive(Debug, Clone)]
struct LazySegmentTree {
    n: usize,
    data: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegmentTree {
    const INF: i64 = i64::MAX;
    const INIT: i64 = (1 << 31) - 1; // Initial value of elements in the array
    
    #[inline]
    fn next_power_of_2(n: usize) -> usize {
        let mut power = 1;
        while power < n {
            power <<= 1;
        }
        return power;
    }
    fn new(n: usize) -> Self {
        let size = Self::next_power_of_2(n);
        Self {
            n: size,
            data: vec![Self::INIT; 2 * size],
            lazy: vec![Self::INF; 2 * size],
        }
    }
    fn lazy_evaluation(&mut self, cell: usize) {
        /* Reflect the changes in the lazy children cells and data cells */
        if self.lazy[cell] == Self::INF {
            return;
        }
        // Composition of actions (lazy): NOT MINIMUM, but UPDATE!!!
        if cell < self.n {
            self.lazy[2 * cell] = self.lazy[cell];
            self.lazy[2 * cell + 1] = self.lazy[cell];
        }
        // Mapping Monoid and Action
        self.data[cell] = self.lazy[cell];
        // Return back to the initial state of lazy after evaluation
        self.lazy[cell] = Self::INF;
    }
    fn update(&mut self, l: usize, r: usize, x: i64) {
        self._update(l, r, x, 1, 0, self.n);
    }
    fn _update(&mut self, l: usize, r: usize, x: i64, cell: usize, cell_l: usize, cell_r: usize) {
        // update lazy first
        self.lazy_evaluation(cell);

        // scope check is second
        if cell_r <= l || r <= cell_l {
            return;
        } else if l <= cell_l && cell_r <= r {
            self.lazy[cell] = x;
            // do not forget to reflect the changes in the data cell
            self.lazy_evaluation(cell);
        } else {
            let mid: usize = (cell_l + cell_r) / 2;
            self._update(l, r, x, 2 * cell, cell_l, mid);
            self._update(l, r, x, 2 * cell + 1, mid, cell_r);
            // update data after children are updated
            self.data[cell] = self.data[2 * cell].min(self.data[2 * cell + 1]);
        }
    }
    fn find_min(&mut self, l: usize, r: usize) -> i64 {
        return self._find_min(l, r, 1, 0, self.n);
    }
    fn _find_min(&mut self, l: usize, r: usize, cell: usize, cell_l: usize, cell_r: usize) -> i64 {
        // reflect lazy first
        self.lazy_evaluation(cell);
        if cell_r <= l || r <= cell_l {
            return Self::INF;
        } else if l <= cell_l && cell_r <= r {
            return self.data[cell];
        } else {
            let mid: usize = (cell_l + cell_r) / 2;
            let left_min: i64 = self._find_min(l, r, 2 * cell, cell_l, mid);
            let right_min: i64 = self._find_min(l, r, 2 * cell + 1, mid, cell_r);
            let min_value: i64 = left_min.min(right_min);
            return min_value;
        }
    }
}

fn main() {
    input!{n: usize, q: usize}
    let mut lazysegtree = LazySegmentTree::new(n);
    for _ in 0..q {
        input!{query_type: usize}
        if query_type == 0 {
            input!{l: usize, r: usize, x: i64}
            lazysegtree.update(l, r+1, x);
        } else {
            input!{l: usize, r: usize}
            println!("{}", lazysegtree.find_min(l, r+1));
        }
    }
}
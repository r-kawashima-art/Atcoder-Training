use proconio::{input, marker::Usize1};
// 全国統一プログラミング王決定戦本戦 D - Deforestation
// Q. Every Tree grows at 1 unit per second. We can cut trees in range [L, R] at a query's time.
// A. Lazy Segment Tree for Range Update and Point Query
struct LazySegTree {
    n: usize,
    tree: Vec<u64>,
    lazy: Vec<Option<u64>>,
}

impl LazySegTree {
    fn new(n: usize) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        LazySegTree {
            n: size,
            tree: vec![0; size * 2],
            lazy: vec![None; size * 2],
        }
    }

    fn push(&mut self, node: usize, l: usize, r: usize) {
        if let Some(val) = self.lazy[node] {
            let mid = (l + r) / 2;
            
            // Left child
            self.tree[node * 2] = val * (mid - l) as u64;
            self.lazy[node * 2] = Some(val);
            
            // Right child
            self.tree[node * 2 + 1] = val * (r - mid) as u64;
            self.lazy[node * 2 + 1] = Some(val);
            
            self.lazy[node] = None;
        }
    }

    fn update(&mut self, ql: usize, qr: usize, val: u64, node: usize, l: usize, r: usize) {
        if ql <= l && r <= qr {
            self.tree[node] = val * (r - l) as u64;
            self.lazy[node] = Some(val);
            return;
        }
        self.push(node, l, r);
        let mid = (l + r) / 2;
        if ql < mid {
            self.update(ql, qr, val, node * 2, l, mid);
        }
        if qr > mid {
            self.update(ql, qr, val, node * 2 + 1, mid, r);
        }
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
    }

    fn query(&mut self, ql: usize, qr: usize, node: usize, l: usize, r: usize) -> u64 {
        if ql <= l && r <= qr {
            return self.tree[node];
        }
        self.push(node, l, r);
        let mid = (l + r) / 2;
        let mut sum = 0;
        if ql < mid {
            sum += self.query(ql, qr, node * 2, l, mid);
        }
        if qr > mid {
            sum += self.query(ql, qr, node * 2 + 1, mid, r);
        }
        sum
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        queries: [(u64, Usize1, Usize1); m],
    }

    let mut seg = LazySegTree::new(n);
    let mut total_score: u64 = 0;

    for (time, left_idx, right_idx) in queries {
        let ql = left_idx;
        let qr = right_idx + 1; // Half-open interval [ql, qr)

        let sum_last_cut_times = seg.query(ql, qr, 1, 0, seg.n);
        let num_elements = (qr - ql) as u64;

        let score = time * num_elements - sum_last_cut_times;
        total_score += score;

        seg.update(ql, qr, time, 1, 0, seg.n);
    }

    println!("{}", total_score);
}

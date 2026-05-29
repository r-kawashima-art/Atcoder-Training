use proconio::input;
const INF: usize = 1 << 31 - 1;
// DSL_2_A - Range Minimum Query
// Q. Range Minimum Query
// A. Segment Tree

struct SegmentTreeRMQ {
    // managed by 1 indexed
    data: Vec<usize>,
    n: usize,
}

impl SegmentTreeRMQ {
    #[inline]
    fn next_power_of_two(n: usize) -> usize {
        let mut power: usize = 1;
        while power < n {
            power <<= 1;
        }
        power
    }
    fn new(n: usize) -> Self {
        let power_of_n = Self::next_power_of_two(n);
        let data = vec![INF; power_of_n * 2];
        SegmentTreeRMQ {data, n: power_of_n}
    }

    fn _get_min(&mut self, left: usize, right: usize, cell_left: usize, cell_right: usize, cell_index: usize) -> usize {
        // Each Cell's perspective is the first.
        if right <= cell_left || cell_right <= left {
            return INF;
        }
        if left <= cell_left && cell_right <= right {
            return self.data[cell_index];
        }
        let cell_mid: usize = (cell_left + cell_right) / 2;
        let value_left: usize = self._get_min(left, right, cell_left, cell_mid, cell_index * 2);
        let value_right: usize = self._get_min(left, right, cell_mid, cell_right, cell_index * 2 + 1);
        return value_left.min(value_right);
    }

    // (left..right)
    fn find(&mut self, left: usize, right: usize) -> usize {
        return self._get_min(left, right + 1, 0, self.n, 1);
    }

    fn find_no_recursion(&self, left: usize, right: usize) -> usize {
        // Parities of l and r matter.
        let mut l = left + self.n;
        let mut r = right + 1 + self.n;
        let mut res = INF;
        while l < r {
            if l & 1 == 1 {
                res = res.min(self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res = res.min(self.data[r]);
            }
            l >>= 1;
            r >>= 1;
        }
        res
    }

    fn update(&mut self, index: usize, value: usize) {
        self.data[index+self.n] = value;
        let mut cur_index: usize = index + self.n;
        cur_index /= 2;
        while cur_index > 0 {
            self.data[cur_index] = self.data[cur_index*2].min(self.data[cur_index*2+1]);
            cur_index /= 2;
        }
    }
}

fn main() {
    input!{n: usize, q: usize, query: [(usize, usize, usize); q]}
    let mut segtree = SegmentTreeRMQ::new(n);
    // 0-indexed but x..=y
    for &(query_type, x, y) in query.iter() {
        // update
        if query_type == 0 {
            segtree.update(x, y);
        } else {
        // find
            let ans = segtree.find(x, y);
            println!("{}", ans);
        }
    }
}

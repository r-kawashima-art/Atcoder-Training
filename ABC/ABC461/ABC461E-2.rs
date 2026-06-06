use proconio::input;

struct FenwickTree {
    n: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![0; n + 1],
        }
    }

    fn add(&mut self, i: usize, x: i64) {
        let mut i = i + 1;
        while i <= self.n {
            self.data[i] += x;
            i += i & i.wrapping_neg();
        }
    }

    fn sum(&self, i: i64) -> i64 {
        if i < 0 {
            return 0;
        }
        let mut i = (i as usize) + 1;
        if i > self.n {
            i = self.n;
        }
        let mut s = 0;
        while i > 0 {
            s += self.data[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut bit_r = FenwickTree::new(q + 1);
    let mut bit_c = FenwickTree::new(q + 1);
    let mut cur_tr = vec![0usize; n + 1];
    let mut cur_tc = vec![0usize; n + 1];

    bit_r.add(0, n as i64);
    bit_c.add(0, n as i64);

    let mut ans: i64 = 0;

    for t in 1..=q {
        input! { ty: i32, val: usize }

        if ty == 1 {
            let r = val;
            let old_t = cur_tr[r];
            // Use BIT to find how many columns have a timestamp smaller than the new row timestamp.
            // At time t, all existing timestamps are <= t-1, so Count(j : T_C[j] < t) is N.
            ans += (n as i64) - bit_c.sum((old_t as i64) - 1);
            
            bit_r.add(old_t, -1);
            bit_r.add(t, 1);
            cur_tr[r] = t;
        } else {
            let c = val;
            let old_t = cur_tc[c];
            // Use BIT to find how many rows have a timestamp larger than the current column timestamp.
            // At time t, no row timestamp is > t.
            ans += bit_r.sum(old_t as i64) - (n as i64);
            
            bit_c.add(old_t, -1);
            bit_c.add(t, 1);
            cur_tc[c] = t;
        }
        println!("{}", ans);
    }
}

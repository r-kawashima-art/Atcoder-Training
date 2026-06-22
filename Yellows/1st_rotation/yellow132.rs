use proconio::input;
struct Fenwick {
    data: Vec<usize>
}
impl Fenwick {
    #[inline]
    fn lsb(index: usize) -> usize {
        return index & index.wrapping_neg();
    }
    fn new(n: usize) -> Self {
        let data: Vec<usize> = vec![0; n + 1];
        return Fenwick {data}
    
    }
    fn add(&mut self, index: usize, value: usize) {
        // 1 indexed
        let mut cur_index: usize = index;
        while cur_index < self.data.len() {
            self.data[cur_index] += value;
            cur_index += Self::lsb(cur_index);
        }
    }
    fn get_sum(&self, left: usize, right: usize) -> usize {
        return self.get_sum_from_start(right) - self.get_sum_from_start(left - 1);
    }
    fn get_sum_from_start(&self, index: usize) -> usize {
        let mut cur_index: usize = index;
        let mut sum_result: usize = 0;
        while cur_index > 0 {
            sum_result += self.data[cur_index];
            cur_index -= Self::lsb(cur_index);
        }
        return sum_result;
    }
}
fn main() {
    input!{n: usize, q: usize, queries: [(usize, usize, usize); q]}
    let mut fenwick = Fenwick::new(n);
    for &(query_type, x, y) in queries.iter() {
        // add data[x] <- y
        if query_type == 0 {
            fenwick.add(x, y);
        } else {
        // get sum of x..=y
            let ans: usize = fenwick.get_sum(x, y);
            println!("{}", ans);
        }
    }
}

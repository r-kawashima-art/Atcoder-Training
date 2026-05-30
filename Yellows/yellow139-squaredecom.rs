use proconio::input;
// DSL_2_A - Range Minimum Query
// Q. Range Minimum Query
// A. Square Decomposition
struct SquareDecomposer {
    n: usize,
    backet_size: usize,
    data: Vec<usize>,
    min_values: Vec<usize>,
}
impl SquareDecomposer {
    const INF: usize = 1 << 32 - 1;
    #[inline]
    fn sqrt(num: usize) -> usize {
        let mut sqrt_num: usize = 0;
        while sqrt_num * sqrt_num <= num {
            sqrt_num += 1;
        }
        return sqrt_num;
    }
    fn new(n: usize) -> Self {
        let backet_size: usize = Self::sqrt(n);
        let data: Vec<usize> = vec![Self::INF; n];
        let min_values: Vec<usize> = vec![Self::INF; backet_size + 1];
        return SquareDecomposer {
            n,
            backet_size,
            data,
            min_values,
        }
    }
    
    fn find(&self, left: usize, right: usize) -> usize {
        let mut result: usize = Self::INF;
        let backet_left: usize = left / self.backet_size;
        let backet_right: usize = right / self.backet_size;
        if backet_left == backet_right {
            for i in left..=right {
                result = result.min(self.data[i]);
            }
            return result;
        }
        // backet_left
        for i in left..(backet_left + 1) * self.backet_size {
            result = result.min(self.data[i]);
        }
        // backets
        for backet_index in backet_left+1..backet_right {
            result = result.min(self.min_values[backet_index]);
        }
        // backet_right
        for i in (backet_right * self.backet_size)..=right {
            result = result.min(self.data[i]);
        }
        return result;
    }

    fn update(&mut self, index: usize, value: usize) {
        self.data[index] = value;
        let backet_index: usize = index / self.backet_size;
        self.min_values[backet_index] = self.min_values[backet_index].min(value);
    }
}

fn main() {
    input!{n: usize, q: usize, query: [(usize, usize, usize); q]}
    let mut square_decomposer = SquareDecomposer::new(n);
    // 0-indexed but x..=y
    for &(query_type, x, y) in query.iter() {
        // update
        if query_type == 0 {
            square_decomposer.update(x, y);
        } else {
        // find
            let ans = square_decomposer.find(x, y);
            println!("{}", ans);
        }
    }
}
use proconio::{input, marker::Usize1};
// 全国統一プログラミング王決定戦本戦 D - Deforestation
// Q. Every Tree grows at 1 unit per second. We can cut trees in range [L, R] at a query's time.
// A. Square Root Decomposition (Bucket Method) for Range Update and Point Query
fn main() {
    input!{n: usize, m: usize, queries: [(usize, Usize1, Usize1); m]}
    let mut heights: Vec<usize> = vec![0; n];
    let n_sqrt: usize = get_sqrt(n);
    // Divide n elements into n_sqrt buckets
    let mut bucket_sums: Vec<usize> = divide_into_buckets(&heights);
    let mut cur_time: usize = 0;
    let mut score: usize = 0;
    for query in 0..m {
        let (time, left_idx, right_idx) = queries[query];
        let growth_time: usize = time - cur_time;
        let plus_score: usize =update_buckets(&mut heights, &mut bucket_sums, growth_time, left_idx, right_idx);
        cur_time = time;
        score += plus_score;
    }
    println!("{}", score);
}

fn update_buckets(heights: &mut Vec<usize>, bucket_sums: &mut Vec<usize>, growth: usize, left: usize, right: usize) -> usize {
    let n_sqrt: usize = bucket_sums.len() - 1;
    let left_bucket: usize = left / n_sqrt;
    let right_bucket: usize = right / n_sqrt;
    for bucket in 0..=n_sqrt {
        bucket_sums[bucket] += growth;
    }
    let mut plus_score: usize = 0;
    
    if left_bucket == right_bucket {
        let left_index: usize = left % n_sqrt;
        let right_index: usize = right % n_sqrt;
        for i in left_index..=right_index {
            plus_score += heights[i];
            heights[i] = 0;
        }
        bucket_sums[left_bucket] = 0;
    } else {
        let left_index: usize = left % n_sqrt;
        let right_index: usize = right % n_sqrt;
        // Left bucket
        for i in left_index..n_sqrt {
            plus_score += heights[left_bucket * n_sqrt + i];
            bucket_sums[left_bucket] -= heights[left_bucket * n_sqrt + i];
            heights[left_bucket * n_sqrt + i] = 0;
        }
        // Middle buckets
        for i in (left_bucket + 1)..(right_bucket) {
            plus_score += bucket_sums[i];
            bucket_sums[i] = 0;
        }
        // Right bucket
        for i in 0..=right_index {
            plus_score += heights[right_bucket * n_sqrt + i];
            bucket_sums[right_bucket] -= heights[right_bucket * n_sqrt + i];
            heights[right_bucket * n_sqrt + i] = 0;
        }
    }
    return plus_score;
}

fn divide_into_buckets(heights: &Vec<usize>) -> Vec<usize> {
    let n: usize = heights.len();
    let n_sqrt: usize = get_sqrt(n);
    let mut bucket_sums: Vec<usize> = vec![0; n_sqrt + 1];
    for bucket in 0..=n_sqrt {
        for i in 0..(n / n_sqrt + 1) {
            let index: usize = bucket * n_sqrt + i;
            if index < n {
                bucket_sums[bucket] += heights[index];
            }
        }
    }
    return bucket_sums;
}

fn get_sqrt(num: usize) -> usize {
    let mut res: usize = 1;
    while res * res < num {
        res += 1;
    }
    return res;
}

use proconio::input;

fn main() {
    input! {n: usize, k: usize, l: usize, r: usize, a: [usize; n]}
    let n1: usize = n / 2;
    let n2: usize = n - n1;
    let left_sums: Vec<Vec<usize>> = get_all_sums(&a[..n1]);
    let right_sums: Vec<Vec<usize>> = get_all_sums(&a[n1..]);
    let mut ans: usize = 0;
    for k1 in 0..=n1 {
        let k2: usize = k - k1;
        if k2 > n2 {
            continue;
        }
        for sum1 in left_sums[k1].iter() {
            let sum2_min: usize = if *sum1 > l { 0 } else { l - *sum1 };
            let sum2_max: usize = match r.checked_sub(*sum1) {
                Some(v) => v,
                None => continue,
            };
            if sum2_max < sum2_min {
                continue;
            }
            let index_min: usize = right_sums[k2].partition_point(|&x| x < sum2_min);
            let index_max: usize = right_sums[k2].partition_point(|&x| x <= sum2_max);
            ans += index_max - index_min;
        }
    }
    println!("{}", ans);
}

fn get_all_sums(array: &[usize]) -> Vec<Vec<usize>> {
    let n: usize = array.len();
    let mut sums: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    sums[0].push(0);
    for &x in array.iter() {
        for k in 0..n {
            for &s in sums[k].clone().iter() {
                sums[k + 1].push(s + x);
            }
        }
    }
    for i in 0..=n {
        sums[i].sort();
    }
    sums
}

use proconio::input;

fn main() {
    input!{n: usize, w_limit: usize, vw: [(i64, usize); n]}
    let n1: usize = n / 2;
    let n2: usize = n - n1;
    let mut sum1: Vec<(i64, usize)> = semi_brute(&vw[..n1]);
    let mut sum2: Vec<(i64, usize)> = semi_brute(&vw[n1..]);
    sum1.sort_by_key(|&(_v, w)| w);
    sum2.sort_by_key(|&(_v, w)| w);
    let mut max_value2: Vec<i64> = vec![-1; sum2.len()+1];
    for i in 1..sum2.len(){
        max_value2[i] = sum2[i].0.max(max_value2[i-1]);
    }
    let mut max_value: i64 = -1;
    for &(value1, weight1) in sum1.iter() {
        let index2: usize = sum2.partition_point(|&(_, w)| w + weight1 <= w_limit);
        if index2 < max_value2.len() && max_value2[index2] != -1 {
            max_value = max_value.max(value1 + max_value2[index2]);
        }
    }
    println!("{}", max_value);
}

fn semi_brute(array: &[(i64, usize)]) -> Vec<(i64, usize)>{
    let n: usize = array.len();
    let bits: usize = 1 << n;
    let mut result: Vec<(i64, usize)> = Vec::new();
    for bit in 0..bits {
        let mut weight: usize = 0;
        let mut value: i64 = 0;
        for i in 0..n {
            if (bit >> i) & 1 == 1 {
                weight += array[i].1;
                value += array[i].0;
            }
        }
        result.push((value, weight));
    }
    result
}

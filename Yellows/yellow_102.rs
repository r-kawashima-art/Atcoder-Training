use proconio::input;

fn main() {
    input!{n: usize, a: [usize; n], }
    let mut reference: Vec<usize> = a.to_vec();
    reference.sort();
    reference.dedup();
    let mut b: Vec<usize> = Vec::new();
    for i in 0..n {
        let index: usize = reference.partition_point(|&x| x < a[i]);
        b.push(index);
    }
    for &ans in b.iter() {
        println!("{}", ans);
    }
}

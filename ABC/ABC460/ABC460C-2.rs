use proconio::input;
// abc460C
// Q. find the number of combination such at b[i] <= 2 * a[i]
// A. Pseudo Shakutori
fn main() {
    input!{n: usize, m: usize, mut a: [usize; n], mut b: [usize; m]}
    a.sort();
    b.sort();
    let mut count: usize = 0;
    let mut ia: usize = 0;
    for ib in 0..m {
        while ia < n && b[ib] > 2 * a[ia] {
            ia += 1;
        }
        if ia < n && a[ia] * 2 >= b[ib] {
            count += 1;
        }
        ia += 1;
    }
    println!("{}", count);
}
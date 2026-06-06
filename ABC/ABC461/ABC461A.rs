use proconio::input;

fn main() {
    input!{a: usize, d: usize}
    if a <= d {
        println!("Yes");
    } else {
        println!("No");
    }
}

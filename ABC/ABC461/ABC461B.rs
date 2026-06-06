use proconio::{input, marker::Usize1};

fn main() {
    input!{n: usize, a: [usize; n], b: [usize; n]}
    let mut is_honest: usize = true;
    for person in 0..n {
        let tool: usize = a[person];
        let true_person: usize = b[tool];
        if person != true_person {
            is_honest = false;
        }
    }
    if is_honest {
        println!("Yes");
    } else {
        println!("No");
    }
}

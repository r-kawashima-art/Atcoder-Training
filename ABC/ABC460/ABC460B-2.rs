use proconio::input;
fn has_intersections(x1: i64, y1: i64, r1: i64, x2: i64, y2: i64, r2: i64) -> bool {
    let dist_square = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);
    let r_sum_square = (r1 + r2) * (r1 + r2);
    let r_diff_square (r1 - r2) * (r1 - r2);
    
    r_diff_square <= dist_square && dist_square <= r_sum_square
}
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            x1: i64, y1: i64, r1: i64,
            x2: i64, y2: i64, r2: i64,
        }
        if has_intersections(x1, y1, r1, x2, y2, r2) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
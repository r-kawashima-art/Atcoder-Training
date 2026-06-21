use proconio::input;
// abc463c - who is the tallest?
// Q. hl = height + leaving time. Who is the tallest at time=t?
// A. Leave the most superb.
fn main() {
    input! {n: usize, hl: [(usize, isize); n], q: usize, tq: [isize; q]}
    // upper_replacement[i] = (higher, later leaving)
    let mut upper_replacement: Vec<(usize, isize)> = Vec::new();
    for &(height, leaving_time) in hl.iter() {
        while upper_replacement.len() > 0 && upper_replacement.last().unwrap().0 <= height {
            upper_replacement.pop();
        }
        upper_replacement.push((height, leaving_time));
    }
    let mut answer: Vec<usize> = vec![0; q];
    for (i, &time) in tq.iter().enumerate() {
        let index: usize = upper_replacement.partition_point(|&(_, lt)| lt <= time);
        answer[i] = upper_replacement[index].0;
    }
    println!(
        "{}",
        answer
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

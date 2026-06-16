// string matching algorithm
use proconio::input;
// abc430E:
// Q. How many rotations of String: A are at least needed to match it to String: B?
// A. z_algorithm
fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {a: String, b: String}
        let double_a: String = a.clone() + &a;
        let matched: Vec<usize> = z_algorithm(&double_a, &b);
        if matched.len() == 0 {
            println!("-1");
            continue;
        }
        let ans: usize = matched[0];
        println!("{}", ans);
    }
}

fn get_z_array(text: &str, pattern: &str) -> Vec<usize> {
    /* precomutes the z_array for z_algorithm
    Args:
        text: The text to search in
        pattern: The pattern to search for
    Returns:
        The z_array
    */
    // check if pattern -> $ -> text
    let combined: Vec<char> = format!("{}${}", pattern, text).chars().collect();
    let n: usize = combined.len();
    let mut z_array: Vec<usize> = vec![0; n];
    // z_window: [left right]
    let mut left: usize = 0;
    let mut right: usize = 0;
    for i in 1..n {
        if i > right {
            // inside the z window
            if i <= right {
                z_array[i] = (right + 1 - i).min(z_array[i - 1]);
            }
            // manually expand the z_window if possible
            while i + z_array[i] < n && combined[z_array[i]] == combined[i + z_array[i]] {
                z_array[i] += 1;
            }
            if i + z_array[i] - 1 > right {
                left = i;
                right = i + z_array[i] - 1;
            }
        }
    }
    return z_array;
}

fn z_algorithm(text: &str, pattern: &str) -> Vec<usize> {
    let z_array: Vec<usize> = get_z_array(text, pattern);
    let n: usize = text.len();
    let m: usize = pattern.len();
    let mut matched: Vec<usize> = Vec::new();
    for i in m + 1..n + m + 1 {
        if z_array[i] == m {
            matched.push(i - m - 1);
        }
    }
    return matched;
}

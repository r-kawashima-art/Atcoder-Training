use proconio::{input, marker::Usize1};
fn main() {
    // m: minimum gem types
    input!{n: usize, k: usize, m: usize, mut cv: [(Usize1, usize); n]}
    let mut is_gem_used: Vec<usize> = vec![0; n];
    cv.sort_by(|a, b| b.1.cmp(&a.1));
    let mut sum_of_topK: usize = 0;
    let mut used_gem_types: usize = 0;
    // used_gems[gem] = vector of prices
    let mut used_gems: Vec<Vec<usize>> = vec![vec![]; m];
    for i in 0..k {
        for &(gem_type, value) in cv.iter() {
            if is_gem_used[gem_type] == 0 {
                used_gem_types += 1;
            }
            is_gem_used[gem_type] += 1;
            sum_of_topK += value; 
        }
    }
    let mut current_return_index: usize = k - 1;
    let mut current_add_index: usize = k;
    while used_gem_types < m {
        while is_gem_used[cv[current_return_index].0] == 1 || cv[current_return_index].0 == cv[current_add_index].0 {
            current_return_index -= 1;
        }
        let (return_gem_type, return_value) = cv[current_return_index];
        let (add_gem_type, add_value) = cv[current_add_index];
        sum_of_topK += add_value;
        sum_of_topK -= return_value;
        is_gem_used[return_gem_type] -= 1;
        is_gem_used[add_gem_type] += 1;
        current_add_index += 1;
        current_return_index -= 1;
        used_gem_types += 1;
    }
    println!("{}", sum_of_topK);
}

use proconio::{input, marker::Usize1};
fn main() {
    // m: minimum gem types
    input!{n: usize, k: usize, m: usize, mut cv: [(Usize1, usize); n]}
    
    // Sort gems by value descending
    cv.sort_by(|a, b| b.1.cmp(&a.1));

    let mut color_counts = vec![0; n];
    let mut current_colors = 0;
    let mut total_value: usize = 0;

    // Start with the top K gems
    for i in 0..k {
        let (c, v) = cv[i];
        if color_counts[c] == 0 {
            current_colors += 1;
        }
        color_counts[c] += 1;
        total_value += v;
    }

    // If we need more distinct colors, perform the best possible swaps
    if current_colors < m {
        let mut replaceable = vec![];
        let mut temp_counts = color_counts.clone();
        // Find gems in top K that can be removed (smallest values first)
        for i in (0..k).rev() {
            let (c, v) = cv[i];
            if temp_counts[c] > 1 {
                replaceable.push(v);
                temp_counts[c] -= 1;
            }
        }

        let mut candidates = vec![];
        let mut seen_in_rest = vec![false; n];
        // Find gems outside top K that provide new colors (largest values first)
        for i in k..n {
            let (c, v) = cv[i];
            if color_counts[c] == 0 && !seen_in_rest[c] {
                candidates.push(v);
                seen_in_rest[c] = true;
            }
        }

        let swaps_needed = m - current_colors;
        for i in 0..swaps_needed {
            total_value = total_value - replaceable[i] + candidates[i];
        }
    }

    println!("{}", total_value);
}

use proconio::input;
// DSL_4_A - Union of Rectangles
// Q. Some rectangles are given. Get the area of the union of the rectangles.
// A. Coordinate compression + 2D Imos
fn main() {
    input! {n: usize, rectangles: [(i64, i64, i64, i64); n]}
    let (x1_compressed, x2_compressed, y1_compressed, y2_compressed, x_coord, y_coord) =
        get_compressed_coord(&rectangles);

    let x_len = x_coord.len();
    let y_len = y_coord.len();
    let mut imos: Vec<Vec<i32>> = vec![vec![0; x_len + 2]; y_len + 2];
    for i in 0..n {
        imos[y1_compressed[i]][x1_compressed[i]] += 1;
        imos[y1_compressed[i]][x2_compressed[i]] -= 1;
        imos[y2_compressed[i]][x1_compressed[i]] -= 1;
        imos[y2_compressed[i]][x2_compressed[i]] += 1;
    }
    for i in 0..y_len {
        for j in 0..x_len {
            imos[i][j + 1] += imos[i][j];
        }
    }
    for i in 0..y_len {
        for j in 0..x_len {
            imos[i + 1][j] += imos[i][j];
        }
    }

    let mut ans: i64 = 0;
    for i in 0..y_len - 1 {
        for j in 0..x_len - 1 {
            if imos[i][j] > 0 {
                ans += (x_coord[j + 1] - x_coord[j]) * (y_coord[i + 1] - y_coord[i]);
            }
        }
    }
    println!("{}", ans);
}

fn get_compressed_coord(
    rectangles: &Vec<(i64, i64, i64, i64)>,
) -> (
    Vec<usize>,
    Vec<usize>,
    Vec<usize>,
    Vec<usize>,
    Vec<i64>,
    Vec<i64>,
) {
    let n: usize = rectangles.len();
    let mut x1_original: Vec<i64> = vec![];
    let mut y1_original: Vec<i64> = vec![];
    let mut x2_original: Vec<i64> = vec![];
    let mut y2_original: Vec<i64> = vec![];
    let mut x_coord: Vec<i64> = vec![];
    let mut y_coord: Vec<i64> = vec![];
    for i in 0..n {
        x1_original.push(rectangles[i].0);
        x2_original.push(rectangles[i].2);
        x_coord.push(rectangles[i].0);
        x_coord.push(rectangles[i].2);
        y1_original.push(rectangles[i].1);
        y2_original.push(rectangles[i].3);
        y_coord.push(rectangles[i].1);
        y_coord.push(rectangles[i].3);
    }
    x_coord.sort();
    x_coord.dedup();
    y_coord.sort();
    y_coord.dedup();
    let x1_compressed: Vec<usize> = x1_original
        .iter()
        .map(|&x| x_coord.binary_search(&x).unwrap())
        .collect();
    let x2_compressed: Vec<usize> = x2_original
        .iter()
        .map(|&x| x_coord.binary_search(&x).unwrap())
        .collect();
    let y1_compressed: Vec<usize> = y1_original
        .iter()
        .map(|&y| y_coord.binary_search(&y).unwrap())
        .collect();
    let y2_compressed: Vec<usize> = y2_original
        .iter()
        .map(|&y| y_coord.binary_search(&y).unwrap())
        .collect();

    return (
        x1_compressed,
        x2_compressed,
        y1_compressed,
        y2_compressed,
        x_coord,
        y_coord,
    );
}

use proconio::input;
// GRL_5_C - LCA: Lowest Common Ancestor
// Q. Given a rooted tree and two vertices, find their lowest common ancestor (LCA).
// A. DFS & Doubling: Get parents of each vertex, then jump up the tree by powers of 2 to find the LCA.
// A. Use the depths to utilize doubling
fn main() {
    input!{n: usize}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for v in 0..n {
        input!{k: usize, neighbors: [usize; k]}
        for &u in neighbors.iter() {
            graph[v].push(u);
        }
    }
    input!{q: usize, queries: [(usize, usize); q]}
    // grand_parent = parents[parents[v]]
    let mut parents: Vec<isize> = vec![-1; n];
    let mut depths: Vec<usize> = vec![0; n];
    get_parents(0, -1, 0, &graph, &mut parents, &mut depths);
    let mut doubling: Vec<Vec<isize>> = vec![vec![-1; n]; 20];
    for v in 0..n {
        doubling[0][v] = parents[v];
    }
    for phase in 1..20 {
        for v in 0..n {
            if doubling[phase - 1][v] == -1 {
                doubling[phase][v] = -1;
            } else {
                doubling[phase][v] = doubling[phase - 1][doubling[phase - 1][v] as usize];
            }
        }
    }
    for &(v1, v2) in queries.iter() {
        let lowest_common_ancestor: usize = lca(v1, v2, &depths, &doubling);
        println!("{}", lowest_common_ancestor);
    }
}

fn get_parents(current: isize, parent: isize, depth: usize, graph: &Vec<Vec<usize>>, parents: &mut Vec<isize>, depths: &mut Vec<usize>) {
    parents[current as usize] = parent;
    depths[current as usize] = depth;
    for &neighbor in graph[current as usize].iter() {
        if neighbor as isize != parent {
            get_parents(neighbor as isize, current, depth + 1, graph, parents, depths);
        }
    }
}

fn lca(mut v1: usize, mut v2: usize, depths: &Vec<usize>, doubling: &Vec<Vec<isize>>) -> usize {
    if depths[v1] < depths[v2] {
        std::mem::swap(&mut v1, &mut v2);
    }
    let d = depths[v1] - depths[v2];
    for phase in 0..20 {
        if (d >> phase) & 1 == 1 {
            v1 = doubling[phase][v1] as usize;
        }
    }
    if v1 == v2 {
        return v1;
    }
    for phase in (0..20).rev() {
        if doubling[phase][v1] != doubling[phase][v2] {
            v1 = doubling[phase][v1] as usize;
            v2 = doubling[phase][v2] as usize;
        }
    }
    doubling[0][v1] as usize
}

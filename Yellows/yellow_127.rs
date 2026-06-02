use proconio::input;
// GRL_7_A - Bipartite Matching
// Q. Bipartite Maximum Matching
// A. Network Maximum Flow (Ford-Fulkerson or Dinic methods)
#[derive(Debug, Copy, Clone)]
struct Edge {
    to: usize,
    capacity: i64,
    rev: usize,
}
impl Edge {
    fn new(to: usize, capacity: i64, rev: usize) -> Self {
        Self { to, capacity, rev}
    }
}
struct Dinic {
    graph: Vec<Vec<Edge>>,
    level: Vec<usize>,
    iter: Vec<usize>,// Current edge pointer
}
impl Dinic {
    const INF: usize = usize::MAX;
    fn new(n: usize) -> Self {
        Self {
            graph: vec![vec![]; n],
            level: vec![0; n],
            iter: vec![0; n],
        }
    }
    fn add_edge(&mut self, from: usize, to: usize, capacity: i64) {
        // Node "to" seen from Node "from"
        let rev_from = self.graph[to].len();
        let rev_to = self.graph[from].len();
        self.graph[from].push(Edge::new(to, capacity, rev_from));
        self.graph[to].push(Edge::new(from, 0, rev_to));
    }
    fn get_max_flow(&mut self, source: usize, sink: usize) -> i64 {
        if source == sink {
            return Self::INF as i64;
        }
        let mut max_flow = 0;
        // while there is a path from source to sink, we can send flow
        while self.bfs_get_level(source, sink) {
            self.iter = vec![0; self.graph.len()];
            loop {
                let flow = self.send_flow(source, sink, i64::MAX);
                if flow == 0 {
                    break;
                }
                max_flow += flow;
            }
        }
        return max_flow;
    }
    fn bfs_get_level(&mut self, source: usize, sink: usize) -> bool {
        self.level = vec![Self::INF; self.graph.len()];
        self.level[source] = 0;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(source);
        while let Some(current) = queue.pop_front() {
            for edge in self.graph[current].iter() {
                if edge.capacity > 0 && self.level[edge.to] == Self::INF {
                    self.level[edge.to] = self.level[current] + 1;
                    queue.push_back(edge.to);
                }
            }
        }

        self.level[sink] != Self::INF
    }

    fn send_flow(&mut self, current: usize, sink: usize, incoming_flow: i64) -> i64 {
        if current == sink {
            return incoming_flow;
        }
        
        while self.iter[current] < self.graph[current].len() {
            let i = self.iter[current];
            let edge = self.graph[current][i]; // Edge is Copy

            if edge.capacity > 0 && self.level[edge.to] == self.level[current] + 1 {
                let current_flow = incoming_flow.min(edge.capacity);
                let sent_flow = self.send_flow(edge.to, sink, current_flow);
                
                if sent_flow > 0 {
                    self.graph[current][i].capacity -= sent_flow;
                    let rev_idx = self.graph[current][i].rev;
                    self.graph[edge.to][rev_idx].capacity += sent_flow;
                    return sent_flow;
                }
            }
            self.iter[current] += 1;
        }
        return 0;
    }
}
fn main() {
    input!{x_num: usize, y_num: usize, edges_num: usize, edges: [(usize, usize); edges_num]}
    let mut network_flow = Dinic::new(x_num + y_num + 2);
    // start: 0, x's indexes: 1..=x_num, y's indexes: x_num+1..=x_num+y_num, end: x_num+y_num+1
    for x_index in 1..=x_num {
        network_flow.add_edge(0, x_index, 1);
    }
    for y_index in x_num+1..=x_num+y_num {
        network_flow.add_edge(y_index, x_num+y_num+1, 1);
    }
    for &(u, v) in edges.iter() {
        let x_index: usize = u + 1;
        let y_index: usize = v + x_num + 1;
        network_flow.add_edge(x_index, y_index, 1);
    }
    let ans = network_flow.get_max_flow(0, x_num+y_num+1);
    println!("{}", ans);
}

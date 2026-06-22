use proconio::input;
// GRL_6_A
// Q. What is the maximum flow from S to T in a directed graph with capacities?
// A. Foldfulkerson's algorithm for Maximum Flow
// Note: Max flow and Min cut are dual problems. The max flow value is equal to the capacity of the min cut.
// continue
#[derive(Debug, Copy, Clone)]
struct Edge {
    to: usize,
    rev: usize,
    capacity: i64,
}

impl Edge {
    fn new(to: usize, rev: usize, capacity: i64) -> Self {
        Edge {to, rev, capacity}
    }
}

struct ResidualGraph {
    graph: Vec<Vec<Edge>>,
}

impl ResidualGraph {
    const INFINITE_SUPPLY: i64 = 1 << 60;
    fn new(n: usize) -> Self {
        ResidualGraph {
            graph: vec![vec![]; n],
        }
    }
    fn add_edge(&mut self, from: usize, to: usize, capacity: i64) {
        let to_in_from_index: usize = self.graph[from].len();
        let from_in_to_index: usize = self.graph[to].len();
        let from_rev_index: usize = self.graph[to].len();
        let forward_edge = Edge::new(to, from_in_to_index, capacity);
        let backward_edge = Edge::new(from, to_in_from_index, 0);
        self.graph[from].push(forward_edge);
        self.graph[to].push(backward_edge);
    }
    fn send_flow_dfs(&mut self, source: usize, sink: usize, incoming_flow: i64, visited: &mut Vec<bool>) -> i64 {
        // DFS to find a path from source to sink and send flow through it. Returns the amount of flow sent.
        if source == sink {
            return incoming_flow;
        }
        visited[source] = true;
        for edge_index in 0..self.graph[source].len() {
            let edge: Edge = self.graph[source][edge_index];
            if visited[edge.to] || edge.capacity <= 0 {
                continue;
            }
            let flow_to_send: i64 = incoming_flow.min(edge.capacity);
            let sent_flow: i64 = self.send_flow_dfs(edge.to, sink, flow_to_send, visited);
            if sent_flow > 0 {
                // Update the capacities of the forward and backward edges
                self.graph[source][edge_index].capacity -= sent_flow;
                self.graph[edge.to][edge.rev].capacity += sent_flow;
                return sent_flow;
            }
        }
        return 0;
    }
    fn get_max_flow(&mut self, source: usize, sink: usize) -> i64 {
        /* Continues sending flow as long as there is a path from source to sink. */
        let mut max_flow = 0;
        loop {
            let mut visited: Vec<bool> = vec![false; self.graph.len()];
            // find ONE path from source to sink and send flow through it
            let sent_flow: i64 = self.send_flow_dfs(source, sink, Self::INFINITE_SUPPLY, &mut visited);
            if sent_flow == 0 {
                break;
            }
            max_flow += sent_flow;
        }
        return max_flow;
    }
}

fn main() {
    input!{vertices: usize, edges: usize, flows: [(usize, usize, i64); edges]}
    let mut graph = ResidualGraph::new(vertices);
    for &(from, to, capacity) in flows.iter() {
        graph.add_edge(from, to, capacity);
    }
    let max_flow = graph.get_max_flow(0, vertices - 1);
    println!("{}", max_flow);
}

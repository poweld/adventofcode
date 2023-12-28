mod my {
    use std::collections::{BinaryHeap, VecDeque};
    use std::cmp::Ordering;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    #[derive(Debug)]
    pub struct Edge {
        node: usize,
        cost: usize,
        direction: Direction,
    }

    pub type Graph = Vec<Vec<Edge>>;

    #[derive(Clone, PartialEq, Eq)]
    struct State {
        cost: usize,
        node: usize,
        directions: VecDeque<Direction>,
    }
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
                .then_with(|| self.node.cmp(&other.node))
        }
    }
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    pub fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
        let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
        let mut heap = BinaryHeap::new();

        // We're at `start`, with a zero cost
        dist[start] = 0;
        heap.push(State { cost: 0, node: start, directions: VecDeque::new() });
        let mut shortest: Option<usize> = None;

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(State { cost, node, directions }) = heap.pop() {
            if directions.len() == 3 && directions.iter().all(|d| d == &directions[0]) {
                continue;
            }

            if node == goal {
                if let Some(shortest_value) = shortest {
                    if cost < shortest_value {
                        shortest.replace(cost);
                    }
                } else {
                    shortest = Some(cost);
                }
                continue;
            }
            // if node == goal { continue; }

            // Important as we may have already found a better way
            if cost > dist[node] { continue; }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for edge in &adj_list[node] {
                let mut directions = directions.clone();
                directions.push_back(edge.direction.clone());
                if directions.len() > 3 {
                    directions.pop_front();
                }
                let next = State { cost: cost + edge.cost, node: edge.node, directions };

                // If so, add it to the frontier and continue
                if next.cost < dist[next.node] {
                    heap.push(next.clone());
                    // Relaxation, we have now found a better way
                    dist[next.node] = next.cost;
                }
            }
        }

        shortest
    }


    pub fn parse(input: &str) -> Graph {
        let lines = input.lines();
        let mut row_count: usize = 0;
        let mut costs: Vec<usize> = Vec::new();
        for line in lines {
            row_count += 1;
            let digits = line.chars().map(|c| (c.to_digit(10).unwrap() as usize));
            costs.extend(digits);
        }
        let col_count = costs.len() / row_count;
        let graph: Graph = (0..costs.len()).map(|node| {
            let mut edges: Vec<Edge> = Vec::new();
            if node % col_count != 0 {
                edges.push(Edge { node: node - 1, cost: costs[node - 1], direction: Direction::West });
            }
            if node % col_count != col_count - 1 {
                edges.push(Edge { node: node + 1, cost: costs[node + 1], direction: Direction::East });
            }
            if node >= col_count {
                edges.push(Edge { node: node - col_count, cost: costs[node - col_count], direction: Direction::North });
            }
            if node < (row_count - 1) * col_count {
                edges.push(Edge { node: node + col_count, cost: costs[node + col_count], direction: Direction::South });
            }
            edges
        }).collect();
        graph
    }
}


use my::*;


pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let graph = parse(&input);
    dbg!(shortest_path(&graph, 0, graph.len() - 1));
    // dbg!(shortest_path(&graph, 0, 5));
    shortest_path(&graph, 0, graph.len() - 1).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 102.to_string());
    }
}

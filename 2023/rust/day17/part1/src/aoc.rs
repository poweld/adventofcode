mod my {
    // use std::collections::{HashSet, HashMap, VecDeque, BinaryHeap};
    // use std::hash::Hash;
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
    }

    pub type Graph = Vec<Vec<Edge>>;

    #[derive(Copy, Clone, PartialEq, Eq)]
    struct State {
        cost: usize,
        position: usize,
        direction: Direction,
    }
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
                .then_with(|| self.position.cmp(&other.position))
        }
    }
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
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
                edges.push(Edge { node: node - 1, cost: costs[node - 1] });
            }
            if node % col_count != col_count - 1 {
                edges.push(Edge { node: node + 1, cost: costs[node + 1] });
            }
            if node >= col_count {
                edges.push(Edge { node: node - col_count, cost: costs[node - col_count] });
            }
            if node < (row_count - 1) * col_count {
                edges.push(Edge { node: node + col_count, cost: costs[node + col_count] });
            }
            edges
        }).collect();
        graph
    }
}


use my::*;


pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    dbg!(parse(&input));
    todo!()
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

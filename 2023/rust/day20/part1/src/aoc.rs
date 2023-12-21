mod my {
    use std::collections::{HashMap, VecDeque};

    pub type EdgeIndex = usize;
    pub type NodeIndex = usize;

    #[derive(Debug)]
    pub struct NodeData {
        // Singly linked lists
        first_forward_edge: Option<EdgeIndex>,
        first_backward_edge: Option<EdgeIndex>,
        module: Module,
    }
    impl NodeData {
        pub fn new(module: Module) -> Self {
            let first_forward_edge = None;
            let first_backward_edge = None;
            Self { first_forward_edge, first_backward_edge, module }
        }
    }

    #[derive(Debug)]
    pub struct EdgeData {
        dest: NodeIndex,
        next_edge: Option<EdgeIndex>,
    }
    impl EdgeData {
        pub fn new(dest: NodeIndex) -> Self {
            let next_edge = None;
            Self { dest, next_edge }
        }
    }

    #[derive(Debug)]
    pub struct Graph {
        nodes: Vec<NodeData>,
        forward_edges: Vec<EdgeData>,
        backward_edges: Vec<EdgeData>,
        low_pulse_count: u64,
        high_pulse_count: u64,
    }
    impl Graph {
        pub fn new() -> Self {
            let nodes = vec![];
            let forward_edges = vec![];
            let backward_edges = vec![];
            let low_pulse_count = 0;
            let high_pulse_count = 0;
            Self { nodes, forward_edges, backward_edges, low_pulse_count, high_pulse_count }
        }
        pub fn add_node(&mut self, module: Module) -> NodeIndex {
            let new_node_index = self.nodes.len();
            self.nodes.push(NodeData::new(module));
            new_node_index
        }
        pub fn add_edge(&mut self, source: NodeIndex, dest: NodeIndex) {
            {
                // Forward edge
                let new_edge_index = self.forward_edges.len();
                let source_node = &mut self.nodes[source];
                let mut new_edge = EdgeData::new(dest);
                new_edge.next_edge = source_node.first_forward_edge;
                self.forward_edges.push(new_edge);
                source_node.first_forward_edge = Some(new_edge_index);
            }
            {
                // Back edge
                let new_edge_index = self.backward_edges.len();
                let source_node = &mut self.nodes[dest];
                let mut new_edge = EdgeData::new(source);
                new_edge.next_edge = source_node.first_backward_edge;
                self.backward_edges.push(new_edge);
                source_node.first_backward_edge = Some(new_edge_index);
            }
        }
        pub fn neighbors(&self, node: NodeIndex) -> Neighbors {
            Neighbors { graph: &self, edge: self.nodes[node].first_forward_edge }
        }
        pub fn parents(&self, node: NodeIndex) -> Parents {
            Parents { graph: &self, edge: self.nodes[node].first_backward_edge }
        }
        pub fn send_pulse(&mut self, source: NodeIndex, dest: NodeIndex, pulse: Pulse) {
            let mut pulse_queue: VecDeque<(NodeIndex, NodeIndex, Pulse)> = VecDeque::new();
            pulse_queue.push_back((source, dest, pulse));
            while !pulse_queue.is_empty() {
                let (source, dest, pulse) = pulse_queue.pop_front().unwrap();
                match pulse {
                    Pulse::Low => self.low_pulse_count += 1,
                    Pulse::High => self.high_pulse_count += 1,
                }
                // TODO had to pull this up here to avoid mutable then immutable borrow issue
                // Can this be worked around in a better way?
                let parents: Vec<NodeIndex> = self.parents(dest).collect();
                let module = &mut self.nodes[dest].module;
                match module {
                    Module::Broadcast => {
                        let neighbors: Vec<NodeIndex> = self.neighbors(dest).collect();
                        for neighbor in neighbors {
                            pulse_queue.push_back((dest, neighbor, pulse));
                        }
                    },
                    Module::FlipFlop(_, ref mut on) => match pulse {
                        Pulse::High => (),
                        Pulse::Low => match on {
                            false => {
                                *on = true;
                                let neighbors: Vec<NodeIndex> = self.neighbors(dest).collect();
                                for neighbor in neighbors {
                                    pulse_queue.push_back((dest, neighbor, Pulse::High));
                                }
                            },
                            true => {
                                *on = false;
                                let neighbors: Vec<NodeIndex> = self.neighbors(dest).collect();
                                for neighbor in neighbors {
                                    pulse_queue.push_back((dest, neighbor, Pulse::Low));
                                }
                            },
                        }
                    },
                    Module::Conjunction(_, ref mut last_pulses) => {
                        last_pulses.insert(source, pulse);
                        let send_high = parents.iter().any(|parent| last_pulses.get(parent) != Some(&Pulse::High));
                        let neighbors: Vec<NodeIndex> = self.neighbors(dest).collect();
                        for neighbor in neighbors {
                            if send_high {
                                pulse_queue.push_back((dest, neighbor, Pulse::High));
                            } else {
                                pulse_queue.push_back((dest, neighbor, Pulse::Low));
                            }
                        }
                    },
                    _ => (),
                }
            }
        }
        pub fn low_pulse_count(&self) -> u64 {
            self.low_pulse_count
        }
        pub fn high_pulse_count(&self) -> u64 {
            self.high_pulse_count
        }
    }

    #[derive(Debug)]
    pub struct Neighbors<'graph> {
        graph: &'graph Graph,
        edge: Option<EdgeIndex>,
    }
    impl Iterator for Neighbors<'_> {
        type Item = NodeIndex;
        fn next(&mut self) -> Option<NodeIndex> {
            if !self.edge.is_some() {
                return None;
            }
            let edge_data = &self.graph.forward_edges[self.edge.unwrap()];
            let result = Some(edge_data.dest);
            self.edge = edge_data.next_edge;
            result
        }
    }

    #[derive(Debug)]
    pub struct Parents<'graph> {
        graph: &'graph Graph,
        edge: Option<EdgeIndex>,
    }
    impl Iterator for Parents<'_> {
        type Item = NodeIndex;
        fn next(&mut self) -> Option<NodeIndex> {
            if !self.edge.is_some() {
                return None;
            }
            let edge_data = &self.graph.backward_edges[self.edge.unwrap()];
            let result = Some(edge_data.dest);
            self.edge = edge_data.next_edge;
            result
        }
    }

    #[derive(Debug)]
    pub enum Module {
        Broadcast,
        FlipFlop(String, bool),
        Conjunction(String, HashMap<NodeIndex, Pulse>),
        Inert(String),  // If no other type is found
    }
    impl Module {
        pub fn new_broadcast() -> Self {
            Self::Broadcast
        }
        pub fn new_flipflop(name: String) -> Self {
            Self::FlipFlop(name, false)
        }
        pub fn new_conjunction(name: String) -> Self {
            Self::Conjunction(name, HashMap::new())
        }
        pub fn new_inert(name: String) -> Self {
            Self::Inert(name)
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Pulse {
        High,
        Low,
    }

    #[derive(Debug)]
    pub struct ParseResult {
        pub nodes: HashMap<String, NodeIndex>,
        pub graph: Graph,
    }
    
    pub fn parse(input: &str) -> ParseResult {
        let modules: HashMap<String, (Option<char>, Vec<String>)> = input.lines()
            .map(|line| {
                let (source, neighbors) = line.split_once(" -> ").unwrap();
                if source == "broadcaster" {
                    let module_char: Option<char> = None;
                    let neighbors = neighbors.split(", ").map(String::from).collect::<Vec<_>>();
                    return (source.to_string(), (module_char, neighbors))
                } else {
                    let source_chars: Vec<char> = source.chars().collect();
                    let module_char: Option<char> = Some(source_chars[0]);
                    let source = source_chars[1..].into_iter().collect::<String>();
                    let neighbors = neighbors.split(", ").map(String::from).collect::<Vec<_>>();
                    return (source.to_string(), (module_char, neighbors))
                }
            })
            .collect();
        let mut graph = Graph::new();
        let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
        for (source, (module_char, _)) in modules.clone() {
            let module = match module_char {
                Some('%') => Module::new_flipflop(source.to_string()),
                Some('&') => Module::new_conjunction(source.to_string()),
                None => Module::new_broadcast(),
                _ => panic!("unexpected module char: {module_char:?}"),
            };
            let node = graph.add_node(module);
            nodes.insert(source, node);
        }
        for (_, (_, neighbors)) in modules.clone() {
            // Second run through to find inert sinks
            for neighbor in neighbors {
                if !nodes.contains_key(&neighbor) {
                    let node = graph.add_node(Module::new_inert(neighbor.to_string()));
                    nodes.insert(neighbor, node);
                }
            }
        }
        for (source, (_, neighbors)) in modules.clone() {
            // Third run through to build edges T_T
            let source_node = nodes.get(&source).unwrap();
            let dest_nodes = neighbors.into_iter().rev().map(|neighbor| nodes.get(&neighbor).unwrap());
            for dest_node in dest_nodes {
                graph.add_edge(*source_node, *dest_node);
            }
        }
        ParseResult { nodes, graph }
    }
}

use my::*;

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { nodes, mut graph } = parse(&input);
    let broadcaster = nodes.get(&String::from("broadcaster")).unwrap();
    for _ in 0..1000 {
        graph.send_pulse(0, *broadcaster, Pulse::Low);
    }
    (graph.low_pulse_count() * graph.high_pulse_count()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 11687500.to_string());
    }
}

use std::fmt::Debug;

mod my {
    pub type EdgeIndex = usize;
    pub type NodeIndex = usize;

    #[derive(Debug)]
    pub struct NodeData {
        first_forward_edge: Option<EdgeIndex>,
        first_backward_edge: Option<EdgeIndex>,
    }
    impl NodeData {
        pub fn new() -> Self {
            let first_forward_edge = None;
            let first_backward_edge = None;
            Self { first_forward_edge, first_backward_edge }
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
        // Singly linked lists
        nodes: Vec<NodeData>,
        forward_edges: Vec<EdgeData>,
        backward_edges: Vec<EdgeData>,
    }
    impl Graph {
        pub fn new() -> Self {
            let nodes = vec![];
            let forward_edges = vec![];
            let backward_edges = vec![];
            Self { nodes, forward_edges, backward_edges }
        }
        pub fn add_node(&mut self) -> NodeIndex {
            let new_node_index = self.nodes.len();
            self.nodes.push(NodeData::new());
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
}

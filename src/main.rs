use std::collections::BTreeSet;


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    name: String,
}
impl Node {
    #[inline] pub fn name(&self) -> &str { self.name.as_str() }
}


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct UEdge {
    smaller_node: Node,
    greater_node: Node,
}
impl UEdge {
    pub fn new(one_node: Node, other_node: Node) -> Self {
        if one_node > other_node {
            return Self::new(other_node, one_node);
        }
        Self {
            smaller_node: one_node,
            greater_node: other_node,
        }
    }

    #[inline] pub fn smaller_node(&self) -> &Node { &self.smaller_node }
    #[inline] pub fn greater_node(&self) -> &Node { &self.greater_node }
}


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct UGraph {
    nodes: BTreeSet<Node>,
    edges: BTreeSet<UEdge>,
}
impl UGraph {
    pub fn new() -> Self {
        Self {
            nodes: BTreeSet::new(),
            edges: BTreeSet::new(),
        }
    }

    #[inline] pub fn nodes(&self) -> &BTreeSet<Node> { &self.nodes }
    #[inline] pub fn edges(&self) -> &BTreeSet<UEdge> { &self.edges }
    #[inline] pub fn add_node(&mut self, node: Node) -> bool { self.nodes.insert(node) }

    pub fn add_edge(&mut self, edge: UEdge) -> bool {
        if !self.nodes.contains(edge.greater_node()) {
            return false;
        }
        if !self.nodes.contains(edge.smaller_node()) {
            return false;
        }
        self.edges.insert(edge)
    }

    pub fn remove_node(&mut self, node: &Node) -> bool {
        // remove edges containing this node
        let mut remove_us = Vec::new();
        for edge in self.edges() {
            if edge.greater_node() == node || edge.smaller_node() == node {
                remove_us.push(edge.clone());
            }
        }
        for edge in remove_us {
            self.remove_edge(&edge);
        }

        self.nodes.remove(node)
    }

    #[inline] pub fn remove_edge(&mut self, edge: &UEdge) -> bool { self.edges.remove(edge) }

    pub fn neighbors_of(&self, node: &Node) -> BTreeSet<&Node> {
        let mut ret = BTreeSet::new();
        for edge in self.edges() {
            if edge.greater_node() == node {
                ret.insert(edge.smaller_node());
            }
            if edge.smaller_node() == node {
                ret.insert(edge.greater_node());
            }
        }
        ret
    }
}


fn is_k5(graph: &UGraph) -> bool {
    if graph.nodes().len() != 5 {
        return false;
    }
    if graph.edges().len() != 10 {
        return false;
    }

    for node in graph.nodes() {
        let neighbors = graph.neighbors_of(node);
        if neighbors.len() != 4 {
            return false;
        }
    }

    true
}


fn is_k33(graph: &UGraph) -> bool {
    if graph.nodes().len() != 6 {
        return false;
    }
    if graph.edges().len() != 9 {
        return false;
    }

    // for illustration purposes, let's pretend the first node is a house
    let first_house = graph.nodes().iter().nth(0).unwrap();
    let utilities = graph.neighbors_of(first_house);
    if utilities.len() != 3 {
        return false;
    }

    for edge in graph.edges() {
        // one must be a utility, the other must not
        let smaller_is_utility = utilities.contains(edge.smaller_node());
        let greater_is_utility = utilities.contains(edge.greater_node());
        if smaller_is_utility == greater_is_utility {
            return false;
        }
    }

    true
}


fn is_planar(graph: &UGraph) {
    let mut minor = graph.clone();

    // remove any self-edges
    {
        let mut remove_us = Vec::new();
        for edge in &graph.edges {
            if edge.greater_node() == edge.smaller_node() {
                remove_us.push(edge);
            }
        }
        for edge in remove_us {
            minor.edges.remove(&edge);
        }
    }

    // remove any lone nodes
    for node in &graph.nodes {
        let mut found = false;
        for edge in &graph.edges {
            if edge.greater_node() == node || edge.smaller_node() == node {
                found = true;
                break;
            }
        }
        if !found {
            minor.remove_node(node);
        }
    }
}


fn main() {
    println!("Hello, world!");
}

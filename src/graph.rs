#[derive(Debug, Clone)]
pub struct GraphNode {
    pub value: usize,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Clone)]
pub struct GraphEdge {
    pub cost: usize,
    pub to: usize, // index of next
}

#[derive(Debug, Clone)]
pub struct WeightedGraph(pub Vec<GraphNode>);

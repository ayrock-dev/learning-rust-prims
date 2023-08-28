mod graph;

use graph::*;

fn main() {
    /**
     * (0) -1-> (1)
     *  ^        |
     *  |        1
     *  1        |
     *  |        V
     * (3) <-1- (2)
     */
    let nodes: Vec<GraphNode> = vec![
        GraphNode {
            value: 2,
            edges: vec![GraphEdge { cost: 1, to: 1 }],
        },
        GraphNode {
            value: 3,
            edges: vec![GraphEdge { cost: 1, to: 2 }],
        },
        GraphNode {
            value: 6,
            edges: vec![GraphEdge { cost: 1, to: 3 }],
        },
        GraphNode {
            value: 7,
            edges: vec![GraphEdge { cost: 1, to: 0 }],
        },
    ];

    let graph = WeightedGraph(nodes);

    dbg!(prims(&graph));
}

/**
 * Prims algorithm. Find the minimal spanning tree in a graph starting from the initial node.
 */
fn prims(graph: &WeightedGraph) -> Vec<GraphNode> {
    let mut minimal_spanning_tree: Vec<GraphNode> = vec![];

    let mut visited = vec![false; graph.0.len()];

    let mut i = 0;
    let mut cursor: Option<&GraphNode>;

    let mut cheapest_edge: Option<&GraphEdge> = None;

    println!("--LOOP--");
    while visited[i].eq(&false) {
        println!("Visiting node @ {}", i);
        cursor = graph.0.get(i);

        if let Some(current_node) = cursor {
            println!("Found a node at @ {}", i);
            minimal_spanning_tree.push(current_node.clone());

            for edge in &current_node.edges {
                if let Some(x) = cheapest_edge {
                    if edge.cost < x.cost {
                        cheapest_edge = Some(edge);
                    }
                } else {
                    cheapest_edge = Some(edge);
                }
            }

            visited[i] = true;
            i = cheapest_edge.unwrap().to;
            println!("The cheapest edge points @ {}", i);
            cheapest_edge = None;
        } else {
            println!("There is no node @ {}. Bad edge.", i);
            break;
        }
    }

    println!("The node @ {} was already visited. We're done.", i);

    return minimal_spanning_tree;
}

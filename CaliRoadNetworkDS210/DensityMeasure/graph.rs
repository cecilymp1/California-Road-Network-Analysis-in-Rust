use std::collections::{HashSet, HashMap};


// Calculate the density of the graph based on the number of nodes and edges:
//`nodes` = reference to a set of node identifiers
//`edges` = ref to a vector of tuples representing edges
//returns the density of the graph as a floating-point number
pub fn calculate_density(nodes: &HashSet<i32>, edges: &Vec<(i32, i32)>) -> f64 {
    let num_nodes = nodes.len() as f64; //number of nodes in graph
    let num_edges = edges.len() as f64; //number of edges in graph

    // Max possible number of edges in a graph with num_nodes nodes
    let max_edges = num_nodes * (num_nodes - 1.0) / 2.0;
    num_edges / max_edges // ->density calculation
}

// This function computes the densest subgraph of the input graph:
// `adjacency_map` is a reference to a map where the key is a node and the value is a set of its neighbors
//This returns a set of node identifiers representing the densest subgraph.
pub fn compute_densest_subgraph(adjacency_map: &HashMap<i32, HashSet<i32>>) -> HashSet<i32> {
    let mut densest_subgraph = HashSet::new(); // Will store nodes of densest subgraph found
    let mut densest_density = 0.0; // the density of the densest subgraph found

    // here we are iterating through each node and its neighbors to determine the densest subgraph
    for (&node, neighbors) in adjacency_map {
        let mut subgraph = HashSet::new();
        subgraph.insert(node);
        for &neighbor in neighbors {
            subgraph.insert(neighbor); // this is adding the current node's neighbors to the subgraph set
        }

         // calculate the density of the subgraph formed by the node and its neighbors:
        let subgraph_density = calculate_density(&subgraph, &edges_in_subgraph(&subgraph, &adjacency_map));
        // the below is if the found subgraph has greater density then we are updating the densest values:
        if subgraph_density > densest_density {
            densest_subgraph = subgraph;
            densest_density = subgraph_density;
        }
    }

    densest_subgraph
}
// Here we are generate a vector of edges that are present in the subgraph.
// `subgraph`= reference to a set of node identifiers representing a subgraph.
//`adjacency_map = reference to a map from node identifiers to sets of their neighbors.
/// This function returns a vector of tuples each representing an edge in the subgraph.
pub fn edges_in_subgraph(subgraph: &HashSet<i32>, adjacency_map: &HashMap<i32, HashSet<i32>>) -> Vec<(i32, i32)> {
    let mut edges = Vec::new(); // store edges present in the subgraph
    for &node in subgraph {
        if let Some(neighbors) = adjacency_map.get(&node) {
            // iterting through the neighbors to find connections in the subgraph
            for &neighbor in neighbors {
                if subgraph.contains(&neighbor) {
                    edges.push((node, neighbor)); // pushing in edge if both nodes are in the subgraph
                }
            }
        }
    }
    edges // returning the list of edges in the subgraph
}
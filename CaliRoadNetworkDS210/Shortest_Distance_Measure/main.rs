mod graph_utils;
mod file_io;

use petgraph::graph::NodeIndex;
use graph_utils::{select_city, precompute_city_distances, approximate_shortest_path};
use file_io::load_road_network_from_file;

fn main() {// Path to the input data file containing road information
    let file_path = "california_roads.txt";
    let k = 10; // The number of pivot cities (k) to use in the approximation algorithm
    let alpha = 1.0; // this is a scaling parameter that affects the approximation of the shortest path
    match load_road_network_from_file(file_path) { // Attempt to load the road network from the specified file
        Ok(graph) => { // If the network is successfully loaded, execute the following block
            // using a heuristics function to select 'k' important cities ( the pivot points) in the network
            let cities = select_city(&graph, k); // precompute and store the distances between the selected pivot points
            let city_distances = precompute_city_distances(&graph, &cities); // defining start and end nodes for the pathfinding alg
            let start_node = NodeIndex::new(0);
            let end_node = NodeIndex::new(1);
            // Calculating approximate shortest path between start and end nodes:
            let approx_distance = approximate_shortest_path(start_node, end_node, &city_distances, alpha);
            println!("Approximate distance from start to end: {}", approx_distance);
        },
        Err(e) => {
            println!("Failed to load the road network: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests;

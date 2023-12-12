use crate::io::read_graph_from_file;
use crate::graph::{calculate_density, compute_densest_subgraph, edges_in_subgraph};
use std::collections::HashSet;
use std::io;

pub fn run(filename: &str) -> io::Result<()> {
    let (nodes, edges, adjacency_map) = read_graph_from_file(filename)?;

    // Calculating the graph density
    let density = calculate_density(&nodes, &edges);
    println!("Graph density: {}", density);

    // Computing the densest subgraph
    let densest_subgraph = compute_densest_subgraph(&adjacency_map);

    // Calculating the density of the densest subgraph
    let densest_density = calculate_density(&densest_subgraph, &edges_in_subgraph(&densest_subgraph, &adjacency_map));
    println!("Densest subgraph: {:?}", densest_subgraph);
    println!("Densest subgraph density: {}", densest_density);

    Ok(())
}

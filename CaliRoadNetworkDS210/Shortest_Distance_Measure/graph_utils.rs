use petgraph::visit::EdgeRef; 
use petgraph::algo::dijkstra;
use petgraph::graphmap::UnGraphMap;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use rand::seq::SliceRandom;

// Here we are selecting `k` random cities from the graph that will later be grabbed from the california road network txt file 

pub fn select_city(graph: &UnGraphMap<usize, u32>, k: usize) -> Vec<NodeIndex<usize>> {
    let mut rng = rand::thread_rng(); // rng for shuffling

    // Here we arecollecting all cities (nodes) into a Vec<NodeIndex<usize>>
    let mut cities: Vec<NodeIndex<usize>> = graph.nodes().map(NodeIndex::new).collect();
    cities.shuffle(&mut rng); // this shuffles the cities randomly
    cities.truncate(k); // keeping only the first `k` cities
    cities
}

//This public module function precomputes the distances between each pair of cities:
pub fn precompute_city_distances(
    graph: &UnGraphMap<usize, u32>,
    cities: &[NodeIndex<usize>],
    // hashmap for each city containing the distances to all other cities
) -> HashMap<NodeIndex<usize>, HashMap<NodeIndex<usize>, f64>> {
    cities
        .iter()
        .map(|&city| {
            let distances = dijkstra(graph, city.index(), None, |e| *e.weight() as f64);
            let distances_with_node_indices = distances
                .into_iter()
                // converting the regular node index to NodeIndex for consistency:
                .map(|(node, distance)| (NodeIndex::new(node), distance))
                .collect();
            (city, distances_with_node_indices)
        })
        .collect()
}

// this public mod function approximates the shortest path between `start_node` and `end_node` using precomputed city distances and a multiplier `alpha`
pub fn approximate_shortest_path(
    start_node: NodeIndex<usize>,
    end_node: NodeIndex<usize>,
    city_distances: &HashMap<NodeIndex<usize>, HashMap<NodeIndex<usize>, f64>>,
    alpha: f64,
) -> f64 {
    city_distances
    // Filter map to calculate the sum of distances from start to city and city to end
        .iter()
        .filter_map(|(_, city_dist)| {
            let dist_start = city_dist.get(&start_node)?;
            let dist_end = city_dist.get(&end_node)?;
            Some(dist_start + dist_end)
        })
        .fold(f64::INFINITY, |acc, distance| acc.min(alpha * distance)) // -> finding the minimum distance and apply the alpha multiplier
}
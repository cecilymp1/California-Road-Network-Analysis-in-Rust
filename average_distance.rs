use petgraph::algo::dijkstra;
use petgraph::graphmap::UnGraphMap;
use std::collections::HashSet; 
use rand::Rng; 

pub fn calculate_average_distance(graph: &UnGraphMap<usize, u32>, monte_carlo_iterations: u64, sample_size: usize) -> Option<f64> {
    let mut rng = rand::thread_rng();
        let mut total_distance: u64 = 0;
        let mut count: u64 = 0;
        let nodes: Vec<usize> = graph.nodes().collect();
        let nodes_len = nodes.len();

        if nodes_len < 2 {// Using 2 becvause we need at least two nodes to calculate a distance
            return None; 
        }

        // sampling a fixed number of end nodes for each start node in an iterative manner using a while loop in a for loop
        for _ in 0..monte_carlo_iterations {
            let start_node = nodes[rng.gen_range(0..nodes_len)];
            let mut end_nodes = HashSet::new();
            while end_nodes.len() < sample_size {
                let end_node = nodes[rng.gen_range(0..nodes_len)];
                if start_node != end_node {
                    end_nodes.insert(end_node);
                }
            }

            //utilizing dijkstra algorithm to get the smallest distance
            let paths = dijkstra(graph, start_node, None, |_| 1);

            for &end_node in end_nodes.iter() {
                if let Some(&distance) = paths.get(&end_node) {
                    total_distance += distance as u64;
                    count += 1;
                }
            }
        }

        if count == 0 {
            None // we are trying to avoid division by zero!!!
        } else {
            Some(total_distance as f64 / count as f64) // Compute the average distance.
        }
    }

extern crate petgraph;
use petgraph::graphmap::UnGraphMap;
use petgraph::algo::dijkstra;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod centrality {
    use super::*;

    pub fn calculate_degree_centrality(graph: &UnGraphMap<usize, u32>) -> HashMap<usize, usize> {
        let mut degree_counts = HashMap::new();
        for node in graph.nodes() {
            degree_counts.insert(node, graph.neighbors(node).count());
        }
        degree_counts
    }

    // Finds the shortest path between two nodes using Dijkstra's algorithm
    // We are running Dijkstra's algorithm for the specified start and goal
    pub fn shortest_path(graph: &UnGraphMap<usize, u32>, start: usize, goal: usize) -> Option<Vec<usize>> {
        let node_map = dijkstra(graph, start, Some(goal), |_| 1);
        let mut path = vec![];
        let mut current = goal;

        // here we are back tracking from goal to start using the `node_map` to recreate the path
        while let Some(&prev) = node_map.get(&current) {
            path.push(current);
            if current == start {
                break;
            }
            current = prev;
        }

        // If the path includes the start, it's valid; otherwise return None
        if path.last() == Some(&start) {
            path.reverse();
            Some(path)
        } else {
            None
        }
    }
}

mod network {
    use super::*;
    // loading in a file and creating an undirected graph
    pub fn load_road_network_from_file(file_path: &str) -> io::Result<UnGraphMap<usize, u32>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut graph = UnGraphMap::new();

        for line in reader.lines() {
            let line = line?;

            if line.starts_with('#') {
                continue;
            }
            // In this rendering of the loading the network file we are splitting the line into node identifiers and adding them as edges
            let nodes: Vec<_> = line
                .split_whitespace()
                .map(|s| s.parse::<usize>())
                .filter_map(Result::ok)
                .collect();

            if nodes.len() == 2 {
                graph.add_edge(nodes[0], nodes[1], 1);
            }
        }

        Ok(graph)
    }
}

mod utils {
    use super::*;
    /// calculating the frequency of each degree in the graph
    pub fn degree_frequency(centrality_map: HashMap<usize, usize>) -> HashMap<usize, usize> {
        let mut frequency_map = HashMap::new();

        // counting the frequency of each degree
        for &degree in centrality_map.values() {
            *frequency_map.entry(degree).or_insert(0) += 1;
        }
        frequency_map
    }
}

fn main() -> io::Result<()> {
    let graph = network::load_road_network_from_file("california_roads.txt")?;

    let degree_centrality_map = centrality::calculate_degree_centrality(&graph);
    let connections_frequency = utils::degree_frequency(degree_centrality_map);

    for (connections, count) in connections_frequency.iter() {
        println!("Number of nodes with {} connections: {}", connections, count);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graphmap::UnGraphMap;

    #[test]
    // making a simple graph and testing the degree centrality computation
    fn test_calculate_degree_centrality() {
        let mut graph = UnGraphMap::<usize, u32>::new();
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(3, 4, 1);

        let degree_centrality = centrality::calculate_degree_centrality(&graph);
        assert_eq!(degree_centrality[&1], 1);
        assert_eq!(degree_centrality[&2], 2);
        assert_eq!(degree_centrality[&3], 2);
        assert_eq!(degree_centrality[&4], 1);
    }

    #[test]
    //linear graph and testing the shortest path function
    fn test_shortest_path() {
        let mut graph = UnGraphMap::<usize, u32>::new();
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(3, 4, 1);

        let path = centrality::shortest_path(&graph, 1, 4);
        assert_eq!(path, Some(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_degree_frequency() {
        let mut centrality_map = HashMap::new();
        centrality_map.insert(1, 2); // node 1 has a degree of 2
        centrality_map.insert(2, 2); 
        centrality_map.insert(3, 3); 

        let frequency = utils::degree_frequency(centrality_map);

        assert_eq!(frequency[&2], 2);
        assert_eq!(frequency[&3], 1);
        assert_eq!(frequency.get(&1), None);
    }

    fn create_test_graph() -> UnGraphMap<usize, u32> {
        let mut graph = UnGraphMap::new();
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(3, 4, 1);
        graph.add_edge(4, 5, 1);
        graph.add_edge(5, 6, 1);
        graph.add_edge(6, 1, 1);
        graph.add_edge(2, 5, 1);
        graph
    }
}